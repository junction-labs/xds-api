impl serde::Serialize for ClusterLoadAssignment {
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
        if !self.endpoints.is_empty() {
            len += 1;
        }
        if !self.named_endpoints.is_empty() {
            len += 1;
        }
        if self.policy.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.endpoint.v3.ClusterLoadAssignment", len)?;
        if !self.cluster_name.is_empty() {
            struct_ser.serialize_field("cluster_name", &self.cluster_name)?;
        }
        if !self.endpoints.is_empty() {
            struct_ser.serialize_field("endpoints", &self.endpoints)?;
        }
        if !self.named_endpoints.is_empty() {
            struct_ser.serialize_field("named_endpoints", &self.named_endpoints)?;
        }
        if let Some(v) = self.policy.as_ref() {
            struct_ser.serialize_field("policy", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterLoadAssignment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_name",
            "clusterName",
            "endpoints",
            "named_endpoints",
            "namedEndpoints",
            "policy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterName,
            Endpoints,
            NamedEndpoints,
            Policy,
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
                            "endpoints" => Ok(GeneratedField::Endpoints),
                            "namedEndpoints" | "named_endpoints" => Ok(GeneratedField::NamedEndpoints),
                            "policy" => Ok(GeneratedField::Policy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterLoadAssignment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.endpoint.v3.ClusterLoadAssignment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClusterLoadAssignment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_name__ = None;
                let mut endpoints__ = None;
                let mut named_endpoints__ = None;
                let mut policy__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClusterName => {
                            if cluster_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterName"));
                            }
                            cluster_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Endpoints => {
                            if endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoints"));
                            }
                            endpoints__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NamedEndpoints => {
                            if named_endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namedEndpoints"));
                            }
                            named_endpoints__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Policy => {
                            if policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policy"));
                            }
                            policy__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ClusterLoadAssignment {
                    cluster_name: cluster_name__.unwrap_or_default(),
                    endpoints: endpoints__.unwrap_or_default(),
                    named_endpoints: named_endpoints__.unwrap_or_default(),
                    policy: policy__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.endpoint.v3.ClusterLoadAssignment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster_load_assignment::Policy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.drop_overloads.is_empty() {
            len += 1;
        }
        if self.overprovisioning_factor.is_some() {
            len += 1;
        }
        if self.endpoint_stale_after.is_some() {
            len += 1;
        }
        if self.weighted_priority_health {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.endpoint.v3.ClusterLoadAssignment.Policy", len)?;
        if !self.drop_overloads.is_empty() {
            struct_ser.serialize_field("drop_overloads", &self.drop_overloads)?;
        }
        if let Some(v) = self.overprovisioning_factor.as_ref() {
            struct_ser.serialize_field("overprovisioning_factor", v)?;
        }
        if let Some(v) = self.endpoint_stale_after.as_ref() {
            struct_ser.serialize_field("endpoint_stale_after", v)?;
        }
        if self.weighted_priority_health {
            struct_ser.serialize_field("weighted_priority_health", &self.weighted_priority_health)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster_load_assignment::Policy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "drop_overloads",
            "dropOverloads",
            "overprovisioning_factor",
            "overprovisioningFactor",
            "endpoint_stale_after",
            "endpointStaleAfter",
            "weighted_priority_health",
            "weightedPriorityHealth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DropOverloads,
            OverprovisioningFactor,
            EndpointStaleAfter,
            WeightedPriorityHealth,
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
                            "dropOverloads" | "drop_overloads" => Ok(GeneratedField::DropOverloads),
                            "overprovisioningFactor" | "overprovisioning_factor" => Ok(GeneratedField::OverprovisioningFactor),
                            "endpointStaleAfter" | "endpoint_stale_after" => Ok(GeneratedField::EndpointStaleAfter),
                            "weightedPriorityHealth" | "weighted_priority_health" => Ok(GeneratedField::WeightedPriorityHealth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster_load_assignment::Policy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.endpoint.v3.ClusterLoadAssignment.Policy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster_load_assignment::Policy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut drop_overloads__ = None;
                let mut overprovisioning_factor__ = None;
                let mut endpoint_stale_after__ = None;
                let mut weighted_priority_health__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DropOverloads => {
                            if drop_overloads__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dropOverloads"));
                            }
                            drop_overloads__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OverprovisioningFactor => {
                            if overprovisioning_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overprovisioningFactor"));
                            }
                            overprovisioning_factor__ = map_.next_value()?;
                        }
                        GeneratedField::EndpointStaleAfter => {
                            if endpoint_stale_after__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointStaleAfter"));
                            }
                            endpoint_stale_after__ = map_.next_value()?;
                        }
                        GeneratedField::WeightedPriorityHealth => {
                            if weighted_priority_health__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weightedPriorityHealth"));
                            }
                            weighted_priority_health__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(cluster_load_assignment::Policy {
                    drop_overloads: drop_overloads__.unwrap_or_default(),
                    overprovisioning_factor: overprovisioning_factor__,
                    endpoint_stale_after: endpoint_stale_after__,
                    weighted_priority_health: weighted_priority_health__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.endpoint.v3.ClusterLoadAssignment.Policy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster_load_assignment::policy::DropOverload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.category.is_empty() {
            len += 1;
        }
        if self.drop_percentage.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.endpoint.v3.ClusterLoadAssignment.Policy.DropOverload", len)?;
        if !self.category.is_empty() {
            struct_ser.serialize_field("category", &self.category)?;
        }
        if let Some(v) = self.drop_percentage.as_ref() {
            struct_ser.serialize_field("drop_percentage", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster_load_assignment::policy::DropOverload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "category",
            "drop_percentage",
            "dropPercentage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Category,
            DropPercentage,
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
                            "category" => Ok(GeneratedField::Category),
                            "dropPercentage" | "drop_percentage" => Ok(GeneratedField::DropPercentage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster_load_assignment::policy::DropOverload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.endpoint.v3.ClusterLoadAssignment.Policy.DropOverload")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster_load_assignment::policy::DropOverload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut category__ = None;
                let mut drop_percentage__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Category => {
                            if category__.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            category__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DropPercentage => {
                            if drop_percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dropPercentage"));
                            }
                            drop_percentage__ = map_.next_value()?;
                        }
                    }
                }
                Ok(cluster_load_assignment::policy::DropOverload {
                    category: category__.unwrap_or_default(),
                    drop_percentage: drop_percentage__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.endpoint.v3.ClusterLoadAssignment.Policy.DropOverload", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClusterStats {
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
        if !self.cluster_service_name.is_empty() {
            len += 1;
        }
        if !self.upstream_locality_stats.is_empty() {
            len += 1;
        }
        if self.total_dropped_requests != 0 {
            len += 1;
        }
        if !self.dropped_requests.is_empty() {
            len += 1;
        }
        if self.load_report_interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.endpoint.v3.ClusterStats", len)?;
        if !self.cluster_name.is_empty() {
            struct_ser.serialize_field("cluster_name", &self.cluster_name)?;
        }
        if !self.cluster_service_name.is_empty() {
            struct_ser.serialize_field("cluster_service_name", &self.cluster_service_name)?;
        }
        if !self.upstream_locality_stats.is_empty() {
            struct_ser.serialize_field("upstream_locality_stats", &self.upstream_locality_stats)?;
        }
        if self.total_dropped_requests != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("total_dropped_requests", ToString::to_string(&self.total_dropped_requests).as_str())?;
        }
        if !self.dropped_requests.is_empty() {
            struct_ser.serialize_field("dropped_requests", &self.dropped_requests)?;
        }
        if let Some(v) = self.load_report_interval.as_ref() {
            struct_ser.serialize_field("load_report_interval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_name",
            "clusterName",
            "cluster_service_name",
            "clusterServiceName",
            "upstream_locality_stats",
            "upstreamLocalityStats",
            "total_dropped_requests",
            "totalDroppedRequests",
            "dropped_requests",
            "droppedRequests",
            "load_report_interval",
            "loadReportInterval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterName,
            ClusterServiceName,
            UpstreamLocalityStats,
            TotalDroppedRequests,
            DroppedRequests,
            LoadReportInterval,
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
                            "clusterServiceName" | "cluster_service_name" => Ok(GeneratedField::ClusterServiceName),
                            "upstreamLocalityStats" | "upstream_locality_stats" => Ok(GeneratedField::UpstreamLocalityStats),
                            "totalDroppedRequests" | "total_dropped_requests" => Ok(GeneratedField::TotalDroppedRequests),
                            "droppedRequests" | "dropped_requests" => Ok(GeneratedField::DroppedRequests),
                            "loadReportInterval" | "load_report_interval" => Ok(GeneratedField::LoadReportInterval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.endpoint.v3.ClusterStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClusterStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_name__ = None;
                let mut cluster_service_name__ = None;
                let mut upstream_locality_stats__ = None;
                let mut total_dropped_requests__ = None;
                let mut dropped_requests__ = None;
                let mut load_report_interval__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClusterName => {
                            if cluster_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterName"));
                            }
                            cluster_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClusterServiceName => {
                            if cluster_service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterServiceName"));
                            }
                            cluster_service_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpstreamLocalityStats => {
                            if upstream_locality_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamLocalityStats"));
                            }
                            upstream_locality_stats__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalDroppedRequests => {
                            if total_dropped_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalDroppedRequests"));
                            }
                            total_dropped_requests__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DroppedRequests => {
                            if dropped_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("droppedRequests"));
                            }
                            dropped_requests__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoadReportInterval => {
                            if load_report_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadReportInterval"));
                            }
                            load_report_interval__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ClusterStats {
                    cluster_name: cluster_name__.unwrap_or_default(),
                    cluster_service_name: cluster_service_name__.unwrap_or_default(),
                    upstream_locality_stats: upstream_locality_stats__.unwrap_or_default(),
                    total_dropped_requests: total_dropped_requests__.unwrap_or_default(),
                    dropped_requests: dropped_requests__.unwrap_or_default(),
                    load_report_interval: load_report_interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.endpoint.v3.ClusterStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster_stats::DroppedRequests {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.category.is_empty() {
            len += 1;
        }
        if self.dropped_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.endpoint.v3.ClusterStats.DroppedRequests", len)?;
        if !self.category.is_empty() {
            struct_ser.serialize_field("category", &self.category)?;
        }
        if self.dropped_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("dropped_count", ToString::to_string(&self.dropped_count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster_stats::DroppedRequests {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "category",
            "dropped_count",
            "droppedCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Category,
            DroppedCount,
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
                            "category" => Ok(GeneratedField::Category),
                            "droppedCount" | "dropped_count" => Ok(GeneratedField::DroppedCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster_stats::DroppedRequests;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.endpoint.v3.ClusterStats.DroppedRequests")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<cluster_stats::DroppedRequests, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut category__ = None;
                let mut dropped_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Category => {
                            if category__.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            category__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DroppedCount => {
                            if dropped_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("droppedCount"));
                            }
                            dropped_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(cluster_stats::DroppedRequests {
                    category: category__.unwrap_or_default(),
                    dropped_count: dropped_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.endpoint.v3.ClusterStats.DroppedRequests", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Endpoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.address.is_some() {
            len += 1;
        }
        if self.health_check_config.is_some() {
            len += 1;
        }
        if !self.hostname.is_empty() {
            len += 1;
        }
        if !self.additional_addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.endpoint.v3.Endpoint", len)?;
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.health_check_config.as_ref() {
            struct_ser.serialize_field("health_check_config", v)?;
        }
        if !self.hostname.is_empty() {
            struct_ser.serialize_field("hostname", &self.hostname)?;
        }
        if !self.additional_addresses.is_empty() {
            struct_ser.serialize_field("additional_addresses", &self.additional_addresses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Endpoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "health_check_config",
            "healthCheckConfig",
            "hostname",
            "additional_addresses",
            "additionalAddresses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            HealthCheckConfig,
            Hostname,
            AdditionalAddresses,
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
                            "address" => Ok(GeneratedField::Address),
                            "healthCheckConfig" | "health_check_config" => Ok(GeneratedField::HealthCheckConfig),
                            "hostname" => Ok(GeneratedField::Hostname),
                            "additionalAddresses" | "additional_addresses" => Ok(GeneratedField::AdditionalAddresses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Endpoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.endpoint.v3.Endpoint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Endpoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut health_check_config__ = None;
                let mut hostname__ = None;
                let mut additional_addresses__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                        GeneratedField::HealthCheckConfig => {
                            if health_check_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthCheckConfig"));
                            }
                            health_check_config__ = map_.next_value()?;
                        }
                        GeneratedField::Hostname => {
                            if hostname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostname"));
                            }
                            hostname__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AdditionalAddresses => {
                            if additional_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalAddresses"));
                            }
                            additional_addresses__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Endpoint {
                    address: address__,
                    health_check_config: health_check_config__,
                    hostname: hostname__.unwrap_or_default(),
                    additional_addresses: additional_addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.endpoint.v3.Endpoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for endpoint::AdditionalAddress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.address.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.endpoint.v3.Endpoint.AdditionalAddress", len)?;
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for endpoint::AdditionalAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = endpoint::AdditionalAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.endpoint.v3.Endpoint.AdditionalAddress")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<endpoint::AdditionalAddress, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                    }
                }
                Ok(endpoint::AdditionalAddress {
                    address: address__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.endpoint.v3.Endpoint.AdditionalAddress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for endpoint::HealthCheckConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.port_value != 0 {
            len += 1;
        }
        if !self.hostname.is_empty() {
            len += 1;
        }
        if self.address.is_some() {
            len += 1;
        }
        if self.disable_active_health_check {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.endpoint.v3.Endpoint.HealthCheckConfig", len)?;
        if self.port_value != 0 {
            struct_ser.serialize_field("port_value", &self.port_value)?;
        }
        if !self.hostname.is_empty() {
            struct_ser.serialize_field("hostname", &self.hostname)?;
        }
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if self.disable_active_health_check {
            struct_ser.serialize_field("disable_active_health_check", &self.disable_active_health_check)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for endpoint::HealthCheckConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "port_value",
            "portValue",
            "hostname",
            "address",
            "disable_active_health_check",
            "disableActiveHealthCheck",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PortValue,
            Hostname,
            Address,
            DisableActiveHealthCheck,
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
                            "portValue" | "port_value" => Ok(GeneratedField::PortValue),
                            "hostname" => Ok(GeneratedField::Hostname),
                            "address" => Ok(GeneratedField::Address),
                            "disableActiveHealthCheck" | "disable_active_health_check" => Ok(GeneratedField::DisableActiveHealthCheck),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = endpoint::HealthCheckConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.endpoint.v3.Endpoint.HealthCheckConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<endpoint::HealthCheckConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut port_value__ = None;
                let mut hostname__ = None;
                let mut address__ = None;
                let mut disable_active_health_check__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PortValue => {
                            if port_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portValue"));
                            }
                            port_value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Hostname => {
                            if hostname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostname"));
                            }
                            hostname__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                        GeneratedField::DisableActiveHealthCheck => {
                            if disable_active_health_check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableActiveHealthCheck"));
                            }
                            disable_active_health_check__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(endpoint::HealthCheckConfig {
                    port_value: port_value__.unwrap_or_default(),
                    hostname: hostname__.unwrap_or_default(),
                    address: address__,
                    disable_active_health_check: disable_active_health_check__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.endpoint.v3.Endpoint.HealthCheckConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EndpointLoadMetricStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.metric_name.is_empty() {
            len += 1;
        }
        if self.num_requests_finished_with_metric != 0 {
            len += 1;
        }
        if self.total_metric_value != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.endpoint.v3.EndpointLoadMetricStats", len)?;
        if !self.metric_name.is_empty() {
            struct_ser.serialize_field("metric_name", &self.metric_name)?;
        }
        if self.num_requests_finished_with_metric != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("num_requests_finished_with_metric", ToString::to_string(&self.num_requests_finished_with_metric).as_str())?;
        }
        if self.total_metric_value != 0. {
            struct_ser.serialize_field("total_metric_value", &self.total_metric_value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EndpointLoadMetricStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metric_name",
            "metricName",
            "num_requests_finished_with_metric",
            "numRequestsFinishedWithMetric",
            "total_metric_value",
            "totalMetricValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MetricName,
            NumRequestsFinishedWithMetric,
            TotalMetricValue,
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
                            "metricName" | "metric_name" => Ok(GeneratedField::MetricName),
                            "numRequestsFinishedWithMetric" | "num_requests_finished_with_metric" => Ok(GeneratedField::NumRequestsFinishedWithMetric),
                            "totalMetricValue" | "total_metric_value" => Ok(GeneratedField::TotalMetricValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EndpointLoadMetricStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.endpoint.v3.EndpointLoadMetricStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EndpointLoadMetricStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metric_name__ = None;
                let mut num_requests_finished_with_metric__ = None;
                let mut total_metric_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MetricName => {
                            if metric_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metricName"));
                            }
                            metric_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NumRequestsFinishedWithMetric => {
                            if num_requests_finished_with_metric__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numRequestsFinishedWithMetric"));
                            }
                            num_requests_finished_with_metric__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalMetricValue => {
                            if total_metric_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalMetricValue"));
                            }
                            total_metric_value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(EndpointLoadMetricStats {
                    metric_name: metric_name__.unwrap_or_default(),
                    num_requests_finished_with_metric: num_requests_finished_with_metric__.unwrap_or_default(),
                    total_metric_value: total_metric_value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.endpoint.v3.EndpointLoadMetricStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LbEndpoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.health_status != 0 {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.load_balancing_weight.is_some() {
            len += 1;
        }
        if self.host_identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.endpoint.v3.LbEndpoint", len)?;
        if self.health_status != 0 {
            let v = super::super::core::v3::HealthStatus::try_from(self.health_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.health_status)))?;
            struct_ser.serialize_field("health_status", &v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.load_balancing_weight.as_ref() {
            struct_ser.serialize_field("load_balancing_weight", v)?;
        }
        if let Some(v) = self.host_identifier.as_ref() {
            match v {
                lb_endpoint::HostIdentifier::Endpoint(v) => {
                    struct_ser.serialize_field("endpoint", v)?;
                }
                lb_endpoint::HostIdentifier::EndpointName(v) => {
                    struct_ser.serialize_field("endpoint_name", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LbEndpoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "health_status",
            "healthStatus",
            "metadata",
            "load_balancing_weight",
            "loadBalancingWeight",
            "endpoint",
            "endpoint_name",
            "endpointName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HealthStatus,
            Metadata,
            LoadBalancingWeight,
            Endpoint,
            EndpointName,
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
                            "healthStatus" | "health_status" => Ok(GeneratedField::HealthStatus),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "loadBalancingWeight" | "load_balancing_weight" => Ok(GeneratedField::LoadBalancingWeight),
                            "endpoint" => Ok(GeneratedField::Endpoint),
                            "endpointName" | "endpoint_name" => Ok(GeneratedField::EndpointName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LbEndpoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.endpoint.v3.LbEndpoint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LbEndpoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut health_status__ = None;
                let mut metadata__ = None;
                let mut load_balancing_weight__ = None;
                let mut host_identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HealthStatus => {
                            if health_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthStatus"));
                            }
                            health_status__ = Some(map_.next_value::<super::super::core::v3::HealthStatus>()? as i32);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::LoadBalancingWeight => {
                            if load_balancing_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadBalancingWeight"));
                            }
                            load_balancing_weight__ = map_.next_value()?;
                        }
                        GeneratedField::Endpoint => {
                            if host_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            host_identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(lb_endpoint::HostIdentifier::Endpoint)
;
                        }
                        GeneratedField::EndpointName => {
                            if host_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointName"));
                            }
                            host_identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(lb_endpoint::HostIdentifier::EndpointName);
                        }
                    }
                }
                Ok(LbEndpoint {
                    health_status: health_status__.unwrap_or_default(),
                    metadata: metadata__,
                    load_balancing_weight: load_balancing_weight__,
                    host_identifier: host_identifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.endpoint.v3.LbEndpoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LbEndpointCollection {
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.endpoint.v3.LbEndpointCollection", len)?;
        if let Some(v) = self.entries.as_ref() {
            struct_ser.serialize_field("entries", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LbEndpointCollection {
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
            type Value = LbEndpointCollection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.endpoint.v3.LbEndpointCollection")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LbEndpointCollection, V::Error>
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
                Ok(LbEndpointCollection {
                    entries: entries__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.endpoint.v3.LbEndpointCollection", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LedsClusterLocalityConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.leds_config.is_some() {
            len += 1;
        }
        if !self.leds_collection_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.endpoint.v3.LedsClusterLocalityConfig", len)?;
        if let Some(v) = self.leds_config.as_ref() {
            struct_ser.serialize_field("leds_config", v)?;
        }
        if !self.leds_collection_name.is_empty() {
            struct_ser.serialize_field("leds_collection_name", &self.leds_collection_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LedsClusterLocalityConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "leds_config",
            "ledsConfig",
            "leds_collection_name",
            "ledsCollectionName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LedsConfig,
            LedsCollectionName,
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
                            "ledsConfig" | "leds_config" => Ok(GeneratedField::LedsConfig),
                            "ledsCollectionName" | "leds_collection_name" => Ok(GeneratedField::LedsCollectionName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LedsClusterLocalityConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.endpoint.v3.LedsClusterLocalityConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LedsClusterLocalityConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut leds_config__ = None;
                let mut leds_collection_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LedsConfig => {
                            if leds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ledsConfig"));
                            }
                            leds_config__ = map_.next_value()?;
                        }
                        GeneratedField::LedsCollectionName => {
                            if leds_collection_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ledsCollectionName"));
                            }
                            leds_collection_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LedsClusterLocalityConfig {
                    leds_config: leds_config__,
                    leds_collection_name: leds_collection_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.endpoint.v3.LedsClusterLocalityConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LocalityLbEndpoints {
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
        if self.metadata.is_some() {
            len += 1;
        }
        if !self.lb_endpoints.is_empty() {
            len += 1;
        }
        if self.load_balancing_weight.is_some() {
            len += 1;
        }
        if self.priority != 0 {
            len += 1;
        }
        if self.proximity.is_some() {
            len += 1;
        }
        if self.lb_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.endpoint.v3.LocalityLbEndpoints", len)?;
        if let Some(v) = self.locality.as_ref() {
            struct_ser.serialize_field("locality", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.lb_endpoints.is_empty() {
            struct_ser.serialize_field("lb_endpoints", &self.lb_endpoints)?;
        }
        if let Some(v) = self.load_balancing_weight.as_ref() {
            struct_ser.serialize_field("load_balancing_weight", v)?;
        }
        if self.priority != 0 {
            struct_ser.serialize_field("priority", &self.priority)?;
        }
        if let Some(v) = self.proximity.as_ref() {
            struct_ser.serialize_field("proximity", v)?;
        }
        if let Some(v) = self.lb_config.as_ref() {
            match v {
                locality_lb_endpoints::LbConfig::LoadBalancerEndpoints(v) => {
                    struct_ser.serialize_field("load_balancer_endpoints", v)?;
                }
                locality_lb_endpoints::LbConfig::LedsClusterLocalityConfig(v) => {
                    struct_ser.serialize_field("leds_cluster_locality_config", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocalityLbEndpoints {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "locality",
            "metadata",
            "lb_endpoints",
            "lbEndpoints",
            "load_balancing_weight",
            "loadBalancingWeight",
            "priority",
            "proximity",
            "load_balancer_endpoints",
            "loadBalancerEndpoints",
            "leds_cluster_locality_config",
            "ledsClusterLocalityConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Locality,
            Metadata,
            LbEndpoints,
            LoadBalancingWeight,
            Priority,
            Proximity,
            LoadBalancerEndpoints,
            LedsClusterLocalityConfig,
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
                            "metadata" => Ok(GeneratedField::Metadata),
                            "lbEndpoints" | "lb_endpoints" => Ok(GeneratedField::LbEndpoints),
                            "loadBalancingWeight" | "load_balancing_weight" => Ok(GeneratedField::LoadBalancingWeight),
                            "priority" => Ok(GeneratedField::Priority),
                            "proximity" => Ok(GeneratedField::Proximity),
                            "loadBalancerEndpoints" | "load_balancer_endpoints" => Ok(GeneratedField::LoadBalancerEndpoints),
                            "ledsClusterLocalityConfig" | "leds_cluster_locality_config" => Ok(GeneratedField::LedsClusterLocalityConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LocalityLbEndpoints;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.endpoint.v3.LocalityLbEndpoints")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LocalityLbEndpoints, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut locality__ = None;
                let mut metadata__ = None;
                let mut lb_endpoints__ = None;
                let mut load_balancing_weight__ = None;
                let mut priority__ = None;
                let mut proximity__ = None;
                let mut lb_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Locality => {
                            if locality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locality"));
                            }
                            locality__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::LbEndpoints => {
                            if lb_endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lbEndpoints"));
                            }
                            lb_endpoints__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoadBalancingWeight => {
                            if load_balancing_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadBalancingWeight"));
                            }
                            load_balancing_weight__ = map_.next_value()?;
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Proximity => {
                            if proximity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proximity"));
                            }
                            proximity__ = map_.next_value()?;
                        }
                        GeneratedField::LoadBalancerEndpoints => {
                            if lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadBalancerEndpoints"));
                            }
                            lb_config__ = map_.next_value::<::std::option::Option<_>>()?.map(locality_lb_endpoints::LbConfig::LoadBalancerEndpoints)
;
                        }
                        GeneratedField::LedsClusterLocalityConfig => {
                            if lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ledsClusterLocalityConfig"));
                            }
                            lb_config__ = map_.next_value::<::std::option::Option<_>>()?.map(locality_lb_endpoints::LbConfig::LedsClusterLocalityConfig)
;
                        }
                    }
                }
                Ok(LocalityLbEndpoints {
                    locality: locality__,
                    metadata: metadata__,
                    lb_endpoints: lb_endpoints__.unwrap_or_default(),
                    load_balancing_weight: load_balancing_weight__,
                    priority: priority__.unwrap_or_default(),
                    proximity: proximity__,
                    lb_config: lb_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.endpoint.v3.LocalityLbEndpoints", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for locality_lb_endpoints::LbEndpointList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.lb_endpoints.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.endpoint.v3.LocalityLbEndpoints.LbEndpointList", len)?;
        if !self.lb_endpoints.is_empty() {
            struct_ser.serialize_field("lb_endpoints", &self.lb_endpoints)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for locality_lb_endpoints::LbEndpointList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "lb_endpoints",
            "lbEndpoints",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LbEndpoints,
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
                            "lbEndpoints" | "lb_endpoints" => Ok(GeneratedField::LbEndpoints),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = locality_lb_endpoints::LbEndpointList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.endpoint.v3.LocalityLbEndpoints.LbEndpointList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<locality_lb_endpoints::LbEndpointList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut lb_endpoints__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LbEndpoints => {
                            if lb_endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lbEndpoints"));
                            }
                            lb_endpoints__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(locality_lb_endpoints::LbEndpointList {
                    lb_endpoints: lb_endpoints__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.endpoint.v3.LocalityLbEndpoints.LbEndpointList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnnamedEndpointLoadMetricStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.num_requests_finished_with_metric != 0 {
            len += 1;
        }
        if self.total_metric_value != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.endpoint.v3.UnnamedEndpointLoadMetricStats", len)?;
        if self.num_requests_finished_with_metric != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("num_requests_finished_with_metric", ToString::to_string(&self.num_requests_finished_with_metric).as_str())?;
        }
        if self.total_metric_value != 0. {
            struct_ser.serialize_field("total_metric_value", &self.total_metric_value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnnamedEndpointLoadMetricStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "num_requests_finished_with_metric",
            "numRequestsFinishedWithMetric",
            "total_metric_value",
            "totalMetricValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NumRequestsFinishedWithMetric,
            TotalMetricValue,
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
                            "numRequestsFinishedWithMetric" | "num_requests_finished_with_metric" => Ok(GeneratedField::NumRequestsFinishedWithMetric),
                            "totalMetricValue" | "total_metric_value" => Ok(GeneratedField::TotalMetricValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnnamedEndpointLoadMetricStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.endpoint.v3.UnnamedEndpointLoadMetricStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnnamedEndpointLoadMetricStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut num_requests_finished_with_metric__ = None;
                let mut total_metric_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NumRequestsFinishedWithMetric => {
                            if num_requests_finished_with_metric__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numRequestsFinishedWithMetric"));
                            }
                            num_requests_finished_with_metric__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalMetricValue => {
                            if total_metric_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalMetricValue"));
                            }
                            total_metric_value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(UnnamedEndpointLoadMetricStats {
                    num_requests_finished_with_metric: num_requests_finished_with_metric__.unwrap_or_default(),
                    total_metric_value: total_metric_value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.endpoint.v3.UnnamedEndpointLoadMetricStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpstreamEndpointStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.address.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.total_successful_requests != 0 {
            len += 1;
        }
        if self.total_requests_in_progress != 0 {
            len += 1;
        }
        if self.total_error_requests != 0 {
            len += 1;
        }
        if self.total_issued_requests != 0 {
            len += 1;
        }
        if !self.load_metric_stats.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.endpoint.v3.UpstreamEndpointStats", len)?;
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if self.total_successful_requests != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("total_successful_requests", ToString::to_string(&self.total_successful_requests).as_str())?;
        }
        if self.total_requests_in_progress != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("total_requests_in_progress", ToString::to_string(&self.total_requests_in_progress).as_str())?;
        }
        if self.total_error_requests != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("total_error_requests", ToString::to_string(&self.total_error_requests).as_str())?;
        }
        if self.total_issued_requests != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("total_issued_requests", ToString::to_string(&self.total_issued_requests).as_str())?;
        }
        if !self.load_metric_stats.is_empty() {
            struct_ser.serialize_field("load_metric_stats", &self.load_metric_stats)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpstreamEndpointStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "metadata",
            "total_successful_requests",
            "totalSuccessfulRequests",
            "total_requests_in_progress",
            "totalRequestsInProgress",
            "total_error_requests",
            "totalErrorRequests",
            "total_issued_requests",
            "totalIssuedRequests",
            "load_metric_stats",
            "loadMetricStats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Metadata,
            TotalSuccessfulRequests,
            TotalRequestsInProgress,
            TotalErrorRequests,
            TotalIssuedRequests,
            LoadMetricStats,
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
                            "address" => Ok(GeneratedField::Address),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "totalSuccessfulRequests" | "total_successful_requests" => Ok(GeneratedField::TotalSuccessfulRequests),
                            "totalRequestsInProgress" | "total_requests_in_progress" => Ok(GeneratedField::TotalRequestsInProgress),
                            "totalErrorRequests" | "total_error_requests" => Ok(GeneratedField::TotalErrorRequests),
                            "totalIssuedRequests" | "total_issued_requests" => Ok(GeneratedField::TotalIssuedRequests),
                            "loadMetricStats" | "load_metric_stats" => Ok(GeneratedField::LoadMetricStats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpstreamEndpointStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.endpoint.v3.UpstreamEndpointStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpstreamEndpointStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut metadata__ = None;
                let mut total_successful_requests__ = None;
                let mut total_requests_in_progress__ = None;
                let mut total_error_requests__ = None;
                let mut total_issued_requests__ = None;
                let mut load_metric_stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::TotalSuccessfulRequests => {
                            if total_successful_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalSuccessfulRequests"));
                            }
                            total_successful_requests__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalRequestsInProgress => {
                            if total_requests_in_progress__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalRequestsInProgress"));
                            }
                            total_requests_in_progress__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalErrorRequests => {
                            if total_error_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalErrorRequests"));
                            }
                            total_error_requests__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalIssuedRequests => {
                            if total_issued_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalIssuedRequests"));
                            }
                            total_issued_requests__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LoadMetricStats => {
                            if load_metric_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadMetricStats"));
                            }
                            load_metric_stats__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpstreamEndpointStats {
                    address: address__,
                    metadata: metadata__,
                    total_successful_requests: total_successful_requests__.unwrap_or_default(),
                    total_requests_in_progress: total_requests_in_progress__.unwrap_or_default(),
                    total_error_requests: total_error_requests__.unwrap_or_default(),
                    total_issued_requests: total_issued_requests__.unwrap_or_default(),
                    load_metric_stats: load_metric_stats__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.endpoint.v3.UpstreamEndpointStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpstreamLocalityStats {
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
        if self.total_successful_requests != 0 {
            len += 1;
        }
        if self.total_requests_in_progress != 0 {
            len += 1;
        }
        if self.total_error_requests != 0 {
            len += 1;
        }
        if self.total_issued_requests != 0 {
            len += 1;
        }
        if self.total_active_connections != 0 {
            len += 1;
        }
        if self.total_new_connections != 0 {
            len += 1;
        }
        if self.total_fail_connections != 0 {
            len += 1;
        }
        if self.cpu_utilization.is_some() {
            len += 1;
        }
        if self.mem_utilization.is_some() {
            len += 1;
        }
        if self.application_utilization.is_some() {
            len += 1;
        }
        if !self.load_metric_stats.is_empty() {
            len += 1;
        }
        if !self.upstream_endpoint_stats.is_empty() {
            len += 1;
        }
        if self.priority != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.endpoint.v3.UpstreamLocalityStats", len)?;
        if let Some(v) = self.locality.as_ref() {
            struct_ser.serialize_field("locality", v)?;
        }
        if self.total_successful_requests != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("total_successful_requests", ToString::to_string(&self.total_successful_requests).as_str())?;
        }
        if self.total_requests_in_progress != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("total_requests_in_progress", ToString::to_string(&self.total_requests_in_progress).as_str())?;
        }
        if self.total_error_requests != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("total_error_requests", ToString::to_string(&self.total_error_requests).as_str())?;
        }
        if self.total_issued_requests != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("total_issued_requests", ToString::to_string(&self.total_issued_requests).as_str())?;
        }
        if self.total_active_connections != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("total_active_connections", ToString::to_string(&self.total_active_connections).as_str())?;
        }
        if self.total_new_connections != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("total_new_connections", ToString::to_string(&self.total_new_connections).as_str())?;
        }
        if self.total_fail_connections != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("total_fail_connections", ToString::to_string(&self.total_fail_connections).as_str())?;
        }
        if let Some(v) = self.cpu_utilization.as_ref() {
            struct_ser.serialize_field("cpu_utilization", v)?;
        }
        if let Some(v) = self.mem_utilization.as_ref() {
            struct_ser.serialize_field("mem_utilization", v)?;
        }
        if let Some(v) = self.application_utilization.as_ref() {
            struct_ser.serialize_field("application_utilization", v)?;
        }
        if !self.load_metric_stats.is_empty() {
            struct_ser.serialize_field("load_metric_stats", &self.load_metric_stats)?;
        }
        if !self.upstream_endpoint_stats.is_empty() {
            struct_ser.serialize_field("upstream_endpoint_stats", &self.upstream_endpoint_stats)?;
        }
        if self.priority != 0 {
            struct_ser.serialize_field("priority", &self.priority)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpstreamLocalityStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "locality",
            "total_successful_requests",
            "totalSuccessfulRequests",
            "total_requests_in_progress",
            "totalRequestsInProgress",
            "total_error_requests",
            "totalErrorRequests",
            "total_issued_requests",
            "totalIssuedRequests",
            "total_active_connections",
            "totalActiveConnections",
            "total_new_connections",
            "totalNewConnections",
            "total_fail_connections",
            "totalFailConnections",
            "cpu_utilization",
            "cpuUtilization",
            "mem_utilization",
            "memUtilization",
            "application_utilization",
            "applicationUtilization",
            "load_metric_stats",
            "loadMetricStats",
            "upstream_endpoint_stats",
            "upstreamEndpointStats",
            "priority",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Locality,
            TotalSuccessfulRequests,
            TotalRequestsInProgress,
            TotalErrorRequests,
            TotalIssuedRequests,
            TotalActiveConnections,
            TotalNewConnections,
            TotalFailConnections,
            CpuUtilization,
            MemUtilization,
            ApplicationUtilization,
            LoadMetricStats,
            UpstreamEndpointStats,
            Priority,
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
                            "totalSuccessfulRequests" | "total_successful_requests" => Ok(GeneratedField::TotalSuccessfulRequests),
                            "totalRequestsInProgress" | "total_requests_in_progress" => Ok(GeneratedField::TotalRequestsInProgress),
                            "totalErrorRequests" | "total_error_requests" => Ok(GeneratedField::TotalErrorRequests),
                            "totalIssuedRequests" | "total_issued_requests" => Ok(GeneratedField::TotalIssuedRequests),
                            "totalActiveConnections" | "total_active_connections" => Ok(GeneratedField::TotalActiveConnections),
                            "totalNewConnections" | "total_new_connections" => Ok(GeneratedField::TotalNewConnections),
                            "totalFailConnections" | "total_fail_connections" => Ok(GeneratedField::TotalFailConnections),
                            "cpuUtilization" | "cpu_utilization" => Ok(GeneratedField::CpuUtilization),
                            "memUtilization" | "mem_utilization" => Ok(GeneratedField::MemUtilization),
                            "applicationUtilization" | "application_utilization" => Ok(GeneratedField::ApplicationUtilization),
                            "loadMetricStats" | "load_metric_stats" => Ok(GeneratedField::LoadMetricStats),
                            "upstreamEndpointStats" | "upstream_endpoint_stats" => Ok(GeneratedField::UpstreamEndpointStats),
                            "priority" => Ok(GeneratedField::Priority),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpstreamLocalityStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.endpoint.v3.UpstreamLocalityStats")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpstreamLocalityStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut locality__ = None;
                let mut total_successful_requests__ = None;
                let mut total_requests_in_progress__ = None;
                let mut total_error_requests__ = None;
                let mut total_issued_requests__ = None;
                let mut total_active_connections__ = None;
                let mut total_new_connections__ = None;
                let mut total_fail_connections__ = None;
                let mut cpu_utilization__ = None;
                let mut mem_utilization__ = None;
                let mut application_utilization__ = None;
                let mut load_metric_stats__ = None;
                let mut upstream_endpoint_stats__ = None;
                let mut priority__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Locality => {
                            if locality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locality"));
                            }
                            locality__ = map_.next_value()?;
                        }
                        GeneratedField::TotalSuccessfulRequests => {
                            if total_successful_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalSuccessfulRequests"));
                            }
                            total_successful_requests__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalRequestsInProgress => {
                            if total_requests_in_progress__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalRequestsInProgress"));
                            }
                            total_requests_in_progress__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalErrorRequests => {
                            if total_error_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalErrorRequests"));
                            }
                            total_error_requests__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalIssuedRequests => {
                            if total_issued_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalIssuedRequests"));
                            }
                            total_issued_requests__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalActiveConnections => {
                            if total_active_connections__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalActiveConnections"));
                            }
                            total_active_connections__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalNewConnections => {
                            if total_new_connections__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalNewConnections"));
                            }
                            total_new_connections__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalFailConnections => {
                            if total_fail_connections__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalFailConnections"));
                            }
                            total_fail_connections__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CpuUtilization => {
                            if cpu_utilization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cpuUtilization"));
                            }
                            cpu_utilization__ = map_.next_value()?;
                        }
                        GeneratedField::MemUtilization => {
                            if mem_utilization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memUtilization"));
                            }
                            mem_utilization__ = map_.next_value()?;
                        }
                        GeneratedField::ApplicationUtilization => {
                            if application_utilization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applicationUtilization"));
                            }
                            application_utilization__ = map_.next_value()?;
                        }
                        GeneratedField::LoadMetricStats => {
                            if load_metric_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadMetricStats"));
                            }
                            load_metric_stats__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpstreamEndpointStats => {
                            if upstream_endpoint_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamEndpointStats"));
                            }
                            upstream_endpoint_stats__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(UpstreamLocalityStats {
                    locality: locality__,
                    total_successful_requests: total_successful_requests__.unwrap_or_default(),
                    total_requests_in_progress: total_requests_in_progress__.unwrap_or_default(),
                    total_error_requests: total_error_requests__.unwrap_or_default(),
                    total_issued_requests: total_issued_requests__.unwrap_or_default(),
                    total_active_connections: total_active_connections__.unwrap_or_default(),
                    total_new_connections: total_new_connections__.unwrap_or_default(),
                    total_fail_connections: total_fail_connections__.unwrap_or_default(),
                    cpu_utilization: cpu_utilization__,
                    mem_utilization: mem_utilization__,
                    application_utilization: application_utilization__,
                    load_metric_stats: load_metric_stats__.unwrap_or_default(),
                    upstream_endpoint_stats: upstream_endpoint_stats__.unwrap_or_default(),
                    priority: priority__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.endpoint.v3.UpstreamLocalityStats", FIELDS, GeneratedVisitor)
    }
}
