impl serde::Serialize for Admin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.access_log.is_empty() {
            len += 1;
        }
        if !self.access_log_path.is_empty() {
            len += 1;
        }
        if !self.profile_path.is_empty() {
            len += 1;
        }
        if self.address.is_some() {
            len += 1;
        }
        if !self.socket_options.is_empty() {
            len += 1;
        }
        if self.ignore_global_conn_limit {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.Admin", len)?;
        if !self.access_log.is_empty() {
            struct_ser.serialize_field("access_log", &self.access_log)?;
        }
        if !self.access_log_path.is_empty() {
            struct_ser.serialize_field("access_log_path", &self.access_log_path)?;
        }
        if !self.profile_path.is_empty() {
            struct_ser.serialize_field("profile_path", &self.profile_path)?;
        }
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if !self.socket_options.is_empty() {
            struct_ser.serialize_field("socket_options", &self.socket_options)?;
        }
        if self.ignore_global_conn_limit {
            struct_ser.serialize_field("ignore_global_conn_limit", &self.ignore_global_conn_limit)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Admin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "access_log",
            "accessLog",
            "access_log_path",
            "accessLogPath",
            "profile_path",
            "profilePath",
            "address",
            "socket_options",
            "socketOptions",
            "ignore_global_conn_limit",
            "ignoreGlobalConnLimit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccessLog,
            AccessLogPath,
            ProfilePath,
            Address,
            SocketOptions,
            IgnoreGlobalConnLimit,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "accessLog" | "access_log" => Ok(GeneratedField::AccessLog),
                            "accessLogPath" | "access_log_path" => Ok(GeneratedField::AccessLogPath),
                            "profilePath" | "profile_path" => Ok(GeneratedField::ProfilePath),
                            "address" => Ok(GeneratedField::Address),
                            "socketOptions" | "socket_options" => Ok(GeneratedField::SocketOptions),
                            "ignoreGlobalConnLimit" | "ignore_global_conn_limit" => Ok(GeneratedField::IgnoreGlobalConnLimit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Admin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.Admin")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Admin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut access_log__ = None;
                let mut access_log_path__ = None;
                let mut profile_path__ = None;
                let mut address__ = None;
                let mut socket_options__ = None;
                let mut ignore_global_conn_limit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccessLog => {
                            if access_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessLog"));
                            }
                            access_log__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AccessLogPath => {
                            if access_log_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessLogPath"));
                            }
                            access_log_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProfilePath => {
                            if profile_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("profilePath"));
                            }
                            profile_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                        GeneratedField::SocketOptions => {
                            if socket_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("socketOptions"));
                            }
                            socket_options__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IgnoreGlobalConnLimit => {
                            if ignore_global_conn_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreGlobalConnLimit"));
                            }
                            ignore_global_conn_limit__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Admin {
                    access_log: access_log__.unwrap_or_default(),
                    access_log_path: access_log_path__.unwrap_or_default(),
                    profile_path: profile_path__.unwrap_or_default(),
                    address: address__,
                    socket_options: socket_options__.unwrap_or_default(),
                    ignore_global_conn_limit: ignore_global_conn_limit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.Admin", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Bootstrap {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.node.is_some() {
            len += 1;
        }
        if !self.node_context_params.is_empty() {
            len += 1;
        }
        if self.static_resources.is_some() {
            len += 1;
        }
        if self.dynamic_resources.is_some() {
            len += 1;
        }
        if self.cluster_manager.is_some() {
            len += 1;
        }
        if self.hds_config.is_some() {
            len += 1;
        }
        if !self.flags_path.is_empty() {
            len += 1;
        }
        if !self.stats_sinks.is_empty() {
            len += 1;
        }
        if self.deferred_stat_options.is_some() {
            len += 1;
        }
        if self.stats_config.is_some() {
            len += 1;
        }
        if self.stats_flush_interval.is_some() {
            len += 1;
        }
        if self.watchdog.is_some() {
            len += 1;
        }
        if self.watchdogs.is_some() {
            len += 1;
        }
        if self.tracing.is_some() {
            len += 1;
        }
        if self.layered_runtime.is_some() {
            len += 1;
        }
        if self.admin.is_some() {
            len += 1;
        }
        if self.overload_manager.is_some() {
            len += 1;
        }
        if self.enable_dispatcher_stats {
            len += 1;
        }
        if !self.header_prefix.is_empty() {
            len += 1;
        }
        if self.stats_server_version_override.is_some() {
            len += 1;
        }
        if self.use_tcp_for_dns_lookups {
            len += 1;
        }
        if self.dns_resolution_config.is_some() {
            len += 1;
        }
        if self.typed_dns_resolver_config.is_some() {
            len += 1;
        }
        if !self.bootstrap_extensions.is_empty() {
            len += 1;
        }
        if !self.fatal_actions.is_empty() {
            len += 1;
        }
        if !self.config_sources.is_empty() {
            len += 1;
        }
        if self.default_config_source.is_some() {
            len += 1;
        }
        if !self.default_socket_interface.is_empty() {
            len += 1;
        }
        if !self.certificate_provider_instances.is_empty() {
            len += 1;
        }
        if !self.inline_headers.is_empty() {
            len += 1;
        }
        if !self.perf_tracing_file_path.is_empty() {
            len += 1;
        }
        if self.default_regex_engine.is_some() {
            len += 1;
        }
        if self.xds_delegate_extension.is_some() {
            len += 1;
        }
        if self.xds_config_tracker_extension.is_some() {
            len += 1;
        }
        if self.listener_manager.is_some() {
            len += 1;
        }
        if self.application_log_config.is_some() {
            len += 1;
        }
        if self.grpc_async_client_manager_config.is_some() {
            len += 1;
        }
        if self.memory_allocator_manager.is_some() {
            len += 1;
        }
        if self.stats_flush.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.Bootstrap", len)?;
        if let Some(v) = self.node.as_ref() {
            struct_ser.serialize_field("node", v)?;
        }
        if !self.node_context_params.is_empty() {
            struct_ser.serialize_field("node_context_params", &self.node_context_params)?;
        }
        if let Some(v) = self.static_resources.as_ref() {
            struct_ser.serialize_field("static_resources", v)?;
        }
        if let Some(v) = self.dynamic_resources.as_ref() {
            struct_ser.serialize_field("dynamic_resources", v)?;
        }
        if let Some(v) = self.cluster_manager.as_ref() {
            struct_ser.serialize_field("cluster_manager", v)?;
        }
        if let Some(v) = self.hds_config.as_ref() {
            struct_ser.serialize_field("hds_config", v)?;
        }
        if !self.flags_path.is_empty() {
            struct_ser.serialize_field("flags_path", &self.flags_path)?;
        }
        if !self.stats_sinks.is_empty() {
            struct_ser.serialize_field("stats_sinks", &self.stats_sinks)?;
        }
        if let Some(v) = self.deferred_stat_options.as_ref() {
            struct_ser.serialize_field("deferred_stat_options", v)?;
        }
        if let Some(v) = self.stats_config.as_ref() {
            struct_ser.serialize_field("stats_config", v)?;
        }
        if let Some(v) = self.stats_flush_interval.as_ref() {
            struct_ser.serialize_field("stats_flush_interval", v)?;
        }
        if let Some(v) = self.watchdog.as_ref() {
            struct_ser.serialize_field("watchdog", v)?;
        }
        if let Some(v) = self.watchdogs.as_ref() {
            struct_ser.serialize_field("watchdogs", v)?;
        }
        if let Some(v) = self.tracing.as_ref() {
            struct_ser.serialize_field("tracing", v)?;
        }
        if let Some(v) = self.layered_runtime.as_ref() {
            struct_ser.serialize_field("layered_runtime", v)?;
        }
        if let Some(v) = self.admin.as_ref() {
            struct_ser.serialize_field("admin", v)?;
        }
        if let Some(v) = self.overload_manager.as_ref() {
            struct_ser.serialize_field("overload_manager", v)?;
        }
        if self.enable_dispatcher_stats {
            struct_ser.serialize_field("enable_dispatcher_stats", &self.enable_dispatcher_stats)?;
        }
        if !self.header_prefix.is_empty() {
            struct_ser.serialize_field("header_prefix", &self.header_prefix)?;
        }
        if let Some(v) = self.stats_server_version_override.as_ref() {
            struct_ser.serialize_field("stats_server_version_override", v)?;
        }
        if self.use_tcp_for_dns_lookups {
            struct_ser.serialize_field("use_tcp_for_dns_lookups", &self.use_tcp_for_dns_lookups)?;
        }
        if let Some(v) = self.dns_resolution_config.as_ref() {
            struct_ser.serialize_field("dns_resolution_config", v)?;
        }
        if let Some(v) = self.typed_dns_resolver_config.as_ref() {
            struct_ser.serialize_field("typed_dns_resolver_config", v)?;
        }
        if !self.bootstrap_extensions.is_empty() {
            struct_ser.serialize_field("bootstrap_extensions", &self.bootstrap_extensions)?;
        }
        if !self.fatal_actions.is_empty() {
            struct_ser.serialize_field("fatal_actions", &self.fatal_actions)?;
        }
        if !self.config_sources.is_empty() {
            struct_ser.serialize_field("config_sources", &self.config_sources)?;
        }
        if let Some(v) = self.default_config_source.as_ref() {
            struct_ser.serialize_field("default_config_source", v)?;
        }
        if !self.default_socket_interface.is_empty() {
            struct_ser.serialize_field("default_socket_interface", &self.default_socket_interface)?;
        }
        if !self.certificate_provider_instances.is_empty() {
            struct_ser.serialize_field("certificate_provider_instances", &self.certificate_provider_instances)?;
        }
        if !self.inline_headers.is_empty() {
            struct_ser.serialize_field("inline_headers", &self.inline_headers)?;
        }
        if !self.perf_tracing_file_path.is_empty() {
            struct_ser.serialize_field("perf_tracing_file_path", &self.perf_tracing_file_path)?;
        }
        if let Some(v) = self.default_regex_engine.as_ref() {
            struct_ser.serialize_field("default_regex_engine", v)?;
        }
        if let Some(v) = self.xds_delegate_extension.as_ref() {
            struct_ser.serialize_field("xds_delegate_extension", v)?;
        }
        if let Some(v) = self.xds_config_tracker_extension.as_ref() {
            struct_ser.serialize_field("xds_config_tracker_extension", v)?;
        }
        if let Some(v) = self.listener_manager.as_ref() {
            struct_ser.serialize_field("listener_manager", v)?;
        }
        if let Some(v) = self.application_log_config.as_ref() {
            struct_ser.serialize_field("application_log_config", v)?;
        }
        if let Some(v) = self.grpc_async_client_manager_config.as_ref() {
            struct_ser.serialize_field("grpc_async_client_manager_config", v)?;
        }
        if let Some(v) = self.memory_allocator_manager.as_ref() {
            struct_ser.serialize_field("memory_allocator_manager", v)?;
        }
        if let Some(v) = self.stats_flush.as_ref() {
            match v {
                bootstrap::StatsFlush::StatsFlushOnAdmin(v) => {
                    struct_ser.serialize_field("stats_flush_on_admin", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Bootstrap {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "node",
            "node_context_params",
            "nodeContextParams",
            "static_resources",
            "staticResources",
            "dynamic_resources",
            "dynamicResources",
            "cluster_manager",
            "clusterManager",
            "hds_config",
            "hdsConfig",
            "flags_path",
            "flagsPath",
            "stats_sinks",
            "statsSinks",
            "deferred_stat_options",
            "deferredStatOptions",
            "stats_config",
            "statsConfig",
            "stats_flush_interval",
            "statsFlushInterval",
            "watchdog",
            "watchdogs",
            "tracing",
            "layered_runtime",
            "layeredRuntime",
            "admin",
            "overload_manager",
            "overloadManager",
            "enable_dispatcher_stats",
            "enableDispatcherStats",
            "header_prefix",
            "headerPrefix",
            "stats_server_version_override",
            "statsServerVersionOverride",
            "use_tcp_for_dns_lookups",
            "useTcpForDnsLookups",
            "dns_resolution_config",
            "dnsResolutionConfig",
            "typed_dns_resolver_config",
            "typedDnsResolverConfig",
            "bootstrap_extensions",
            "bootstrapExtensions",
            "fatal_actions",
            "fatalActions",
            "config_sources",
            "configSources",
            "default_config_source",
            "defaultConfigSource",
            "default_socket_interface",
            "defaultSocketInterface",
            "certificate_provider_instances",
            "certificateProviderInstances",
            "inline_headers",
            "inlineHeaders",
            "perf_tracing_file_path",
            "perfTracingFilePath",
            "default_regex_engine",
            "defaultRegexEngine",
            "xds_delegate_extension",
            "xdsDelegateExtension",
            "xds_config_tracker_extension",
            "xdsConfigTrackerExtension",
            "listener_manager",
            "listenerManager",
            "application_log_config",
            "applicationLogConfig",
            "grpc_async_client_manager_config",
            "grpcAsyncClientManagerConfig",
            "memory_allocator_manager",
            "memoryAllocatorManager",
            "stats_flush_on_admin",
            "statsFlushOnAdmin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Node,
            NodeContextParams,
            StaticResources,
            DynamicResources,
            ClusterManager,
            HdsConfig,
            FlagsPath,
            StatsSinks,
            DeferredStatOptions,
            StatsConfig,
            StatsFlushInterval,
            Watchdog,
            Watchdogs,
            Tracing,
            LayeredRuntime,
            Admin,
            OverloadManager,
            EnableDispatcherStats,
            HeaderPrefix,
            StatsServerVersionOverride,
            UseTcpForDnsLookups,
            DnsResolutionConfig,
            TypedDnsResolverConfig,
            BootstrapExtensions,
            FatalActions,
            ConfigSources,
            DefaultConfigSource,
            DefaultSocketInterface,
            CertificateProviderInstances,
            InlineHeaders,
            PerfTracingFilePath,
            DefaultRegexEngine,
            XdsDelegateExtension,
            XdsConfigTrackerExtension,
            ListenerManager,
            ApplicationLogConfig,
            GrpcAsyncClientManagerConfig,
            MemoryAllocatorManager,
            StatsFlushOnAdmin,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "node" => Ok(GeneratedField::Node),
                            "nodeContextParams" | "node_context_params" => Ok(GeneratedField::NodeContextParams),
                            "staticResources" | "static_resources" => Ok(GeneratedField::StaticResources),
                            "dynamicResources" | "dynamic_resources" => Ok(GeneratedField::DynamicResources),
                            "clusterManager" | "cluster_manager" => Ok(GeneratedField::ClusterManager),
                            "hdsConfig" | "hds_config" => Ok(GeneratedField::HdsConfig),
                            "flagsPath" | "flags_path" => Ok(GeneratedField::FlagsPath),
                            "statsSinks" | "stats_sinks" => Ok(GeneratedField::StatsSinks),
                            "deferredStatOptions" | "deferred_stat_options" => Ok(GeneratedField::DeferredStatOptions),
                            "statsConfig" | "stats_config" => Ok(GeneratedField::StatsConfig),
                            "statsFlushInterval" | "stats_flush_interval" => Ok(GeneratedField::StatsFlushInterval),
                            "watchdog" => Ok(GeneratedField::Watchdog),
                            "watchdogs" => Ok(GeneratedField::Watchdogs),
                            "tracing" => Ok(GeneratedField::Tracing),
                            "layeredRuntime" | "layered_runtime" => Ok(GeneratedField::LayeredRuntime),
                            "admin" => Ok(GeneratedField::Admin),
                            "overloadManager" | "overload_manager" => Ok(GeneratedField::OverloadManager),
                            "enableDispatcherStats" | "enable_dispatcher_stats" => Ok(GeneratedField::EnableDispatcherStats),
                            "headerPrefix" | "header_prefix" => Ok(GeneratedField::HeaderPrefix),
                            "statsServerVersionOverride" | "stats_server_version_override" => Ok(GeneratedField::StatsServerVersionOverride),
                            "useTcpForDnsLookups" | "use_tcp_for_dns_lookups" => Ok(GeneratedField::UseTcpForDnsLookups),
                            "dnsResolutionConfig" | "dns_resolution_config" => Ok(GeneratedField::DnsResolutionConfig),
                            "typedDnsResolverConfig" | "typed_dns_resolver_config" => Ok(GeneratedField::TypedDnsResolverConfig),
                            "bootstrapExtensions" | "bootstrap_extensions" => Ok(GeneratedField::BootstrapExtensions),
                            "fatalActions" | "fatal_actions" => Ok(GeneratedField::FatalActions),
                            "configSources" | "config_sources" => Ok(GeneratedField::ConfigSources),
                            "defaultConfigSource" | "default_config_source" => Ok(GeneratedField::DefaultConfigSource),
                            "defaultSocketInterface" | "default_socket_interface" => Ok(GeneratedField::DefaultSocketInterface),
                            "certificateProviderInstances" | "certificate_provider_instances" => Ok(GeneratedField::CertificateProviderInstances),
                            "inlineHeaders" | "inline_headers" => Ok(GeneratedField::InlineHeaders),
                            "perfTracingFilePath" | "perf_tracing_file_path" => Ok(GeneratedField::PerfTracingFilePath),
                            "defaultRegexEngine" | "default_regex_engine" => Ok(GeneratedField::DefaultRegexEngine),
                            "xdsDelegateExtension" | "xds_delegate_extension" => Ok(GeneratedField::XdsDelegateExtension),
                            "xdsConfigTrackerExtension" | "xds_config_tracker_extension" => Ok(GeneratedField::XdsConfigTrackerExtension),
                            "listenerManager" | "listener_manager" => Ok(GeneratedField::ListenerManager),
                            "applicationLogConfig" | "application_log_config" => Ok(GeneratedField::ApplicationLogConfig),
                            "grpcAsyncClientManagerConfig" | "grpc_async_client_manager_config" => Ok(GeneratedField::GrpcAsyncClientManagerConfig),
                            "memoryAllocatorManager" | "memory_allocator_manager" => Ok(GeneratedField::MemoryAllocatorManager),
                            "statsFlushOnAdmin" | "stats_flush_on_admin" => Ok(GeneratedField::StatsFlushOnAdmin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Bootstrap;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.Bootstrap")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Bootstrap, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut node__ = None;
                let mut node_context_params__ = None;
                let mut static_resources__ = None;
                let mut dynamic_resources__ = None;
                let mut cluster_manager__ = None;
                let mut hds_config__ = None;
                let mut flags_path__ = None;
                let mut stats_sinks__ = None;
                let mut deferred_stat_options__ = None;
                let mut stats_config__ = None;
                let mut stats_flush_interval__ = None;
                let mut watchdog__ = None;
                let mut watchdogs__ = None;
                let mut tracing__ = None;
                let mut layered_runtime__ = None;
                let mut admin__ = None;
                let mut overload_manager__ = None;
                let mut enable_dispatcher_stats__ = None;
                let mut header_prefix__ = None;
                let mut stats_server_version_override__ = None;
                let mut use_tcp_for_dns_lookups__ = None;
                let mut dns_resolution_config__ = None;
                let mut typed_dns_resolver_config__ = None;
                let mut bootstrap_extensions__ = None;
                let mut fatal_actions__ = None;
                let mut config_sources__ = None;
                let mut default_config_source__ = None;
                let mut default_socket_interface__ = None;
                let mut certificate_provider_instances__ = None;
                let mut inline_headers__ = None;
                let mut perf_tracing_file_path__ = None;
                let mut default_regex_engine__ = None;
                let mut xds_delegate_extension__ = None;
                let mut xds_config_tracker_extension__ = None;
                let mut listener_manager__ = None;
                let mut application_log_config__ = None;
                let mut grpc_async_client_manager_config__ = None;
                let mut memory_allocator_manager__ = None;
                let mut stats_flush__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = map_.next_value()?;
                        }
                        GeneratedField::NodeContextParams => {
                            if node_context_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeContextParams"));
                            }
                            node_context_params__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StaticResources => {
                            if static_resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticResources"));
                            }
                            static_resources__ = map_.next_value()?;
                        }
                        GeneratedField::DynamicResources => {
                            if dynamic_resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicResources"));
                            }
                            dynamic_resources__ = map_.next_value()?;
                        }
                        GeneratedField::ClusterManager => {
                            if cluster_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterManager"));
                            }
                            cluster_manager__ = map_.next_value()?;
                        }
                        GeneratedField::HdsConfig => {
                            if hds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hdsConfig"));
                            }
                            hds_config__ = map_.next_value()?;
                        }
                        GeneratedField::FlagsPath => {
                            if flags_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flagsPath"));
                            }
                            flags_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StatsSinks => {
                            if stats_sinks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statsSinks"));
                            }
                            stats_sinks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DeferredStatOptions => {
                            if deferred_stat_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deferredStatOptions"));
                            }
                            deferred_stat_options__ = map_.next_value()?;
                        }
                        GeneratedField::StatsConfig => {
                            if stats_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statsConfig"));
                            }
                            stats_config__ = map_.next_value()?;
                        }
                        GeneratedField::StatsFlushInterval => {
                            if stats_flush_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statsFlushInterval"));
                            }
                            stats_flush_interval__ = map_.next_value()?;
                        }
                        GeneratedField::Watchdog => {
                            if watchdog__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watchdog"));
                            }
                            watchdog__ = map_.next_value()?;
                        }
                        GeneratedField::Watchdogs => {
                            if watchdogs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watchdogs"));
                            }
                            watchdogs__ = map_.next_value()?;
                        }
                        GeneratedField::Tracing => {
                            if tracing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracing"));
                            }
                            tracing__ = map_.next_value()?;
                        }
                        GeneratedField::LayeredRuntime => {
                            if layered_runtime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("layeredRuntime"));
                            }
                            layered_runtime__ = map_.next_value()?;
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = map_.next_value()?;
                        }
                        GeneratedField::OverloadManager => {
                            if overload_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overloadManager"));
                            }
                            overload_manager__ = map_.next_value()?;
                        }
                        GeneratedField::EnableDispatcherStats => {
                            if enable_dispatcher_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableDispatcherStats"));
                            }
                            enable_dispatcher_stats__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HeaderPrefix => {
                            if header_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerPrefix"));
                            }
                            header_prefix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StatsServerVersionOverride => {
                            if stats_server_version_override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statsServerVersionOverride"));
                            }
                            stats_server_version_override__ = map_.next_value()?;
                        }
                        GeneratedField::UseTcpForDnsLookups => {
                            if use_tcp_for_dns_lookups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useTcpForDnsLookups"));
                            }
                            use_tcp_for_dns_lookups__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DnsResolutionConfig => {
                            if dns_resolution_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsResolutionConfig"));
                            }
                            dns_resolution_config__ = map_.next_value()?;
                        }
                        GeneratedField::TypedDnsResolverConfig => {
                            if typed_dns_resolver_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedDnsResolverConfig"));
                            }
                            typed_dns_resolver_config__ = map_.next_value()?;
                        }
                        GeneratedField::BootstrapExtensions => {
                            if bootstrap_extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bootstrapExtensions"));
                            }
                            bootstrap_extensions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FatalActions => {
                            if fatal_actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fatalActions"));
                            }
                            fatal_actions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConfigSources => {
                            if config_sources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configSources"));
                            }
                            config_sources__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefaultConfigSource => {
                            if default_config_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultConfigSource"));
                            }
                            default_config_source__ = map_.next_value()?;
                        }
                        GeneratedField::DefaultSocketInterface => {
                            if default_socket_interface__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultSocketInterface"));
                            }
                            default_socket_interface__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CertificateProviderInstances => {
                            if certificate_provider_instances__.is_some() {
                                return Err(serde::de::Error::duplicate_field("certificateProviderInstances"));
                            }
                            certificate_provider_instances__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::InlineHeaders => {
                            if inline_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inlineHeaders"));
                            }
                            inline_headers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PerfTracingFilePath => {
                            if perf_tracing_file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perfTracingFilePath"));
                            }
                            perf_tracing_file_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefaultRegexEngine => {
                            if default_regex_engine__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultRegexEngine"));
                            }
                            default_regex_engine__ = map_.next_value()?;
                        }
                        GeneratedField::XdsDelegateExtension => {
                            if xds_delegate_extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("xdsDelegateExtension"));
                            }
                            xds_delegate_extension__ = map_.next_value()?;
                        }
                        GeneratedField::XdsConfigTrackerExtension => {
                            if xds_config_tracker_extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("xdsConfigTrackerExtension"));
                            }
                            xds_config_tracker_extension__ = map_.next_value()?;
                        }
                        GeneratedField::ListenerManager => {
                            if listener_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listenerManager"));
                            }
                            listener_manager__ = map_.next_value()?;
                        }
                        GeneratedField::ApplicationLogConfig => {
                            if application_log_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applicationLogConfig"));
                            }
                            application_log_config__ = map_.next_value()?;
                        }
                        GeneratedField::GrpcAsyncClientManagerConfig => {
                            if grpc_async_client_manager_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcAsyncClientManagerConfig"));
                            }
                            grpc_async_client_manager_config__ = map_.next_value()?;
                        }
                        GeneratedField::MemoryAllocatorManager => {
                            if memory_allocator_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memoryAllocatorManager"));
                            }
                            memory_allocator_manager__ = map_.next_value()?;
                        }
                        GeneratedField::StatsFlushOnAdmin => {
                            if stats_flush__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statsFlushOnAdmin"));
                            }
                            stats_flush__ = map_.next_value::<::std::option::Option<_>>()?.map(bootstrap::StatsFlush::StatsFlushOnAdmin);
                        }
                    }
                }
                Ok(Bootstrap {
                    node: node__,
                    node_context_params: node_context_params__.unwrap_or_default(),
                    static_resources: static_resources__,
                    dynamic_resources: dynamic_resources__,
                    cluster_manager: cluster_manager__,
                    hds_config: hds_config__,
                    flags_path: flags_path__.unwrap_or_default(),
                    stats_sinks: stats_sinks__.unwrap_or_default(),
                    deferred_stat_options: deferred_stat_options__,
                    stats_config: stats_config__,
                    stats_flush_interval: stats_flush_interval__,
                    watchdog: watchdog__,
                    watchdogs: watchdogs__,
                    tracing: tracing__,
                    layered_runtime: layered_runtime__,
                    admin: admin__,
                    overload_manager: overload_manager__,
                    enable_dispatcher_stats: enable_dispatcher_stats__.unwrap_or_default(),
                    header_prefix: header_prefix__.unwrap_or_default(),
                    stats_server_version_override: stats_server_version_override__,
                    use_tcp_for_dns_lookups: use_tcp_for_dns_lookups__.unwrap_or_default(),
                    dns_resolution_config: dns_resolution_config__,
                    typed_dns_resolver_config: typed_dns_resolver_config__,
                    bootstrap_extensions: bootstrap_extensions__.unwrap_or_default(),
                    fatal_actions: fatal_actions__.unwrap_or_default(),
                    config_sources: config_sources__.unwrap_or_default(),
                    default_config_source: default_config_source__,
                    default_socket_interface: default_socket_interface__.unwrap_or_default(),
                    certificate_provider_instances: certificate_provider_instances__.unwrap_or_default(),
                    inline_headers: inline_headers__.unwrap_or_default(),
                    perf_tracing_file_path: perf_tracing_file_path__.unwrap_or_default(),
                    default_regex_engine: default_regex_engine__,
                    xds_delegate_extension: xds_delegate_extension__,
                    xds_config_tracker_extension: xds_config_tracker_extension__,
                    listener_manager: listener_manager__,
                    application_log_config: application_log_config__,
                    grpc_async_client_manager_config: grpc_async_client_manager_config__,
                    memory_allocator_manager: memory_allocator_manager__,
                    stats_flush: stats_flush__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.Bootstrap", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for bootstrap::ApplicationLogConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.log_format.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.Bootstrap.ApplicationLogConfig", len)?;
        if let Some(v) = self.log_format.as_ref() {
            struct_ser.serialize_field("log_format", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for bootstrap::ApplicationLogConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "log_format",
            "logFormat",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LogFormat,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "logFormat" | "log_format" => Ok(GeneratedField::LogFormat),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = bootstrap::ApplicationLogConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.Bootstrap.ApplicationLogConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<bootstrap::ApplicationLogConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut log_format__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LogFormat => {
                            if log_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logFormat"));
                            }
                            log_format__ = map_.next_value()?;
                        }
                    }
                }
                Ok(bootstrap::ApplicationLogConfig {
                    log_format: log_format__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.Bootstrap.ApplicationLogConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for bootstrap::application_log_config::LogFormat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.log_format.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.Bootstrap.ApplicationLogConfig.LogFormat", len)?;
        if let Some(v) = self.log_format.as_ref() {
            match v {
                bootstrap::application_log_config::log_format::LogFormat::JsonFormat(v) => {
                    struct_ser.serialize_field("json_format", v)?;
                }
                bootstrap::application_log_config::log_format::LogFormat::TextFormat(v) => {
                    struct_ser.serialize_field("text_format", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for bootstrap::application_log_config::LogFormat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "json_format",
            "jsonFormat",
            "text_format",
            "textFormat",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            JsonFormat,
            TextFormat,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "jsonFormat" | "json_format" => Ok(GeneratedField::JsonFormat),
                            "textFormat" | "text_format" => Ok(GeneratedField::TextFormat),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = bootstrap::application_log_config::LogFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.Bootstrap.ApplicationLogConfig.LogFormat")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<bootstrap::application_log_config::LogFormat, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut log_format__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::JsonFormat => {
                            if log_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jsonFormat"));
                            }
                            log_format__ = map_.next_value::<::std::option::Option<_>>()?.map(bootstrap::application_log_config::log_format::LogFormat::JsonFormat)
;
                        }
                        GeneratedField::TextFormat => {
                            if log_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("textFormat"));
                            }
                            log_format__ = map_.next_value::<::std::option::Option<_>>()?.map(bootstrap::application_log_config::log_format::LogFormat::TextFormat);
                        }
                    }
                }
                Ok(bootstrap::application_log_config::LogFormat {
                    log_format: log_format__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.Bootstrap.ApplicationLogConfig.LogFormat", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for bootstrap::DeferredStatOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.enable_deferred_creation_stats {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.Bootstrap.DeferredStatOptions", len)?;
        if self.enable_deferred_creation_stats {
            struct_ser.serialize_field("enable_deferred_creation_stats", &self.enable_deferred_creation_stats)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for bootstrap::DeferredStatOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enable_deferred_creation_stats",
            "enableDeferredCreationStats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EnableDeferredCreationStats,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "enableDeferredCreationStats" | "enable_deferred_creation_stats" => Ok(GeneratedField::EnableDeferredCreationStats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = bootstrap::DeferredStatOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.Bootstrap.DeferredStatOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<bootstrap::DeferredStatOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enable_deferred_creation_stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EnableDeferredCreationStats => {
                            if enable_deferred_creation_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableDeferredCreationStats"));
                            }
                            enable_deferred_creation_stats__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(bootstrap::DeferredStatOptions {
                    enable_deferred_creation_stats: enable_deferred_creation_stats__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.Bootstrap.DeferredStatOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for bootstrap::DynamicResources {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.lds_config.is_some() {
            len += 1;
        }
        if !self.lds_resources_locator.is_empty() {
            len += 1;
        }
        if self.cds_config.is_some() {
            len += 1;
        }
        if !self.cds_resources_locator.is_empty() {
            len += 1;
        }
        if self.ads_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.Bootstrap.DynamicResources", len)?;
        if let Some(v) = self.lds_config.as_ref() {
            struct_ser.serialize_field("lds_config", v)?;
        }
        if !self.lds_resources_locator.is_empty() {
            struct_ser.serialize_field("lds_resources_locator", &self.lds_resources_locator)?;
        }
        if let Some(v) = self.cds_config.as_ref() {
            struct_ser.serialize_field("cds_config", v)?;
        }
        if !self.cds_resources_locator.is_empty() {
            struct_ser.serialize_field("cds_resources_locator", &self.cds_resources_locator)?;
        }
        if let Some(v) = self.ads_config.as_ref() {
            struct_ser.serialize_field("ads_config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for bootstrap::DynamicResources {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "lds_config",
            "ldsConfig",
            "lds_resources_locator",
            "ldsResourcesLocator",
            "cds_config",
            "cdsConfig",
            "cds_resources_locator",
            "cdsResourcesLocator",
            "ads_config",
            "adsConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LdsConfig,
            LdsResourcesLocator,
            CdsConfig,
            CdsResourcesLocator,
            AdsConfig,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "ldsConfig" | "lds_config" => Ok(GeneratedField::LdsConfig),
                            "ldsResourcesLocator" | "lds_resources_locator" => Ok(GeneratedField::LdsResourcesLocator),
                            "cdsConfig" | "cds_config" => Ok(GeneratedField::CdsConfig),
                            "cdsResourcesLocator" | "cds_resources_locator" => Ok(GeneratedField::CdsResourcesLocator),
                            "adsConfig" | "ads_config" => Ok(GeneratedField::AdsConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = bootstrap::DynamicResources;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.Bootstrap.DynamicResources")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<bootstrap::DynamicResources, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut lds_config__ = None;
                let mut lds_resources_locator__ = None;
                let mut cds_config__ = None;
                let mut cds_resources_locator__ = None;
                let mut ads_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LdsConfig => {
                            if lds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ldsConfig"));
                            }
                            lds_config__ = map_.next_value()?;
                        }
                        GeneratedField::LdsResourcesLocator => {
                            if lds_resources_locator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ldsResourcesLocator"));
                            }
                            lds_resources_locator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CdsConfig => {
                            if cds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cdsConfig"));
                            }
                            cds_config__ = map_.next_value()?;
                        }
                        GeneratedField::CdsResourcesLocator => {
                            if cds_resources_locator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cdsResourcesLocator"));
                            }
                            cds_resources_locator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AdsConfig => {
                            if ads_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adsConfig"));
                            }
                            ads_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(bootstrap::DynamicResources {
                    lds_config: lds_config__,
                    lds_resources_locator: lds_resources_locator__.unwrap_or_default(),
                    cds_config: cds_config__,
                    cds_resources_locator: cds_resources_locator__.unwrap_or_default(),
                    ads_config: ads_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.Bootstrap.DynamicResources", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for bootstrap::GrpcAsyncClientManagerConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_cached_entry_idle_duration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.Bootstrap.GrpcAsyncClientManagerConfig", len)?;
        if let Some(v) = self.max_cached_entry_idle_duration.as_ref() {
            struct_ser.serialize_field("max_cached_entry_idle_duration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for bootstrap::GrpcAsyncClientManagerConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_cached_entry_idle_duration",
            "maxCachedEntryIdleDuration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxCachedEntryIdleDuration,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "maxCachedEntryIdleDuration" | "max_cached_entry_idle_duration" => Ok(GeneratedField::MaxCachedEntryIdleDuration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = bootstrap::GrpcAsyncClientManagerConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.Bootstrap.GrpcAsyncClientManagerConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<bootstrap::GrpcAsyncClientManagerConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_cached_entry_idle_duration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxCachedEntryIdleDuration => {
                            if max_cached_entry_idle_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxCachedEntryIdleDuration"));
                            }
                            max_cached_entry_idle_duration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(bootstrap::GrpcAsyncClientManagerConfig {
                    max_cached_entry_idle_duration: max_cached_entry_idle_duration__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.Bootstrap.GrpcAsyncClientManagerConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for bootstrap::StaticResources {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.listeners.is_empty() {
            len += 1;
        }
        if !self.clusters.is_empty() {
            len += 1;
        }
        if !self.secrets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.Bootstrap.StaticResources", len)?;
        if !self.listeners.is_empty() {
            struct_ser.serialize_field("listeners", &self.listeners)?;
        }
        if !self.clusters.is_empty() {
            struct_ser.serialize_field("clusters", &self.clusters)?;
        }
        if !self.secrets.is_empty() {
            struct_ser.serialize_field("secrets", &self.secrets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for bootstrap::StaticResources {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "listeners",
            "clusters",
            "secrets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Listeners,
            Clusters,
            Secrets,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "listeners" => Ok(GeneratedField::Listeners),
                            "clusters" => Ok(GeneratedField::Clusters),
                            "secrets" => Ok(GeneratedField::Secrets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = bootstrap::StaticResources;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.Bootstrap.StaticResources")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<bootstrap::StaticResources, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut listeners__ = None;
                let mut clusters__ = None;
                let mut secrets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Listeners => {
                            if listeners__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listeners"));
                            }
                            listeners__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Clusters => {
                            if clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusters"));
                            }
                            clusters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Secrets => {
                            if secrets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secrets"));
                            }
                            secrets__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(bootstrap::StaticResources {
                    listeners: listeners__.unwrap_or_default(),
                    clusters: clusters__.unwrap_or_default(),
                    secrets: secrets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.Bootstrap.StaticResources", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClusterManager {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.local_cluster_name.is_empty() {
            len += 1;
        }
        if self.outlier_detection.is_some() {
            len += 1;
        }
        if self.upstream_bind_config.is_some() {
            len += 1;
        }
        if self.load_stats_config.is_some() {
            len += 1;
        }
        if self.enable_deferred_cluster_creation {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.ClusterManager", len)?;
        if !self.local_cluster_name.is_empty() {
            struct_ser.serialize_field("local_cluster_name", &self.local_cluster_name)?;
        }
        if let Some(v) = self.outlier_detection.as_ref() {
            struct_ser.serialize_field("outlier_detection", v)?;
        }
        if let Some(v) = self.upstream_bind_config.as_ref() {
            struct_ser.serialize_field("upstream_bind_config", v)?;
        }
        if let Some(v) = self.load_stats_config.as_ref() {
            struct_ser.serialize_field("load_stats_config", v)?;
        }
        if self.enable_deferred_cluster_creation {
            struct_ser.serialize_field("enable_deferred_cluster_creation", &self.enable_deferred_cluster_creation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterManager {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "local_cluster_name",
            "localClusterName",
            "outlier_detection",
            "outlierDetection",
            "upstream_bind_config",
            "upstreamBindConfig",
            "load_stats_config",
            "loadStatsConfig",
            "enable_deferred_cluster_creation",
            "enableDeferredClusterCreation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LocalClusterName,
            OutlierDetection,
            UpstreamBindConfig,
            LoadStatsConfig,
            EnableDeferredClusterCreation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "localClusterName" | "local_cluster_name" => Ok(GeneratedField::LocalClusterName),
                            "outlierDetection" | "outlier_detection" => Ok(GeneratedField::OutlierDetection),
                            "upstreamBindConfig" | "upstream_bind_config" => Ok(GeneratedField::UpstreamBindConfig),
                            "loadStatsConfig" | "load_stats_config" => Ok(GeneratedField::LoadStatsConfig),
                            "enableDeferredClusterCreation" | "enable_deferred_cluster_creation" => Ok(GeneratedField::EnableDeferredClusterCreation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterManager;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.ClusterManager")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClusterManager, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut local_cluster_name__ = None;
                let mut outlier_detection__ = None;
                let mut upstream_bind_config__ = None;
                let mut load_stats_config__ = None;
                let mut enable_deferred_cluster_creation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LocalClusterName => {
                            if local_cluster_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localClusterName"));
                            }
                            local_cluster_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OutlierDetection => {
                            if outlier_detection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outlierDetection"));
                            }
                            outlier_detection__ = map_.next_value()?;
                        }
                        GeneratedField::UpstreamBindConfig => {
                            if upstream_bind_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamBindConfig"));
                            }
                            upstream_bind_config__ = map_.next_value()?;
                        }
                        GeneratedField::LoadStatsConfig => {
                            if load_stats_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadStatsConfig"));
                            }
                            load_stats_config__ = map_.next_value()?;
                        }
                        GeneratedField::EnableDeferredClusterCreation => {
                            if enable_deferred_cluster_creation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableDeferredClusterCreation"));
                            }
                            enable_deferred_cluster_creation__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ClusterManager {
                    local_cluster_name: local_cluster_name__.unwrap_or_default(),
                    outlier_detection: outlier_detection__,
                    upstream_bind_config: upstream_bind_config__,
                    load_stats_config: load_stats_config__,
                    enable_deferred_cluster_creation: enable_deferred_cluster_creation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.ClusterManager", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster_manager::OutlierDetection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.event_log_path.is_empty() {
            len += 1;
        }
        if self.event_service.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.ClusterManager.OutlierDetection", len)?;
        if !self.event_log_path.is_empty() {
            struct_ser.serialize_field("event_log_path", &self.event_log_path)?;
        }
        if let Some(v) = self.event_service.as_ref() {
            struct_ser.serialize_field("event_service", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster_manager::OutlierDetection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_log_path",
            "eventLogPath",
            "event_service",
            "eventService",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventLogPath,
            EventService,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "eventLogPath" | "event_log_path" => Ok(GeneratedField::EventLogPath),
                            "eventService" | "event_service" => Ok(GeneratedField::EventService),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster_manager::OutlierDetection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.ClusterManager.OutlierDetection")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster_manager::OutlierDetection, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event_log_path__ = None;
                let mut event_service__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EventLogPath => {
                            if event_log_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventLogPath"));
                            }
                            event_log_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EventService => {
                            if event_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventService"));
                            }
                            event_service__ = map_.next_value()?;
                        }
                    }
                }
                Ok(cluster_manager::OutlierDetection {
                    event_log_path: event_log_path__.unwrap_or_default(),
                    event_service: event_service__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.ClusterManager.OutlierDetection", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CustomInlineHeader {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.inline_header_name.is_empty() {
            len += 1;
        }
        if self.inline_header_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.CustomInlineHeader", len)?;
        if !self.inline_header_name.is_empty() {
            struct_ser.serialize_field("inline_header_name", &self.inline_header_name)?;
        }
        if self.inline_header_type != 0 {
            let v = custom_inline_header::InlineHeaderType::try_from(self.inline_header_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.inline_header_type)))?;
            struct_ser.serialize_field("inline_header_type", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CustomInlineHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "inline_header_name",
            "inlineHeaderName",
            "inline_header_type",
            "inlineHeaderType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InlineHeaderName,
            InlineHeaderType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "inlineHeaderName" | "inline_header_name" => Ok(GeneratedField::InlineHeaderName),
                            "inlineHeaderType" | "inline_header_type" => Ok(GeneratedField::InlineHeaderType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CustomInlineHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.CustomInlineHeader")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CustomInlineHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut inline_header_name__ = None;
                let mut inline_header_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InlineHeaderName => {
                            if inline_header_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inlineHeaderName"));
                            }
                            inline_header_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InlineHeaderType => {
                            if inline_header_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inlineHeaderType"));
                            }
                            inline_header_type__ = Some(map_.next_value::<custom_inline_header::InlineHeaderType>()? as i32);
                        }
                    }
                }
                Ok(CustomInlineHeader {
                    inline_header_name: inline_header_name__.unwrap_or_default(),
                    inline_header_type: inline_header_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.CustomInlineHeader", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for custom_inline_header::InlineHeaderType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::RequestHeader => "REQUEST_HEADER",
            Self::RequestTrailer => "REQUEST_TRAILER",
            Self::ResponseHeader => "RESPONSE_HEADER",
            Self::ResponseTrailer => "RESPONSE_TRAILER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for custom_inline_header::InlineHeaderType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "REQUEST_HEADER",
            "REQUEST_TRAILER",
            "RESPONSE_HEADER",
            "RESPONSE_TRAILER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = custom_inline_header::InlineHeaderType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "REQUEST_HEADER" => Ok(custom_inline_header::InlineHeaderType::RequestHeader),
                    "REQUEST_TRAILER" => Ok(custom_inline_header::InlineHeaderType::RequestTrailer),
                    "RESPONSE_HEADER" => Ok(custom_inline_header::InlineHeaderType::ResponseHeader),
                    "RESPONSE_TRAILER" => Ok(custom_inline_header::InlineHeaderType::ResponseTrailer),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for FatalAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.FatalAction", len)?;
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FatalAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Config,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "config" => Ok(GeneratedField::Config),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FatalAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.FatalAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FatalAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FatalAction {
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.FatalAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LayeredRuntime {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.layers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.LayeredRuntime", len)?;
        if !self.layers.is_empty() {
            struct_ser.serialize_field("layers", &self.layers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LayeredRuntime {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "layers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Layers,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "layers" => Ok(GeneratedField::Layers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LayeredRuntime;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.LayeredRuntime")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LayeredRuntime, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut layers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Layers => {
                            if layers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("layers"));
                            }
                            layers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LayeredRuntime {
                    layers: layers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.LayeredRuntime", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MemoryAllocatorManager {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bytes_to_release != 0 {
            len += 1;
        }
        if self.memory_release_interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.MemoryAllocatorManager", len)?;
        if self.bytes_to_release != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("bytes_to_release", ToString::to_string(&self.bytes_to_release).as_str())?;
        }
        if let Some(v) = self.memory_release_interval.as_ref() {
            struct_ser.serialize_field("memory_release_interval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MemoryAllocatorManager {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bytes_to_release",
            "bytesToRelease",
            "memory_release_interval",
            "memoryReleaseInterval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BytesToRelease,
            MemoryReleaseInterval,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "bytesToRelease" | "bytes_to_release" => Ok(GeneratedField::BytesToRelease),
                            "memoryReleaseInterval" | "memory_release_interval" => Ok(GeneratedField::MemoryReleaseInterval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MemoryAllocatorManager;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.MemoryAllocatorManager")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MemoryAllocatorManager, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bytes_to_release__ = None;
                let mut memory_release_interval__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BytesToRelease => {
                            if bytes_to_release__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bytesToRelease"));
                            }
                            bytes_to_release__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MemoryReleaseInterval => {
                            if memory_release_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memoryReleaseInterval"));
                            }
                            memory_release_interval__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MemoryAllocatorManager {
                    bytes_to_release: bytes_to_release__.unwrap_or_default(),
                    memory_release_interval: memory_release_interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.MemoryAllocatorManager", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Runtime {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.symlink_root.is_empty() {
            len += 1;
        }
        if !self.subdirectory.is_empty() {
            len += 1;
        }
        if !self.override_subdirectory.is_empty() {
            len += 1;
        }
        if self.base.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.Runtime", len)?;
        if !self.symlink_root.is_empty() {
            struct_ser.serialize_field("symlink_root", &self.symlink_root)?;
        }
        if !self.subdirectory.is_empty() {
            struct_ser.serialize_field("subdirectory", &self.subdirectory)?;
        }
        if !self.override_subdirectory.is_empty() {
            struct_ser.serialize_field("override_subdirectory", &self.override_subdirectory)?;
        }
        if let Some(v) = self.base.as_ref() {
            struct_ser.serialize_field("base", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Runtime {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "symlink_root",
            "symlinkRoot",
            "subdirectory",
            "override_subdirectory",
            "overrideSubdirectory",
            "base",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SymlinkRoot,
            Subdirectory,
            OverrideSubdirectory,
            Base,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "symlinkRoot" | "symlink_root" => Ok(GeneratedField::SymlinkRoot),
                            "subdirectory" => Ok(GeneratedField::Subdirectory),
                            "overrideSubdirectory" | "override_subdirectory" => Ok(GeneratedField::OverrideSubdirectory),
                            "base" => Ok(GeneratedField::Base),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Runtime;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.Runtime")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Runtime, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut symlink_root__ = None;
                let mut subdirectory__ = None;
                let mut override_subdirectory__ = None;
                let mut base__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SymlinkRoot => {
                            if symlink_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symlinkRoot"));
                            }
                            symlink_root__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Subdirectory => {
                            if subdirectory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subdirectory"));
                            }
                            subdirectory__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OverrideSubdirectory => {
                            if override_subdirectory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrideSubdirectory"));
                            }
                            override_subdirectory__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Base => {
                            if base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("base"));
                            }
                            base__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Runtime {
                    symlink_root: symlink_root__.unwrap_or_default(),
                    subdirectory: subdirectory__.unwrap_or_default(),
                    override_subdirectory: override_subdirectory__.unwrap_or_default(),
                    base: base__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.Runtime", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuntimeLayer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.layer_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.RuntimeLayer", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.layer_specifier.as_ref() {
            match v {
                runtime_layer::LayerSpecifier::StaticLayer(v) => {
                    struct_ser.serialize_field("static_layer", v)?;
                }
                runtime_layer::LayerSpecifier::DiskLayer(v) => {
                    struct_ser.serialize_field("disk_layer", v)?;
                }
                runtime_layer::LayerSpecifier::AdminLayer(v) => {
                    struct_ser.serialize_field("admin_layer", v)?;
                }
                runtime_layer::LayerSpecifier::RtdsLayer(v) => {
                    struct_ser.serialize_field("rtds_layer", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuntimeLayer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "static_layer",
            "staticLayer",
            "disk_layer",
            "diskLayer",
            "admin_layer",
            "adminLayer",
            "rtds_layer",
            "rtdsLayer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            StaticLayer,
            DiskLayer,
            AdminLayer,
            RtdsLayer,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "staticLayer" | "static_layer" => Ok(GeneratedField::StaticLayer),
                            "diskLayer" | "disk_layer" => Ok(GeneratedField::DiskLayer),
                            "adminLayer" | "admin_layer" => Ok(GeneratedField::AdminLayer),
                            "rtdsLayer" | "rtds_layer" => Ok(GeneratedField::RtdsLayer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuntimeLayer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.RuntimeLayer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RuntimeLayer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut layer_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StaticLayer => {
                            if layer_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticLayer"));
                            }
                            layer_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(runtime_layer::LayerSpecifier::StaticLayer)
;
                        }
                        GeneratedField::DiskLayer => {
                            if layer_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("diskLayer"));
                            }
                            layer_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(runtime_layer::LayerSpecifier::DiskLayer)
;
                        }
                        GeneratedField::AdminLayer => {
                            if layer_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adminLayer"));
                            }
                            layer_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(runtime_layer::LayerSpecifier::AdminLayer)
;
                        }
                        GeneratedField::RtdsLayer => {
                            if layer_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rtdsLayer"));
                            }
                            layer_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(runtime_layer::LayerSpecifier::RtdsLayer)
;
                        }
                    }
                }
                Ok(RuntimeLayer {
                    name: name__.unwrap_or_default(),
                    layer_specifier: layer_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.RuntimeLayer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for runtime_layer::AdminLayer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.RuntimeLayer.AdminLayer", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for runtime_layer::AdminLayer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = runtime_layer::AdminLayer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.RuntimeLayer.AdminLayer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<runtime_layer::AdminLayer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(runtime_layer::AdminLayer {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.RuntimeLayer.AdminLayer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for runtime_layer::DiskLayer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.symlink_root.is_empty() {
            len += 1;
        }
        if !self.subdirectory.is_empty() {
            len += 1;
        }
        if self.append_service_cluster {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.RuntimeLayer.DiskLayer", len)?;
        if !self.symlink_root.is_empty() {
            struct_ser.serialize_field("symlink_root", &self.symlink_root)?;
        }
        if !self.subdirectory.is_empty() {
            struct_ser.serialize_field("subdirectory", &self.subdirectory)?;
        }
        if self.append_service_cluster {
            struct_ser.serialize_field("append_service_cluster", &self.append_service_cluster)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for runtime_layer::DiskLayer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "symlink_root",
            "symlinkRoot",
            "subdirectory",
            "append_service_cluster",
            "appendServiceCluster",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SymlinkRoot,
            Subdirectory,
            AppendServiceCluster,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "symlinkRoot" | "symlink_root" => Ok(GeneratedField::SymlinkRoot),
                            "subdirectory" => Ok(GeneratedField::Subdirectory),
                            "appendServiceCluster" | "append_service_cluster" => Ok(GeneratedField::AppendServiceCluster),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = runtime_layer::DiskLayer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.RuntimeLayer.DiskLayer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<runtime_layer::DiskLayer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut symlink_root__ = None;
                let mut subdirectory__ = None;
                let mut append_service_cluster__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SymlinkRoot => {
                            if symlink_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symlinkRoot"));
                            }
                            symlink_root__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Subdirectory => {
                            if subdirectory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subdirectory"));
                            }
                            subdirectory__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppendServiceCluster => {
                            if append_service_cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appendServiceCluster"));
                            }
                            append_service_cluster__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(runtime_layer::DiskLayer {
                    symlink_root: symlink_root__.unwrap_or_default(),
                    subdirectory: subdirectory__.unwrap_or_default(),
                    append_service_cluster: append_service_cluster__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.RuntimeLayer.DiskLayer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for runtime_layer::RtdsLayer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.rtds_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.RuntimeLayer.RtdsLayer", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.rtds_config.as_ref() {
            struct_ser.serialize_field("rtds_config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for runtime_layer::RtdsLayer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "rtds_config",
            "rtdsConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            RtdsConfig,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "rtdsConfig" | "rtds_config" => Ok(GeneratedField::RtdsConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = runtime_layer::RtdsLayer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.RuntimeLayer.RtdsLayer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<runtime_layer::RtdsLayer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut rtds_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RtdsConfig => {
                            if rtds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rtdsConfig"));
                            }
                            rtds_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(runtime_layer::RtdsLayer {
                    name: name__.unwrap_or_default(),
                    rtds_config: rtds_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.RuntimeLayer.RtdsLayer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Watchdog {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.actions.is_empty() {
            len += 1;
        }
        if self.miss_timeout.is_some() {
            len += 1;
        }
        if self.megamiss_timeout.is_some() {
            len += 1;
        }
        if self.kill_timeout.is_some() {
            len += 1;
        }
        if self.max_kill_timeout_jitter.is_some() {
            len += 1;
        }
        if self.multikill_timeout.is_some() {
            len += 1;
        }
        if self.multikill_threshold.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.Watchdog", len)?;
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        if let Some(v) = self.miss_timeout.as_ref() {
            struct_ser.serialize_field("miss_timeout", v)?;
        }
        if let Some(v) = self.megamiss_timeout.as_ref() {
            struct_ser.serialize_field("megamiss_timeout", v)?;
        }
        if let Some(v) = self.kill_timeout.as_ref() {
            struct_ser.serialize_field("kill_timeout", v)?;
        }
        if let Some(v) = self.max_kill_timeout_jitter.as_ref() {
            struct_ser.serialize_field("max_kill_timeout_jitter", v)?;
        }
        if let Some(v) = self.multikill_timeout.as_ref() {
            struct_ser.serialize_field("multikill_timeout", v)?;
        }
        if let Some(v) = self.multikill_threshold.as_ref() {
            struct_ser.serialize_field("multikill_threshold", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Watchdog {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "actions",
            "miss_timeout",
            "missTimeout",
            "megamiss_timeout",
            "megamissTimeout",
            "kill_timeout",
            "killTimeout",
            "max_kill_timeout_jitter",
            "maxKillTimeoutJitter",
            "multikill_timeout",
            "multikillTimeout",
            "multikill_threshold",
            "multikillThreshold",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Actions,
            MissTimeout,
            MegamissTimeout,
            KillTimeout,
            MaxKillTimeoutJitter,
            MultikillTimeout,
            MultikillThreshold,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "actions" => Ok(GeneratedField::Actions),
                            "missTimeout" | "miss_timeout" => Ok(GeneratedField::MissTimeout),
                            "megamissTimeout" | "megamiss_timeout" => Ok(GeneratedField::MegamissTimeout),
                            "killTimeout" | "kill_timeout" => Ok(GeneratedField::KillTimeout),
                            "maxKillTimeoutJitter" | "max_kill_timeout_jitter" => Ok(GeneratedField::MaxKillTimeoutJitter),
                            "multikillTimeout" | "multikill_timeout" => Ok(GeneratedField::MultikillTimeout),
                            "multikillThreshold" | "multikill_threshold" => Ok(GeneratedField::MultikillThreshold),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Watchdog;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.Watchdog")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Watchdog, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut actions__ = None;
                let mut miss_timeout__ = None;
                let mut megamiss_timeout__ = None;
                let mut kill_timeout__ = None;
                let mut max_kill_timeout_jitter__ = None;
                let mut multikill_timeout__ = None;
                let mut multikill_threshold__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MissTimeout => {
                            if miss_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("missTimeout"));
                            }
                            miss_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::MegamissTimeout => {
                            if megamiss_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("megamissTimeout"));
                            }
                            megamiss_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::KillTimeout => {
                            if kill_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("killTimeout"));
                            }
                            kill_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::MaxKillTimeoutJitter => {
                            if max_kill_timeout_jitter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxKillTimeoutJitter"));
                            }
                            max_kill_timeout_jitter__ = map_.next_value()?;
                        }
                        GeneratedField::MultikillTimeout => {
                            if multikill_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multikillTimeout"));
                            }
                            multikill_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::MultikillThreshold => {
                            if multikill_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multikillThreshold"));
                            }
                            multikill_threshold__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Watchdog {
                    actions: actions__.unwrap_or_default(),
                    miss_timeout: miss_timeout__,
                    megamiss_timeout: megamiss_timeout__,
                    kill_timeout: kill_timeout__,
                    max_kill_timeout_jitter: max_kill_timeout_jitter__,
                    multikill_timeout: multikill_timeout__,
                    multikill_threshold: multikill_threshold__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.Watchdog", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for watchdog::WatchdogAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config.is_some() {
            len += 1;
        }
        if self.event != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.Watchdog.WatchdogAction", len)?;
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if self.event != 0 {
            let v = watchdog::watchdog_action::WatchdogEvent::try_from(self.event)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.event)))?;
            struct_ser.serialize_field("event", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for watchdog::WatchdogAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config",
            "event",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Config,
            Event,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "config" => Ok(GeneratedField::Config),
                            "event" => Ok(GeneratedField::Event),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = watchdog::WatchdogAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.Watchdog.WatchdogAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<watchdog::WatchdogAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config__ = None;
                let mut event__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::Event => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("event"));
                            }
                            event__ = Some(map_.next_value::<watchdog::watchdog_action::WatchdogEvent>()? as i32);
                        }
                    }
                }
                Ok(watchdog::WatchdogAction {
                    config: config__,
                    event: event__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.Watchdog.WatchdogAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for watchdog::watchdog_action::WatchdogEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::Kill => "KILL",
            Self::Multikill => "MULTIKILL",
            Self::Megamiss => "MEGAMISS",
            Self::Miss => "MISS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for watchdog::watchdog_action::WatchdogEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "KILL",
            "MULTIKILL",
            "MEGAMISS",
            "MISS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = watchdog::watchdog_action::WatchdogEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN" => Ok(watchdog::watchdog_action::WatchdogEvent::Unknown),
                    "KILL" => Ok(watchdog::watchdog_action::WatchdogEvent::Kill),
                    "MULTIKILL" => Ok(watchdog::watchdog_action::WatchdogEvent::Multikill),
                    "MEGAMISS" => Ok(watchdog::watchdog_action::WatchdogEvent::Megamiss),
                    "MISS" => Ok(watchdog::watchdog_action::WatchdogEvent::Miss),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Watchdogs {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.main_thread_watchdog.is_some() {
            len += 1;
        }
        if self.worker_watchdog.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v3.Watchdogs", len)?;
        if let Some(v) = self.main_thread_watchdog.as_ref() {
            struct_ser.serialize_field("main_thread_watchdog", v)?;
        }
        if let Some(v) = self.worker_watchdog.as_ref() {
            struct_ser.serialize_field("worker_watchdog", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Watchdogs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "main_thread_watchdog",
            "mainThreadWatchdog",
            "worker_watchdog",
            "workerWatchdog",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MainThreadWatchdog,
            WorkerWatchdog,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "mainThreadWatchdog" | "main_thread_watchdog" => Ok(GeneratedField::MainThreadWatchdog),
                            "workerWatchdog" | "worker_watchdog" => Ok(GeneratedField::WorkerWatchdog),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Watchdogs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v3.Watchdogs")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Watchdogs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut main_thread_watchdog__ = None;
                let mut worker_watchdog__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MainThreadWatchdog => {
                            if main_thread_watchdog__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mainThreadWatchdog"));
                            }
                            main_thread_watchdog__ = map_.next_value()?;
                        }
                        GeneratedField::WorkerWatchdog => {
                            if worker_watchdog__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workerWatchdog"));
                            }
                            worker_watchdog__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Watchdogs {
                    main_thread_watchdog: main_thread_watchdog__,
                    worker_watchdog: worker_watchdog__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v3.Watchdogs", FIELDS, GeneratedVisitor)
    }
}
