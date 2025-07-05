impl serde::Serialize for EnvoyMobileHttpConnectionManager {
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
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.EnvoyMobileHttpConnectionManager", len)?;
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnvoyMobileHttpConnectionManager {
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
            type Value = EnvoyMobileHttpConnectionManager;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.EnvoyMobileHttpConnectionManager")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EnvoyMobileHttpConnectionManager, V::Error>
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
                Ok(EnvoyMobileHttpConnectionManager {
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.EnvoyMobileHttpConnectionManager", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpConnectionManager {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.codec_type != 0 {
            len += 1;
        }
        if !self.stat_prefix.is_empty() {
            len += 1;
        }
        if !self.http_filters.is_empty() {
            len += 1;
        }
        if self.add_user_agent.is_some() {
            len += 1;
        }
        if self.tracing.is_some() {
            len += 1;
        }
        if self.common_http_protocol_options.is_some() {
            len += 1;
        }
        if self.http1_safe_max_connection_duration {
            len += 1;
        }
        if self.http_protocol_options.is_some() {
            len += 1;
        }
        if self.http2_protocol_options.is_some() {
            len += 1;
        }
        if self.http3_protocol_options.is_some() {
            len += 1;
        }
        if !self.server_name.is_empty() {
            len += 1;
        }
        if self.server_header_transformation != 0 {
            len += 1;
        }
        if self.scheme_header_transformation.is_some() {
            len += 1;
        }
        if self.max_request_headers_kb.is_some() {
            len += 1;
        }
        if self.stream_idle_timeout.is_some() {
            len += 1;
        }
        if self.request_timeout.is_some() {
            len += 1;
        }
        if self.request_headers_timeout.is_some() {
            len += 1;
        }
        if self.drain_timeout.is_some() {
            len += 1;
        }
        if self.delayed_close_timeout.is_some() {
            len += 1;
        }
        if !self.access_log.is_empty() {
            len += 1;
        }
        if self.access_log_flush_interval.is_some() {
            len += 1;
        }
        if self.flush_access_log_on_new_request {
            len += 1;
        }
        if self.access_log_options.is_some() {
            len += 1;
        }
        if self.use_remote_address.is_some() {
            len += 1;
        }
        if self.xff_num_trusted_hops != 0 {
            len += 1;
        }
        if !self.original_ip_detection_extensions.is_empty() {
            len += 1;
        }
        if !self.early_header_mutation_extensions.is_empty() {
            len += 1;
        }
        if self.internal_address_config.is_some() {
            len += 1;
        }
        if self.skip_xff_append {
            len += 1;
        }
        if !self.via.is_empty() {
            len += 1;
        }
        if self.generate_request_id.is_some() {
            len += 1;
        }
        if self.preserve_external_request_id {
            len += 1;
        }
        if self.always_set_request_id_in_response {
            len += 1;
        }
        if self.forward_client_cert_details != 0 {
            len += 1;
        }
        if self.set_current_client_cert_details.is_some() {
            len += 1;
        }
        if self.proxy_100_continue {
            len += 1;
        }
        if self.represent_ipv4_remote_address_as_ipv4_mapped_ipv6 {
            len += 1;
        }
        if !self.upgrade_configs.is_empty() {
            len += 1;
        }
        if self.normalize_path.is_some() {
            len += 1;
        }
        if self.merge_slashes {
            len += 1;
        }
        if self.path_with_escaped_slashes_action != 0 {
            len += 1;
        }
        if self.request_id_extension.is_some() {
            len += 1;
        }
        if self.local_reply_config.is_some() {
            len += 1;
        }
        if self.strip_matching_host_port {
            len += 1;
        }
        if self.stream_error_on_invalid_http_message.is_some() {
            len += 1;
        }
        if self.path_normalization_options.is_some() {
            len += 1;
        }
        if self.strip_trailing_host_dot {
            len += 1;
        }
        if self.proxy_status_config.is_some() {
            len += 1;
        }
        if self.typed_header_validation_config.is_some() {
            len += 1;
        }
        if self.append_x_forwarded_port {
            len += 1;
        }
        if self.append_local_overload {
            len += 1;
        }
        if self.add_proxy_protocol_connection_state.is_some() {
            len += 1;
        }
        if self.route_specifier.is_some() {
            len += 1;
        }
        if self.strip_port_mode.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager", len)?;
        if self.codec_type != 0 {
            let v = http_connection_manager::CodecType::try_from(self.codec_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.codec_type)))?;
            struct_ser.serialize_field("codec_type", &v)?;
        }
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("stat_prefix", &self.stat_prefix)?;
        }
        if !self.http_filters.is_empty() {
            struct_ser.serialize_field("http_filters", &self.http_filters)?;
        }
        if let Some(v) = self.add_user_agent.as_ref() {
            struct_ser.serialize_field("add_user_agent", v)?;
        }
        if let Some(v) = self.tracing.as_ref() {
            struct_ser.serialize_field("tracing", v)?;
        }
        if let Some(v) = self.common_http_protocol_options.as_ref() {
            struct_ser.serialize_field("common_http_protocol_options", v)?;
        }
        if self.http1_safe_max_connection_duration {
            struct_ser.serialize_field("http1_safe_max_connection_duration", &self.http1_safe_max_connection_duration)?;
        }
        if let Some(v) = self.http_protocol_options.as_ref() {
            struct_ser.serialize_field("http_protocol_options", v)?;
        }
        if let Some(v) = self.http2_protocol_options.as_ref() {
            struct_ser.serialize_field("http2_protocol_options", v)?;
        }
        if let Some(v) = self.http3_protocol_options.as_ref() {
            struct_ser.serialize_field("http3_protocol_options", v)?;
        }
        if !self.server_name.is_empty() {
            struct_ser.serialize_field("server_name", &self.server_name)?;
        }
        if self.server_header_transformation != 0 {
            let v = http_connection_manager::ServerHeaderTransformation::try_from(self.server_header_transformation)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.server_header_transformation)))?;
            struct_ser.serialize_field("server_header_transformation", &v)?;
        }
        if let Some(v) = self.scheme_header_transformation.as_ref() {
            struct_ser.serialize_field("scheme_header_transformation", v)?;
        }
        if let Some(v) = self.max_request_headers_kb.as_ref() {
            struct_ser.serialize_field("max_request_headers_kb", v)?;
        }
        if let Some(v) = self.stream_idle_timeout.as_ref() {
            struct_ser.serialize_field("stream_idle_timeout", v)?;
        }
        if let Some(v) = self.request_timeout.as_ref() {
            struct_ser.serialize_field("request_timeout", v)?;
        }
        if let Some(v) = self.request_headers_timeout.as_ref() {
            struct_ser.serialize_field("request_headers_timeout", v)?;
        }
        if let Some(v) = self.drain_timeout.as_ref() {
            struct_ser.serialize_field("drain_timeout", v)?;
        }
        if let Some(v) = self.delayed_close_timeout.as_ref() {
            struct_ser.serialize_field("delayed_close_timeout", v)?;
        }
        if !self.access_log.is_empty() {
            struct_ser.serialize_field("access_log", &self.access_log)?;
        }
        if let Some(v) = self.access_log_flush_interval.as_ref() {
            struct_ser.serialize_field("access_log_flush_interval", v)?;
        }
        if self.flush_access_log_on_new_request {
            struct_ser.serialize_field("flush_access_log_on_new_request", &self.flush_access_log_on_new_request)?;
        }
        if let Some(v) = self.access_log_options.as_ref() {
            struct_ser.serialize_field("access_log_options", v)?;
        }
        if let Some(v) = self.use_remote_address.as_ref() {
            struct_ser.serialize_field("use_remote_address", v)?;
        }
        if self.xff_num_trusted_hops != 0 {
            struct_ser.serialize_field("xff_num_trusted_hops", &self.xff_num_trusted_hops)?;
        }
        if !self.original_ip_detection_extensions.is_empty() {
            struct_ser.serialize_field("original_ip_detection_extensions", &self.original_ip_detection_extensions)?;
        }
        if !self.early_header_mutation_extensions.is_empty() {
            struct_ser.serialize_field("early_header_mutation_extensions", &self.early_header_mutation_extensions)?;
        }
        if let Some(v) = self.internal_address_config.as_ref() {
            struct_ser.serialize_field("internal_address_config", v)?;
        }
        if self.skip_xff_append {
            struct_ser.serialize_field("skip_xff_append", &self.skip_xff_append)?;
        }
        if !self.via.is_empty() {
            struct_ser.serialize_field("via", &self.via)?;
        }
        if let Some(v) = self.generate_request_id.as_ref() {
            struct_ser.serialize_field("generate_request_id", v)?;
        }
        if self.preserve_external_request_id {
            struct_ser.serialize_field("preserve_external_request_id", &self.preserve_external_request_id)?;
        }
        if self.always_set_request_id_in_response {
            struct_ser.serialize_field("always_set_request_id_in_response", &self.always_set_request_id_in_response)?;
        }
        if self.forward_client_cert_details != 0 {
            let v = http_connection_manager::ForwardClientCertDetails::try_from(self.forward_client_cert_details)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.forward_client_cert_details)))?;
            struct_ser.serialize_field("forward_client_cert_details", &v)?;
        }
        if let Some(v) = self.set_current_client_cert_details.as_ref() {
            struct_ser.serialize_field("set_current_client_cert_details", v)?;
        }
        if self.proxy_100_continue {
            struct_ser.serialize_field("proxy_100_continue", &self.proxy_100_continue)?;
        }
        if self.represent_ipv4_remote_address_as_ipv4_mapped_ipv6 {
            struct_ser.serialize_field("represent_ipv4_remote_address_as_ipv4_mapped_ipv6", &self.represent_ipv4_remote_address_as_ipv4_mapped_ipv6)?;
        }
        if !self.upgrade_configs.is_empty() {
            struct_ser.serialize_field("upgrade_configs", &self.upgrade_configs)?;
        }
        if let Some(v) = self.normalize_path.as_ref() {
            struct_ser.serialize_field("normalize_path", v)?;
        }
        if self.merge_slashes {
            struct_ser.serialize_field("merge_slashes", &self.merge_slashes)?;
        }
        if self.path_with_escaped_slashes_action != 0 {
            let v = http_connection_manager::PathWithEscapedSlashesAction::try_from(self.path_with_escaped_slashes_action)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.path_with_escaped_slashes_action)))?;
            struct_ser.serialize_field("path_with_escaped_slashes_action", &v)?;
        }
        if let Some(v) = self.request_id_extension.as_ref() {
            struct_ser.serialize_field("request_id_extension", v)?;
        }
        if let Some(v) = self.local_reply_config.as_ref() {
            struct_ser.serialize_field("local_reply_config", v)?;
        }
        if self.strip_matching_host_port {
            struct_ser.serialize_field("strip_matching_host_port", &self.strip_matching_host_port)?;
        }
        if let Some(v) = self.stream_error_on_invalid_http_message.as_ref() {
            struct_ser.serialize_field("stream_error_on_invalid_http_message", v)?;
        }
        if let Some(v) = self.path_normalization_options.as_ref() {
            struct_ser.serialize_field("path_normalization_options", v)?;
        }
        if self.strip_trailing_host_dot {
            struct_ser.serialize_field("strip_trailing_host_dot", &self.strip_trailing_host_dot)?;
        }
        if let Some(v) = self.proxy_status_config.as_ref() {
            struct_ser.serialize_field("proxy_status_config", v)?;
        }
        if let Some(v) = self.typed_header_validation_config.as_ref() {
            struct_ser.serialize_field("typed_header_validation_config", v)?;
        }
        if self.append_x_forwarded_port {
            struct_ser.serialize_field("append_x_forwarded_port", &self.append_x_forwarded_port)?;
        }
        if self.append_local_overload {
            struct_ser.serialize_field("append_local_overload", &self.append_local_overload)?;
        }
        if let Some(v) = self.add_proxy_protocol_connection_state.as_ref() {
            struct_ser.serialize_field("add_proxy_protocol_connection_state", v)?;
        }
        if let Some(v) = self.route_specifier.as_ref() {
            match v {
                http_connection_manager::RouteSpecifier::Rds(v) => {
                    struct_ser.serialize_field("rds", v)?;
                }
                http_connection_manager::RouteSpecifier::RouteConfig(v) => {
                    struct_ser.serialize_field("route_config", v)?;
                }
                http_connection_manager::RouteSpecifier::ScopedRoutes(v) => {
                    struct_ser.serialize_field("scoped_routes", v)?;
                }
            }
        }
        if let Some(v) = self.strip_port_mode.as_ref() {
            match v {
                http_connection_manager::StripPortMode::StripAnyHostPort(v) => {
                    struct_ser.serialize_field("strip_any_host_port", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpConnectionManager {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "codec_type",
            "codecType",
            "stat_prefix",
            "statPrefix",
            "http_filters",
            "httpFilters",
            "add_user_agent",
            "addUserAgent",
            "tracing",
            "common_http_protocol_options",
            "commonHttpProtocolOptions",
            "http1_safe_max_connection_duration",
            "http1SafeMaxConnectionDuration",
            "http_protocol_options",
            "httpProtocolOptions",
            "http2_protocol_options",
            "http2ProtocolOptions",
            "http3_protocol_options",
            "http3ProtocolOptions",
            "server_name",
            "serverName",
            "server_header_transformation",
            "serverHeaderTransformation",
            "scheme_header_transformation",
            "schemeHeaderTransformation",
            "max_request_headers_kb",
            "maxRequestHeadersKb",
            "stream_idle_timeout",
            "streamIdleTimeout",
            "request_timeout",
            "requestTimeout",
            "request_headers_timeout",
            "requestHeadersTimeout",
            "drain_timeout",
            "drainTimeout",
            "delayed_close_timeout",
            "delayedCloseTimeout",
            "access_log",
            "accessLog",
            "access_log_flush_interval",
            "accessLogFlushInterval",
            "flush_access_log_on_new_request",
            "flushAccessLogOnNewRequest",
            "access_log_options",
            "accessLogOptions",
            "use_remote_address",
            "useRemoteAddress",
            "xff_num_trusted_hops",
            "xffNumTrustedHops",
            "original_ip_detection_extensions",
            "originalIpDetectionExtensions",
            "early_header_mutation_extensions",
            "earlyHeaderMutationExtensions",
            "internal_address_config",
            "internalAddressConfig",
            "skip_xff_append",
            "skipXffAppend",
            "via",
            "generate_request_id",
            "generateRequestId",
            "preserve_external_request_id",
            "preserveExternalRequestId",
            "always_set_request_id_in_response",
            "alwaysSetRequestIdInResponse",
            "forward_client_cert_details",
            "forwardClientCertDetails",
            "set_current_client_cert_details",
            "setCurrentClientCertDetails",
            "proxy_100_continue",
            "proxy100Continue",
            "represent_ipv4_remote_address_as_ipv4_mapped_ipv6",
            "representIpv4RemoteAddressAsIpv4MappedIpv6",
            "upgrade_configs",
            "upgradeConfigs",
            "normalize_path",
            "normalizePath",
            "merge_slashes",
            "mergeSlashes",
            "path_with_escaped_slashes_action",
            "pathWithEscapedSlashesAction",
            "request_id_extension",
            "requestIdExtension",
            "local_reply_config",
            "localReplyConfig",
            "strip_matching_host_port",
            "stripMatchingHostPort",
            "stream_error_on_invalid_http_message",
            "streamErrorOnInvalidHttpMessage",
            "path_normalization_options",
            "pathNormalizationOptions",
            "strip_trailing_host_dot",
            "stripTrailingHostDot",
            "proxy_status_config",
            "proxyStatusConfig",
            "typed_header_validation_config",
            "typedHeaderValidationConfig",
            "append_x_forwarded_port",
            "appendXForwardedPort",
            "append_local_overload",
            "appendLocalOverload",
            "add_proxy_protocol_connection_state",
            "addProxyProtocolConnectionState",
            "rds",
            "route_config",
            "routeConfig",
            "scoped_routes",
            "scopedRoutes",
            "strip_any_host_port",
            "stripAnyHostPort",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodecType,
            StatPrefix,
            HttpFilters,
            AddUserAgent,
            Tracing,
            CommonHttpProtocolOptions,
            Http1SafeMaxConnectionDuration,
            HttpProtocolOptions,
            Http2ProtocolOptions,
            Http3ProtocolOptions,
            ServerName,
            ServerHeaderTransformation,
            SchemeHeaderTransformation,
            MaxRequestHeadersKb,
            StreamIdleTimeout,
            RequestTimeout,
            RequestHeadersTimeout,
            DrainTimeout,
            DelayedCloseTimeout,
            AccessLog,
            AccessLogFlushInterval,
            FlushAccessLogOnNewRequest,
            AccessLogOptions,
            UseRemoteAddress,
            XffNumTrustedHops,
            OriginalIpDetectionExtensions,
            EarlyHeaderMutationExtensions,
            InternalAddressConfig,
            SkipXffAppend,
            Via,
            GenerateRequestId,
            PreserveExternalRequestId,
            AlwaysSetRequestIdInResponse,
            ForwardClientCertDetails,
            SetCurrentClientCertDetails,
            Proxy100Continue,
            RepresentIpv4RemoteAddressAsIpv4MappedIpv6,
            UpgradeConfigs,
            NormalizePath,
            MergeSlashes,
            PathWithEscapedSlashesAction,
            RequestIdExtension,
            LocalReplyConfig,
            StripMatchingHostPort,
            StreamErrorOnInvalidHttpMessage,
            PathNormalizationOptions,
            StripTrailingHostDot,
            ProxyStatusConfig,
            TypedHeaderValidationConfig,
            AppendXForwardedPort,
            AppendLocalOverload,
            AddProxyProtocolConnectionState,
            Rds,
            RouteConfig,
            ScopedRoutes,
            StripAnyHostPort,
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
                            "codecType" | "codec_type" => Ok(GeneratedField::CodecType),
                            "statPrefix" | "stat_prefix" => Ok(GeneratedField::StatPrefix),
                            "httpFilters" | "http_filters" => Ok(GeneratedField::HttpFilters),
                            "addUserAgent" | "add_user_agent" => Ok(GeneratedField::AddUserAgent),
                            "tracing" => Ok(GeneratedField::Tracing),
                            "commonHttpProtocolOptions" | "common_http_protocol_options" => Ok(GeneratedField::CommonHttpProtocolOptions),
                            "http1SafeMaxConnectionDuration" | "http1_safe_max_connection_duration" => Ok(GeneratedField::Http1SafeMaxConnectionDuration),
                            "httpProtocolOptions" | "http_protocol_options" => Ok(GeneratedField::HttpProtocolOptions),
                            "http2ProtocolOptions" | "http2_protocol_options" => Ok(GeneratedField::Http2ProtocolOptions),
                            "http3ProtocolOptions" | "http3_protocol_options" => Ok(GeneratedField::Http3ProtocolOptions),
                            "serverName" | "server_name" => Ok(GeneratedField::ServerName),
                            "serverHeaderTransformation" | "server_header_transformation" => Ok(GeneratedField::ServerHeaderTransformation),
                            "schemeHeaderTransformation" | "scheme_header_transformation" => Ok(GeneratedField::SchemeHeaderTransformation),
                            "maxRequestHeadersKb" | "max_request_headers_kb" => Ok(GeneratedField::MaxRequestHeadersKb),
                            "streamIdleTimeout" | "stream_idle_timeout" => Ok(GeneratedField::StreamIdleTimeout),
                            "requestTimeout" | "request_timeout" => Ok(GeneratedField::RequestTimeout),
                            "requestHeadersTimeout" | "request_headers_timeout" => Ok(GeneratedField::RequestHeadersTimeout),
                            "drainTimeout" | "drain_timeout" => Ok(GeneratedField::DrainTimeout),
                            "delayedCloseTimeout" | "delayed_close_timeout" => Ok(GeneratedField::DelayedCloseTimeout),
                            "accessLog" | "access_log" => Ok(GeneratedField::AccessLog),
                            "accessLogFlushInterval" | "access_log_flush_interval" => Ok(GeneratedField::AccessLogFlushInterval),
                            "flushAccessLogOnNewRequest" | "flush_access_log_on_new_request" => Ok(GeneratedField::FlushAccessLogOnNewRequest),
                            "accessLogOptions" | "access_log_options" => Ok(GeneratedField::AccessLogOptions),
                            "useRemoteAddress" | "use_remote_address" => Ok(GeneratedField::UseRemoteAddress),
                            "xffNumTrustedHops" | "xff_num_trusted_hops" => Ok(GeneratedField::XffNumTrustedHops),
                            "originalIpDetectionExtensions" | "original_ip_detection_extensions" => Ok(GeneratedField::OriginalIpDetectionExtensions),
                            "earlyHeaderMutationExtensions" | "early_header_mutation_extensions" => Ok(GeneratedField::EarlyHeaderMutationExtensions),
                            "internalAddressConfig" | "internal_address_config" => Ok(GeneratedField::InternalAddressConfig),
                            "skipXffAppend" | "skip_xff_append" => Ok(GeneratedField::SkipXffAppend),
                            "via" => Ok(GeneratedField::Via),
                            "generateRequestId" | "generate_request_id" => Ok(GeneratedField::GenerateRequestId),
                            "preserveExternalRequestId" | "preserve_external_request_id" => Ok(GeneratedField::PreserveExternalRequestId),
                            "alwaysSetRequestIdInResponse" | "always_set_request_id_in_response" => Ok(GeneratedField::AlwaysSetRequestIdInResponse),
                            "forwardClientCertDetails" | "forward_client_cert_details" => Ok(GeneratedField::ForwardClientCertDetails),
                            "setCurrentClientCertDetails" | "set_current_client_cert_details" => Ok(GeneratedField::SetCurrentClientCertDetails),
                            "proxy100Continue" | "proxy_100_continue" => Ok(GeneratedField::Proxy100Continue),
                            "representIpv4RemoteAddressAsIpv4MappedIpv6" | "represent_ipv4_remote_address_as_ipv4_mapped_ipv6" => Ok(GeneratedField::RepresentIpv4RemoteAddressAsIpv4MappedIpv6),
                            "upgradeConfigs" | "upgrade_configs" => Ok(GeneratedField::UpgradeConfigs),
                            "normalizePath" | "normalize_path" => Ok(GeneratedField::NormalizePath),
                            "mergeSlashes" | "merge_slashes" => Ok(GeneratedField::MergeSlashes),
                            "pathWithEscapedSlashesAction" | "path_with_escaped_slashes_action" => Ok(GeneratedField::PathWithEscapedSlashesAction),
                            "requestIdExtension" | "request_id_extension" => Ok(GeneratedField::RequestIdExtension),
                            "localReplyConfig" | "local_reply_config" => Ok(GeneratedField::LocalReplyConfig),
                            "stripMatchingHostPort" | "strip_matching_host_port" => Ok(GeneratedField::StripMatchingHostPort),
                            "streamErrorOnInvalidHttpMessage" | "stream_error_on_invalid_http_message" => Ok(GeneratedField::StreamErrorOnInvalidHttpMessage),
                            "pathNormalizationOptions" | "path_normalization_options" => Ok(GeneratedField::PathNormalizationOptions),
                            "stripTrailingHostDot" | "strip_trailing_host_dot" => Ok(GeneratedField::StripTrailingHostDot),
                            "proxyStatusConfig" | "proxy_status_config" => Ok(GeneratedField::ProxyStatusConfig),
                            "typedHeaderValidationConfig" | "typed_header_validation_config" => Ok(GeneratedField::TypedHeaderValidationConfig),
                            "appendXForwardedPort" | "append_x_forwarded_port" => Ok(GeneratedField::AppendXForwardedPort),
                            "appendLocalOverload" | "append_local_overload" => Ok(GeneratedField::AppendLocalOverload),
                            "addProxyProtocolConnectionState" | "add_proxy_protocol_connection_state" => Ok(GeneratedField::AddProxyProtocolConnectionState),
                            "rds" => Ok(GeneratedField::Rds),
                            "routeConfig" | "route_config" => Ok(GeneratedField::RouteConfig),
                            "scopedRoutes" | "scoped_routes" => Ok(GeneratedField::ScopedRoutes),
                            "stripAnyHostPort" | "strip_any_host_port" => Ok(GeneratedField::StripAnyHostPort),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpConnectionManager;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpConnectionManager, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut codec_type__ = None;
                let mut stat_prefix__ = None;
                let mut http_filters__ = None;
                let mut add_user_agent__ = None;
                let mut tracing__ = None;
                let mut common_http_protocol_options__ = None;
                let mut http1_safe_max_connection_duration__ = None;
                let mut http_protocol_options__ = None;
                let mut http2_protocol_options__ = None;
                let mut http3_protocol_options__ = None;
                let mut server_name__ = None;
                let mut server_header_transformation__ = None;
                let mut scheme_header_transformation__ = None;
                let mut max_request_headers_kb__ = None;
                let mut stream_idle_timeout__ = None;
                let mut request_timeout__ = None;
                let mut request_headers_timeout__ = None;
                let mut drain_timeout__ = None;
                let mut delayed_close_timeout__ = None;
                let mut access_log__ = None;
                let mut access_log_flush_interval__ = None;
                let mut flush_access_log_on_new_request__ = None;
                let mut access_log_options__ = None;
                let mut use_remote_address__ = None;
                let mut xff_num_trusted_hops__ = None;
                let mut original_ip_detection_extensions__ = None;
                let mut early_header_mutation_extensions__ = None;
                let mut internal_address_config__ = None;
                let mut skip_xff_append__ = None;
                let mut via__ = None;
                let mut generate_request_id__ = None;
                let mut preserve_external_request_id__ = None;
                let mut always_set_request_id_in_response__ = None;
                let mut forward_client_cert_details__ = None;
                let mut set_current_client_cert_details__ = None;
                let mut proxy_100_continue__ = None;
                let mut represent_ipv4_remote_address_as_ipv4_mapped_ipv6__ = None;
                let mut upgrade_configs__ = None;
                let mut normalize_path__ = None;
                let mut merge_slashes__ = None;
                let mut path_with_escaped_slashes_action__ = None;
                let mut request_id_extension__ = None;
                let mut local_reply_config__ = None;
                let mut strip_matching_host_port__ = None;
                let mut stream_error_on_invalid_http_message__ = None;
                let mut path_normalization_options__ = None;
                let mut strip_trailing_host_dot__ = None;
                let mut proxy_status_config__ = None;
                let mut typed_header_validation_config__ = None;
                let mut append_x_forwarded_port__ = None;
                let mut append_local_overload__ = None;
                let mut add_proxy_protocol_connection_state__ = None;
                let mut route_specifier__ = None;
                let mut strip_port_mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodecType => {
                            if codec_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codecType"));
                            }
                            codec_type__ = Some(map_.next_value::<http_connection_manager::CodecType>()? as i32);
                        }
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HttpFilters => {
                            if http_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpFilters"));
                            }
                            http_filters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AddUserAgent => {
                            if add_user_agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addUserAgent"));
                            }
                            add_user_agent__ = map_.next_value()?;
                        }
                        GeneratedField::Tracing => {
                            if tracing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracing"));
                            }
                            tracing__ = map_.next_value()?;
                        }
                        GeneratedField::CommonHttpProtocolOptions => {
                            if common_http_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonHttpProtocolOptions"));
                            }
                            common_http_protocol_options__ = map_.next_value()?;
                        }
                        GeneratedField::Http1SafeMaxConnectionDuration => {
                            if http1_safe_max_connection_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http1SafeMaxConnectionDuration"));
                            }
                            http1_safe_max_connection_duration__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HttpProtocolOptions => {
                            if http_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpProtocolOptions"));
                            }
                            http_protocol_options__ = map_.next_value()?;
                        }
                        GeneratedField::Http2ProtocolOptions => {
                            if http2_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http2ProtocolOptions"));
                            }
                            http2_protocol_options__ = map_.next_value()?;
                        }
                        GeneratedField::Http3ProtocolOptions => {
                            if http3_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http3ProtocolOptions"));
                            }
                            http3_protocol_options__ = map_.next_value()?;
                        }
                        GeneratedField::ServerName => {
                            if server_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverName"));
                            }
                            server_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServerHeaderTransformation => {
                            if server_header_transformation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverHeaderTransformation"));
                            }
                            server_header_transformation__ = Some(map_.next_value::<http_connection_manager::ServerHeaderTransformation>()? as i32);
                        }
                        GeneratedField::SchemeHeaderTransformation => {
                            if scheme_header_transformation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemeHeaderTransformation"));
                            }
                            scheme_header_transformation__ = map_.next_value()?;
                        }
                        GeneratedField::MaxRequestHeadersKb => {
                            if max_request_headers_kb__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRequestHeadersKb"));
                            }
                            max_request_headers_kb__ = map_.next_value()?;
                        }
                        GeneratedField::StreamIdleTimeout => {
                            if stream_idle_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("streamIdleTimeout"));
                            }
                            stream_idle_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::RequestTimeout => {
                            if request_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestTimeout"));
                            }
                            request_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::RequestHeadersTimeout => {
                            if request_headers_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersTimeout"));
                            }
                            request_headers_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::DrainTimeout => {
                            if drain_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drainTimeout"));
                            }
                            drain_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::DelayedCloseTimeout => {
                            if delayed_close_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delayedCloseTimeout"));
                            }
                            delayed_close_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::AccessLog => {
                            if access_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessLog"));
                            }
                            access_log__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AccessLogFlushInterval => {
                            if access_log_flush_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessLogFlushInterval"));
                            }
                            access_log_flush_interval__ = map_.next_value()?;
                        }
                        GeneratedField::FlushAccessLogOnNewRequest => {
                            if flush_access_log_on_new_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flushAccessLogOnNewRequest"));
                            }
                            flush_access_log_on_new_request__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AccessLogOptions => {
                            if access_log_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessLogOptions"));
                            }
                            access_log_options__ = map_.next_value()?;
                        }
                        GeneratedField::UseRemoteAddress => {
                            if use_remote_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useRemoteAddress"));
                            }
                            use_remote_address__ = map_.next_value()?;
                        }
                        GeneratedField::XffNumTrustedHops => {
                            if xff_num_trusted_hops__.is_some() {
                                return Err(serde::de::Error::duplicate_field("xffNumTrustedHops"));
                            }
                            xff_num_trusted_hops__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OriginalIpDetectionExtensions => {
                            if original_ip_detection_extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originalIpDetectionExtensions"));
                            }
                            original_ip_detection_extensions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EarlyHeaderMutationExtensions => {
                            if early_header_mutation_extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("earlyHeaderMutationExtensions"));
                            }
                            early_header_mutation_extensions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InternalAddressConfig => {
                            if internal_address_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("internalAddressConfig"));
                            }
                            internal_address_config__ = map_.next_value()?;
                        }
                        GeneratedField::SkipXffAppend => {
                            if skip_xff_append__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipXffAppend"));
                            }
                            skip_xff_append__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Via => {
                            if via__.is_some() {
                                return Err(serde::de::Error::duplicate_field("via"));
                            }
                            via__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GenerateRequestId => {
                            if generate_request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("generateRequestId"));
                            }
                            generate_request_id__ = map_.next_value()?;
                        }
                        GeneratedField::PreserveExternalRequestId => {
                            if preserve_external_request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preserveExternalRequestId"));
                            }
                            preserve_external_request_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AlwaysSetRequestIdInResponse => {
                            if always_set_request_id_in_response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alwaysSetRequestIdInResponse"));
                            }
                            always_set_request_id_in_response__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ForwardClientCertDetails => {
                            if forward_client_cert_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forwardClientCertDetails"));
                            }
                            forward_client_cert_details__ = Some(map_.next_value::<http_connection_manager::ForwardClientCertDetails>()? as i32);
                        }
                        GeneratedField::SetCurrentClientCertDetails => {
                            if set_current_client_cert_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("setCurrentClientCertDetails"));
                            }
                            set_current_client_cert_details__ = map_.next_value()?;
                        }
                        GeneratedField::Proxy100Continue => {
                            if proxy_100_continue__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proxy100Continue"));
                            }
                            proxy_100_continue__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RepresentIpv4RemoteAddressAsIpv4MappedIpv6 => {
                            if represent_ipv4_remote_address_as_ipv4_mapped_ipv6__.is_some() {
                                return Err(serde::de::Error::duplicate_field("representIpv4RemoteAddressAsIpv4MappedIpv6"));
                            }
                            represent_ipv4_remote_address_as_ipv4_mapped_ipv6__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpgradeConfigs => {
                            if upgrade_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upgradeConfigs"));
                            }
                            upgrade_configs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NormalizePath => {
                            if normalize_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("normalizePath"));
                            }
                            normalize_path__ = map_.next_value()?;
                        }
                        GeneratedField::MergeSlashes => {
                            if merge_slashes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mergeSlashes"));
                            }
                            merge_slashes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PathWithEscapedSlashesAction => {
                            if path_with_escaped_slashes_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathWithEscapedSlashesAction"));
                            }
                            path_with_escaped_slashes_action__ = Some(map_.next_value::<http_connection_manager::PathWithEscapedSlashesAction>()? as i32);
                        }
                        GeneratedField::RequestIdExtension => {
                            if request_id_extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestIdExtension"));
                            }
                            request_id_extension__ = map_.next_value()?;
                        }
                        GeneratedField::LocalReplyConfig => {
                            if local_reply_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localReplyConfig"));
                            }
                            local_reply_config__ = map_.next_value()?;
                        }
                        GeneratedField::StripMatchingHostPort => {
                            if strip_matching_host_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripMatchingHostPort"));
                            }
                            strip_matching_host_port__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StreamErrorOnInvalidHttpMessage => {
                            if stream_error_on_invalid_http_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("streamErrorOnInvalidHttpMessage"));
                            }
                            stream_error_on_invalid_http_message__ = map_.next_value()?;
                        }
                        GeneratedField::PathNormalizationOptions => {
                            if path_normalization_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathNormalizationOptions"));
                            }
                            path_normalization_options__ = map_.next_value()?;
                        }
                        GeneratedField::StripTrailingHostDot => {
                            if strip_trailing_host_dot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripTrailingHostDot"));
                            }
                            strip_trailing_host_dot__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProxyStatusConfig => {
                            if proxy_status_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proxyStatusConfig"));
                            }
                            proxy_status_config__ = map_.next_value()?;
                        }
                        GeneratedField::TypedHeaderValidationConfig => {
                            if typed_header_validation_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedHeaderValidationConfig"));
                            }
                            typed_header_validation_config__ = map_.next_value()?;
                        }
                        GeneratedField::AppendXForwardedPort => {
                            if append_x_forwarded_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appendXForwardedPort"));
                            }
                            append_x_forwarded_port__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppendLocalOverload => {
                            if append_local_overload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appendLocalOverload"));
                            }
                            append_local_overload__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AddProxyProtocolConnectionState => {
                            if add_proxy_protocol_connection_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addProxyProtocolConnectionState"));
                            }
                            add_proxy_protocol_connection_state__ = map_.next_value()?;
                        }
                        GeneratedField::Rds => {
                            if route_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rds"));
                            }
                            route_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(http_connection_manager::RouteSpecifier::Rds)
