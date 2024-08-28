impl serde::Serialize for Router {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.dynamic_stats.is_some() {
            len += 1;
        }
        if self.start_child_span {
            len += 1;
        }
        if !self.upstream_log.is_empty() {
            len += 1;
        }
        if self.upstream_log_options.is_some() {
            len += 1;
        }
        if self.suppress_envoy_headers {
            len += 1;
        }
        if !self.strict_check_headers.is_empty() {
            len += 1;
        }
        if self.respect_expected_rq_timeout {
            len += 1;
        }
        if self.suppress_grpc_request_failure_code_stats {
            len += 1;
        }
        if !self.upstream_http_filters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.router.v3.Router", len)?;
        if let Some(v) = self.dynamic_stats.as_ref() {
            struct_ser.serialize_field("dynamic_stats", v)?;
        }
        if self.start_child_span {
            struct_ser.serialize_field("start_child_span", &self.start_child_span)?;
        }
        if !self.upstream_log.is_empty() {
            struct_ser.serialize_field("upstream_log", &self.upstream_log)?;
        }
        if let Some(v) = self.upstream_log_options.as_ref() {
            struct_ser.serialize_field("upstream_log_options", v)?;
        }
        if self.suppress_envoy_headers {
            struct_ser.serialize_field("suppress_envoy_headers", &self.suppress_envoy_headers)?;
        }
        if !self.strict_check_headers.is_empty() {
            struct_ser.serialize_field("strict_check_headers", &self.strict_check_headers)?;
        }
        if self.respect_expected_rq_timeout {
            struct_ser.serialize_field("respect_expected_rq_timeout", &self.respect_expected_rq_timeout)?;
        }
        if self.suppress_grpc_request_failure_code_stats {
            struct_ser.serialize_field("suppress_grpc_request_failure_code_stats", &self.suppress_grpc_request_failure_code_stats)?;
        }
        if !self.upstream_http_filters.is_empty() {
            struct_ser.serialize_field("upstream_http_filters", &self.upstream_http_filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Router {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dynamic_stats",
            "dynamicStats",
            "start_child_span",
            "startChildSpan",
            "upstream_log",
            "upstreamLog",
            "upstream_log_options",
            "upstreamLogOptions",
            "suppress_envoy_headers",
            "suppressEnvoyHeaders",
            "strict_check_headers",
            "strictCheckHeaders",
            "respect_expected_rq_timeout",
            "respectExpectedRqTimeout",
            "suppress_grpc_request_failure_code_stats",
            "suppressGrpcRequestFailureCodeStats",
            "upstream_http_filters",
            "upstreamHttpFilters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DynamicStats,
            StartChildSpan,
            UpstreamLog,
            UpstreamLogOptions,
            SuppressEnvoyHeaders,
            StrictCheckHeaders,
            RespectExpectedRqTimeout,
            SuppressGrpcRequestFailureCodeStats,
            UpstreamHttpFilters,
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
                            "dynamicStats" | "dynamic_stats" => Ok(GeneratedField::DynamicStats),
                            "startChildSpan" | "start_child_span" => Ok(GeneratedField::StartChildSpan),
                            "upstreamLog" | "upstream_log" => Ok(GeneratedField::UpstreamLog),
                            "upstreamLogOptions" | "upstream_log_options" => Ok(GeneratedField::UpstreamLogOptions),
                            "suppressEnvoyHeaders" | "suppress_envoy_headers" => Ok(GeneratedField::SuppressEnvoyHeaders),
                            "strictCheckHeaders" | "strict_check_headers" => Ok(GeneratedField::StrictCheckHeaders),
                            "respectExpectedRqTimeout" | "respect_expected_rq_timeout" => Ok(GeneratedField::RespectExpectedRqTimeout),
                            "suppressGrpcRequestFailureCodeStats" | "suppress_grpc_request_failure_code_stats" => Ok(GeneratedField::SuppressGrpcRequestFailureCodeStats),
                            "upstreamHttpFilters" | "upstream_http_filters" => Ok(GeneratedField::UpstreamHttpFilters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Router;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.router.v3.Router")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Router, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dynamic_stats__ = None;
                let mut start_child_span__ = None;
                let mut upstream_log__ = None;
                let mut upstream_log_options__ = None;
                let mut suppress_envoy_headers__ = None;
                let mut strict_check_headers__ = None;
                let mut respect_expected_rq_timeout__ = None;
                let mut suppress_grpc_request_failure_code_stats__ = None;
                let mut upstream_http_filters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DynamicStats => {
                            if dynamic_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicStats"));
                            }
                            dynamic_stats__ = map_.next_value()?;
                        }
                        GeneratedField::StartChildSpan => {
                            if start_child_span__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startChildSpan"));
                            }
                            start_child_span__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpstreamLog => {
                            if upstream_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamLog"));
                            }
                            upstream_log__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpstreamLogOptions => {
                            if upstream_log_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamLogOptions"));
                            }
                            upstream_log_options__ = map_.next_value()?;
                        }
                        GeneratedField::SuppressEnvoyHeaders => {
                            if suppress_envoy_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suppressEnvoyHeaders"));
                            }
                            suppress_envoy_headers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StrictCheckHeaders => {
                            if strict_check_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("strictCheckHeaders"));
                            }
                            strict_check_headers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RespectExpectedRqTimeout => {
                            if respect_expected_rq_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("respectExpectedRqTimeout"));
                            }
                            respect_expected_rq_timeout__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SuppressGrpcRequestFailureCodeStats => {
                            if suppress_grpc_request_failure_code_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suppressGrpcRequestFailureCodeStats"));
                            }
                            suppress_grpc_request_failure_code_stats__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpstreamHttpFilters => {
                            if upstream_http_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamHttpFilters"));
                            }
                            upstream_http_filters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Router {
                    dynamic_stats: dynamic_stats__,
                    start_child_span: start_child_span__.unwrap_or_default(),
                    upstream_log: upstream_log__.unwrap_or_default(),
                    upstream_log_options: upstream_log_options__,
                    suppress_envoy_headers: suppress_envoy_headers__.unwrap_or_default(),
                    strict_check_headers: strict_check_headers__.unwrap_or_default(),
                    respect_expected_rq_timeout: respect_expected_rq_timeout__.unwrap_or_default(),
                    suppress_grpc_request_failure_code_stats: suppress_grpc_request_failure_code_stats__.unwrap_or_default(),
                    upstream_http_filters: upstream_http_filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.router.v3.Router", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for router::UpstreamAccessLogOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.flush_upstream_log_on_upstream_stream {
            len += 1;
        }
        if self.upstream_log_flush_interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.router.v3.Router.UpstreamAccessLogOptions", len)?;
        if self.flush_upstream_log_on_upstream_stream {
            struct_ser.serialize_field("flush_upstream_log_on_upstream_stream", &self.flush_upstream_log_on_upstream_stream)?;
        }
        if let Some(v) = self.upstream_log_flush_interval.as_ref() {
            struct_ser.serialize_field("upstream_log_flush_interval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for router::UpstreamAccessLogOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "flush_upstream_log_on_upstream_stream",
            "flushUpstreamLogOnUpstreamStream",
            "upstream_log_flush_interval",
            "upstreamLogFlushInterval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FlushUpstreamLogOnUpstreamStream,
            UpstreamLogFlushInterval,
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
                            "flushUpstreamLogOnUpstreamStream" | "flush_upstream_log_on_upstream_stream" => Ok(GeneratedField::FlushUpstreamLogOnUpstreamStream),
                            "upstreamLogFlushInterval" | "upstream_log_flush_interval" => Ok(GeneratedField::UpstreamLogFlushInterval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = router::UpstreamAccessLogOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.router.v3.Router.UpstreamAccessLogOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<router::UpstreamAccessLogOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut flush_upstream_log_on_upstream_stream__ = None;
                let mut upstream_log_flush_interval__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FlushUpstreamLogOnUpstreamStream => {
                            if flush_upstream_log_on_upstream_stream__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flushUpstreamLogOnUpstreamStream"));
                            }
                            flush_upstream_log_on_upstream_stream__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpstreamLogFlushInterval => {
                            if upstream_log_flush_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamLogFlushInterval"));
                            }
                            upstream_log_flush_interval__ = map_.next_value()?;
                        }
                    }
                }
                Ok(router::UpstreamAccessLogOptions {
                    flush_upstream_log_on_upstream_stream: flush_upstream_log_on_upstream_stream__.unwrap_or_default(),
                    upstream_log_flush_interval: upstream_log_flush_interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.router.v3.Router.UpstreamAccessLogOptions", FIELDS, GeneratedVisitor)
    }
}
