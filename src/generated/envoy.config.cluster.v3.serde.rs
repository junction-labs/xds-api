impl serde::Serialize for CircuitBreakers {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.thresholds.is_empty() {
            len += 1;
        }
        if !self.per_host_thresholds.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.CircuitBreakers", len)?;
        if !self.thresholds.is_empty() {
            struct_ser.serialize_field("thresholds", &self.thresholds)?;
        }
        if !self.per_host_thresholds.is_empty() {
            struct_ser.serialize_field("per_host_thresholds", &self.per_host_thresholds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CircuitBreakers {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "thresholds",
            "per_host_thresholds",
            "perHostThresholds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Thresholds,
            PerHostThresholds,
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
                            "thresholds" => Ok(GeneratedField::Thresholds),
                            "perHostThresholds" | "per_host_thresholds" => Ok(GeneratedField::PerHostThresholds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CircuitBreakers;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.CircuitBreakers")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CircuitBreakers, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut thresholds__ = None;
                let mut per_host_thresholds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Thresholds => {
                            if thresholds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("thresholds"));
                            }
                            thresholds__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PerHostThresholds => {
                            if per_host_thresholds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perHostThresholds"));
                            }
                            per_host_thresholds__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CircuitBreakers {
                    thresholds: thresholds__.unwrap_or_default(),
                    per_host_thresholds: per_host_thresholds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.CircuitBreakers", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for circuit_breakers::Thresholds {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.priority != 0 {
            len += 1;
        }
        if self.max_connections.is_some() {
            len += 1;
        }
        if self.max_pending_requests.is_some() {
            len += 1;
        }
        if self.max_requests.is_some() {
            len += 1;
        }
        if self.max_retries.is_some() {
            len += 1;
        }
        if self.retry_budget.is_some() {
            len += 1;
        }
        if self.track_remaining {
            len += 1;
        }
        if self.max_connection_pools.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.CircuitBreakers.Thresholds", len)?;
        if self.priority != 0 {
            let v = super::super::core::v3::RoutingPriority::try_from(self.priority)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.priority)))?;
            struct_ser.serialize_field("priority", &v)?;
        }
        if let Some(v) = self.max_connections.as_ref() {
            struct_ser.serialize_field("max_connections", v)?;
        }
        if let Some(v) = self.max_pending_requests.as_ref() {
            struct_ser.serialize_field("max_pending_requests", v)?;
        }
        if let Some(v) = self.max_requests.as_ref() {
            struct_ser.serialize_field("max_requests", v)?;
        }
        if let Some(v) = self.max_retries.as_ref() {
            struct_ser.serialize_field("max_retries", v)?;
        }
        if let Some(v) = self.retry_budget.as_ref() {
            struct_ser.serialize_field("retry_budget", v)?;
        }
        if self.track_remaining {
            struct_ser.serialize_field("track_remaining", &self.track_remaining)?;
        }
        if let Some(v) = self.max_connection_pools.as_ref() {
            struct_ser.serialize_field("max_connection_pools", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for circuit_breakers::Thresholds {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "priority",
            "max_connections",
            "maxConnections",
            "max_pending_requests",
            "maxPendingRequests",
            "max_requests",
            "maxRequests",
            "max_retries",
            "maxRetries",
            "retry_budget",
            "retryBudget",
            "track_remaining",
            "trackRemaining",
            "max_connection_pools",
            "maxConnectionPools",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Priority,
            MaxConnections,
            MaxPendingRequests,
            MaxRequests,
            MaxRetries,
            RetryBudget,
            TrackRemaining,
            MaxConnectionPools,
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
                            "priority" => Ok(GeneratedField::Priority),
                            "maxConnections" | "max_connections" => Ok(GeneratedField::MaxConnections),
                            "maxPendingRequests" | "max_pending_requests" => Ok(GeneratedField::MaxPendingRequests),
                            "maxRequests" | "max_requests" => Ok(GeneratedField::MaxRequests),
                            "maxRetries" | "max_retries" => Ok(GeneratedField::MaxRetries),
                            "retryBudget" | "retry_budget" => Ok(GeneratedField::RetryBudget),
                            "trackRemaining" | "track_remaining" => Ok(GeneratedField::TrackRemaining),
                            "maxConnectionPools" | "max_connection_pools" => Ok(GeneratedField::MaxConnectionPools),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = circuit_breakers::Thresholds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.CircuitBreakers.Thresholds")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<circuit_breakers::Thresholds, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut priority__ = None;
                let mut max_connections__ = None;
                let mut max_pending_requests__ = None;
                let mut max_requests__ = None;
                let mut max_retries__ = None;
                let mut retry_budget__ = None;
                let mut track_remaining__ = None;
                let mut max_connection_pools__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = Some(map_.next_value::<super::super::core::v3::RoutingPriority>()? as i32);
                        }
                        GeneratedField::MaxConnections => {
                            if max_connections__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxConnections"));
                            }
                            max_connections__ = map_.next_value()?;
                        }
                        GeneratedField::MaxPendingRequests => {
                            if max_pending_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPendingRequests"));
                            }
                            max_pending_requests__ = map_.next_value()?;
                        }
                        GeneratedField::MaxRequests => {
                            if max_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRequests"));
                            }
                            max_requests__ = map_.next_value()?;
                        }
                        GeneratedField::MaxRetries => {
                            if max_retries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRetries"));
                            }
                            max_retries__ = map_.next_value()?;
                        }
                        GeneratedField::RetryBudget => {
                            if retry_budget__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryBudget"));
                            }
                            retry_budget__ = map_.next_value()?;
                        }
                        GeneratedField::TrackRemaining => {
                            if track_remaining__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackRemaining"));
                            }
                            track_remaining__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxConnectionPools => {
                            if max_connection_pools__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxConnectionPools"));
                            }
                            max_connection_pools__ = map_.next_value()?;
                        }
                    }
                }
                Ok(circuit_breakers::Thresholds {
                    priority: priority__.unwrap_or_default(),
                    max_connections: max_connections__,
                    max_pending_requests: max_pending_requests__,
                    max_requests: max_requests__,
                    max_retries: max_retries__,
                    retry_budget: retry_budget__,
                    track_remaining: track_remaining__.unwrap_or_default(),
                    max_connection_pools: max_connection_pools__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.CircuitBreakers.Thresholds", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for circuit_breakers::thresholds::RetryBudget {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.budget_percent.is_some() {
            len += 1;
        }
        if self.min_retry_concurrency.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.CircuitBreakers.Thresholds.RetryBudget", len)?;
        if let Some(v) = self.budget_percent.as_ref() {
            struct_ser.serialize_field("budget_percent", v)?;
        }
        if let Some(v) = self.min_retry_concurrency.as_ref() {
            struct_ser.serialize_field("min_retry_concurrency", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for circuit_breakers::thresholds::RetryBudget {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "budget_percent",
            "budgetPercent",
            "min_retry_concurrency",
            "minRetryConcurrency",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BudgetPercent,
            MinRetryConcurrency,
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
                            "budgetPercent" | "budget_percent" => Ok(GeneratedField::BudgetPercent),
                            "minRetryConcurrency" | "min_retry_concurrency" => Ok(GeneratedField::MinRetryConcurrency),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = circuit_breakers::thresholds::RetryBudget;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.CircuitBreakers.Thresholds.RetryBudget")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<circuit_breakers::thresholds::RetryBudget, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut budget_percent__ = None;
                let mut min_retry_concurrency__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BudgetPercent => {
                            if budget_percent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("budgetPercent"));
                            }
                            budget_percent__ = map_.next_value()?;
                        }
                        GeneratedField::MinRetryConcurrency => {
                            if min_retry_concurrency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minRetryConcurrency"));
                            }
                            min_retry_concurrency__ = map_.next_value()?;
                        }
                    }
                }
                Ok(circuit_breakers::thresholds::RetryBudget {
                    budget_percent: budget_percent__,
                    min_retry_concurrency: min_retry_concurrency__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.CircuitBreakers.Thresholds.RetryBudget", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Cluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transport_socket_matches.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.alt_stat_name.is_empty() {
            len += 1;
        }
        if self.eds_cluster_config.is_some() {
            len += 1;
        }
        if self.connect_timeout.is_some() {
            len += 1;
        }
        if self.per_connection_buffer_limit_bytes.is_some() {
            len += 1;
        }
        if self.lb_policy != 0 {
            len += 1;
        }
        if self.load_assignment.is_some() {
            len += 1;
        }
        if !self.health_checks.is_empty() {
            len += 1;
        }
        if self.max_requests_per_connection.is_some() {
            len += 1;
        }
        if self.circuit_breakers.is_some() {
            len += 1;
        }
        if self.upstream_http_protocol_options.is_some() {
            len += 1;
        }
        if self.common_http_protocol_options.is_some() {
            len += 1;
        }
        if self.http_protocol_options.is_some() {
            len += 1;
        }
        if self.http2_protocol_options.is_some() {
            len += 1;
        }
        if !self.typed_extension_protocol_options.is_empty() {
            len += 1;
        }
        if self.dns_refresh_rate.is_some() {
            len += 1;
        }
        if self.dns_jitter.is_some() {
            len += 1;
        }
        if self.dns_failure_refresh_rate.is_some() {
            len += 1;
        }
        if self.respect_dns_ttl {
            len += 1;
        }
        if self.dns_lookup_family != 0 {
            len += 1;
        }
        if !self.dns_resolvers.is_empty() {
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
        if self.wait_for_warm_on_init.is_some() {
            len += 1;
        }
        if self.outlier_detection.is_some() {
            len += 1;
        }
        if self.cleanup_interval.is_some() {
            len += 1;
        }
        if self.upstream_bind_config.is_some() {
            len += 1;
        }
        if self.lb_subset_config.is_some() {
            len += 1;
        }
        if self.common_lb_config.is_some() {
            len += 1;
        }
        if self.transport_socket.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.protocol_selection != 0 {
            len += 1;
        }
        if self.upstream_connection_options.is_some() {
            len += 1;
        }
        if self.close_connections_on_host_health_failure {
            len += 1;
        }
        if self.ignore_health_on_host_removal {
            len += 1;
        }
        if !self.filters.is_empty() {
            len += 1;
        }
        if self.load_balancing_policy.is_some() {
            len += 1;
        }
        if self.lrs_server.is_some() {
            len += 1;
        }
        if !self.lrs_report_endpoint_metrics.is_empty() {
            len += 1;
        }
        if self.track_timeout_budgets {
            len += 1;
        }
        if self.upstream_config.is_some() {
            len += 1;
        }
        if self.track_cluster_stats.is_some() {
            len += 1;
        }
        if self.preconnect_policy.is_some() {
            len += 1;
        }
        if self.connection_pool_per_downstream_connection {
            len += 1;
        }
        if self.cluster_discovery_type.is_some() {
            len += 1;
        }
        if self.lb_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster", len)?;
        if !self.transport_socket_matches.is_empty() {
            struct_ser.serialize_field("transport_socket_matches", &self.transport_socket_matches)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.alt_stat_name.is_empty() {
            struct_ser.serialize_field("alt_stat_name", &self.alt_stat_name)?;
        }
        if let Some(v) = self.eds_cluster_config.as_ref() {
            struct_ser.serialize_field("eds_cluster_config", v)?;
        }
        if let Some(v) = self.connect_timeout.as_ref() {
            struct_ser.serialize_field("connect_timeout", v)?;
        }
        if let Some(v) = self.per_connection_buffer_limit_bytes.as_ref() {
            struct_ser.serialize_field("per_connection_buffer_limit_bytes", v)?;
        }
        if self.lb_policy != 0 {
            let v = cluster::LbPolicy::try_from(self.lb_policy)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.lb_policy)))?;
            struct_ser.serialize_field("lb_policy", &v)?;
        }
        if let Some(v) = self.load_assignment.as_ref() {
            struct_ser.serialize_field("load_assignment", v)?;
        }
        if !self.health_checks.is_empty() {
            struct_ser.serialize_field("health_checks", &self.health_checks)?;
        }
        if let Some(v) = self.max_requests_per_connection.as_ref() {
            struct_ser.serialize_field("max_requests_per_connection", v)?;
        }
        if let Some(v) = self.circuit_breakers.as_ref() {
            struct_ser.serialize_field("circuit_breakers", v)?;
        }
        if let Some(v) = self.upstream_http_protocol_options.as_ref() {
            struct_ser.serialize_field("upstream_http_protocol_options", v)?;
        }
        if let Some(v) = self.common_http_protocol_options.as_ref() {
            struct_ser.serialize_field("common_http_protocol_options", v)?;
        }
        if let Some(v) = self.http_protocol_options.as_ref() {
            struct_ser.serialize_field("http_protocol_options", v)?;
        }
        if let Some(v) = self.http2_protocol_options.as_ref() {
            struct_ser.serialize_field("http2_protocol_options", v)?;
        }
        if !self.typed_extension_protocol_options.is_empty() {
            struct_ser.serialize_field("typed_extension_protocol_options", &self.typed_extension_protocol_options)?;
        }
        if let Some(v) = self.dns_refresh_rate.as_ref() {
            struct_ser.serialize_field("dns_refresh_rate", v)?;
        }
        if let Some(v) = self.dns_jitter.as_ref() {
            struct_ser.serialize_field("dns_jitter", v)?;
        }
        if let Some(v) = self.dns_failure_refresh_rate.as_ref() {
            struct_ser.serialize_field("dns_failure_refresh_rate", v)?;
        }
        if self.respect_dns_ttl {
            struct_ser.serialize_field("respect_dns_ttl", &self.respect_dns_ttl)?;
        }
        if self.dns_lookup_family != 0 {
            let v = cluster::DnsLookupFamily::try_from(self.dns_lookup_family)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.dns_lookup_family)))?;
            struct_ser.serialize_field("dns_lookup_family", &v)?;
        }
        if !self.dns_resolvers.is_empty() {
            struct_ser.serialize_field("dns_resolvers", &self.dns_resolvers)?;
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
        if let Some(v) = self.wait_for_warm_on_init.as_ref() {
            struct_ser.serialize_field("wait_for_warm_on_init", v)?;
        }
        if let Some(v) = self.outlier_detection.as_ref() {
            struct_ser.serialize_field("outlier_detection", v)?;
        }
        if let Some(v) = self.cleanup_interval.as_ref() {
            struct_ser.serialize_field("cleanup_interval", v)?;
        }
        if let Some(v) = self.upstream_bind_config.as_ref() {
            struct_ser.serialize_field("upstream_bind_config", v)?;
        }
        if let Some(v) = self.lb_subset_config.as_ref() {
            struct_ser.serialize_field("lb_subset_config", v)?;
        }
        if let Some(v) = self.common_lb_config.as_ref() {
            struct_ser.serialize_field("common_lb_config", v)?;
        }
        if let Some(v) = self.transport_socket.as_ref() {
            struct_ser.serialize_field("transport_socket", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if self.protocol_selection != 0 {
            let v = cluster::ClusterProtocolSelection::try_from(self.protocol_selection)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.protocol_selection)))?;
            struct_ser.serialize_field("protocol_selection", &v)?;
        }
        if let Some(v) = self.upstream_connection_options.as_ref() {
            struct_ser.serialize_field("upstream_connection_options", v)?;
        }
        if self.close_connections_on_host_health_failure {
            struct_ser.serialize_field("close_connections_on_host_health_failure", &self.close_connections_on_host_health_failure)?;
        }
        if self.ignore_health_on_host_removal {
            struct_ser.serialize_field("ignore_health_on_host_removal", &self.ignore_health_on_host_removal)?;
        }
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        if let Some(v) = self.load_balancing_policy.as_ref() {
            struct_ser.serialize_field("load_balancing_policy", v)?;
        }
        if let Some(v) = self.lrs_server.as_ref() {
            struct_ser.serialize_field("lrs_server", v)?;
        }
        if !self.lrs_report_endpoint_metrics.is_empty() {
            struct_ser.serialize_field("lrs_report_endpoint_metrics", &self.lrs_report_endpoint_metrics)?;
        }
        if self.track_timeout_budgets {
            struct_ser.serialize_field("track_timeout_budgets", &self.track_timeout_budgets)?;
        }
        if let Some(v) = self.upstream_config.as_ref() {
            struct_ser.serialize_field("upstream_config", v)?;
        }
        if let Some(v) = self.track_cluster_stats.as_ref() {
            struct_ser.serialize_field("track_cluster_stats", v)?;
        }
        if let Some(v) = self.preconnect_policy.as_ref() {
            struct_ser.serialize_field("preconnect_policy", v)?;
        }
        if self.connection_pool_per_downstream_connection {
            struct_ser.serialize_field("connection_pool_per_downstream_connection", &self.connection_pool_per_downstream_connection)?;
        }
        if let Some(v) = self.cluster_discovery_type.as_ref() {
            match v {
                cluster::ClusterDiscoveryType::Type(v) => {
                    let v = cluster::DiscoveryType::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("type", &v)?;
                }
                cluster::ClusterDiscoveryType::ClusterType(v) => {
                    struct_ser.serialize_field("cluster_type", v)?;
                }
            }
        }
        if let Some(v) = self.lb_config.as_ref() {
            match v {
                cluster::LbConfig::RingHashLbConfig(v) => {
                    struct_ser.serialize_field("ring_hash_lb_config", v)?;
                }
                cluster::LbConfig::MaglevLbConfig(v) => {
                    struct_ser.serialize_field("maglev_lb_config", v)?;
                }
                cluster::LbConfig::OriginalDstLbConfig(v) => {
                    struct_ser.serialize_field("original_dst_lb_config", v)?;
                }
                cluster::LbConfig::LeastRequestLbConfig(v) => {
                    struct_ser.serialize_field("least_request_lb_config", v)?;
                }
                cluster::LbConfig::RoundRobinLbConfig(v) => {
                    struct_ser.serialize_field("round_robin_lb_config", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Cluster {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transport_socket_matches",
            "transportSocketMatches",
            "name",
            "alt_stat_name",
            "altStatName",
            "eds_cluster_config",
            "edsClusterConfig",
            "connect_timeout",
            "connectTimeout",
            "per_connection_buffer_limit_bytes",
            "perConnectionBufferLimitBytes",
            "lb_policy",
            "lbPolicy",
            "load_assignment",
            "loadAssignment",
            "health_checks",
            "healthChecks",
            "max_requests_per_connection",
            "maxRequestsPerConnection",
            "circuit_breakers",
            "circuitBreakers",
            "upstream_http_protocol_options",
            "upstreamHttpProtocolOptions",
            "common_http_protocol_options",
            "commonHttpProtocolOptions",
            "http_protocol_options",
            "httpProtocolOptions",
            "http2_protocol_options",
            "http2ProtocolOptions",
            "typed_extension_protocol_options",
            "typedExtensionProtocolOptions",
            "dns_refresh_rate",
            "dnsRefreshRate",
            "dns_jitter",
            "dnsJitter",
            "dns_failure_refresh_rate",
            "dnsFailureRefreshRate",
            "respect_dns_ttl",
            "respectDnsTtl",
            "dns_lookup_family",
            "dnsLookupFamily",
            "dns_resolvers",
            "dnsResolvers",
            "use_tcp_for_dns_lookups",
            "useTcpForDnsLookups",
            "dns_resolution_config",
            "dnsResolutionConfig",
            "typed_dns_resolver_config",
            "typedDnsResolverConfig",
            "wait_for_warm_on_init",
            "waitForWarmOnInit",
            "outlier_detection",
            "outlierDetection",
            "cleanup_interval",
            "cleanupInterval",
            "upstream_bind_config",
            "upstreamBindConfig",
            "lb_subset_config",
            "lbSubsetConfig",
            "common_lb_config",
            "commonLbConfig",
            "transport_socket",
            "transportSocket",
            "metadata",
            "protocol_selection",
            "protocolSelection",
            "upstream_connection_options",
            "upstreamConnectionOptions",
            "close_connections_on_host_health_failure",
            "closeConnectionsOnHostHealthFailure",
            "ignore_health_on_host_removal",
            "ignoreHealthOnHostRemoval",
            "filters",
            "load_balancing_policy",
            "loadBalancingPolicy",
            "lrs_server",
            "lrsServer",
            "lrs_report_endpoint_metrics",
            "lrsReportEndpointMetrics",
            "track_timeout_budgets",
            "trackTimeoutBudgets",
            "upstream_config",
            "upstreamConfig",
            "track_cluster_stats",
            "trackClusterStats",
            "preconnect_policy",
            "preconnectPolicy",
            "connection_pool_per_downstream_connection",
            "connectionPoolPerDownstreamConnection",
            "type",
            "cluster_type",
            "clusterType",
            "ring_hash_lb_config",
            "ringHashLbConfig",
            "maglev_lb_config",
            "maglevLbConfig",
            "original_dst_lb_config",
            "originalDstLbConfig",
            "least_request_lb_config",
            "leastRequestLbConfig",
            "round_robin_lb_config",
            "roundRobinLbConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransportSocketMatches,
            Name,
            AltStatName,
            EdsClusterConfig,
            ConnectTimeout,
            PerConnectionBufferLimitBytes,
            LbPolicy,
            LoadAssignment,
            HealthChecks,
            MaxRequestsPerConnection,
            CircuitBreakers,
            UpstreamHttpProtocolOptions,
            CommonHttpProtocolOptions,
            HttpProtocolOptions,
            Http2ProtocolOptions,
            TypedExtensionProtocolOptions,
            DnsRefreshRate,
            DnsJitter,
            DnsFailureRefreshRate,
            RespectDnsTtl,
            DnsLookupFamily,
            DnsResolvers,
            UseTcpForDnsLookups,
            DnsResolutionConfig,
            TypedDnsResolverConfig,
            WaitForWarmOnInit,
            OutlierDetection,
            CleanupInterval,
            UpstreamBindConfig,
            LbSubsetConfig,
            CommonLbConfig,
            TransportSocket,
            Metadata,
            ProtocolSelection,
            UpstreamConnectionOptions,
            CloseConnectionsOnHostHealthFailure,
            IgnoreHealthOnHostRemoval,
            Filters,
            LoadBalancingPolicy,
            LrsServer,
            LrsReportEndpointMetrics,
            TrackTimeoutBudgets,
            UpstreamConfig,
            TrackClusterStats,
            PreconnectPolicy,
            ConnectionPoolPerDownstreamConnection,
            Type,
            ClusterType,
            RingHashLbConfig,
            MaglevLbConfig,
            OriginalDstLbConfig,
            LeastRequestLbConfig,
            RoundRobinLbConfig,
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
                            "transportSocketMatches" | "transport_socket_matches" => Ok(GeneratedField::TransportSocketMatches),
                            "name" => Ok(GeneratedField::Name),
                            "altStatName" | "alt_stat_name" => Ok(GeneratedField::AltStatName),
                            "edsClusterConfig" | "eds_cluster_config" => Ok(GeneratedField::EdsClusterConfig),
                            "connectTimeout" | "connect_timeout" => Ok(GeneratedField::ConnectTimeout),
                            "perConnectionBufferLimitBytes" | "per_connection_buffer_limit_bytes" => Ok(GeneratedField::PerConnectionBufferLimitBytes),
                            "lbPolicy" | "lb_policy" => Ok(GeneratedField::LbPolicy),
                            "loadAssignment" | "load_assignment" => Ok(GeneratedField::LoadAssignment),
                            "healthChecks" | "health_checks" => Ok(GeneratedField::HealthChecks),
                            "maxRequestsPerConnection" | "max_requests_per_connection" => Ok(GeneratedField::MaxRequestsPerConnection),
                            "circuitBreakers" | "circuit_breakers" => Ok(GeneratedField::CircuitBreakers),
                            "upstreamHttpProtocolOptions" | "upstream_http_protocol_options" => Ok(GeneratedField::UpstreamHttpProtocolOptions),
                            "commonHttpProtocolOptions" | "common_http_protocol_options" => Ok(GeneratedField::CommonHttpProtocolOptions),
                            "httpProtocolOptions" | "http_protocol_options" => Ok(GeneratedField::HttpProtocolOptions),
                            "http2ProtocolOptions" | "http2_protocol_options" => Ok(GeneratedField::Http2ProtocolOptions),
                            "typedExtensionProtocolOptions" | "typed_extension_protocol_options" => Ok(GeneratedField::TypedExtensionProtocolOptions),
                            "dnsRefreshRate" | "dns_refresh_rate" => Ok(GeneratedField::DnsRefreshRate),
                            "dnsJitter" | "dns_jitter" => Ok(GeneratedField::DnsJitter),
                            "dnsFailureRefreshRate" | "dns_failure_refresh_rate" => Ok(GeneratedField::DnsFailureRefreshRate),
                            "respectDnsTtl" | "respect_dns_ttl" => Ok(GeneratedField::RespectDnsTtl),
                            "dnsLookupFamily" | "dns_lookup_family" => Ok(GeneratedField::DnsLookupFamily),
                            "dnsResolvers" | "dns_resolvers" => Ok(GeneratedField::DnsResolvers),
                            "useTcpForDnsLookups" | "use_tcp_for_dns_lookups" => Ok(GeneratedField::UseTcpForDnsLookups),
                            "dnsResolutionConfig" | "dns_resolution_config" => Ok(GeneratedField::DnsResolutionConfig),
                            "typedDnsResolverConfig" | "typed_dns_resolver_config" => Ok(GeneratedField::TypedDnsResolverConfig),
                            "waitForWarmOnInit" | "wait_for_warm_on_init" => Ok(GeneratedField::WaitForWarmOnInit),
                            "outlierDetection" | "outlier_detection" => Ok(GeneratedField::OutlierDetection),
                            "cleanupInterval" | "cleanup_interval" => Ok(GeneratedField::CleanupInterval),
                            "upstreamBindConfig" | "upstream_bind_config" => Ok(GeneratedField::UpstreamBindConfig),
                            "lbSubsetConfig" | "lb_subset_config" => Ok(GeneratedField::LbSubsetConfig),
                            "commonLbConfig" | "common_lb_config" => Ok(GeneratedField::CommonLbConfig),
                            "transportSocket" | "transport_socket" => Ok(GeneratedField::TransportSocket),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "protocolSelection" | "protocol_selection" => Ok(GeneratedField::ProtocolSelection),
                            "upstreamConnectionOptions" | "upstream_connection_options" => Ok(GeneratedField::UpstreamConnectionOptions),
                            "closeConnectionsOnHostHealthFailure" | "close_connections_on_host_health_failure" => Ok(GeneratedField::CloseConnectionsOnHostHealthFailure),
                            "ignoreHealthOnHostRemoval" | "ignore_health_on_host_removal" => Ok(GeneratedField::IgnoreHealthOnHostRemoval),
                            "filters" => Ok(GeneratedField::Filters),
                            "loadBalancingPolicy" | "load_balancing_policy" => Ok(GeneratedField::LoadBalancingPolicy),
                            "lrsServer" | "lrs_server" => Ok(GeneratedField::LrsServer),
                            "lrsReportEndpointMetrics" | "lrs_report_endpoint_metrics" => Ok(GeneratedField::LrsReportEndpointMetrics),
                            "trackTimeoutBudgets" | "track_timeout_budgets" => Ok(GeneratedField::TrackTimeoutBudgets),
                            "upstreamConfig" | "upstream_config" => Ok(GeneratedField::UpstreamConfig),
                            "trackClusterStats" | "track_cluster_stats" => Ok(GeneratedField::TrackClusterStats),
                            "preconnectPolicy" | "preconnect_policy" => Ok(GeneratedField::PreconnectPolicy),
                            "connectionPoolPerDownstreamConnection" | "connection_pool_per_downstream_connection" => Ok(GeneratedField::ConnectionPoolPerDownstreamConnection),
                            "type" => Ok(GeneratedField::Type),
                            "clusterType" | "cluster_type" => Ok(GeneratedField::ClusterType),
                            "ringHashLbConfig" | "ring_hash_lb_config" => Ok(GeneratedField::RingHashLbConfig),
                            "maglevLbConfig" | "maglev_lb_config" => Ok(GeneratedField::MaglevLbConfig),
                            "originalDstLbConfig" | "original_dst_lb_config" => Ok(GeneratedField::OriginalDstLbConfig),
                            "leastRequestLbConfig" | "least_request_lb_config" => Ok(GeneratedField::LeastRequestLbConfig),
                            "roundRobinLbConfig" | "round_robin_lb_config" => Ok(GeneratedField::RoundRobinLbConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Cluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Cluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transport_socket_matches__ = None;
                let mut name__ = None;
                let mut alt_stat_name__ = None;
                let mut eds_cluster_config__ = None;
                let mut connect_timeout__ = None;
                let mut per_connection_buffer_limit_bytes__ = None;
                let mut lb_policy__ = None;
                let mut load_assignment__ = None;
                let mut health_checks__ = None;
                let mut max_requests_per_connection__ = None;
                let mut circuit_breakers__ = None;
                let mut upstream_http_protocol_options__ = None;
                let mut common_http_protocol_options__ = None;
                let mut http_protocol_options__ = None;
                let mut http2_protocol_options__ = None;
                let mut typed_extension_protocol_options__ = None;
                let mut dns_refresh_rate__ = None;
                let mut dns_jitter__ = None;
                let mut dns_failure_refresh_rate__ = None;
                let mut respect_dns_ttl__ = None;
                let mut dns_lookup_family__ = None;
                let mut dns_resolvers__ = None;
                let mut use_tcp_for_dns_lookups__ = None;
                let mut dns_resolution_config__ = None;
                let mut typed_dns_resolver_config__ = None;
                let mut wait_for_warm_on_init__ = None;
                let mut outlier_detection__ = None;
                let mut cleanup_interval__ = None;
                let mut upstream_bind_config__ = None;
                let mut lb_subset_config__ = None;
                let mut common_lb_config__ = None;
                let mut transport_socket__ = None;
                let mut metadata__ = None;
                let mut protocol_selection__ = None;
                let mut upstream_connection_options__ = None;
                let mut close_connections_on_host_health_failure__ = None;
                let mut ignore_health_on_host_removal__ = None;
                let mut filters__ = None;
                let mut load_balancing_policy__ = None;
                let mut lrs_server__ = None;
                let mut lrs_report_endpoint_metrics__ = None;
                let mut track_timeout_budgets__ = None;
                let mut upstream_config__ = None;
                let mut track_cluster_stats__ = None;
                let mut preconnect_policy__ = None;
                let mut connection_pool_per_downstream_connection__ = None;
                let mut cluster_discovery_type__ = None;
                let mut lb_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TransportSocketMatches => {
                            if transport_socket_matches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportSocketMatches"));
                            }
                            transport_socket_matches__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AltStatName => {
                            if alt_stat_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("altStatName"));
                            }
                            alt_stat_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EdsClusterConfig => {
                            if eds_cluster_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("edsClusterConfig"));
                            }
                            eds_cluster_config__ = map_.next_value()?;
                        }
                        GeneratedField::ConnectTimeout => {
                            if connect_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectTimeout"));
                            }
                            connect_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::PerConnectionBufferLimitBytes => {
                            if per_connection_buffer_limit_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perConnectionBufferLimitBytes"));
                            }
                            per_connection_buffer_limit_bytes__ = map_.next_value()?;
                        }
                        GeneratedField::LbPolicy => {
                            if lb_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lbPolicy"));
                            }
                            lb_policy__ = Some(map_.next_value::<cluster::LbPolicy>()? as i32);
                        }
                        GeneratedField::LoadAssignment => {
                            if load_assignment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadAssignment"));
                            }
                            load_assignment__ = map_.next_value()?;
                        }
                        GeneratedField::HealthChecks => {
                            if health_checks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthChecks"));
                            }
                            health_checks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxRequestsPerConnection => {
                            if max_requests_per_connection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRequestsPerConnection"));
                            }
                            max_requests_per_connection__ = map_.next_value()?;
                        }
                        GeneratedField::CircuitBreakers => {
                            if circuit_breakers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("circuitBreakers"));
                            }
                            circuit_breakers__ = map_.next_value()?;
                        }
                        GeneratedField::UpstreamHttpProtocolOptions => {
                            if upstream_http_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamHttpProtocolOptions"));
                            }
                            upstream_http_protocol_options__ = map_.next_value()?;
                        }
                        GeneratedField::CommonHttpProtocolOptions => {
                            if common_http_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonHttpProtocolOptions"));
                            }
                            common_http_protocol_options__ = map_.next_value()?;
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
                        GeneratedField::TypedExtensionProtocolOptions => {
                            if typed_extension_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedExtensionProtocolOptions"));
                            }
                            typed_extension_protocol_options__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::DnsRefreshRate => {
                            if dns_refresh_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsRefreshRate"));
                            }
                            dns_refresh_rate__ = map_.next_value()?;
                        }
                        GeneratedField::DnsJitter => {
                            if dns_jitter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsJitter"));
                            }
                            dns_jitter__ = map_.next_value()?;
                        }
                        GeneratedField::DnsFailureRefreshRate => {
                            if dns_failure_refresh_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsFailureRefreshRate"));
                            }
                            dns_failure_refresh_rate__ = map_.next_value()?;
                        }
                        GeneratedField::RespectDnsTtl => {
                            if respect_dns_ttl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("respectDnsTtl"));
                            }
                            respect_dns_ttl__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DnsLookupFamily => {
                            if dns_lookup_family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsLookupFamily"));
                            }
                            dns_lookup_family__ = Some(map_.next_value::<cluster::DnsLookupFamily>()? as i32);
                        }
                        GeneratedField::DnsResolvers => {
                            if dns_resolvers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsResolvers"));
                            }
                            dns_resolvers__ = Some(map_.next_value()?);
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
                        GeneratedField::WaitForWarmOnInit => {
                            if wait_for_warm_on_init__.is_some() {
                                return Err(serde::de::Error::duplicate_field("waitForWarmOnInit"));
                            }
                            wait_for_warm_on_init__ = map_.next_value()?;
                        }
                        GeneratedField::OutlierDetection => {
                            if outlier_detection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outlierDetection"));
                            }
                            outlier_detection__ = map_.next_value()?;
                        }
                        GeneratedField::CleanupInterval => {
                            if cleanup_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cleanupInterval"));
                            }
                            cleanup_interval__ = map_.next_value()?;
                        }
                        GeneratedField::UpstreamBindConfig => {
                            if upstream_bind_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamBindConfig"));
                            }
                            upstream_bind_config__ = map_.next_value()?;
                        }
                        GeneratedField::LbSubsetConfig => {
                            if lb_subset_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lbSubsetConfig"));
                            }
                            lb_subset_config__ = map_.next_value()?;
                        }
                        GeneratedField::CommonLbConfig => {
                            if common_lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonLbConfig"));
                            }
                            common_lb_config__ = map_.next_value()?;
                        }
                        GeneratedField::TransportSocket => {
                            if transport_socket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportSocket"));
                            }
                            transport_socket__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::ProtocolSelection => {
                            if protocol_selection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolSelection"));
                            }
                            protocol_selection__ = Some(map_.next_value::<cluster::ClusterProtocolSelection>()? as i32);
                        }
                        GeneratedField::UpstreamConnectionOptions => {
                            if upstream_connection_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamConnectionOptions"));
                            }
                            upstream_connection_options__ = map_.next_value()?;
                        }
                        GeneratedField::CloseConnectionsOnHostHealthFailure => {
                            if close_connections_on_host_health_failure__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closeConnectionsOnHostHealthFailure"));
                            }
                            close_connections_on_host_health_failure__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IgnoreHealthOnHostRemoval => {
                            if ignore_health_on_host_removal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreHealthOnHostRemoval"));
                            }
                            ignore_health_on_host_removal__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoadBalancingPolicy => {
                            if load_balancing_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadBalancingPolicy"));
                            }
                            load_balancing_policy__ = map_.next_value()?;
                        }
                        GeneratedField::LrsServer => {
                            if lrs_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lrsServer"));
                            }
                            lrs_server__ = map_.next_value()?;
                        }
                        GeneratedField::LrsReportEndpointMetrics => {
                            if lrs_report_endpoint_metrics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lrsReportEndpointMetrics"));
                            }
                            lrs_report_endpoint_metrics__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TrackTimeoutBudgets => {
                            if track_timeout_budgets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackTimeoutBudgets"));
                            }
                            track_timeout_budgets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpstreamConfig => {
                            if upstream_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamConfig"));
                            }
                            upstream_config__ = map_.next_value()?;
                        }
                        GeneratedField::TrackClusterStats => {
                            if track_cluster_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackClusterStats"));
                            }
                            track_cluster_stats__ = map_.next_value()?;
                        }
                        GeneratedField::PreconnectPolicy => {
                            if preconnect_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preconnectPolicy"));
                            }
                            preconnect_policy__ = map_.next_value()?;
                        }
                        GeneratedField::ConnectionPoolPerDownstreamConnection => {
                            if connection_pool_per_downstream_connection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionPoolPerDownstreamConnection"));
                            }
                            connection_pool_per_downstream_connection__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if cluster_discovery_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            cluster_discovery_type__ = map_.next_value::<::std::option::Option<cluster::DiscoveryType>>()?.map(|x| cluster::ClusterDiscoveryType::Type(x as i32));
                        }
                        GeneratedField::ClusterType => {
                            if cluster_discovery_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterType"));
                            }
                            cluster_discovery_type__ = map_.next_value::<::std::option::Option<_>>()?.map(cluster::ClusterDiscoveryType::ClusterType)