;
                        }
                        GeneratedField::RouteConfig => {
                            if route_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeConfig"));
                            }
                            route_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(http_connection_manager::RouteSpecifier::RouteConfig)
;
                        }
                        GeneratedField::ScopedRoutes => {
                            if route_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopedRoutes"));
                            }
                            route_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(http_connection_manager::RouteSpecifier::ScopedRoutes)
;
                        }
                        GeneratedField::StripAnyHostPort => {
                            if strip_port_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripAnyHostPort"));
                            }
                            strip_port_mode__ = map_.next_value::<::std::option::Option<_>>()?.map(http_connection_manager::StripPortMode::StripAnyHostPort);
                        }
                    }
                }
                Ok(HttpConnectionManager {
                    codec_type: codec_type__.unwrap_or_default(),
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    http_filters: http_filters__.unwrap_or_default(),
                    add_user_agent: add_user_agent__,
                    tracing: tracing__,
                    common_http_protocol_options: common_http_protocol_options__,
                    http1_safe_max_connection_duration: http1_safe_max_connection_duration__.unwrap_or_default(),
                    http_protocol_options: http_protocol_options__,
                    http2_protocol_options: http2_protocol_options__,
                    http3_protocol_options: http3_protocol_options__,
                    server_name: server_name__.unwrap_or_default(),
                    server_header_transformation: server_header_transformation__.unwrap_or_default(),
                    scheme_header_transformation: scheme_header_transformation__,
                    max_request_headers_kb: max_request_headers_kb__,
                    stream_idle_timeout: stream_idle_timeout__,
                    request_timeout: request_timeout__,
                    request_headers_timeout: request_headers_timeout__,
                    drain_timeout: drain_timeout__,
                    delayed_close_timeout: delayed_close_timeout__,
                    access_log: access_log__.unwrap_or_default(),
                    access_log_flush_interval: access_log_flush_interval__,
                    flush_access_log_on_new_request: flush_access_log_on_new_request__.unwrap_or_default(),
                    access_log_options: access_log_options__,
                    use_remote_address: use_remote_address__,
                    xff_num_trusted_hops: xff_num_trusted_hops__.unwrap_or_default(),
                    original_ip_detection_extensions: original_ip_detection_extensions__.unwrap_or_default(),
                    early_header_mutation_extensions: early_header_mutation_extensions__.unwrap_or_default(),
                    internal_address_config: internal_address_config__,
                    skip_xff_append: skip_xff_append__.unwrap_or_default(),
                    via: via__.unwrap_or_default(),
                    generate_request_id: generate_request_id__,
                    preserve_external_request_id: preserve_external_request_id__.unwrap_or_default(),
                    always_set_request_id_in_response: always_set_request_id_in_response__.unwrap_or_default(),
                    forward_client_cert_details: forward_client_cert_details__.unwrap_or_default(),
                    set_current_client_cert_details: set_current_client_cert_details__,
                    proxy_100_continue: proxy_100_continue__.unwrap_or_default(),
                    represent_ipv4_remote_address_as_ipv4_mapped_ipv6: represent_ipv4_remote_address_as_ipv4_mapped_ipv6__.unwrap_or_default(),
                    upgrade_configs: upgrade_configs__.unwrap_or_default(),
                    normalize_path: normalize_path__,
                    merge_slashes: merge_slashes__.unwrap_or_default(),
                    path_with_escaped_slashes_action: path_with_escaped_slashes_action__.unwrap_or_default(),
                    request_id_extension: request_id_extension__,
                    local_reply_config: local_reply_config__,
                    strip_matching_host_port: strip_matching_host_port__.unwrap_or_default(),
                    stream_error_on_invalid_http_message: stream_error_on_invalid_http_message__,
                    path_normalization_options: path_normalization_options__,
                    strip_trailing_host_dot: strip_trailing_host_dot__.unwrap_or_default(),
                    proxy_status_config: proxy_status_config__,
                    typed_header_validation_config: typed_header_validation_config__,
                    append_x_forwarded_port: append_x_forwarded_port__.unwrap_or_default(),
                    append_local_overload: append_local_overload__.unwrap_or_default(),
                    add_proxy_protocol_connection_state: add_proxy_protocol_connection_state__,
                    route_specifier: route_specifier__,
                    strip_port_mode: strip_port_mode__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::CodecType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Auto => "AUTO",
            Self::Http1 => "HTTP1",
            Self::Http2 => "HTTP2",
            Self::Http3 => "HTTP3",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::CodecType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUTO",
            "HTTP1",
            "HTTP2",
            "HTTP3",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::CodecType;

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
                    "AUTO" => Ok(http_connection_manager::CodecType::Auto),
                    "HTTP1" => Ok(http_connection_manager::CodecType::Http1),
                    "HTTP2" => Ok(http_connection_manager::CodecType::Http2),
                    "HTTP3" => Ok(http_connection_manager::CodecType::Http3),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::ForwardClientCertDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Sanitize => "SANITIZE",
            Self::ForwardOnly => "FORWARD_ONLY",
            Self::AppendForward => "APPEND_FORWARD",
            Self::SanitizeSet => "SANITIZE_SET",
            Self::AlwaysForwardOnly => "ALWAYS_FORWARD_ONLY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::ForwardClientCertDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SANITIZE",
            "FORWARD_ONLY",
            "APPEND_FORWARD",
            "SANITIZE_SET",
            "ALWAYS_FORWARD_ONLY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::ForwardClientCertDetails;

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
                    "SANITIZE" => Ok(http_connection_manager::ForwardClientCertDetails::Sanitize),
                    "FORWARD_ONLY" => Ok(http_connection_manager::ForwardClientCertDetails::ForwardOnly),
                    "APPEND_FORWARD" => Ok(http_connection_manager::ForwardClientCertDetails::AppendForward),
                    "SANITIZE_SET" => Ok(http_connection_manager::ForwardClientCertDetails::SanitizeSet),
                    "ALWAYS_FORWARD_ONLY" => Ok(http_connection_manager::ForwardClientCertDetails::AlwaysForwardOnly),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::HcmAccessLogOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.access_log_flush_interval.is_some() {
            len += 1;
        }
        if self.flush_access_log_on_new_request {
            len += 1;
        }
        if self.flush_log_on_tunnel_successfully_established {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.HcmAccessLogOptions", len)?;
        if let Some(v) = self.access_log_flush_interval.as_ref() {
            struct_ser.serialize_field("access_log_flush_interval", v)?;
        }
        if self.flush_access_log_on_new_request {
            struct_ser.serialize_field("flush_access_log_on_new_request", &self.flush_access_log_on_new_request)?;
        }
        if self.flush_log_on_tunnel_successfully_established {
            struct_ser.serialize_field("flush_log_on_tunnel_successfully_established", &self.flush_log_on_tunnel_successfully_established)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::HcmAccessLogOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "access_log_flush_interval",
            "accessLogFlushInterval",
            "flush_access_log_on_new_request",
            "flushAccessLogOnNewRequest",
            "flush_log_on_tunnel_successfully_established",
            "flushLogOnTunnelSuccessfullyEstablished",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccessLogFlushInterval,
            FlushAccessLogOnNewRequest,
            FlushLogOnTunnelSuccessfullyEstablished,
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
                            "accessLogFlushInterval" | "access_log_flush_interval" => Ok(GeneratedField::AccessLogFlushInterval),
                            "flushAccessLogOnNewRequest" | "flush_access_log_on_new_request" => Ok(GeneratedField::FlushAccessLogOnNewRequest),
                            "flushLogOnTunnelSuccessfullyEstablished" | "flush_log_on_tunnel_successfully_established" => Ok(GeneratedField::FlushLogOnTunnelSuccessfullyEstablished),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::HcmAccessLogOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.HcmAccessLogOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<http_connection_manager::HcmAccessLogOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut access_log_flush_interval__ = None;
                let mut flush_access_log_on_new_request__ = None;
                let mut flush_log_on_tunnel_successfully_established__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccessLogFlushInterval => {
                            if access_log_flush_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessLogFlushInterval"));
                            }
                            access_log_flush_interval__ = map_.next_value()?;
                        }
                        GeneratedField::FlushAccessLogOnNewRequest => {
                            if flush_access_log_on_new_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flushAccessLogOnNewRequest"));
                            }
                            flush_access_log_on_new_request__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FlushLogOnTunnelSuccessfullyEstablished => {
                            if flush_log_on_tunnel_successfully_established__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flushLogOnTunnelSuccessfullyEstablished"));
                            }
                            flush_log_on_tunnel_successfully_established__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(http_connection_manager::HcmAccessLogOptions {
                    access_log_flush_interval: access_log_flush_interval__,
                    flush_access_log_on_new_request: flush_access_log_on_new_request__.unwrap_or_default(),
                    flush_log_on_tunnel_successfully_established: flush_log_on_tunnel_successfully_established__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.HcmAccessLogOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::InternalAddressConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.unix_sockets {
            len += 1;
        }
        if !self.cidr_ranges.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.InternalAddressConfig", len)?;
        if self.unix_sockets {
            struct_ser.serialize_field("unix_sockets", &self.unix_sockets)?;
        }
        if !self.cidr_ranges.is_empty() {
            struct_ser.serialize_field("cidr_ranges", &self.cidr_ranges)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::InternalAddressConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "unix_sockets",
            "unixSockets",
            "cidr_ranges",
            "cidrRanges",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UnixSockets,
            CidrRanges,
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
                            "unixSockets" | "unix_sockets" => Ok(GeneratedField::UnixSockets),
                            "cidrRanges" | "cidr_ranges" => Ok(GeneratedField::CidrRanges),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::InternalAddressConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.InternalAddressConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<http_connection_manager::InternalAddressConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut unix_sockets__ = None;
                let mut cidr_ranges__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UnixSockets => {
                            if unix_sockets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unixSockets"));
                            }
                            unix_sockets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CidrRanges => {
                            if cidr_ranges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cidrRanges"));
                            }
                            cidr_ranges__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(http_connection_manager::InternalAddressConfig {
                    unix_sockets: unix_sockets__.unwrap_or_default(),
                    cidr_ranges: cidr_ranges__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.InternalAddressConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::PathNormalizationOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.forwarding_transformation.is_some() {
            len += 1;
        }
        if self.http_filter_transformation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.PathNormalizationOptions", len)?;
        if let Some(v) = self.forwarding_transformation.as_ref() {
            struct_ser.serialize_field("forwarding_transformation", v)?;
        }
        if let Some(v) = self.http_filter_transformation.as_ref() {
            struct_ser.serialize_field("http_filter_transformation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::PathNormalizationOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "forwarding_transformation",
            "forwardingTransformation",
            "http_filter_transformation",
            "httpFilterTransformation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ForwardingTransformation,
            HttpFilterTransformation,
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
                            "forwardingTransformation" | "forwarding_transformation" => Ok(GeneratedField::ForwardingTransformation),
                            "httpFilterTransformation" | "http_filter_transformation" => Ok(GeneratedField::HttpFilterTransformation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::PathNormalizationOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.PathNormalizationOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<http_connection_manager::PathNormalizationOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut forwarding_transformation__ = None;
                let mut http_filter_transformation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ForwardingTransformation => {
                            if forwarding_transformation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forwardingTransformation"));
                            }
                            forwarding_transformation__ = map_.next_value()?;
                        }
                        GeneratedField::HttpFilterTransformation => {
                            if http_filter_transformation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpFilterTransformation"));
                            }
                            http_filter_transformation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(http_connection_manager::PathNormalizationOptions {
                    forwarding_transformation: forwarding_transformation__,
                    http_filter_transformation: http_filter_transformation__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.PathNormalizationOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::PathWithEscapedSlashesAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::ImplementationSpecificDefault => "IMPLEMENTATION_SPECIFIC_DEFAULT",
            Self::KeepUnchanged => "KEEP_UNCHANGED",
            Self::RejectRequest => "REJECT_REQUEST",
            Self::UnescapeAndRedirect => "UNESCAPE_AND_REDIRECT",
            Self::UnescapeAndForward => "UNESCAPE_AND_FORWARD",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::PathWithEscapedSlashesAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "IMPLEMENTATION_SPECIFIC_DEFAULT",
            "KEEP_UNCHANGED",
            "REJECT_REQUEST",
            "UNESCAPE_AND_REDIRECT",
            "UNESCAPE_AND_FORWARD",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::PathWithEscapedSlashesAction;

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
                    "IMPLEMENTATION_SPECIFIC_DEFAULT" => Ok(http_connection_manager::PathWithEscapedSlashesAction::ImplementationSpecificDefault),
                    "KEEP_UNCHANGED" => Ok(http_connection_manager::PathWithEscapedSlashesAction::KeepUnchanged),
                    "REJECT_REQUEST" => Ok(http_connection_manager::PathWithEscapedSlashesAction::RejectRequest),
                    "UNESCAPE_AND_REDIRECT" => Ok(http_connection_manager::PathWithEscapedSlashesAction::UnescapeAndRedirect),
                    "UNESCAPE_AND_FORWARD" => Ok(http_connection_manager::PathWithEscapedSlashesAction::UnescapeAndForward),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::ProxyStatusConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.remove_details {
            len += 1;
        }
        if self.remove_connection_termination_details {
            len += 1;
        }
        if self.remove_response_flags {
            len += 1;
        }
        if self.set_recommended_response_code {
            len += 1;
        }
        if self.proxy_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.ProxyStatusConfig", len)?;
        if self.remove_details {
            struct_ser.serialize_field("remove_details", &self.remove_details)?;
        }
        if self.remove_connection_termination_details {
            struct_ser.serialize_field("remove_connection_termination_details", &self.remove_connection_termination_details)?;
        }
        if self.remove_response_flags {
            struct_ser.serialize_field("remove_response_flags", &self.remove_response_flags)?;
        }
        if self.set_recommended_response_code {
            struct_ser.serialize_field("set_recommended_response_code", &self.set_recommended_response_code)?;
        }
        if let Some(v) = self.proxy_name.as_ref() {
            match v {
                http_connection_manager::proxy_status_config::ProxyName::UseNodeId(v) => {
                    struct_ser.serialize_field("use_node_id", v)?;
                }
                http_connection_manager::proxy_status_config::ProxyName::LiteralProxyName(v) => {
                    struct_ser.serialize_field("literal_proxy_name", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::ProxyStatusConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remove_details",
            "removeDetails",
            "remove_connection_termination_details",
            "removeConnectionTerminationDetails",
            "remove_response_flags",
            "removeResponseFlags",
            "set_recommended_response_code",
            "setRecommendedResponseCode",
            "use_node_id",
            "useNodeId",
            "literal_proxy_name",
            "literalProxyName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemoveDetails,
            RemoveConnectionTerminationDetails,
            RemoveResponseFlags,
            SetRecommendedResponseCode,
            UseNodeId,
            LiteralProxyName,
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
                            "removeDetails" | "remove_details" => Ok(GeneratedField::RemoveDetails),
                            "removeConnectionTerminationDetails" | "remove_connection_termination_details" => Ok(GeneratedField::RemoveConnectionTerminationDetails),
                            "removeResponseFlags" | "remove_response_flags" => Ok(GeneratedField::RemoveResponseFlags),
                            "setRecommendedResponseCode" | "set_recommended_response_code" => Ok(GeneratedField::SetRecommendedResponseCode),
                            "useNodeId" | "use_node_id" => Ok(GeneratedField::UseNodeId),
                            "literalProxyName" | "literal_proxy_name" => Ok(GeneratedField::LiteralProxyName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::ProxyStatusConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.ProxyStatusConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<http_connection_manager::ProxyStatusConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut remove_details__ = None;
                let mut remove_connection_termination_details__ = None;
                let mut remove_response_flags__ = None;
                let mut set_recommended_response_code__ = None;
                let mut proxy_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RemoveDetails => {
                            if remove_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("removeDetails"));
                            }
                            remove_details__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RemoveConnectionTerminationDetails => {
                            if remove_connection_termination_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("removeConnectionTerminationDetails"));
                            }
                            remove_connection_termination_details__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RemoveResponseFlags => {
                            if remove_response_flags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("removeResponseFlags"));
                            }
                            remove_response_flags__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SetRecommendedResponseCode => {
                            if set_recommended_response_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("setRecommendedResponseCode"));
                            }
                            set_recommended_response_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UseNodeId => {
                            if proxy_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useNodeId"));
                            }
                            proxy_name__ = map_.next_value::<::std::option::Option<_>>()?.map(http_connection_manager::proxy_status_config::ProxyName::UseNodeId);
                        }
                        GeneratedField::LiteralProxyName => {
                            if proxy_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("literalProxyName"));
                            }
                            proxy_name__ = map_.next_value::<::std::option::Option<_>>()?.map(http_connection_manager::proxy_status_config::ProxyName::LiteralProxyName);
                        }
                    }
                }
                Ok(http_connection_manager::ProxyStatusConfig {
                    remove_details: remove_details__.unwrap_or_default(),
                    remove_connection_termination_details: remove_connection_termination_details__.unwrap_or_default(),
                    remove_response_flags: remove_response_flags__.unwrap_or_default(),
                    set_recommended_response_code: set_recommended_response_code__.unwrap_or_default(),
                    proxy_name: proxy_name__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.ProxyStatusConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::ServerHeaderTransformation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Overwrite => "OVERWRITE",
            Self::AppendIfAbsent => "APPEND_IF_ABSENT",
            Self::PassThrough => "PASS_THROUGH",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::ServerHeaderTransformation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OVERWRITE",
            "APPEND_IF_ABSENT",
            "PASS_THROUGH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::ServerHeaderTransformation;

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
                    "OVERWRITE" => Ok(http_connection_manager::ServerHeaderTransformation::Overwrite),
                    "APPEND_IF_ABSENT" => Ok(http_connection_manager::ServerHeaderTransformation::AppendIfAbsent),
                    "PASS_THROUGH" => Ok(http_connection_manager::ServerHeaderTransformation::PassThrough),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::SetCurrentClientCertDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subject.is_some() {
            len += 1;
        }
        if self.cert {
            len += 1;
        }
        if self.chain {
            len += 1;
        }
        if self.dns {
            len += 1;
        }
        if self.uri {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.SetCurrentClientCertDetails", len)?;
        if let Some(v) = self.subject.as_ref() {
            struct_ser.serialize_field("subject", v)?;
        }
        if self.cert {
            struct_ser.serialize_field("cert", &self.cert)?;
        }
        if self.chain {
            struct_ser.serialize_field("chain", &self.chain)?;
        }
        if self.dns {
            struct_ser.serialize_field("dns", &self.dns)?;
        }
        if self.uri {
            struct_ser.serialize_field("uri", &self.uri)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::SetCurrentClientCertDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subject",
            "cert",
            "chain",
            "dns",
            "uri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subject,
            Cert,
            Chain,
            Dns,
            Uri,
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
                            "subject" => Ok(GeneratedField::Subject),
                            "cert" => Ok(GeneratedField::Cert),
                            "chain" => Ok(GeneratedField::Chain),
                            "dns" => Ok(GeneratedField::Dns),
                            "uri" => Ok(GeneratedField::Uri),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::SetCurrentClientCertDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.SetCurrentClientCertDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<http_connection_manager::SetCurrentClientCertDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subject__ = None;
                let mut cert__ = None;
                let mut chain__ = None;
                let mut dns__ = None;
                let mut uri__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = map_.next_value()?;
                        }
                        GeneratedField::Cert => {
                            if cert__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cert"));
                            }
                            cert__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Chain => {
                            if chain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chain"));
                            }
                            chain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Dns => {
                            if dns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dns"));
                            }
                            dns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Uri => {
                            if uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            uri__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(http_connection_manager::SetCurrentClientCertDetails {
                    subject: subject__,
                    cert: cert__.unwrap_or_default(),
                    chain: chain__.unwrap_or_default(),
                    dns: dns__.unwrap_or_default(),
                    uri: uri__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.SetCurrentClientCertDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::Tracing {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.client_sampling.is_some() {
            len += 1;
        }
        if self.random_sampling.is_some() {
            len += 1;
        }
        if self.overall_sampling.is_some() {
            len += 1;
        }
        if self.verbose {
            len += 1;
        }
        if self.max_path_tag_length.is_some() {
            len += 1;
        }
        if !self.custom_tags.is_empty() {
            len += 1;
        }
        if self.provider.is_some() {
            len += 1;
        }
        if self.spawn_upstream_span.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.Tracing", len)?;
        if let Some(v) = self.client_sampling.as_ref() {
            struct_ser.serialize_field("client_sampling", v)?;
        }
        if let Some(v) = self.random_sampling.as_ref() {
            struct_ser.serialize_field("random_sampling", v)?;
        }
        if let Some(v) = self.overall_sampling.as_ref() {
            struct_ser.serialize_field("overall_sampling", v)?;
        }
        if self.verbose {
            struct_ser.serialize_field("verbose", &self.verbose)?;
        }
        if let Some(v) = self.max_path_tag_length.as_ref() {
            struct_ser.serialize_field("max_path_tag_length", v)?;
        }
        if !self.custom_tags.is_empty() {
            struct_ser.serialize_field("custom_tags", &self.custom_tags)?;
        }
        if let Some(v) = self.provider.as_ref() {
            struct_ser.serialize_field("provider", v)?;
        }
        if let Some(v) = self.spawn_upstream_span.as_ref() {
            struct_ser.serialize_field("spawn_upstream_span", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::Tracing {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_sampling",
            "clientSampling",
            "random_sampling",
            "randomSampling",
            "overall_sampling",
            "overallSampling",
            "verbose",
            "max_path_tag_length",
            "maxPathTagLength",
            "custom_tags",
            "customTags",
            "provider",
            "spawn_upstream_span",
            "spawnUpstreamSpan",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientSampling,
            RandomSampling,
            OverallSampling,
            Verbose,
            MaxPathTagLength,
            CustomTags,
            Provider,
            SpawnUpstreamSpan,
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
                            "clientSampling" | "client_sampling" => Ok(GeneratedField::ClientSampling),
                            "randomSampling" | "random_sampling" => Ok(GeneratedField::RandomSampling),
                            "overallSampling" | "overall_sampling" => Ok(GeneratedField::OverallSampling),
                            "verbose" => Ok(GeneratedField::Verbose),
                            "maxPathTagLength" | "max_path_tag_length" => Ok(GeneratedField::MaxPathTagLength),
                            "customTags" | "custom_tags" => Ok(GeneratedField::CustomTags),
                            "provider" => Ok(GeneratedField::Provider),
                            "spawnUpstreamSpan" | "spawn_upstream_span" => Ok(GeneratedField::SpawnUpstreamSpan),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::Tracing;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.Tracing")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<http_connection_manager::Tracing, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_sampling__ = None;
                let mut random_sampling__ = None;
                let mut overall_sampling__ = None;
                let mut verbose__ = None;
                let mut max_path_tag_length__ = None;
                let mut custom_tags__ = None;
                let mut provider__ = None;
                let mut spawn_upstream_span__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientSampling => {
                            if client_sampling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientSampling"));
                            }
                            client_sampling__ = map_.next_value()?;
                        }
                        GeneratedField::RandomSampling => {
                            if random_sampling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("randomSampling"));
                            }
                            random_sampling__ = map_.next_value()?;
                        }
                        GeneratedField::OverallSampling => {
                            if overall_sampling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overallSampling"));
                            }
                            overall_sampling__ = map_.next_value()?;
                        }
                        GeneratedField::Verbose => {
                            if verbose__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verbose"));
                            }
                            verbose__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxPathTagLength => {
                            if max_path_tag_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPathTagLength"));
                            }
                            max_path_tag_length__ = map_.next_value()?;
                        }
                        GeneratedField::CustomTags => {
                            if custom_tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customTags"));
                            }
                            custom_tags__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = map_.next_value()?;
                        }
                        GeneratedField::SpawnUpstreamSpan => {
                            if spawn_upstream_span__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spawnUpstreamSpan"));
                            }
                            spawn_upstream_span__ = map_.next_value()?;
                        }
                    }
                }
                Ok(http_connection_manager::Tracing {
                    client_sampling: client_sampling__,
                    random_sampling: random_sampling__,
                    overall_sampling: overall_sampling__,
                    verbose: verbose__.unwrap_or_default(),
                    max_path_tag_length: max_path_tag_length__,
                    custom_tags: custom_tags__.unwrap_or_default(),
                    provider: provider__,
                    spawn_upstream_span: spawn_upstream_span__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.Tracing", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::tracing::OperationName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Ingress => "INGRESS",
            Self::Egress => "EGRESS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::tracing::OperationName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "INGRESS",
            "EGRESS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::tracing::OperationName;

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
                    "INGRESS" => Ok(http_connection_manager::tracing::OperationName::Ingress),
                    "EGRESS" => Ok(http_connection_manager::tracing::OperationName::Egress),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::UpgradeConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.upgrade_type.is_empty() {
            len += 1;
        }
        if !self.filters.is_empty() {
            len += 1;
        }
        if self.enabled.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.UpgradeConfig", len)?;
        if !self.upgrade_type.is_empty() {
            struct_ser.serialize_field("upgrade_type", &self.upgrade_type)?;
        }
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        if let Some(v) = self.enabled.as_ref() {
            struct_ser.serialize_field("enabled", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::UpgradeConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "upgrade_type",
            "upgradeType",
            "filters",
            "enabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UpgradeType,
            Filters,
            Enabled,
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
                            "upgradeType" | "upgrade_type" => Ok(GeneratedField::UpgradeType),
                            "filters" => Ok(GeneratedField::Filters),
                            "enabled" => Ok(GeneratedField::Enabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::UpgradeConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.UpgradeConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<http_connection_manager::UpgradeConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut upgrade_type__ = None;
                let mut filters__ = None;
                let mut enabled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UpgradeType => {
                            if upgrade_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upgradeType"));
                            }
                            upgrade_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = map_.next_value()?;
                        }
                    }
                }
                Ok(http_connection_manager::UpgradeConfig {
                    upgrade_type: upgrade_type__.unwrap_or_default(),
                    filters: filters__.unwrap_or_default(),
                    enabled: enabled__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.UpgradeConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpFilter {
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
        if self.is_optional {
            len += 1;
        }
        if self.disabled {
            len += 1;
        }
        if self.config_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpFilter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.is_optional {
            struct_ser.serialize_field("is_optional", &self.is_optional)?;
        }
        if self.disabled {
            struct_ser.serialize_field("disabled", &self.disabled)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                http_filter::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typed_config", v)?;
                }
                http_filter::ConfigType::ConfigDiscovery(v) => {
                    struct_ser.serialize_field("config_discovery", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "is_optional",
            "isOptional",
            "disabled",
            "typed_config",
            "typedConfig",
            "config_discovery",
            "configDiscovery",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            IsOptional,
            Disabled,
            TypedConfig,
            ConfigDiscovery,
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
                            "isOptional" | "is_optional" => Ok(GeneratedField::IsOptional),
                            "disabled" => Ok(GeneratedField::Disabled),
                            "typedConfig" | "typed_config" => Ok(GeneratedField::TypedConfig),
                            "configDiscovery" | "config_discovery" => Ok(GeneratedField::ConfigDiscovery),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.HttpFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut is_optional__ = None;
                let mut disabled__ = None;
                let mut config_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsOptional => {
                            if is_optional__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isOptional"));
                            }
                            is_optional__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Disabled => {
                            if disabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            disabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map_.next_value::<::std::option::Option<_>>()?.map(http_filter::ConfigType::TypedConfig)
;
                        }
                        GeneratedField::ConfigDiscovery => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configDiscovery"));
                            }
                            config_type__ = map_.next_value::<::std::option::Option<_>>()?.map(http_filter::ConfigType::ConfigDiscovery)
;
                        }
                    }
                }
                Ok(HttpFilter {
                    name: name__.unwrap_or_default(),
                    is_optional: is_optional__.unwrap_or_default(),
                    disabled: disabled__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.HttpFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LocalReplyConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.mappers.is_empty() {
            len += 1;
        }
        if self.body_format.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.LocalReplyConfig", len)?;
        if !self.mappers.is_empty() {
            struct_ser.serialize_field("mappers", &self.mappers)?;
        }
        if let Some(v) = self.body_format.as_ref() {
            struct_ser.serialize_field("body_format", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocalReplyConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "mappers",
            "body_format",
            "bodyFormat",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Mappers,
            BodyFormat,
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
                            "mappers" => Ok(GeneratedField::Mappers),
                            "bodyFormat" | "body_format" => Ok(GeneratedField::BodyFormat),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LocalReplyConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.LocalReplyConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LocalReplyConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut mappers__ = None;
                let mut body_format__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Mappers => {
                            if mappers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mappers"));
                            }
                            mappers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BodyFormat => {
                            if body_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodyFormat"));
                            }
                            body_format__ = map_.next_value()?;
                        }
                    }
                }
                Ok(LocalReplyConfig {
                    mappers: mappers__.unwrap_or_default(),
                    body_format: body_format__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.LocalReplyConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Rds {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config_source.is_some() {
            len += 1;
        }
        if !self.route_config_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.Rds", len)?;
        if let Some(v) = self.config_source.as_ref() {
            struct_ser.serialize_field("config_source", v)?;
        }
        if !self.route_config_name.is_empty() {
            struct_ser.serialize_field("route_config_name", &self.route_config_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Rds {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config_source",
            "configSource",
            "route_config_name",
            "routeConfigName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConfigSource,
            RouteConfigName,
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
                            "configSource" | "config_source" => Ok(GeneratedField::ConfigSource),
                            "routeConfigName" | "route_config_name" => Ok(GeneratedField::RouteConfigName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Rds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.Rds")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Rds, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config_source__ = None;
                let mut route_config_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConfigSource => {
                            if config_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configSource"));
                            }
                            config_source__ = map_.next_value()?;
                        }
                        GeneratedField::RouteConfigName => {
                            if route_config_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeConfigName"));
                            }
                            route_config_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Rds {
                    config_source: config_source__,
                    route_config_name: route_config_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.Rds", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestIdExtension {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.typed_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.RequestIDExtension", len)?;
        if let Some(v) = self.typed_config.as_ref() {
            struct_ser.serialize_field("typed_config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestIdExtension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TypedConfig,
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
                            "typedConfig" | "typed_config" => Ok(GeneratedField::TypedConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestIdExtension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.RequestIDExtension")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestIdExtension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut typed_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TypedConfig => {
                            if typed_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            typed_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RequestIdExtension {
                    typed_config: typed_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.RequestIDExtension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseMapper {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.filter.is_some() {
            len += 1;
        }
        if self.status_code.is_some() {
            len += 1;
        }
        if self.body.is_some() {
            len += 1;
        }
        if self.body_format_override.is_some() {
            len += 1;
        }
        if !self.headers_to_add.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.ResponseMapper", len)?;
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        if let Some(v) = self.status_code.as_ref() {
            struct_ser.serialize_field("status_code", v)?;
        }
        if let Some(v) = self.body.as_ref() {
            struct_ser.serialize_field("body", v)?;
        }
        if let Some(v) = self.body_format_override.as_ref() {
            struct_ser.serialize_field("body_format_override", v)?;
        }
        if !self.headers_to_add.is_empty() {
            struct_ser.serialize_field("headers_to_add", &self.headers_to_add)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseMapper {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filter",
            "status_code",
            "statusCode",
            "body",
            "body_format_override",
            "bodyFormatOverride",
            "headers_to_add",
            "headersToAdd",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filter,
            StatusCode,
            Body,
            BodyFormatOverride,
            HeadersToAdd,
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
                            "filter" => Ok(GeneratedField::Filter),
                            "statusCode" | "status_code" => Ok(GeneratedField::StatusCode),
                            "body" => Ok(GeneratedField::Body),
                            "bodyFormatOverride" | "body_format_override" => Ok(GeneratedField::BodyFormatOverride),
                            "headersToAdd" | "headers_to_add" => Ok(GeneratedField::HeadersToAdd),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseMapper;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.ResponseMapper")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponseMapper, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter__ = None;
                let mut status_code__ = None;
                let mut body__ = None;
                let mut body_format_override__ = None;
                let mut headers_to_add__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map_.next_value()?;
                        }
                        GeneratedField::StatusCode => {
                            if status_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusCode"));
                            }
                            status_code__ = map_.next_value()?;
                        }
                        GeneratedField::Body => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("body"));
                            }
                            body__ = map_.next_value()?;
                        }
                        GeneratedField::BodyFormatOverride => {
                            if body_format_override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodyFormatOverride"));
                            }
                            body_format_override__ = map_.next_value()?;
                        }
                        GeneratedField::HeadersToAdd => {
                            if headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headersToAdd"));
                            }
                            headers_to_add__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResponseMapper {
                    filter: filter__,
                    status_code: status_code__,
                    body: body__,
                    body_format_override: body_format_override__,
                    headers_to_add: headers_to_add__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.ResponseMapper", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScopedRds {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.scoped_rds_config_source.is_some() {
            len += 1;
        }
        if !self.srds_resources_locator.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.ScopedRds", len)?;
        if let Some(v) = self.scoped_rds_config_source.as_ref() {
            struct_ser.serialize_field("scoped_rds_config_source", v)?;
        }
        if !self.srds_resources_locator.is_empty() {
            struct_ser.serialize_field("srds_resources_locator", &self.srds_resources_locator)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScopedRds {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "scoped_rds_config_source",
            "scopedRdsConfigSource",
            "srds_resources_locator",
            "srdsResourcesLocator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ScopedRdsConfigSource,
            SrdsResourcesLocator,
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
                            "scopedRdsConfigSource" | "scoped_rds_config_source" => Ok(GeneratedField::ScopedRdsConfigSource),
                            "srdsResourcesLocator" | "srds_resources_locator" => Ok(GeneratedField::SrdsResourcesLocator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScopedRds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.ScopedRds")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ScopedRds, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut scoped_rds_config_source__ = None;
                let mut srds_resources_locator__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ScopedRdsConfigSource => {
                            if scoped_rds_config_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopedRdsConfigSource"));
                            }
                            scoped_rds_config_source__ = map_.next_value()?;
                        }
                        GeneratedField::SrdsResourcesLocator => {
                            if srds_resources_locator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srdsResourcesLocator"));
                            }
                            srds_resources_locator__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ScopedRds {
                    scoped_rds_config_source: scoped_rds_config_source__,
                    srds_resources_locator: srds_resources_locator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.ScopedRds", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScopedRouteConfigurationsList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.scoped_route_configurations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.ScopedRouteConfigurationsList", len)?;
        if !self.scoped_route_configurations.is_empty() {
            struct_ser.serialize_field("scoped_route_configurations", &self.scoped_route_configurations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScopedRouteConfigurationsList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "scoped_route_configurations",
            "scopedRouteConfigurations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ScopedRouteConfigurations,
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
                            "scopedRouteConfigurations" | "scoped_route_configurations" => Ok(GeneratedField::ScopedRouteConfigurations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScopedRouteConfigurationsList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.ScopedRouteConfigurationsList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ScopedRouteConfigurationsList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut scoped_route_configurations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ScopedRouteConfigurations => {
                            if scoped_route_configurations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopedRouteConfigurations"));
                            }
                            scoped_route_configurations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ScopedRouteConfigurationsList {
                    scoped_route_configurations: scoped_route_configurations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.ScopedRouteConfigurationsList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScopedRoutes {
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
        if self.scope_key_builder.is_some() {
            len += 1;
        }
        if self.rds_config_source.is_some() {
            len += 1;
        }
        if self.config_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.scope_key_builder.as_ref() {
            struct_ser.serialize_field("scope_key_builder", v)?;
        }
        if let Some(v) = self.rds_config_source.as_ref() {
            struct_ser.serialize_field("rds_config_source", v)?;
        }
        if let Some(v) = self.config_specifier.as_ref() {
            match v {
                scoped_routes::ConfigSpecifier::ScopedRouteConfigurationsList(v) => {
                    struct_ser.serialize_field("scoped_route_configurations_list", v)?;
                }
                scoped_routes::ConfigSpecifier::ScopedRds(v) => {
                    struct_ser.serialize_field("scoped_rds", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScopedRoutes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "scope_key_builder",
            "scopeKeyBuilder",
            "rds_config_source",
            "rdsConfigSource",
            "scoped_route_configurations_list",
            "scopedRouteConfigurationsList",
            "scoped_rds",
            "scopedRds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ScopeKeyBuilder,
            RdsConfigSource,
            ScopedRouteConfigurationsList,
            ScopedRds,
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
                            "scopeKeyBuilder" | "scope_key_builder" => Ok(GeneratedField::ScopeKeyBuilder),
                            "rdsConfigSource" | "rds_config_source" => Ok(GeneratedField::RdsConfigSource),
                            "scopedRouteConfigurationsList" | "scoped_route_configurations_list" => Ok(GeneratedField::ScopedRouteConfigurationsList),
                            "scopedRds" | "scoped_rds" => Ok(GeneratedField::ScopedRds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScopedRoutes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ScopedRoutes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut scope_key_builder__ = None;
                let mut rds_config_source__ = None;
                let mut config_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ScopeKeyBuilder => {
                            if scope_key_builder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopeKeyBuilder"));
                            }
                            scope_key_builder__ = map_.next_value()?;
                        }
                        GeneratedField::RdsConfigSource => {
                            if rds_config_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rdsConfigSource"));
                            }
                            rds_config_source__ = map_.next_value()?;
                        }
                        GeneratedField::ScopedRouteConfigurationsList => {
                            if config_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopedRouteConfigurationsList"));
                            }
                            config_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(scoped_routes::ConfigSpecifier::ScopedRouteConfigurationsList)
;
                        }
                        GeneratedField::ScopedRds => {
                            if config_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopedRds"));
                            }
                            config_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(scoped_routes::ConfigSpecifier::ScopedRds)
;
                        }
                    }
                }
                Ok(ScopedRoutes {
                    name: name__.unwrap_or_default(),
                    scope_key_builder: scope_key_builder__,
                    rds_config_source: rds_config_source__,
                    config_specifier: config_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scoped_routes::ScopeKeyBuilder {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fragments.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder", len)?;
        if !self.fragments.is_empty() {
            struct_ser.serialize_field("fragments", &self.fragments)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for scoped_routes::ScopeKeyBuilder {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fragments",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fragments,
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
                            "fragments" => Ok(GeneratedField::Fragments),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scoped_routes::ScopeKeyBuilder;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<scoped_routes::ScopeKeyBuilder, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fragments__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fragments => {
                            if fragments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fragments"));
                            }
                            fragments__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(scoped_routes::ScopeKeyBuilder {
                    fragments: fragments__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scoped_routes::scope_key_builder::FragmentBuilder {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder", len)?;
        if let Some(v) = self.r#type.as_ref() {
            match v {
                scoped_routes::scope_key_builder::fragment_builder::Type::HeaderValueExtractor(v) => {
                    struct_ser.serialize_field("header_value_extractor", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for scoped_routes::scope_key_builder::FragmentBuilder {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header_value_extractor",
            "headerValueExtractor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HeaderValueExtractor,
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
                            "headerValueExtractor" | "header_value_extractor" => Ok(GeneratedField::HeaderValueExtractor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scoped_routes::scope_key_builder::FragmentBuilder;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<scoped_routes::scope_key_builder::FragmentBuilder, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HeaderValueExtractor => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerValueExtractor"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(scoped_routes::scope_key_builder::fragment_builder::Type::HeaderValueExtractor)
;
                        }
                    }
                }
                Ok(scoped_routes::scope_key_builder::FragmentBuilder {
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractor {
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
        if !self.element_separator.is_empty() {
            len += 1;
        }
        if self.extract_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.element_separator.is_empty() {
            struct_ser.serialize_field("element_separator", &self.element_separator)?;
        }
        if let Some(v) = self.extract_type.as_ref() {
            match v {
                scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractType::Index(v) => {
                    struct_ser.serialize_field("index", v)?;
                }
                scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractType::Element(v) => {
                    struct_ser.serialize_field("element", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "element_separator",
            "elementSeparator",
            "index",
            "element",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ElementSeparator,
            Index,
            Element,
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
                            "elementSeparator" | "element_separator" => Ok(GeneratedField::ElementSeparator),
                            "index" => Ok(GeneratedField::Index),
                            "element" => Ok(GeneratedField::Element),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut element_separator__ = None;
                let mut extract_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ElementSeparator => {
                            if element_separator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("elementSeparator"));
                            }
                            element_separator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Index => {
                            if extract_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            extract_type__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractType::Index(x.0));
                        }
                        GeneratedField::Element => {
                            if extract_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("element"));
                            }
                            extract_type__ = map_.next_value::<::std::option::Option<_>>()?.map(scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractType::Element)
;
                        }
                    }
                }
                Ok(scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractor {
                    name: name__.unwrap_or_default(),
                    element_separator: element_separator__.unwrap_or_default(),
                    extract_type: extract_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.separator.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor.KvElement", len)?;
        if !self.separator.is_empty() {
            struct_ser.serialize_field("separator", &self.separator)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "separator",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Separator,
            Key,
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
                            "separator" => Ok(GeneratedField::Separator),
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor.KvElement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut separator__ = None;
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Separator => {
                            if separator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("separator"));
                            }
                            separator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElement {
                    separator: separator__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor.KvElement", FIELDS, GeneratedVisitor)
    }
}
