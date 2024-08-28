// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileStatusAnnotation {
    /// The entity is work-in-progress and subject to breaking changes.
    #[prost(bool, tag = "1")]
    pub work_in_progress: bool,
}
impl ::prost::Name for FileStatusAnnotation {
    const NAME: &'static str = "FileStatusAnnotation";
    const PACKAGE: &'static str = "xds.annotations.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "xds.annotations.v3.FileStatusAnnotation".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/xds.annotations.v3.FileStatusAnnotation".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageStatusAnnotation {
    /// The entity is work-in-progress and subject to breaking changes.
    #[prost(bool, tag = "1")]
    pub work_in_progress: bool,
}
impl ::prost::Name for MessageStatusAnnotation {
    const NAME: &'static str = "MessageStatusAnnotation";
    const PACKAGE: &'static str = "xds.annotations.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "xds.annotations.v3.MessageStatusAnnotation".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/xds.annotations.v3.MessageStatusAnnotation".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldStatusAnnotation {
    /// The entity is work-in-progress and subject to breaking changes.
    #[prost(bool, tag = "1")]
    pub work_in_progress: bool,
}
impl ::prost::Name for FieldStatusAnnotation {
    const NAME: &'static str = "FieldStatusAnnotation";
    const PACKAGE: &'static str = "xds.annotations.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "xds.annotations.v3.FieldStatusAnnotation".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/xds.annotations.v3.FieldStatusAnnotation".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusAnnotation {
    /// The entity is work-in-progress and subject to breaking changes.
    #[prost(bool, tag = "1")]
    pub work_in_progress: bool,
    /// The entity belongs to a package with the given version status.
    #[prost(enumeration = "PackageVersionStatus", tag = "2")]
    pub package_version_status: i32,
}
impl ::prost::Name for StatusAnnotation {
    const NAME: &'static str = "StatusAnnotation";
    const PACKAGE: &'static str = "xds.annotations.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "xds.annotations.v3.StatusAnnotation".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/xds.annotations.v3.StatusAnnotation".into()
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PackageVersionStatus {
    /// Unknown package version status.
    Unknown = 0,
    /// This version of the package is frozen.
    Frozen = 1,
    /// This version of the package is the active development version.
    Active = 2,
    /// This version of the package is the candidate for the next major version. It
    /// is typically machine generated from the active development version.
    NextMajorVersionCandidate = 3,
}
impl PackageVersionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PackageVersionStatus::Unknown => "UNKNOWN",
            PackageVersionStatus::Frozen => "FROZEN",
            PackageVersionStatus::Active => "ACTIVE",
            PackageVersionStatus::NextMajorVersionCandidate => {
                "NEXT_MAJOR_VERSION_CANDIDATE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "FROZEN" => Some(Self::Frozen),
            "ACTIVE" => Some(Self::Active),
            "NEXT_MAJOR_VERSION_CANDIDATE" => Some(Self::NextMajorVersionCandidate),
            _ => None,
        }
    }
}