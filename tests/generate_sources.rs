use std::{
    collections::HashMap,
    env, fs,
    io::{self},
    path::{Path, PathBuf},
};

use anyhow::Context;
use prost::Message;
use prost_build::Module;
use prost_types::FileDescriptorSet;
use serde::Deserialize;
use xshell::{cmd, Shell};

/// Generate sources and check that the generated changes have been committed.
///
/// If you are working on this repo, you can set `XDS_API_SKIP_GEN_SRC` to skip
/// generating sources on every `cargo test` and
/// `XDS_API_SKIP_GEN_SRC_DIRTY_CHECK` to skip the dirty check.
///
/// These flags are opt-out instead of opt-in so the checks run in CI and fail
/// on pushes that get protos and generated sources out-of-sync.
#[test]
fn generate_sources() -> anyhow::Result<()> {
    let sh = Shell::new().unwrap();
    if env::var("XDS_API_SKIP_GEN_SRC").is_err() {
        generate_xds_api(&sh)?;
    }

    if env::var("XDS_API_SKIP_GEN_SRC_DIRTY_CHECK").is_err() {
        check_dirty_repo(&sh)?;
    }

    Ok(())
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Generate the xds-api definitions by downloading protobuf dependencies and running
/// tonic-build.
///
/// This depends on:
///
/// - The dependencies declared in xds-api/proto-deps.toml
///
/// - Prior knowledge of which XDS APIs we care about (they're hardcoded into
///   the compile_xds_api function, go look there for details).
///
/// - Having access to the internet to pull dependencies.
///
/// - A local install of protoc, found either by using the executable named
///   `protoc` in your path or defined with the PROTOC environment variable.
///   See https://docs.rs/prost-build/latest/prost_build/#sourcing-protoc for
///   all of the gory detail.
///
/// - A local install of git.
///
/// Generation defaults to using a cache directory under the project's `target/`
/// dir. To make this cache survive cargo clean, or to force a fresh directory
/// every time you build, set the `XDS_GIT_CACHE` env variable to a new path.
fn generate_xds_api(sh: &Shell) -> anyhow::Result<()> {
    let project_root = project_root();
    let proto_deps = envoy_deps(&project_root)?;

    eprintln!("### Checking protoc version...");
    let protoc_version = check_protoc_version(sh)?;
    let expected_version = read_protoc_version(&project_root)?;
    if protoc_version != expected_version {
        anyhow::bail!(
            "protoc version ({protoc_version}) doesn't match pinned version ({expected_version})"
        )
    }

    eprintln!("### Collecting protobuf dependencies...");

    let git_dir = env::var("XDS_GIT_CACHE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| project_root.join("target/generate-xds-api-dev"));
    match fs::create_dir(&git_dir) {
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => (),
        r => r?,
    };

    sync_envoy_protos(sh, &git_dir, &proto_deps)?;
    eprintln!("### Generating code...");
    compile_xds_api(sh, &project_root, &git_dir, &proto_deps)?;

    eprintln!("### Okay!");
    Ok(())
}

fn check_dirty_repo(sh: &Shell) -> anyhow::Result<()> {
    let git_status = cmd!(sh, "git status --porcelain").read()?;
    if !git_status.is_empty() {
        anyhow::bail!(
            "Uncomitted git changes found. Make sure you commit protobufs before pushing."
        )
    }
    Ok(())
}

fn sync_envoy_protos<P: AsRef<Path>>(
    sh: &Shell,
    working_dir: P,
    protos: &ProtoDeps,
) -> anyhow::Result<()> {
    for (name, info) in &protos.dependencies {
        let target_dir = working_dir.as_ref().join(name);
        let repo_url = &info.repo_url;
        if !target_dir.exists() {
            cmd!(
                sh,
                "git clone --depth 1 --filter=blob:none {repo_url} {target_dir}"
            )
            .run()?;
        }

        let _dir = sh.push_dir(target_dir);
        let commit_or_branch = &info.git_ref;
        cmd!(sh, "git fetch --tags").run()?;
        cmd!(sh, "git checkout {commit_or_branch}").run()?;
    }

    Ok(())
}

fn compile_xds_api<P: AsRef<Path>, Q: AsRef<Path>>(
    sh: &Shell,
    project_root: P,
    build_root: Q,
    proto_deps: &ProtoDeps,
) -> anyhow::Result<()> {
    let build_root = build_root.as_ref();

    let proto_paths = glob_protos(
        &build_root.join("envoy-data-plane"),
        &[
            "envoy/type/**/v3/*.proto",
            "envoy/config/**/v3/*.proto",
            "envoy/service/**/v3/*.proto",
            "envoy/extensions/filters/network/http_connection_manager/v3/*.proto",
            "envoy/extensions/filters/http/router/v3/*.proto",
            "envoy/extensions/clusters/aggregate/v3/*.proto",
        ],
    );

    let includes: Vec<_> = proto_deps
        .dependencies
        .iter()
        .map(|(name, dep_info)| {
            let include = build_root.join(name);
            match &dep_info.proto_root {
                Some(proto_root) => include.join(proto_root),
                None => include,
            }
        })
        .collect();

    let out_dir = &project_root.as_ref().join("src/generated");
    cmd!(sh, "rm -r {out_dir}").run()?;
    cmd!(sh, "mkdir -p {out_dir}").run()?;

    let descriptor_path = project_root.as_ref().join("src/xds-descriptors.bin");

    let mut prost_config = prost_build::Config::new();
    prost_config.enable_type_names();
    prost_config.type_name_domain([".envoy"], "type.googleapis.com");

    eprintln!("+ tonic_build");
    tonic_build::configure()
        .out_dir(out_dir)
        .file_descriptor_set_path(&descriptor_path)
        .emit_rerun_if_changed(false)
        .include_file("mod.rs")
        .compile_well_known_types(true)
        .compile_protos_with_config(prost_config, &proto_paths, &includes)?;

    eprintln!("+ pbjson_build");
    let encoded_descriptor_set = std::fs::read(&descriptor_path).unwrap();
    pbjson_build::Builder::new()
        .register_descriptors(&encoded_descriptor_set)?
        .preserve_proto_field_names()
        .exclude([".google.protobuf.Any"])
        .out_dir(out_dir)
        .build(&["."])?;

    eprintln!("+ regenerate includes with serde impls");
    let fds = FileDescriptorSet::decode(encoded_descriptor_set.as_slice())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    generate_includes(project_root.as_ref().join("src/generated/mod.rs"), fds)?;

    Ok(())
}

fn envoy_deps<P: AsRef<Path>>(project_root: P) -> anyhow::Result<ProtoDeps> {
    let deps_file = project_root.as_ref().join("protobufs.toml");
    let deps = toml::from_str(
        &fs::read_to_string(&deps_file)
            .with_context(|| format!("failed to load deps file: {}", deps_file.display()))?,
    )?;
    Ok(deps)
}

#[derive(Debug, Deserialize)]
struct ProtoDeps {
    dependencies: HashMap<String, ProtoDep>,
}

#[derive(Debug, Deserialize)]
struct ProtoDep {
    repo_url: String,
    git_ref: String,
    proto_root: Option<String>,
}

fn glob_protos<P: AsRef<Path>>(root: &P, globs: &[&'static str]) -> Vec<PathBuf> {
    let root = root.as_ref();

    let mut paths = vec![];

    for g in globs {
        let proto_glob = root.join(g).to_string_lossy().into_owned();
        let globbed = glob::glob(&proto_glob)
            .expect("invalid glob pattern")
            .filter_map(Result::ok);

        paths.extend(globbed);
    }

    paths
}

fn generate_includes<P: AsRef<Path>>(target_path: P, fds: FileDescriptorSet) -> io::Result<()> {
    let modules = {
        let mut modules: Vec<_> = fds
            .file
            .into_iter()
            .map(|descriptor| Module::from_protobuf_package_name(descriptor.package()))
            .collect();
        modules.sort();
        modules.dedup();
        modules
    };

    let file_names: HashMap<_, _> = modules
        .iter()
        .map(|module| (module.clone(), module.to_file_name_or("_")))
        .collect();

    let mut buf = Vec::new();

    write_line(
        &mut buf,
        0,
        "// This file is generated by xds-api/xtask. Do not edit!",
    )?;

    let mut stack = Vec::<String>::new();
    for module in modules {
        let module_parts: Vec<_> = module.parts().collect();

        while !module_starts_with(&module_parts, &stack) {
            stack.pop();
            write_line(&mut buf, stack.len(), "}")?;
        }
        while stack.len() < module.len() {
            write_line(
                &mut buf,
                stack.len(),
                &format!("pub mod {} {{", module_parts[stack.len()]),
            )?;
            stack.push(module_parts[stack.len()].to_owned())
        }

        let file_name = file_names
            .get(&module)
            .expect("missing filename for module");

        write_line(
            &mut buf,
            stack.len(),
            &format!("include!(\"{file_name}\");"),
        )?;

        if let Some(serde_filename) = module_serde_filename(&module) {
            write_line(&mut buf, stack.len(), "#[cfg(feature = \"pbjson\")]")?;
            write_line(
                &mut buf,
                stack.len(),
                &format!("include!(\"{serde_filename}\");"),
            )?;
        }
    }

    for depth in (0..stack.len()).rev() {
        write_line(&mut buf, depth, "}")?;
    }

    fs::write(target_path.as_ref(), &buf).unwrap();

    Ok(())
}

fn module_starts_with<T: AsRef<str>>(haystack: &[&str], needle: &[T]) -> bool {
    haystack
        .iter()
        .zip(needle.iter())
        .all(|(a, b)| *a == b.as_ref())
}

fn module_serde_filename(m: &Module) -> Option<String> {
    let mut parts: Vec<_> = m.parts().map(|s| s.to_string()).collect();

    if parts.is_empty() {
        return None;
    }

    parts.push("serde".to_string());
    parts.push("rs".to_string());

    Some(parts.join("."))
}

fn write_line<W: io::Write>(w: &mut W, depth: usize, line: &str) -> io::Result<()> {
    let line = format!(
        "{spacing}{line}\n",
        spacing = "    ".to_owned().repeat(depth),
        line = line,
    );
    w.write_all(line.as_bytes())
}

fn check_protoc_version(sh: &Shell) -> anyhow::Result<String> {
    let protoc_path = env::var("PROTOC").unwrap_or_else(|_| "protoc".to_string());
    let output = cmd!(sh, "{protoc_path} --version").read()?;
    let Some(version) = output.split_ascii_whitespace().last() else {
        anyhow::bail!("oops: couldn't parse protoc version");
    };
    Ok(version.to_string())
}

fn read_protoc_version(root_dir: &Path) -> io::Result<String> {
    let version = std::fs::read_to_string(root_dir.join(".protoc-version"))?;
    Ok(version.trim().to_string())
}