;
                        }
                        GeneratedField::RingHashLbConfig => {
                            if lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ringHashLbConfig"));
                            }
                            lb_config__ = map_.next_value::<::std::option::Option<_>>()?.map(cluster::LbConfig::RingHashLbConfig)
;
                        }
                        GeneratedField::MaglevLbConfig => {
                            if lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maglevLbConfig"));
                            }
                            lb_config__ = map_.next_value::<::std::option::Option<_>>()?.map(cluster::LbConfig::MaglevLbConfig)
;
                        }
                        GeneratedField::OriginalDstLbConfig => {
                            if lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originalDstLbConfig"));
                            }
                            lb_config__ = map_.next_value::<::std::option::Option<_>>()?.map(cluster::LbConfig::OriginalDstLbConfig)
;
                        }
                        GeneratedField::LeastRequestLbConfig => {
                            if lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leastRequestLbConfig"));
                            }
                            lb_config__ = map_.next_value::<::std::option::Option<_>>()?.map(cluster::LbConfig::LeastRequestLbConfig)
;
                        }
                        GeneratedField::RoundRobinLbConfig => {
                            if lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roundRobinLbConfig"));
                            }
                            lb_config__ = map_.next_value::<::std::option::Option<_>>()?.map(cluster::LbConfig::RoundRobinLbConfig)
