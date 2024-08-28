impl serde::Serialize for Capability {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.health_check_protocols.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.health.v3.Capability", len)?;
        if !self.health_check_protocols.is_empty() {
            let v = self.health_check_protocols.iter().cloned().map(|v| {
                capability::Protocol::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("health_check_protocols", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Capability {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "health_check_protocols",
            "healthCheckProtocols",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HealthCheckProtocols,
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
                            "healthCheckProtocols" | "health_check_protocols" => Ok(GeneratedField::HealthCheckProtocols),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Capability;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.health.v3.Capability")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Capability, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut health_check_protocols__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HealthCheckProtocols => {
                            if health_check_protocols__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthCheckProtocols"));
                            }
                            health_check_protocols__ = Some(map_.next_value::<Vec<capability::Protocol>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(Capability {
                    health_check_protocols: health_check_protocols__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.health.v3.Capability", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for capability::Protocol {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Http => "HTTP",
            Self::Tcp => "TCP",
            Self::Redis => "REDIS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for capability::Protocol {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "HTTP",
            "TCP",
            "REDIS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = capability::Protocol;

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
                    "HTTP" => Ok(capability::Protocol::Http),
                    "TCP" => Ok(capability::Protocol::Tcp),
                    "REDIS" => Ok(capability::Protocol::Redis),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ClusterEndpointsHealth {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster_name.is_empty() {
            len += 1;
        }
        if !self.locality_endpoints_health.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.health.v3.ClusterEndpointsHealth", len)?;
        if !self.cluster_name.is_empty() {
            struct_ser.serialize_field("cluster_name", &self.cluster_name)?;
        }
        if !self.locality_endpoints_health.is_empty() {
            struct_ser.serialize_field("locality_endpoints_health", &self.locality_endpoints_health)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterEndpointsHealth {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_name",
            "clusterName",
            "locality_endpoints_health",
            "localityEndpointsHealth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterName,
            LocalityEndpointsHealth,
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
                            "clusterName" | "cluster_name" => Ok(GeneratedField::ClusterName),
                            "localityEndpointsHealth" | "locality_endpoints_health" => Ok(GeneratedField::LocalityEndpointsHealth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterEndpointsHealth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.health.v3.ClusterEndpointsHealth")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClusterEndpointsHealth, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_name__ = None;
                let mut locality_endpoints_health__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClusterName => {
                            if cluster_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterName"));
                            }
                            cluster_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LocalityEndpointsHealth => {
                            if locality_endpoints_health__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localityEndpointsHealth"));
                            }
                            locality_endpoints_health__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ClusterEndpointsHealth {
                    cluster_name: cluster_name__.unwrap_or_default(),
                    locality_endpoints_health: locality_endpoints_health__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.health.v3.ClusterEndpointsHealth", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClusterHealthCheck {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster_name.is_empty() {
            len += 1;
        }
        if !self.health_checks.is_empty() {
            len += 1;
        }
        if !self.locality_endpoints.is_empty() {
            len += 1;
        }
        if !self.transport_socket_matches.is_empty() {
            len += 1;
        }
        if self.upstream_bind_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.health.v3.ClusterHealthCheck", len)?;
        if !self.cluster_name.is_empty() {
            struct_ser.serialize_field("cluster_name", &self.cluster_name)?;
        }
        if !self.health_checks.is_empty() {
            struct_ser.serialize_field("health_checks", &self.health_checks)?;
        }
        if !self.locality_endpoints.is_empty() {
            struct_ser.serialize_field("locality_endpoints", &self.locality_endpoints)?;
        }
        if !self.transport_socket_matches.is_empty() {
            struct_ser.serialize_field("transport_socket_matches", &self.transport_socket_matches)?;
        }
        if let Some(v) = self.upstream_bind_config.as_ref() {
            struct_ser.serialize_field("upstream_bind_config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterHealthCheck {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_name",
            "clusterName",
            "health_checks",
            "healthChecks",
            "locality_endpoints",
            "localityEndpoints",
            "transport_socket_matches",
            "transportSocketMatches",
            "upstream_bind_config",
            "upstreamBindConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterName,
            HealthChecks,
            LocalityEndpoints,
            TransportSocketMatches,
            UpstreamBindConfig,
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
                            "clusterName" | "cluster_name" => Ok(GeneratedField::ClusterName),
                            "healthChecks" | "health_checks" => Ok(GeneratedField::HealthChecks),
                            "localityEndpoints" | "locality_endpoints" => Ok(GeneratedField::LocalityEndpoints),
                            "transportSocketMatches" | "transport_socket_matches" => Ok(GeneratedField::TransportSocketMatches),
                            "upstreamBindConfig" | "upstream_bind_config" => Ok(GeneratedField::UpstreamBindConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterHealthCheck;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.health.v3.ClusterHealthCheck")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClusterHealthCheck, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_name__ = None;
                let mut health_checks__ = None;
                let mut locality_endpoints__ = None;
                let mut transport_socket_matches__ = None;
                let mut upstream_bind_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClusterName => {
                            if cluster_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterName"));
                            }
                            cluster_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HealthChecks => {
                            if health_checks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthChecks"));
                            }
                            health_checks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LocalityEndpoints => {
                            if locality_endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localityEndpoints"));
                            }
                            locality_endpoints__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TransportSocketMatches => {
                            if transport_socket_matches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportSocketMatches"));
                            }
                            transport_socket_matches__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpstreamBindConfig => {
                            if upstream_bind_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamBindConfig"));
                            }
                            upstream_bind_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ClusterHealthCheck {
                    cluster_name: cluster_name__.unwrap_or_default(),
                    health_checks: health_checks__.unwrap_or_default(),
                    locality_endpoints: locality_endpoints__.unwrap_or_default(),
                    transport_socket_matches: transport_socket_matches__.unwrap_or_default(),
                    upstream_bind_config: upstream_bind_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.health.v3.ClusterHealthCheck", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EndpointHealth {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.endpoint.is_some() {
            len += 1;
        }
        if self.health_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.health.v3.EndpointHealth", len)?;
        if let Some(v) = self.endpoint.as_ref() {
            struct_ser.serialize_field("endpoint", v)?;
        }
        if self.health_status != 0 {
            let v = super::super::super::config::core::v3::HealthStatus::try_from(self.health_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.health_status)))?;
            struct_ser.serialize_field("health_status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EndpointHealth {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "endpoint",
            "health_status",
            "healthStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Endpoint,
            HealthStatus,
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
                            "endpoint" => Ok(GeneratedField::Endpoint),
                            "healthStatus" | "health_status" => Ok(GeneratedField::HealthStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EndpointHealth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.health.v3.EndpointHealth")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EndpointHealth, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut endpoint__ = None;
                let mut health_status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Endpoint => {
                            if endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            endpoint__ = map_.next_value()?;
                        }
                        GeneratedField::HealthStatus => {
                            if health_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthStatus"));
                            }
                            health_status__ = Some(map_.next_value::<super::super::super::config::core::v3::HealthStatus>()? as i32);
                        }
                    }
                }
                Ok(EndpointHealth {
                    endpoint: endpoint__,
                    health_status: health_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.health.v3.EndpointHealth", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EndpointHealthResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.endpoints_health.is_empty() {
            len += 1;
        }
        if !self.cluster_endpoints_health.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.health.v3.EndpointHealthResponse", len)?;
        if !self.endpoints_health.is_empty() {
            struct_ser.serialize_field("endpoints_health", &self.endpoints_health)?;
        }
        if !self.cluster_endpoints_health.is_empty() {
            struct_ser.serialize_field("cluster_endpoints_health", &self.cluster_endpoints_health)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EndpointHealthResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "endpoints_health",
            "endpointsHealth",
            "cluster_endpoints_health",
            "clusterEndpointsHealth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EndpointsHealth,
            ClusterEndpointsHealth,
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
                            "endpointsHealth" | "endpoints_health" => Ok(GeneratedField::EndpointsHealth),
                            "clusterEndpointsHealth" | "cluster_endpoints_health" => Ok(GeneratedField::ClusterEndpointsHealth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EndpointHealthResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.health.v3.EndpointHealthResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EndpointHealthResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut endpoints_health__ = None;
                let mut cluster_endpoints_health__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EndpointsHealth => {
                            if endpoints_health__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointsHealth"));
                            }
                            endpoints_health__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClusterEndpointsHealth => {
                            if cluster_endpoints_health__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterEndpointsHealth"));
                            }
                            cluster_endpoints_health__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EndpointHealthResponse {
                    endpoints_health: endpoints_health__.unwrap_or_default(),
                    cluster_endpoints_health: cluster_endpoints_health__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.health.v3.EndpointHealthResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HdsDummy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.service.health.v3.HdsDummy", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HdsDummy {
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
            type Value = HdsDummy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.health.v3.HdsDummy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HdsDummy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HdsDummy {
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.health.v3.HdsDummy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheckRequest {
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
        if self.capability.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.health.v3.HealthCheckRequest", len)?;
        if let Some(v) = self.node.as_ref() {
            struct_ser.serialize_field("node", v)?;
        }
        if let Some(v) = self.capability.as_ref() {
            struct_ser.serialize_field("capability", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "node",
            "capability",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Node,
            Capability,
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
                            "capability" => Ok(GeneratedField::Capability),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthCheckRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.health.v3.HealthCheckRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HealthCheckRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut node__ = None;
                let mut capability__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = map_.next_value()?;
                        }
                        GeneratedField::Capability => {
                            if capability__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capability"));
                            }
                            capability__ = map_.next_value()?;
                        }
                    }
                }
                Ok(HealthCheckRequest {
                    node: node__,
                    capability: capability__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.health.v3.HealthCheckRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheckRequestOrEndpointHealthResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.health.v3.HealthCheckRequestOrEndpointHealthResponse", len)?;
        if let Some(v) = self.request_type.as_ref() {
            match v {
                health_check_request_or_endpoint_health_response::RequestType::HealthCheckRequest(v) => {
                    struct_ser.serialize_field("health_check_request", v)?;
                }
                health_check_request_or_endpoint_health_response::RequestType::EndpointHealthResponse(v) => {
                    struct_ser.serialize_field("endpoint_health_response", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckRequestOrEndpointHealthResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "health_check_request",
            "healthCheckRequest",
            "endpoint_health_response",
            "endpointHealthResponse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HealthCheckRequest,
            EndpointHealthResponse,
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
                            "healthCheckRequest" | "health_check_request" => Ok(GeneratedField::HealthCheckRequest),
                            "endpointHealthResponse" | "endpoint_health_response" => Ok(GeneratedField::EndpointHealthResponse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthCheckRequestOrEndpointHealthResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.health.v3.HealthCheckRequestOrEndpointHealthResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HealthCheckRequestOrEndpointHealthResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HealthCheckRequest => {
                            if request_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthCheckRequest"));
                            }
                            request_type__ = map_.next_value::<::std::option::Option<_>>()?.map(health_check_request_or_endpoint_health_response::RequestType::HealthCheckRequest)
;
                        }
                        GeneratedField::EndpointHealthResponse => {
                            if request_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointHealthResponse"));
                            }
                            request_type__ = map_.next_value::<::std::option::Option<_>>()?.map(health_check_request_or_endpoint_health_response::RequestType::EndpointHealthResponse)
;
                        }
                    }
                }
                Ok(HealthCheckRequestOrEndpointHealthResponse {
                    request_type: request_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.health.v3.HealthCheckRequestOrEndpointHealthResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheckSpecifier {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster_health_checks.is_empty() {
            len += 1;
        }
        if self.interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.health.v3.HealthCheckSpecifier", len)?;
        if !self.cluster_health_checks.is_empty() {
            struct_ser.serialize_field("cluster_health_checks", &self.cluster_health_checks)?;
        }
        if let Some(v) = self.interval.as_ref() {
            struct_ser.serialize_field("interval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckSpecifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_health_checks",
            "clusterHealthChecks",
            "interval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterHealthChecks,
            Interval,
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
                            "clusterHealthChecks" | "cluster_health_checks" => Ok(GeneratedField::ClusterHealthChecks),
                            "interval" => Ok(GeneratedField::Interval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthCheckSpecifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.health.v3.HealthCheckSpecifier")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HealthCheckSpecifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_health_checks__ = None;
                let mut interval__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClusterHealthChecks => {
                            if cluster_health_checks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterHealthChecks"));
                            }
                            cluster_health_checks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = map_.next_value()?;
                        }
                    }
                }
                Ok(HealthCheckSpecifier {
                    cluster_health_checks: cluster_health_checks__.unwrap_or_default(),
                    interval: interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.health.v3.HealthCheckSpecifier", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LocalityEndpoints {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.locality.is_some() {
            len += 1;
        }
        if !self.endpoints.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.health.v3.LocalityEndpoints", len)?;
        if let Some(v) = self.locality.as_ref() {
            struct_ser.serialize_field("locality", v)?;
        }
        if !self.endpoints.is_empty() {
            struct_ser.serialize_field("endpoints", &self.endpoints)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocalityEndpoints {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "locality",
            "endpoints",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Locality,
            Endpoints,
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
                            "locality" => Ok(GeneratedField::Locality),
                            "endpoints" => Ok(GeneratedField::Endpoints),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LocalityEndpoints;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.health.v3.LocalityEndpoints")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LocalityEndpoints, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut locality__ = None;
                let mut endpoints__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Locality => {
                            if locality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locality"));
                            }
                            locality__ = map_.next_value()?;
                        }
                        GeneratedField::Endpoints => {
                            if endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoints"));
                            }
                            endpoints__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LocalityEndpoints {
                    locality: locality__,
                    endpoints: endpoints__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.health.v3.LocalityEndpoints", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LocalityEndpointsHealth {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.locality.is_some() {
            len += 1;
        }
        if !self.endpoints_health.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.health.v3.LocalityEndpointsHealth", len)?;
        if let Some(v) = self.locality.as_ref() {
            struct_ser.serialize_field("locality", v)?;
        }
        if !self.endpoints_health.is_empty() {
            struct_ser.serialize_field("endpoints_health", &self.endpoints_health)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocalityEndpointsHealth {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "locality",
            "endpoints_health",
            "endpointsHealth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Locality,
            EndpointsHealth,
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
                            "locality" => Ok(GeneratedField::Locality),
                            "endpointsHealth" | "endpoints_health" => Ok(GeneratedField::EndpointsHealth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LocalityEndpointsHealth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.health.v3.LocalityEndpointsHealth")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LocalityEndpointsHealth, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut locality__ = None;
                let mut endpoints_health__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Locality => {
                            if locality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locality"));
                            }
                            locality__ = map_.next_value()?;
                        }
                        GeneratedField::EndpointsHealth => {
                            if endpoints_health__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointsHealth"));
                            }
                            endpoints_health__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LocalityEndpointsHealth {
                    locality: locality__,
                    endpoints_health: endpoints_health__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.health.v3.LocalityEndpointsHealth", FIELDS, GeneratedVisitor)
    }
}