;
                        }
                    }
                }
                Ok(Cluster {
                    transport_socket_matches: transport_socket_matches__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    alt_stat_name: alt_stat_name__.unwrap_or_default(),
                    eds_cluster_config: eds_cluster_config__,
                    connect_timeout: connect_timeout__,
                    per_connection_buffer_limit_bytes: per_connection_buffer_limit_bytes__,
                    lb_policy: lb_policy__.unwrap_or_default(),
                    load_assignment: load_assignment__,
                    health_checks: health_checks__.unwrap_or_default(),
                    max_requests_per_connection: max_requests_per_connection__,
                    circuit_breakers: circuit_breakers__,
                    upstream_http_protocol_options: upstream_http_protocol_options__,
                    common_http_protocol_options: common_http_protocol_options__,
                    http_protocol_options: http_protocol_options__,
                    http2_protocol_options: http2_protocol_options__,
                    typed_extension_protocol_options: typed_extension_protocol_options__.unwrap_or_default(),
                    dns_refresh_rate: dns_refresh_rate__,
                    dns_jitter: dns_jitter__,
                    dns_failure_refresh_rate: dns_failure_refresh_rate__,
                    respect_dns_ttl: respect_dns_ttl__.unwrap_or_default(),
                    dns_lookup_family: dns_lookup_family__.unwrap_or_default(),
                    dns_resolvers: dns_resolvers__.unwrap_or_default(),
                    use_tcp_for_dns_lookups: use_tcp_for_dns_lookups__.unwrap_or_default(),
                    dns_resolution_config: dns_resolution_config__,
                    typed_dns_resolver_config: typed_dns_resolver_config__,
                    wait_for_warm_on_init: wait_for_warm_on_init__,
                    outlier_detection: outlier_detection__,
                    cleanup_interval: cleanup_interval__,
                    upstream_bind_config: upstream_bind_config__,
                    lb_subset_config: lb_subset_config__,
                    common_lb_config: common_lb_config__,
                    transport_socket: transport_socket__,
                    metadata: metadata__,
                    protocol_selection: protocol_selection__.unwrap_or_default(),
                    upstream_connection_options: upstream_connection_options__,
                    close_connections_on_host_health_failure: close_connections_on_host_health_failure__.unwrap_or_default(),
                    ignore_health_on_host_removal: ignore_health_on_host_removal__.unwrap_or_default(),
                    filters: filters__.unwrap_or_default(),
                    load_balancing_policy: load_balancing_policy__,
                    lrs_server: lrs_server__,
                    lrs_report_endpoint_metrics: lrs_report_endpoint_metrics__.unwrap_or_default(),
                    track_timeout_budgets: track_timeout_budgets__.unwrap_or_default(),
                    upstream_config: upstream_config__,
                    track_cluster_stats: track_cluster_stats__,
                    preconnect_policy: preconnect_policy__,
                    connection_pool_per_downstream_connection: connection_pool_per_downstream_connection__.unwrap_or_default(),
                    cluster_discovery_type: cluster_discovery_type__,
                    lb_config: lb_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::ClusterProtocolSelection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::UseConfiguredProtocol => "USE_CONFIGURED_PROTOCOL",
            Self::UseDownstreamProtocol => "USE_DOWNSTREAM_PROTOCOL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for cluster::ClusterProtocolSelection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "USE_CONFIGURED_PROTOCOL",
            "USE_DOWNSTREAM_PROTOCOL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::ClusterProtocolSelection;

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
                    "USE_CONFIGURED_PROTOCOL" => Ok(cluster::ClusterProtocolSelection::UseConfiguredProtocol),
                    "USE_DOWNSTREAM_PROTOCOL" => Ok(cluster::ClusterProtocolSelection::UseDownstreamProtocol),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::CommonLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.healthy_panic_threshold.is_some() {
            len += 1;
        }
        if self.update_merge_window.is_some() {
            len += 1;
        }
        if self.ignore_new_hosts_until_first_hc {
            len += 1;
        }
        if self.close_connections_on_host_set_change {
            len += 1;
        }
        if self.consistent_hashing_lb_config.is_some() {
            len += 1;
        }
        if self.override_host_status.is_some() {
            len += 1;
        }
        if self.locality_config_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster.CommonLbConfig", len)?;
        if let Some(v) = self.healthy_panic_threshold.as_ref() {
            struct_ser.serialize_field("healthy_panic_threshold", v)?;
        }
        if let Some(v) = self.update_merge_window.as_ref() {
            struct_ser.serialize_field("update_merge_window", v)?;
        }
        if self.ignore_new_hosts_until_first_hc {
            struct_ser.serialize_field("ignore_new_hosts_until_first_hc", &self.ignore_new_hosts_until_first_hc)?;
        }
        if self.close_connections_on_host_set_change {
            struct_ser.serialize_field("close_connections_on_host_set_change", &self.close_connections_on_host_set_change)?;
        }
        if let Some(v) = self.consistent_hashing_lb_config.as_ref() {
            struct_ser.serialize_field("consistent_hashing_lb_config", v)?;
        }
        if let Some(v) = self.override_host_status.as_ref() {
            struct_ser.serialize_field("override_host_status", v)?;
        }
        if let Some(v) = self.locality_config_specifier.as_ref() {
            match v {
                cluster::common_lb_config::LocalityConfigSpecifier::ZoneAwareLbConfig(v) => {
                    struct_ser.serialize_field("zone_aware_lb_config", v)?;
                }
                cluster::common_lb_config::LocalityConfigSpecifier::LocalityWeightedLbConfig(v) => {
                    struct_ser.serialize_field("locality_weighted_lb_config", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::CommonLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "healthy_panic_threshold",
            "healthyPanicThreshold",
            "update_merge_window",
            "updateMergeWindow",
            "ignore_new_hosts_until_first_hc",
            "ignoreNewHostsUntilFirstHc",
            "close_connections_on_host_set_change",
            "closeConnectionsOnHostSetChange",
            "consistent_hashing_lb_config",
            "consistentHashingLbConfig",
            "override_host_status",
            "overrideHostStatus",
            "zone_aware_lb_config",
            "zoneAwareLbConfig",
            "locality_weighted_lb_config",
            "localityWeightedLbConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HealthyPanicThreshold,
            UpdateMergeWindow,
            IgnoreNewHostsUntilFirstHc,
            CloseConnectionsOnHostSetChange,
            ConsistentHashingLbConfig,
            OverrideHostStatus,
            ZoneAwareLbConfig,
            LocalityWeightedLbConfig,
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
                            "healthyPanicThreshold" | "healthy_panic_threshold" => Ok(GeneratedField::HealthyPanicThreshold),
                            "updateMergeWindow" | "update_merge_window" => Ok(GeneratedField::UpdateMergeWindow),
                            "ignoreNewHostsUntilFirstHc" | "ignore_new_hosts_until_first_hc" => Ok(GeneratedField::IgnoreNewHostsUntilFirstHc),
                            "closeConnectionsOnHostSetChange" | "close_connections_on_host_set_change" => Ok(GeneratedField::CloseConnectionsOnHostSetChange),
                            "consistentHashingLbConfig" | "consistent_hashing_lb_config" => Ok(GeneratedField::ConsistentHashingLbConfig),
                            "overrideHostStatus" | "override_host_status" => Ok(GeneratedField::OverrideHostStatus),
                            "zoneAwareLbConfig" | "zone_aware_lb_config" => Ok(GeneratedField::ZoneAwareLbConfig),
                            "localityWeightedLbConfig" | "locality_weighted_lb_config" => Ok(GeneratedField::LocalityWeightedLbConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::CommonLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster.CommonLbConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster::CommonLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut healthy_panic_threshold__ = None;
                let mut update_merge_window__ = None;
                let mut ignore_new_hosts_until_first_hc__ = None;
                let mut close_connections_on_host_set_change__ = None;
                let mut consistent_hashing_lb_config__ = None;
                let mut override_host_status__ = None;
                let mut locality_config_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HealthyPanicThreshold => {
                            if healthy_panic_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthyPanicThreshold"));
                            }
                            healthy_panic_threshold__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMergeWindow => {
                            if update_merge_window__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMergeWindow"));
                            }
                            update_merge_window__ = map_.next_value()?;
                        }
                        GeneratedField::IgnoreNewHostsUntilFirstHc => {
                            if ignore_new_hosts_until_first_hc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreNewHostsUntilFirstHc"));
                            }
                            ignore_new_hosts_until_first_hc__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CloseConnectionsOnHostSetChange => {
                            if close_connections_on_host_set_change__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closeConnectionsOnHostSetChange"));
                            }
                            close_connections_on_host_set_change__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConsistentHashingLbConfig => {
                            if consistent_hashing_lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistentHashingLbConfig"));
                            }
                            consistent_hashing_lb_config__ = map_.next_value()?;
                        }
                        GeneratedField::OverrideHostStatus => {
                            if override_host_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrideHostStatus"));
                            }
                            override_host_status__ = map_.next_value()?;
                        }
                        GeneratedField::ZoneAwareLbConfig => {
                            if locality_config_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneAwareLbConfig"));
                            }
                            locality_config_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(cluster::common_lb_config::LocalityConfigSpecifier::ZoneAwareLbConfig)
;
                        }
                        GeneratedField::LocalityWeightedLbConfig => {
                            if locality_config_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localityWeightedLbConfig"));
                            }
                            locality_config_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(cluster::common_lb_config::LocalityConfigSpecifier::LocalityWeightedLbConfig)
;
                        }
                    }
                }
                Ok(cluster::CommonLbConfig {
                    healthy_panic_threshold: healthy_panic_threshold__,
                    update_merge_window: update_merge_window__,
                    ignore_new_hosts_until_first_hc: ignore_new_hosts_until_first_hc__.unwrap_or_default(),
                    close_connections_on_host_set_change: close_connections_on_host_set_change__.unwrap_or_default(),
                    consistent_hashing_lb_config: consistent_hashing_lb_config__,
                    override_host_status: override_host_status__,
                    locality_config_specifier: locality_config_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster.CommonLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::common_lb_config::ConsistentHashingLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.use_hostname_for_hashing {
            len += 1;
        }
        if self.hash_balance_factor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster.CommonLbConfig.ConsistentHashingLbConfig", len)?;
        if self.use_hostname_for_hashing {
            struct_ser.serialize_field("use_hostname_for_hashing", &self.use_hostname_for_hashing)?;
        }
        if let Some(v) = self.hash_balance_factor.as_ref() {
            struct_ser.serialize_field("hash_balance_factor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::common_lb_config::ConsistentHashingLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "use_hostname_for_hashing",
            "useHostnameForHashing",
            "hash_balance_factor",
            "hashBalanceFactor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UseHostnameForHashing,
            HashBalanceFactor,
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
                            "useHostnameForHashing" | "use_hostname_for_hashing" => Ok(GeneratedField::UseHostnameForHashing),
                            "hashBalanceFactor" | "hash_balance_factor" => Ok(GeneratedField::HashBalanceFactor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::common_lb_config::ConsistentHashingLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster.CommonLbConfig.ConsistentHashingLbConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster::common_lb_config::ConsistentHashingLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut use_hostname_for_hashing__ = None;
                let mut hash_balance_factor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UseHostnameForHashing => {
                            if use_hostname_for_hashing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useHostnameForHashing"));
                            }
                            use_hostname_for_hashing__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HashBalanceFactor => {
                            if hash_balance_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashBalanceFactor"));
                            }
                            hash_balance_factor__ = map_.next_value()?;
                        }
                    }
                }
                Ok(cluster::common_lb_config::ConsistentHashingLbConfig {
                    use_hostname_for_hashing: use_hostname_for_hashing__.unwrap_or_default(),
                    hash_balance_factor: hash_balance_factor__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster.CommonLbConfig.ConsistentHashingLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::common_lb_config::LocalityWeightedLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster.CommonLbConfig.LocalityWeightedLbConfig", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::common_lb_config::LocalityWeightedLbConfig {
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
            type Value = cluster::common_lb_config::LocalityWeightedLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster.CommonLbConfig.LocalityWeightedLbConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster::common_lb_config::LocalityWeightedLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(cluster::common_lb_config::LocalityWeightedLbConfig {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster.CommonLbConfig.LocalityWeightedLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::common_lb_config::ZoneAwareLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.routing_enabled.is_some() {
            len += 1;
        }
        if self.min_cluster_size.is_some() {
            len += 1;
        }
        if self.fail_traffic_on_panic {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster.CommonLbConfig.ZoneAwareLbConfig", len)?;
        if let Some(v) = self.routing_enabled.as_ref() {
            struct_ser.serialize_field("routing_enabled", v)?;
        }
        if let Some(v) = self.min_cluster_size.as_ref() {
            struct_ser.serialize_field("min_cluster_size", v)?;
        }
        if self.fail_traffic_on_panic {
            struct_ser.serialize_field("fail_traffic_on_panic", &self.fail_traffic_on_panic)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::common_lb_config::ZoneAwareLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "routing_enabled",
            "routingEnabled",
            "min_cluster_size",
            "minClusterSize",
            "fail_traffic_on_panic",
            "failTrafficOnPanic",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoutingEnabled,
            MinClusterSize,
            FailTrafficOnPanic,
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
                            "routingEnabled" | "routing_enabled" => Ok(GeneratedField::RoutingEnabled),
                            "minClusterSize" | "min_cluster_size" => Ok(GeneratedField::MinClusterSize),
                            "failTrafficOnPanic" | "fail_traffic_on_panic" => Ok(GeneratedField::FailTrafficOnPanic),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::common_lb_config::ZoneAwareLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster.CommonLbConfig.ZoneAwareLbConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster::common_lb_config::ZoneAwareLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut routing_enabled__ = None;
                let mut min_cluster_size__ = None;
                let mut fail_traffic_on_panic__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoutingEnabled => {
                            if routing_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routingEnabled"));
                            }
                            routing_enabled__ = map_.next_value()?;
                        }
                        GeneratedField::MinClusterSize => {
                            if min_cluster_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minClusterSize"));
                            }
                            min_cluster_size__ = map_.next_value()?;
                        }
                        GeneratedField::FailTrafficOnPanic => {
                            if fail_traffic_on_panic__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failTrafficOnPanic"));
                            }
                            fail_traffic_on_panic__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(cluster::common_lb_config::ZoneAwareLbConfig {
                    routing_enabled: routing_enabled__,
                    min_cluster_size: min_cluster_size__,
                    fail_traffic_on_panic: fail_traffic_on_panic__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster.CommonLbConfig.ZoneAwareLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::CustomClusterType {
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
        if self.typed_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster.CustomClusterType", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.typed_config.as_ref() {
            struct_ser.serialize_field("typed_config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::CustomClusterType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            "name" => Ok(GeneratedField::Name),
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
            type Value = cluster::CustomClusterType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster.CustomClusterType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster::CustomClusterType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut typed_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypedConfig => {
                            if typed_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            typed_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(cluster::CustomClusterType {
                    name: name__.unwrap_or_default(),
                    typed_config: typed_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster.CustomClusterType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::DiscoveryType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Static => "STATIC",
            Self::StrictDns => "STRICT_DNS",
            Self::LogicalDns => "LOGICAL_DNS",
            Self::Eds => "EDS",
            Self::OriginalDst => "ORIGINAL_DST",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for cluster::DiscoveryType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATIC",
            "STRICT_DNS",
            "LOGICAL_DNS",
            "EDS",
            "ORIGINAL_DST",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::DiscoveryType;

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
                    "STATIC" => Ok(cluster::DiscoveryType::Static),
                    "STRICT_DNS" => Ok(cluster::DiscoveryType::StrictDns),
                    "LOGICAL_DNS" => Ok(cluster::DiscoveryType::LogicalDns),
                    "EDS" => Ok(cluster::DiscoveryType::Eds),
                    "ORIGINAL_DST" => Ok(cluster::DiscoveryType::OriginalDst),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::DnsLookupFamily {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Auto => "AUTO",
            Self::V4Only => "V4_ONLY",
            Self::V6Only => "V6_ONLY",
            Self::V4Preferred => "V4_PREFERRED",
            Self::All => "ALL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for cluster::DnsLookupFamily {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUTO",
            "V4_ONLY",
            "V6_ONLY",
            "V4_PREFERRED",
            "ALL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::DnsLookupFamily;

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
                    "AUTO" => Ok(cluster::DnsLookupFamily::Auto),
                    "V4_ONLY" => Ok(cluster::DnsLookupFamily::V4Only),
                    "V6_ONLY" => Ok(cluster::DnsLookupFamily::V6Only),
                    "V4_PREFERRED" => Ok(cluster::DnsLookupFamily::V4Preferred),
                    "ALL" => Ok(cluster::DnsLookupFamily::All),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::EdsClusterConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.eds_config.is_some() {
            len += 1;
        }
        if !self.service_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster.EdsClusterConfig", len)?;
        if let Some(v) = self.eds_config.as_ref() {
            struct_ser.serialize_field("eds_config", v)?;
        }
        if !self.service_name.is_empty() {
            struct_ser.serialize_field("service_name", &self.service_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::EdsClusterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "eds_config",
            "edsConfig",
            "service_name",
            "serviceName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EdsConfig,
            ServiceName,
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
                            "edsConfig" | "eds_config" => Ok(GeneratedField::EdsConfig),
                            "serviceName" | "service_name" => Ok(GeneratedField::ServiceName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::EdsClusterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster.EdsClusterConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster::EdsClusterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut eds_config__ = None;
                let mut service_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EdsConfig => {
                            if eds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("edsConfig"));
                            }
                            eds_config__ = map_.next_value()?;
                        }
                        GeneratedField::ServiceName => {
                            if service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            service_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(cluster::EdsClusterConfig {
                    eds_config: eds_config__,
                    service_name: service_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster.EdsClusterConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::LbPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::RoundRobin => "ROUND_ROBIN",
            Self::LeastRequest => "LEAST_REQUEST",
            Self::RingHash => "RING_HASH",
            Self::Random => "RANDOM",
            Self::Maglev => "MAGLEV",
            Self::ClusterProvided => "CLUSTER_PROVIDED",
            Self::LoadBalancingPolicyConfig => "LOAD_BALANCING_POLICY_CONFIG",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for cluster::LbPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ROUND_ROBIN",
            "LEAST_REQUEST",
            "RING_HASH",
            "RANDOM",
            "MAGLEV",
            "CLUSTER_PROVIDED",
            "LOAD_BALANCING_POLICY_CONFIG",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::LbPolicy;

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
                    "ROUND_ROBIN" => Ok(cluster::LbPolicy::RoundRobin),
                    "LEAST_REQUEST" => Ok(cluster::LbPolicy::LeastRequest),
                    "RING_HASH" => Ok(cluster::LbPolicy::RingHash),
                    "RANDOM" => Ok(cluster::LbPolicy::Random),
                    "MAGLEV" => Ok(cluster::LbPolicy::Maglev),
                    "CLUSTER_PROVIDED" => Ok(cluster::LbPolicy::ClusterProvided),
                    "LOAD_BALANCING_POLICY_CONFIG" => Ok(cluster::LbPolicy::LoadBalancingPolicyConfig),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::LbSubsetConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.fallback_policy != 0 {
            len += 1;
        }
        if self.default_subset.is_some() {
            len += 1;
        }
        if !self.subset_selectors.is_empty() {
            len += 1;
        }
        if self.locality_weight_aware {
            len += 1;
        }
        if self.scale_locality_weight {
            len += 1;
        }
        if self.panic_mode_any {
            len += 1;
        }
        if self.list_as_any {
            len += 1;
        }
        if self.metadata_fallback_policy != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster.LbSubsetConfig", len)?;
        if self.fallback_policy != 0 {
            let v = cluster::lb_subset_config::LbSubsetFallbackPolicy::try_from(self.fallback_policy)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.fallback_policy)))?;
            struct_ser.serialize_field("fallback_policy", &v)?;
        }
        if let Some(v) = self.default_subset.as_ref() {
            struct_ser.serialize_field("default_subset", v)?;
        }
        if !self.subset_selectors.is_empty() {
            struct_ser.serialize_field("subset_selectors", &self.subset_selectors)?;
        }
        if self.locality_weight_aware {
            struct_ser.serialize_field("locality_weight_aware", &self.locality_weight_aware)?;
        }
        if self.scale_locality_weight {
            struct_ser.serialize_field("scale_locality_weight", &self.scale_locality_weight)?;
        }
        if self.panic_mode_any {
            struct_ser.serialize_field("panic_mode_any", &self.panic_mode_any)?;
        }
        if self.list_as_any {
            struct_ser.serialize_field("list_as_any", &self.list_as_any)?;
        }
        if self.metadata_fallback_policy != 0 {
            let v = cluster::lb_subset_config::LbSubsetMetadataFallbackPolicy::try_from(self.metadata_fallback_policy)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.metadata_fallback_policy)))?;
            struct_ser.serialize_field("metadata_fallback_policy", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::LbSubsetConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fallback_policy",
            "fallbackPolicy",
            "default_subset",
            "defaultSubset",
            "subset_selectors",
            "subsetSelectors",
            "locality_weight_aware",
            "localityWeightAware",
            "scale_locality_weight",
            "scaleLocalityWeight",
            "panic_mode_any",
            "panicModeAny",
            "list_as_any",
            "listAsAny",
            "metadata_fallback_policy",
            "metadataFallbackPolicy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FallbackPolicy,
            DefaultSubset,
            SubsetSelectors,
            LocalityWeightAware,
            ScaleLocalityWeight,
            PanicModeAny,
            ListAsAny,
            MetadataFallbackPolicy,
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
                            "fallbackPolicy" | "fallback_policy" => Ok(GeneratedField::FallbackPolicy),
                            "defaultSubset" | "default_subset" => Ok(GeneratedField::DefaultSubset),
                            "subsetSelectors" | "subset_selectors" => Ok(GeneratedField::SubsetSelectors),
                            "localityWeightAware" | "locality_weight_aware" => Ok(GeneratedField::LocalityWeightAware),
                            "scaleLocalityWeight" | "scale_locality_weight" => Ok(GeneratedField::ScaleLocalityWeight),
                            "panicModeAny" | "panic_mode_any" => Ok(GeneratedField::PanicModeAny),
                            "listAsAny" | "list_as_any" => Ok(GeneratedField::ListAsAny),
                            "metadataFallbackPolicy" | "metadata_fallback_policy" => Ok(GeneratedField::MetadataFallbackPolicy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::LbSubsetConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster.LbSubsetConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster::LbSubsetConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fallback_policy__ = None;
                let mut default_subset__ = None;
                let mut subset_selectors__ = None;
                let mut locality_weight_aware__ = None;
                let mut scale_locality_weight__ = None;
                let mut panic_mode_any__ = None;
                let mut list_as_any__ = None;
                let mut metadata_fallback_policy__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FallbackPolicy => {
                            if fallback_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fallbackPolicy"));
                            }
                            fallback_policy__ = Some(map_.next_value::<cluster::lb_subset_config::LbSubsetFallbackPolicy>()? as i32);
                        }
                        GeneratedField::DefaultSubset => {
                            if default_subset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultSubset"));
                            }
                            default_subset__ = map_.next_value()?;
                        }
                        GeneratedField::SubsetSelectors => {
                            if subset_selectors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subsetSelectors"));
                            }
                            subset_selectors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LocalityWeightAware => {
                            if locality_weight_aware__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localityWeightAware"));
                            }
                            locality_weight_aware__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ScaleLocalityWeight => {
                            if scale_locality_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scaleLocalityWeight"));
                            }
                            scale_locality_weight__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PanicModeAny => {
                            if panic_mode_any__.is_some() {
                                return Err(serde::de::Error::duplicate_field("panicModeAny"));
                            }
                            panic_mode_any__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ListAsAny => {
                            if list_as_any__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listAsAny"));
                            }
                            list_as_any__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MetadataFallbackPolicy => {
                            if metadata_fallback_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataFallbackPolicy"));
                            }
                            metadata_fallback_policy__ = Some(map_.next_value::<cluster::lb_subset_config::LbSubsetMetadataFallbackPolicy>()? as i32);
                        }
                    }
                }
                Ok(cluster::LbSubsetConfig {
                    fallback_policy: fallback_policy__.unwrap_or_default(),
                    default_subset: default_subset__,
                    subset_selectors: subset_selectors__.unwrap_or_default(),
                    locality_weight_aware: locality_weight_aware__.unwrap_or_default(),
                    scale_locality_weight: scale_locality_weight__.unwrap_or_default(),
                    panic_mode_any: panic_mode_any__.unwrap_or_default(),
                    list_as_any: list_as_any__.unwrap_or_default(),
                    metadata_fallback_policy: metadata_fallback_policy__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster.LbSubsetConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::lb_subset_config::LbSubsetFallbackPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::NoFallback => "NO_FALLBACK",
            Self::AnyEndpoint => "ANY_ENDPOINT",
            Self::DefaultSubset => "DEFAULT_SUBSET",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for cluster::lb_subset_config::LbSubsetFallbackPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NO_FALLBACK",
            "ANY_ENDPOINT",
            "DEFAULT_SUBSET",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::lb_subset_config::LbSubsetFallbackPolicy;

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
                    "NO_FALLBACK" => Ok(cluster::lb_subset_config::LbSubsetFallbackPolicy::NoFallback),
                    "ANY_ENDPOINT" => Ok(cluster::lb_subset_config::LbSubsetFallbackPolicy::AnyEndpoint),
                    "DEFAULT_SUBSET" => Ok(cluster::lb_subset_config::LbSubsetFallbackPolicy::DefaultSubset),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::lb_subset_config::LbSubsetMetadataFallbackPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::MetadataNoFallback => "METADATA_NO_FALLBACK",
            Self::FallbackList => "FALLBACK_LIST",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for cluster::lb_subset_config::LbSubsetMetadataFallbackPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "METADATA_NO_FALLBACK",
            "FALLBACK_LIST",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::lb_subset_config::LbSubsetMetadataFallbackPolicy;

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
                    "METADATA_NO_FALLBACK" => Ok(cluster::lb_subset_config::LbSubsetMetadataFallbackPolicy::MetadataNoFallback),
                    "FALLBACK_LIST" => Ok(cluster::lb_subset_config::LbSubsetMetadataFallbackPolicy::FallbackList),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::lb_subset_config::LbSubsetSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.keys.is_empty() {
            len += 1;
        }
        if self.single_host_per_subset {
            len += 1;
        }
        if self.fallback_policy != 0 {
            len += 1;
        }
        if !self.fallback_keys_subset.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetSelector", len)?;
        if !self.keys.is_empty() {
            struct_ser.serialize_field("keys", &self.keys)?;
        }
        if self.single_host_per_subset {
            struct_ser.serialize_field("single_host_per_subset", &self.single_host_per_subset)?;
        }
        if self.fallback_policy != 0 {
            let v = cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy::try_from(self.fallback_policy)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.fallback_policy)))?;
            struct_ser.serialize_field("fallback_policy", &v)?;
        }
        if !self.fallback_keys_subset.is_empty() {
            struct_ser.serialize_field("fallback_keys_subset", &self.fallback_keys_subset)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::lb_subset_config::LbSubsetSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "keys",
            "single_host_per_subset",
            "singleHostPerSubset",
            "fallback_policy",
            "fallbackPolicy",
            "fallback_keys_subset",
            "fallbackKeysSubset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Keys,
            SingleHostPerSubset,
            FallbackPolicy,
            FallbackKeysSubset,
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
                            "keys" => Ok(GeneratedField::Keys),
                            "singleHostPerSubset" | "single_host_per_subset" => Ok(GeneratedField::SingleHostPerSubset),
                            "fallbackPolicy" | "fallback_policy" => Ok(GeneratedField::FallbackPolicy),
                            "fallbackKeysSubset" | "fallback_keys_subset" => Ok(GeneratedField::FallbackKeysSubset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::lb_subset_config::LbSubsetSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetSelector")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster::lb_subset_config::LbSubsetSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut keys__ = None;
                let mut single_host_per_subset__ = None;
                let mut fallback_policy__ = None;
                let mut fallback_keys_subset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Keys => {
                            if keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keys"));
                            }
                            keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SingleHostPerSubset => {
                            if single_host_per_subset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("singleHostPerSubset"));
                            }
                            single_host_per_subset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FallbackPolicy => {
                            if fallback_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fallbackPolicy"));
                            }
                            fallback_policy__ = Some(map_.next_value::<cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy>()? as i32);
                        }
                        GeneratedField::FallbackKeysSubset => {
                            if fallback_keys_subset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fallbackKeysSubset"));
                            }
                            fallback_keys_subset__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(cluster::lb_subset_config::LbSubsetSelector {
                    keys: keys__.unwrap_or_default(),
                    single_host_per_subset: single_host_per_subset__.unwrap_or_default(),
                    fallback_policy: fallback_policy__.unwrap_or_default(),
                    fallback_keys_subset: fallback_keys_subset__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::NotDefined => "NOT_DEFINED",
            Self::NoFallback => "NO_FALLBACK",
            Self::AnyEndpoint => "ANY_ENDPOINT",
            Self::DefaultSubset => "DEFAULT_SUBSET",
            Self::KeysSubset => "KEYS_SUBSET",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NOT_DEFINED",
            "NO_FALLBACK",
            "ANY_ENDPOINT",
            "DEFAULT_SUBSET",
            "KEYS_SUBSET",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy;

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
                    "NOT_DEFINED" => Ok(cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy::NotDefined),
                    "NO_FALLBACK" => Ok(cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy::NoFallback),
                    "ANY_ENDPOINT" => Ok(cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy::AnyEndpoint),
                    "DEFAULT_SUBSET" => Ok(cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy::DefaultSubset),
                    "KEYS_SUBSET" => Ok(cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy::KeysSubset),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::LeastRequestLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.choice_count.is_some() {
            len += 1;
        }
        if self.active_request_bias.is_some() {
            len += 1;
        }
        if self.slow_start_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster.LeastRequestLbConfig", len)?;
        if let Some(v) = self.choice_count.as_ref() {
            struct_ser.serialize_field("choice_count", v)?;
        }
        if let Some(v) = self.active_request_bias.as_ref() {
            struct_ser.serialize_field("active_request_bias", v)?;
        }
        if let Some(v) = self.slow_start_config.as_ref() {
            struct_ser.serialize_field("slow_start_config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::LeastRequestLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "choice_count",
            "choiceCount",
            "active_request_bias",
            "activeRequestBias",
            "slow_start_config",
            "slowStartConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChoiceCount,
            ActiveRequestBias,
            SlowStartConfig,
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
                            "choiceCount" | "choice_count" => Ok(GeneratedField::ChoiceCount),
                            "activeRequestBias" | "active_request_bias" => Ok(GeneratedField::ActiveRequestBias),
                            "slowStartConfig" | "slow_start_config" => Ok(GeneratedField::SlowStartConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::LeastRequestLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster.LeastRequestLbConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster::LeastRequestLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut choice_count__ = None;
                let mut active_request_bias__ = None;
                let mut slow_start_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChoiceCount => {
                            if choice_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("choiceCount"));
                            }
                            choice_count__ = map_.next_value()?;
                        }
                        GeneratedField::ActiveRequestBias => {
                            if active_request_bias__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activeRequestBias"));
                            }
                            active_request_bias__ = map_.next_value()?;
                        }
                        GeneratedField::SlowStartConfig => {
                            if slow_start_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slowStartConfig"));
                            }
                            slow_start_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(cluster::LeastRequestLbConfig {
                    choice_count: choice_count__,
                    active_request_bias: active_request_bias__,
                    slow_start_config: slow_start_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster.LeastRequestLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::MaglevLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.table_size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster.MaglevLbConfig", len)?;
        if let Some(v) = self.table_size.as_ref() {
            struct_ser.serialize_field("table_size", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::MaglevLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_size",
            "tableSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableSize,
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
                            "tableSize" | "table_size" => Ok(GeneratedField::TableSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::MaglevLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster.MaglevLbConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster::MaglevLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableSize => {
                            if table_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableSize"));
                            }
                            table_size__ = map_.next_value()?;
                        }
                    }
                }
                Ok(cluster::MaglevLbConfig {
                    table_size: table_size__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster.MaglevLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::OriginalDstLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.use_http_header {
            len += 1;
        }
        if !self.http_header_name.is_empty() {
            len += 1;
        }
        if self.upstream_port_override.is_some() {
            len += 1;
        }
        if self.metadata_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster.OriginalDstLbConfig", len)?;
        if self.use_http_header {
            struct_ser.serialize_field("use_http_header", &self.use_http_header)?;
        }
        if !self.http_header_name.is_empty() {
            struct_ser.serialize_field("http_header_name", &self.http_header_name)?;
        }
        if let Some(v) = self.upstream_port_override.as_ref() {
            struct_ser.serialize_field("upstream_port_override", v)?;
        }
        if let Some(v) = self.metadata_key.as_ref() {
            struct_ser.serialize_field("metadata_key", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::OriginalDstLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "use_http_header",
            "useHttpHeader",
            "http_header_name",
            "httpHeaderName",
            "upstream_port_override",
            "upstreamPortOverride",
            "metadata_key",
            "metadataKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UseHttpHeader,
            HttpHeaderName,
            UpstreamPortOverride,
            MetadataKey,
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
                            "useHttpHeader" | "use_http_header" => Ok(GeneratedField::UseHttpHeader),
                            "httpHeaderName" | "http_header_name" => Ok(GeneratedField::HttpHeaderName),
                            "upstreamPortOverride" | "upstream_port_override" => Ok(GeneratedField::UpstreamPortOverride),
                            "metadataKey" | "metadata_key" => Ok(GeneratedField::MetadataKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::OriginalDstLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster.OriginalDstLbConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster::OriginalDstLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut use_http_header__ = None;
                let mut http_header_name__ = None;
                let mut upstream_port_override__ = None;
                let mut metadata_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UseHttpHeader => {
                            if use_http_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useHttpHeader"));
                            }
                            use_http_header__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HttpHeaderName => {
                            if http_header_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpHeaderName"));
                            }
                            http_header_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpstreamPortOverride => {
                            if upstream_port_override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamPortOverride"));
                            }
                            upstream_port_override__ = map_.next_value()?;
                        }
                        GeneratedField::MetadataKey => {
                            if metadata_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataKey"));
                            }
                            metadata_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(cluster::OriginalDstLbConfig {
                    use_http_header: use_http_header__.unwrap_or_default(),
                    http_header_name: http_header_name__.unwrap_or_default(),
                    upstream_port_override: upstream_port_override__,
                    metadata_key: metadata_key__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster.OriginalDstLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::PreconnectPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.per_upstream_preconnect_ratio.is_some() {
            len += 1;
        }
        if self.predictive_preconnect_ratio.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster.PreconnectPolicy", len)?;
        if let Some(v) = self.per_upstream_preconnect_ratio.as_ref() {
            struct_ser.serialize_field("per_upstream_preconnect_ratio", v)?;
        }
        if let Some(v) = self.predictive_preconnect_ratio.as_ref() {
            struct_ser.serialize_field("predictive_preconnect_ratio", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::PreconnectPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "per_upstream_preconnect_ratio",
            "perUpstreamPreconnectRatio",
            "predictive_preconnect_ratio",
            "predictivePreconnectRatio",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PerUpstreamPreconnectRatio,
            PredictivePreconnectRatio,
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
                            "perUpstreamPreconnectRatio" | "per_upstream_preconnect_ratio" => Ok(GeneratedField::PerUpstreamPreconnectRatio),
                            "predictivePreconnectRatio" | "predictive_preconnect_ratio" => Ok(GeneratedField::PredictivePreconnectRatio),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::PreconnectPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster.PreconnectPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster::PreconnectPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut per_upstream_preconnect_ratio__ = None;
                let mut predictive_preconnect_ratio__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PerUpstreamPreconnectRatio => {
                            if per_upstream_preconnect_ratio__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perUpstreamPreconnectRatio"));
                            }
                            per_upstream_preconnect_ratio__ = map_.next_value()?;
                        }
                        GeneratedField::PredictivePreconnectRatio => {
                            if predictive_preconnect_ratio__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predictivePreconnectRatio"));
                            }
                            predictive_preconnect_ratio__ = map_.next_value()?;
                        }
                    }
                }
                Ok(cluster::PreconnectPolicy {
                    per_upstream_preconnect_ratio: per_upstream_preconnect_ratio__,
                    predictive_preconnect_ratio: predictive_preconnect_ratio__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster.PreconnectPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::RefreshRate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base_interval.is_some() {
            len += 1;
        }
        if self.max_interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster.RefreshRate", len)?;
        if let Some(v) = self.base_interval.as_ref() {
            struct_ser.serialize_field("base_interval", v)?;
        }
        if let Some(v) = self.max_interval.as_ref() {
            struct_ser.serialize_field("max_interval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::RefreshRate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_interval",
            "baseInterval",
            "max_interval",
            "maxInterval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseInterval,
            MaxInterval,
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
                            "baseInterval" | "base_interval" => Ok(GeneratedField::BaseInterval),
                            "maxInterval" | "max_interval" => Ok(GeneratedField::MaxInterval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::RefreshRate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster.RefreshRate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster::RefreshRate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_interval__ = None;
                let mut max_interval__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseInterval => {
                            if base_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseInterval"));
                            }
                            base_interval__ = map_.next_value()?;
                        }
                        GeneratedField::MaxInterval => {
                            if max_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxInterval"));
                            }
                            max_interval__ = map_.next_value()?;
                        }
                    }
                }
                Ok(cluster::RefreshRate {
                    base_interval: base_interval__,
                    max_interval: max_interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster.RefreshRate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::RingHashLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.minimum_ring_size.is_some() {
            len += 1;
        }
        if self.hash_function != 0 {
            len += 1;
        }
        if self.maximum_ring_size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster.RingHashLbConfig", len)?;
        if let Some(v) = self.minimum_ring_size.as_ref() {
            struct_ser.serialize_field("minimum_ring_size", v)?;
        }
        if self.hash_function != 0 {
            let v = cluster::ring_hash_lb_config::HashFunction::try_from(self.hash_function)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.hash_function)))?;
            struct_ser.serialize_field("hash_function", &v)?;
        }
        if let Some(v) = self.maximum_ring_size.as_ref() {
            struct_ser.serialize_field("maximum_ring_size", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::RingHashLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "minimum_ring_size",
            "minimumRingSize",
            "hash_function",
            "hashFunction",
            "maximum_ring_size",
            "maximumRingSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinimumRingSize,
            HashFunction,
            MaximumRingSize,
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
                            "minimumRingSize" | "minimum_ring_size" => Ok(GeneratedField::MinimumRingSize),
                            "hashFunction" | "hash_function" => Ok(GeneratedField::HashFunction),
                            "maximumRingSize" | "maximum_ring_size" => Ok(GeneratedField::MaximumRingSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::RingHashLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster.RingHashLbConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster::RingHashLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut minimum_ring_size__ = None;
                let mut hash_function__ = None;
                let mut maximum_ring_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinimumRingSize => {
                            if minimum_ring_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minimumRingSize"));
                            }
                            minimum_ring_size__ = map_.next_value()?;
                        }
                        GeneratedField::HashFunction => {
                            if hash_function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashFunction"));
                            }
                            hash_function__ = Some(map_.next_value::<cluster::ring_hash_lb_config::HashFunction>()? as i32);
                        }
                        GeneratedField::MaximumRingSize => {
                            if maximum_ring_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maximumRingSize"));
                            }
                            maximum_ring_size__ = map_.next_value()?;
                        }
                    }
                }
                Ok(cluster::RingHashLbConfig {
                    minimum_ring_size: minimum_ring_size__,
                    hash_function: hash_function__.unwrap_or_default(),
                    maximum_ring_size: maximum_ring_size__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster.RingHashLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::ring_hash_lb_config::HashFunction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::XxHash => "XX_HASH",
            Self::MurmurHash2 => "MURMUR_HASH_2",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for cluster::ring_hash_lb_config::HashFunction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "XX_HASH",
            "MURMUR_HASH_2",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::ring_hash_lb_config::HashFunction;

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
                    "XX_HASH" => Ok(cluster::ring_hash_lb_config::HashFunction::XxHash),
                    "MURMUR_HASH_2" => Ok(cluster::ring_hash_lb_config::HashFunction::MurmurHash2),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::RoundRobinLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.slow_start_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster.RoundRobinLbConfig", len)?;
        if let Some(v) = self.slow_start_config.as_ref() {
            struct_ser.serialize_field("slow_start_config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::RoundRobinLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "slow_start_config",
            "slowStartConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SlowStartConfig,
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
                            "slowStartConfig" | "slow_start_config" => Ok(GeneratedField::SlowStartConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::RoundRobinLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster.RoundRobinLbConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster::RoundRobinLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut slow_start_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SlowStartConfig => {
                            if slow_start_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slowStartConfig"));
                            }
                            slow_start_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(cluster::RoundRobinLbConfig {
                    slow_start_config: slow_start_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster.RoundRobinLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::SlowStartConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.slow_start_window.is_some() {
            len += 1;
        }
        if self.aggression.is_some() {
            len += 1;
        }
        if self.min_weight_percent.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster.SlowStartConfig", len)?;
        if let Some(v) = self.slow_start_window.as_ref() {
            struct_ser.serialize_field("slow_start_window", v)?;
        }
        if let Some(v) = self.aggression.as_ref() {
            struct_ser.serialize_field("aggression", v)?;
        }
        if let Some(v) = self.min_weight_percent.as_ref() {
            struct_ser.serialize_field("min_weight_percent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::SlowStartConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "slow_start_window",
            "slowStartWindow",
            "aggression",
            "min_weight_percent",
            "minWeightPercent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SlowStartWindow,
            Aggression,
            MinWeightPercent,
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
                            "slowStartWindow" | "slow_start_window" => Ok(GeneratedField::SlowStartWindow),
                            "aggression" => Ok(GeneratedField::Aggression),
                            "minWeightPercent" | "min_weight_percent" => Ok(GeneratedField::MinWeightPercent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::SlowStartConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster.SlowStartConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster::SlowStartConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut slow_start_window__ = None;
                let mut aggression__ = None;
                let mut min_weight_percent__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SlowStartWindow => {
                            if slow_start_window__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slowStartWindow"));
                            }
                            slow_start_window__ = map_.next_value()?;
                        }
                        GeneratedField::Aggression => {
                            if aggression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggression"));
                            }
                            aggression__ = map_.next_value()?;
                        }
                        GeneratedField::MinWeightPercent => {
                            if min_weight_percent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minWeightPercent"));
                            }
                            min_weight_percent__ = map_.next_value()?;
                        }
                    }
                }
                Ok(cluster::SlowStartConfig {
                    slow_start_window: slow_start_window__,
                    aggression: aggression__,
                    min_weight_percent: min_weight_percent__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster.SlowStartConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::TransportSocketMatch {
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
        if self.r#match.is_some() {
            len += 1;
        }
        if self.transport_socket.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Cluster.TransportSocketMatch", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.r#match.as_ref() {
            struct_ser.serialize_field("match", v)?;
        }
        if let Some(v) = self.transport_socket.as_ref() {
            struct_ser.serialize_field("transport_socket", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::TransportSocketMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "match",
            "transport_socket",
            "transportSocket",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Match,
            TransportSocket,
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
                            "match" => Ok(GeneratedField::Match),
                            "transportSocket" | "transport_socket" => Ok(GeneratedField::TransportSocket),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::TransportSocketMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Cluster.TransportSocketMatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster::TransportSocketMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#match__ = None;
                let mut transport_socket__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Match => {
                            if r#match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            r#match__ = map_.next_value()?;
                        }
                        GeneratedField::TransportSocket => {
                            if transport_socket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportSocket"));
                            }
                            transport_socket__ = map_.next_value()?;
                        }
                    }
                }
                Ok(cluster::TransportSocketMatch {
                    name: name__.unwrap_or_default(),
                    r#match: r#match__,
                    transport_socket: transport_socket__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Cluster.TransportSocketMatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClusterCollection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.entries.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.ClusterCollection", len)?;
        if let Some(v) = self.entries.as_ref() {
            struct_ser.serialize_field("entries", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterCollection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entries,
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
                            "entries" => Ok(GeneratedField::Entries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterCollection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.ClusterCollection")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClusterCollection, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ClusterCollection {
                    entries: entries__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.ClusterCollection", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Filter {
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
        if self.typed_config.is_some() {
            len += 1;
        }
        if self.config_discovery.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.Filter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.typed_config.as_ref() {
            struct_ser.serialize_field("typed_config", v)?;
        }
        if let Some(v) = self.config_discovery.as_ref() {
            struct_ser.serialize_field("config_discovery", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Filter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "typed_config",
            "typedConfig",
            "config_discovery",
            "configDiscovery",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
            type Value = Filter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.Filter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Filter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut typed_config__ = None;
                let mut config_discovery__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypedConfig => {
                            if typed_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            typed_config__ = map_.next_value()?;
                        }
                        GeneratedField::ConfigDiscovery => {
                            if config_discovery__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configDiscovery"));
                            }
                            config_discovery__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Filter {
                    name: name__.unwrap_or_default(),
                    typed_config: typed_config__,
                    config_discovery: config_discovery__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.Filter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoadBalancingPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.policies.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.LoadBalancingPolicy", len)?;
        if !self.policies.is_empty() {
            struct_ser.serialize_field("policies", &self.policies)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoadBalancingPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "policies",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Policies,
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
                            "policies" => Ok(GeneratedField::Policies),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoadBalancingPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.LoadBalancingPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoadBalancingPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policies__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Policies => {
                            if policies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policies"));
                            }
                            policies__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LoadBalancingPolicy {
                    policies: policies__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.LoadBalancingPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for load_balancing_policy::Policy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.typed_extension_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.LoadBalancingPolicy.Policy", len)?;
        if let Some(v) = self.typed_extension_config.as_ref() {
            struct_ser.serialize_field("typed_extension_config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for load_balancing_policy::Policy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "typed_extension_config",
            "typedExtensionConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TypedExtensionConfig,
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
                            "typedExtensionConfig" | "typed_extension_config" => Ok(GeneratedField::TypedExtensionConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = load_balancing_policy::Policy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.LoadBalancingPolicy.Policy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<load_balancing_policy::Policy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut typed_extension_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TypedExtensionConfig => {
                            if typed_extension_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedExtensionConfig"));
                            }
                            typed_extension_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(load_balancing_policy::Policy {
                    typed_extension_config: typed_extension_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.LoadBalancingPolicy.Policy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OutlierDetection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consecutive_5xx.is_some() {
            len += 1;
        }
        if self.interval.is_some() {
            len += 1;
        }
        if self.base_ejection_time.is_some() {
            len += 1;
        }
        if self.max_ejection_percent.is_some() {
            len += 1;
        }
        if self.enforcing_consecutive_5xx.is_some() {
            len += 1;
        }
        if self.enforcing_success_rate.is_some() {
            len += 1;
        }
        if self.success_rate_minimum_hosts.is_some() {
            len += 1;
        }
        if self.success_rate_request_volume.is_some() {
            len += 1;
        }
        if self.success_rate_stdev_factor.is_some() {
            len += 1;
        }
        if self.consecutive_gateway_failure.is_some() {
            len += 1;
        }
        if self.enforcing_consecutive_gateway_failure.is_some() {
            len += 1;
        }
        if self.split_external_local_origin_errors {
            len += 1;
        }
        if self.consecutive_local_origin_failure.is_some() {
            len += 1;
        }
        if self.enforcing_consecutive_local_origin_failure.is_some() {
            len += 1;
        }
        if self.enforcing_local_origin_success_rate.is_some() {
            len += 1;
        }
        if self.failure_percentage_threshold.is_some() {
            len += 1;
        }
        if self.enforcing_failure_percentage.is_some() {
            len += 1;
        }
        if self.enforcing_failure_percentage_local_origin.is_some() {
            len += 1;
        }
        if self.failure_percentage_minimum_hosts.is_some() {
            len += 1;
        }
        if self.failure_percentage_request_volume.is_some() {
            len += 1;
        }
        if self.max_ejection_time.is_some() {
            len += 1;
        }
        if self.max_ejection_time_jitter.is_some() {
            len += 1;
        }
        if self.successful_active_health_check_uneject_host.is_some() {
            len += 1;
        }
        if !self.monitors.is_empty() {
            len += 1;
        }
        if self.always_eject_one_host.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.OutlierDetection", len)?;
        if let Some(v) = self.consecutive_5xx.as_ref() {
            struct_ser.serialize_field("consecutive_5xx", v)?;
        }
        if let Some(v) = self.interval.as_ref() {
            struct_ser.serialize_field("interval", v)?;
        }
        if let Some(v) = self.base_ejection_time.as_ref() {
            struct_ser.serialize_field("base_ejection_time", v)?;
        }
        if let Some(v) = self.max_ejection_percent.as_ref() {
            struct_ser.serialize_field("max_ejection_percent", v)?;
        }
        if let Some(v) = self.enforcing_consecutive_5xx.as_ref() {
            struct_ser.serialize_field("enforcing_consecutive_5xx", v)?;
        }
        if let Some(v) = self.enforcing_success_rate.as_ref() {
            struct_ser.serialize_field("enforcing_success_rate", v)?;
        }
        if let Some(v) = self.success_rate_minimum_hosts.as_ref() {
            struct_ser.serialize_field("success_rate_minimum_hosts", v)?;
        }
        if let Some(v) = self.success_rate_request_volume.as_ref() {
            struct_ser.serialize_field("success_rate_request_volume", v)?;
        }
        if let Some(v) = self.success_rate_stdev_factor.as_ref() {
            struct_ser.serialize_field("success_rate_stdev_factor", v)?;
        }
        if let Some(v) = self.consecutive_gateway_failure.as_ref() {
            struct_ser.serialize_field("consecutive_gateway_failure", v)?;
        }
        if let Some(v) = self.enforcing_consecutive_gateway_failure.as_ref() {
            struct_ser.serialize_field("enforcing_consecutive_gateway_failure", v)?;
        }
        if self.split_external_local_origin_errors {
            struct_ser.serialize_field("split_external_local_origin_errors", &self.split_external_local_origin_errors)?;
        }
        if let Some(v) = self.consecutive_local_origin_failure.as_ref() {
            struct_ser.serialize_field("consecutive_local_origin_failure", v)?;
        }
        if let Some(v) = self.enforcing_consecutive_local_origin_failure.as_ref() {
            struct_ser.serialize_field("enforcing_consecutive_local_origin_failure", v)?;
        }
        if let Some(v) = self.enforcing_local_origin_success_rate.as_ref() {
            struct_ser.serialize_field("enforcing_local_origin_success_rate", v)?;
        }
        if let Some(v) = self.failure_percentage_threshold.as_ref() {
            struct_ser.serialize_field("failure_percentage_threshold", v)?;
        }
        if let Some(v) = self.enforcing_failure_percentage.as_ref() {
            struct_ser.serialize_field("enforcing_failure_percentage", v)?;
        }
        if let Some(v) = self.enforcing_failure_percentage_local_origin.as_ref() {
            struct_ser.serialize_field("enforcing_failure_percentage_local_origin", v)?;
        }
        if let Some(v) = self.failure_percentage_minimum_hosts.as_ref() {
            struct_ser.serialize_field("failure_percentage_minimum_hosts", v)?;
        }
        if let Some(v) = self.failure_percentage_request_volume.as_ref() {
            struct_ser.serialize_field("failure_percentage_request_volume", v)?;
        }
        if let Some(v) = self.max_ejection_time.as_ref() {
            struct_ser.serialize_field("max_ejection_time", v)?;
        }
        if let Some(v) = self.max_ejection_time_jitter.as_ref() {
            struct_ser.serialize_field("max_ejection_time_jitter", v)?;
        }
        if let Some(v) = self.successful_active_health_check_uneject_host.as_ref() {
            struct_ser.serialize_field("successful_active_health_check_uneject_host", v)?;
        }
        if !self.monitors.is_empty() {
            struct_ser.serialize_field("monitors", &self.monitors)?;
        }
        if let Some(v) = self.always_eject_one_host.as_ref() {
            struct_ser.serialize_field("always_eject_one_host", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OutlierDetection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consecutive_5xx",
            "consecutive5xx",
            "interval",
            "base_ejection_time",
            "baseEjectionTime",
            "max_ejection_percent",
            "maxEjectionPercent",
            "enforcing_consecutive_5xx",
            "enforcingConsecutive5xx",
            "enforcing_success_rate",
            "enforcingSuccessRate",
            "success_rate_minimum_hosts",
            "successRateMinimumHosts",
            "success_rate_request_volume",
            "successRateRequestVolume",
            "success_rate_stdev_factor",
            "successRateStdevFactor",
            "consecutive_gateway_failure",
            "consecutiveGatewayFailure",
            "enforcing_consecutive_gateway_failure",
            "enforcingConsecutiveGatewayFailure",
            "split_external_local_origin_errors",
            "splitExternalLocalOriginErrors",
            "consecutive_local_origin_failure",
            "consecutiveLocalOriginFailure",
            "enforcing_consecutive_local_origin_failure",
            "enforcingConsecutiveLocalOriginFailure",
            "enforcing_local_origin_success_rate",
            "enforcingLocalOriginSuccessRate",
            "failure_percentage_threshold",
            "failurePercentageThreshold",
            "enforcing_failure_percentage",
            "enforcingFailurePercentage",
            "enforcing_failure_percentage_local_origin",
            "enforcingFailurePercentageLocalOrigin",
            "failure_percentage_minimum_hosts",
            "failurePercentageMinimumHosts",
            "failure_percentage_request_volume",
            "failurePercentageRequestVolume",
            "max_ejection_time",
            "maxEjectionTime",
            "max_ejection_time_jitter",
            "maxEjectionTimeJitter",
            "successful_active_health_check_uneject_host",
            "successfulActiveHealthCheckUnejectHost",
            "monitors",
            "always_eject_one_host",
            "alwaysEjectOneHost",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Consecutive5xx,
            Interval,
            BaseEjectionTime,
            MaxEjectionPercent,
            EnforcingConsecutive5xx,
            EnforcingSuccessRate,
            SuccessRateMinimumHosts,
            SuccessRateRequestVolume,
            SuccessRateStdevFactor,
            ConsecutiveGatewayFailure,
            EnforcingConsecutiveGatewayFailure,
            SplitExternalLocalOriginErrors,
            ConsecutiveLocalOriginFailure,
            EnforcingConsecutiveLocalOriginFailure,
            EnforcingLocalOriginSuccessRate,
            FailurePercentageThreshold,
            EnforcingFailurePercentage,
            EnforcingFailurePercentageLocalOrigin,
            FailurePercentageMinimumHosts,
            FailurePercentageRequestVolume,
            MaxEjectionTime,
            MaxEjectionTimeJitter,
            SuccessfulActiveHealthCheckUnejectHost,
            Monitors,
            AlwaysEjectOneHost,
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
                            "consecutive5xx" | "consecutive_5xx" => Ok(GeneratedField::Consecutive5xx),
                            "interval" => Ok(GeneratedField::Interval),
                            "baseEjectionTime" | "base_ejection_time" => Ok(GeneratedField::BaseEjectionTime),
                            "maxEjectionPercent" | "max_ejection_percent" => Ok(GeneratedField::MaxEjectionPercent),
                            "enforcingConsecutive5xx" | "enforcing_consecutive_5xx" => Ok(GeneratedField::EnforcingConsecutive5xx),
                            "enforcingSuccessRate" | "enforcing_success_rate" => Ok(GeneratedField::EnforcingSuccessRate),
                            "successRateMinimumHosts" | "success_rate_minimum_hosts" => Ok(GeneratedField::SuccessRateMinimumHosts),
                            "successRateRequestVolume" | "success_rate_request_volume" => Ok(GeneratedField::SuccessRateRequestVolume),
                            "successRateStdevFactor" | "success_rate_stdev_factor" => Ok(GeneratedField::SuccessRateStdevFactor),
                            "consecutiveGatewayFailure" | "consecutive_gateway_failure" => Ok(GeneratedField::ConsecutiveGatewayFailure),
                            "enforcingConsecutiveGatewayFailure" | "enforcing_consecutive_gateway_failure" => Ok(GeneratedField::EnforcingConsecutiveGatewayFailure),
                            "splitExternalLocalOriginErrors" | "split_external_local_origin_errors" => Ok(GeneratedField::SplitExternalLocalOriginErrors),
                            "consecutiveLocalOriginFailure" | "consecutive_local_origin_failure" => Ok(GeneratedField::ConsecutiveLocalOriginFailure),
                            "enforcingConsecutiveLocalOriginFailure" | "enforcing_consecutive_local_origin_failure" => Ok(GeneratedField::EnforcingConsecutiveLocalOriginFailure),
                            "enforcingLocalOriginSuccessRate" | "enforcing_local_origin_success_rate" => Ok(GeneratedField::EnforcingLocalOriginSuccessRate),
                            "failurePercentageThreshold" | "failure_percentage_threshold" => Ok(GeneratedField::FailurePercentageThreshold),
                            "enforcingFailurePercentage" | "enforcing_failure_percentage" => Ok(GeneratedField::EnforcingFailurePercentage),
                            "enforcingFailurePercentageLocalOrigin" | "enforcing_failure_percentage_local_origin" => Ok(GeneratedField::EnforcingFailurePercentageLocalOrigin),
                            "failurePercentageMinimumHosts" | "failure_percentage_minimum_hosts" => Ok(GeneratedField::FailurePercentageMinimumHosts),
                            "failurePercentageRequestVolume" | "failure_percentage_request_volume" => Ok(GeneratedField::FailurePercentageRequestVolume),
                            "maxEjectionTime" | "max_ejection_time" => Ok(GeneratedField::MaxEjectionTime),
                            "maxEjectionTimeJitter" | "max_ejection_time_jitter" => Ok(GeneratedField::MaxEjectionTimeJitter),
                            "successfulActiveHealthCheckUnejectHost" | "successful_active_health_check_uneject_host" => Ok(GeneratedField::SuccessfulActiveHealthCheckUnejectHost),
                            "monitors" => Ok(GeneratedField::Monitors),
                            "alwaysEjectOneHost" | "always_eject_one_host" => Ok(GeneratedField::AlwaysEjectOneHost),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OutlierDetection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.OutlierDetection")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OutlierDetection, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consecutive_5xx__ = None;
                let mut interval__ = None;
                let mut base_ejection_time__ = None;
                let mut max_ejection_percent__ = None;
                let mut enforcing_consecutive_5xx__ = None;
                let mut enforcing_success_rate__ = None;
                let mut success_rate_minimum_hosts__ = None;
                let mut success_rate_request_volume__ = None;
                let mut success_rate_stdev_factor__ = None;
                let mut consecutive_gateway_failure__ = None;
                let mut enforcing_consecutive_gateway_failure__ = None;
                let mut split_external_local_origin_errors__ = None;
                let mut consecutive_local_origin_failure__ = None;
                let mut enforcing_consecutive_local_origin_failure__ = None;
                let mut enforcing_local_origin_success_rate__ = None;
                let mut failure_percentage_threshold__ = None;
                let mut enforcing_failure_percentage__ = None;
                let mut enforcing_failure_percentage_local_origin__ = None;
                let mut failure_percentage_minimum_hosts__ = None;
                let mut failure_percentage_request_volume__ = None;
                let mut max_ejection_time__ = None;
                let mut max_ejection_time_jitter__ = None;
                let mut successful_active_health_check_uneject_host__ = None;
                let mut monitors__ = None;
                let mut always_eject_one_host__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Consecutive5xx => {
                            if consecutive_5xx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consecutive5xx"));
                            }
                            consecutive_5xx__ = map_.next_value()?;
                        }
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = map_.next_value()?;
                        }
                        GeneratedField::BaseEjectionTime => {
                            if base_ejection_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseEjectionTime"));
                            }
                            base_ejection_time__ = map_.next_value()?;
                        }
                        GeneratedField::MaxEjectionPercent => {
                            if max_ejection_percent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxEjectionPercent"));
                            }
                            max_ejection_percent__ = map_.next_value()?;
                        }
                        GeneratedField::EnforcingConsecutive5xx => {
                            if enforcing_consecutive_5xx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enforcingConsecutive5xx"));
                            }
                            enforcing_consecutive_5xx__ = map_.next_value()?;
                        }
                        GeneratedField::EnforcingSuccessRate => {
                            if enforcing_success_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enforcingSuccessRate"));
                            }
                            enforcing_success_rate__ = map_.next_value()?;
                        }
                        GeneratedField::SuccessRateMinimumHosts => {
                            if success_rate_minimum_hosts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("successRateMinimumHosts"));
                            }
                            success_rate_minimum_hosts__ = map_.next_value()?;
                        }
                        GeneratedField::SuccessRateRequestVolume => {
                            if success_rate_request_volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("successRateRequestVolume"));
                            }
                            success_rate_request_volume__ = map_.next_value()?;
                        }
                        GeneratedField::SuccessRateStdevFactor => {
                            if success_rate_stdev_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("successRateStdevFactor"));
                            }
                            success_rate_stdev_factor__ = map_.next_value()?;
                        }
                        GeneratedField::ConsecutiveGatewayFailure => {
                            if consecutive_gateway_failure__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consecutiveGatewayFailure"));
                            }
                            consecutive_gateway_failure__ = map_.next_value()?;
                        }
                        GeneratedField::EnforcingConsecutiveGatewayFailure => {
                            if enforcing_consecutive_gateway_failure__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enforcingConsecutiveGatewayFailure"));
                            }
                            enforcing_consecutive_gateway_failure__ = map_.next_value()?;
                        }
                        GeneratedField::SplitExternalLocalOriginErrors => {
                            if split_external_local_origin_errors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("splitExternalLocalOriginErrors"));
                            }
                            split_external_local_origin_errors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConsecutiveLocalOriginFailure => {
                            if consecutive_local_origin_failure__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consecutiveLocalOriginFailure"));
                            }
                            consecutive_local_origin_failure__ = map_.next_value()?;
                        }
                        GeneratedField::EnforcingConsecutiveLocalOriginFailure => {
                            if enforcing_consecutive_local_origin_failure__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enforcingConsecutiveLocalOriginFailure"));
                            }
                            enforcing_consecutive_local_origin_failure__ = map_.next_value()?;
                        }
                        GeneratedField::EnforcingLocalOriginSuccessRate => {
                            if enforcing_local_origin_success_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enforcingLocalOriginSuccessRate"));
                            }
                            enforcing_local_origin_success_rate__ = map_.next_value()?;
                        }
                        GeneratedField::FailurePercentageThreshold => {
                            if failure_percentage_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failurePercentageThreshold"));
                            }
                            failure_percentage_threshold__ = map_.next_value()?;
                        }
                        GeneratedField::EnforcingFailurePercentage => {
                            if enforcing_failure_percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enforcingFailurePercentage"));
                            }
                            enforcing_failure_percentage__ = map_.next_value()?;
                        }
                        GeneratedField::EnforcingFailurePercentageLocalOrigin => {
                            if enforcing_failure_percentage_local_origin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enforcingFailurePercentageLocalOrigin"));
                            }
                            enforcing_failure_percentage_local_origin__ = map_.next_value()?;
                        }
                        GeneratedField::FailurePercentageMinimumHosts => {
                            if failure_percentage_minimum_hosts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failurePercentageMinimumHosts"));
                            }
                            failure_percentage_minimum_hosts__ = map_.next_value()?;
                        }
                        GeneratedField::FailurePercentageRequestVolume => {
                            if failure_percentage_request_volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failurePercentageRequestVolume"));
                            }
                            failure_percentage_request_volume__ = map_.next_value()?;
                        }
                        GeneratedField::MaxEjectionTime => {
                            if max_ejection_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxEjectionTime"));
                            }
                            max_ejection_time__ = map_.next_value()?;
                        }
                        GeneratedField::MaxEjectionTimeJitter => {
                            if max_ejection_time_jitter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxEjectionTimeJitter"));
                            }
                            max_ejection_time_jitter__ = map_.next_value()?;
                        }
                        GeneratedField::SuccessfulActiveHealthCheckUnejectHost => {
                            if successful_active_health_check_uneject_host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("successfulActiveHealthCheckUnejectHost"));
                            }
                            successful_active_health_check_uneject_host__ = map_.next_value()?;
                        }
                        GeneratedField::Monitors => {
                            if monitors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("monitors"));
                            }
                            monitors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AlwaysEjectOneHost => {
                            if always_eject_one_host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alwaysEjectOneHost"));
                            }
                            always_eject_one_host__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OutlierDetection {
                    consecutive_5xx: consecutive_5xx__,
                    interval: interval__,
                    base_ejection_time: base_ejection_time__,
                    max_ejection_percent: max_ejection_percent__,
                    enforcing_consecutive_5xx: enforcing_consecutive_5xx__,
                    enforcing_success_rate: enforcing_success_rate__,
                    success_rate_minimum_hosts: success_rate_minimum_hosts__,
                    success_rate_request_volume: success_rate_request_volume__,
                    success_rate_stdev_factor: success_rate_stdev_factor__,
                    consecutive_gateway_failure: consecutive_gateway_failure__,
                    enforcing_consecutive_gateway_failure: enforcing_consecutive_gateway_failure__,
                    split_external_local_origin_errors: split_external_local_origin_errors__.unwrap_or_default(),
                    consecutive_local_origin_failure: consecutive_local_origin_failure__,
                    enforcing_consecutive_local_origin_failure: enforcing_consecutive_local_origin_failure__,
                    enforcing_local_origin_success_rate: enforcing_local_origin_success_rate__,
                    failure_percentage_threshold: failure_percentage_threshold__,
                    enforcing_failure_percentage: enforcing_failure_percentage__,
                    enforcing_failure_percentage_local_origin: enforcing_failure_percentage_local_origin__,
                    failure_percentage_minimum_hosts: failure_percentage_minimum_hosts__,
                    failure_percentage_request_volume: failure_percentage_request_volume__,
                    max_ejection_time: max_ejection_time__,
                    max_ejection_time_jitter: max_ejection_time_jitter__,
                    successful_active_health_check_uneject_host: successful_active_health_check_uneject_host__,
                    monitors: monitors__.unwrap_or_default(),
                    always_eject_one_host: always_eject_one_host__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.OutlierDetection", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TrackClusterStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timeout_budgets {
            len += 1;
        }
        if self.request_response_sizes {
            len += 1;
        }
        if self.per_endpoint_stats {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.TrackClusterStats", len)?;
        if self.timeout_budgets {
            struct_ser.serialize_field("timeout_budgets", &self.timeout_budgets)?;
        }
        if self.request_response_sizes {
            struct_ser.serialize_field("request_response_sizes", &self.request_response_sizes)?;
        }
        if self.per_endpoint_stats {
            struct_ser.serialize_field("per_endpoint_stats", &self.per_endpoint_stats)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TrackClusterStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timeout_budgets",
            "timeoutBudgets",
            "request_response_sizes",
            "requestResponseSizes",
            "per_endpoint_stats",
            "perEndpointStats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TimeoutBudgets,
            RequestResponseSizes,
            PerEndpointStats,
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
                            "timeoutBudgets" | "timeout_budgets" => Ok(GeneratedField::TimeoutBudgets),
                            "requestResponseSizes" | "request_response_sizes" => Ok(GeneratedField::RequestResponseSizes),
                            "perEndpointStats" | "per_endpoint_stats" => Ok(GeneratedField::PerEndpointStats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TrackClusterStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.TrackClusterStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TrackClusterStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timeout_budgets__ = None;
                let mut request_response_sizes__ = None;
                let mut per_endpoint_stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TimeoutBudgets => {
                            if timeout_budgets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutBudgets"));
                            }
                            timeout_budgets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestResponseSizes => {
                            if request_response_sizes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestResponseSizes"));
                            }
                            request_response_sizes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PerEndpointStats => {
                            if per_endpoint_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perEndpointStats"));
                            }
                            per_endpoint_stats__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TrackClusterStats {
                    timeout_budgets: timeout_budgets__.unwrap_or_default(),
                    request_response_sizes: request_response_sizes__.unwrap_or_default(),
                    per_endpoint_stats: per_endpoint_stats__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.TrackClusterStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpstreamConnectionOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tcp_keepalive.is_some() {
            len += 1;
        }
        if self.set_local_interface_name_on_upstream_connections {
            len += 1;
        }
        if self.happy_eyeballs_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.UpstreamConnectionOptions", len)?;
        if let Some(v) = self.tcp_keepalive.as_ref() {
            struct_ser.serialize_field("tcp_keepalive", v)?;
        }
        if self.set_local_interface_name_on_upstream_connections {
            struct_ser.serialize_field("set_local_interface_name_on_upstream_connections", &self.set_local_interface_name_on_upstream_connections)?;
        }
        if let Some(v) = self.happy_eyeballs_config.as_ref() {
            struct_ser.serialize_field("happy_eyeballs_config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpstreamConnectionOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tcp_keepalive",
            "tcpKeepalive",
            "set_local_interface_name_on_upstream_connections",
            "setLocalInterfaceNameOnUpstreamConnections",
            "happy_eyeballs_config",
            "happyEyeballsConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TcpKeepalive,
            SetLocalInterfaceNameOnUpstreamConnections,
            HappyEyeballsConfig,
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
                            "tcpKeepalive" | "tcp_keepalive" => Ok(GeneratedField::TcpKeepalive),
                            "setLocalInterfaceNameOnUpstreamConnections" | "set_local_interface_name_on_upstream_connections" => Ok(GeneratedField::SetLocalInterfaceNameOnUpstreamConnections),
                            "happyEyeballsConfig" | "happy_eyeballs_config" => Ok(GeneratedField::HappyEyeballsConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpstreamConnectionOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.UpstreamConnectionOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpstreamConnectionOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tcp_keepalive__ = None;
                let mut set_local_interface_name_on_upstream_connections__ = None;
                let mut happy_eyeballs_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TcpKeepalive => {
                            if tcp_keepalive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tcpKeepalive"));
                            }
                            tcp_keepalive__ = map_.next_value()?;
                        }
                        GeneratedField::SetLocalInterfaceNameOnUpstreamConnections => {
                            if set_local_interface_name_on_upstream_connections__.is_some() {
                                return Err(serde::de::Error::duplicate_field("setLocalInterfaceNameOnUpstreamConnections"));
                            }
                            set_local_interface_name_on_upstream_connections__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HappyEyeballsConfig => {
                            if happy_eyeballs_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("happyEyeballsConfig"));
                            }
                            happy_eyeballs_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpstreamConnectionOptions {
                    tcp_keepalive: tcp_keepalive__,
                    set_local_interface_name_on_upstream_connections: set_local_interface_name_on_upstream_connections__.unwrap_or_default(),
                    happy_eyeballs_config: happy_eyeballs_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.UpstreamConnectionOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for upstream_connection_options::FirstAddressFamilyVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Default => "DEFAULT",
            Self::V4 => "V4",
            Self::V6 => "V6",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for upstream_connection_options::FirstAddressFamilyVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEFAULT",
            "V4",
            "V6",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = upstream_connection_options::FirstAddressFamilyVersion;

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
                    "DEFAULT" => Ok(upstream_connection_options::FirstAddressFamilyVersion::Default),
                    "V4" => Ok(upstream_connection_options::FirstAddressFamilyVersion::V4),
                    "V6" => Ok(upstream_connection_options::FirstAddressFamilyVersion::V6),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for upstream_connection_options::HappyEyeballsConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.first_address_family_version != 0 {
            len += 1;
        }
        if self.first_address_family_count.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.cluster.v3.UpstreamConnectionOptions.HappyEyeballsConfig", len)?;
        if self.first_address_family_version != 0 {
            let v = upstream_connection_options::FirstAddressFamilyVersion::try_from(self.first_address_family_version)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.first_address_family_version)))?;
            struct_ser.serialize_field("first_address_family_version", &v)?;
        }
        if let Some(v) = self.first_address_family_count.as_ref() {
            struct_ser.serialize_field("first_address_family_count", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for upstream_connection_options::HappyEyeballsConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "first_address_family_version",
            "firstAddressFamilyVersion",
            "first_address_family_count",
            "firstAddressFamilyCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FirstAddressFamilyVersion,
            FirstAddressFamilyCount,
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
                            "firstAddressFamilyVersion" | "first_address_family_version" => Ok(GeneratedField::FirstAddressFamilyVersion),
                            "firstAddressFamilyCount" | "first_address_family_count" => Ok(GeneratedField::FirstAddressFamilyCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = upstream_connection_options::HappyEyeballsConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.cluster.v3.UpstreamConnectionOptions.HappyEyeballsConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<upstream_connection_options::HappyEyeballsConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut first_address_family_version__ = None;
                let mut first_address_family_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FirstAddressFamilyVersion => {
                            if first_address_family_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstAddressFamilyVersion"));
                            }
                            first_address_family_version__ = Some(map_.next_value::<upstream_connection_options::FirstAddressFamilyVersion>()? as i32);
                        }
                        GeneratedField::FirstAddressFamilyCount => {
                            if first_address_family_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstAddressFamilyCount"));
                            }
                            first_address_family_count__ = map_.next_value()?;
                        }
                    }
                }
                Ok(upstream_connection_options::HappyEyeballsConfig {
                    first_address_family_version: first_address_family_version__.unwrap_or_default(),
                    first_address_family_count: first_address_family_count__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.cluster.v3.UpstreamConnectionOptions.HappyEyeballsConfig", FIELDS, GeneratedVisitor)
    }
}
