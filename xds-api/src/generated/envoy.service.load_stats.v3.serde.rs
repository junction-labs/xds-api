impl serde::Serialize for LoadStatsRequest {
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
        if !self.cluster_stats.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.load_stats.v3.LoadStatsRequest", len)?;
        if let Some(v) = self.node.as_ref() {
            struct_ser.serialize_field("node", v)?;
        }
        if !self.cluster_stats.is_empty() {
            struct_ser.serialize_field("cluster_stats", &self.cluster_stats)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoadStatsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "node",
            "cluster_stats",
            "clusterStats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Node,
            ClusterStats,
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
                            "clusterStats" | "cluster_stats" => Ok(GeneratedField::ClusterStats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoadStatsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.load_stats.v3.LoadStatsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoadStatsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut node__ = None;
                let mut cluster_stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = map_.next_value()?;
                        }
                        GeneratedField::ClusterStats => {
                            if cluster_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterStats"));
                            }
                            cluster_stats__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LoadStatsRequest {
                    node: node__,
                    cluster_stats: cluster_stats__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.load_stats.v3.LoadStatsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoadStatsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.clusters.is_empty() {
            len += 1;
        }
        if self.send_all_clusters {
            len += 1;
        }
        if self.load_reporting_interval.is_some() {
            len += 1;
        }
        if self.report_endpoint_granularity {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.load_stats.v3.LoadStatsResponse", len)?;
        if !self.clusters.is_empty() {
            struct_ser.serialize_field("clusters", &self.clusters)?;
        }
        if self.send_all_clusters {
            struct_ser.serialize_field("send_all_clusters", &self.send_all_clusters)?;
        }
        if let Some(v) = self.load_reporting_interval.as_ref() {
            struct_ser.serialize_field("load_reporting_interval", v)?;
        }
        if self.report_endpoint_granularity {
            struct_ser.serialize_field("report_endpoint_granularity", &self.report_endpoint_granularity)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoadStatsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "clusters",
            "send_all_clusters",
            "sendAllClusters",
            "load_reporting_interval",
            "loadReportingInterval",
            "report_endpoint_granularity",
            "reportEndpointGranularity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Clusters,
            SendAllClusters,
            LoadReportingInterval,
            ReportEndpointGranularity,
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
                            "clusters" => Ok(GeneratedField::Clusters),
                            "sendAllClusters" | "send_all_clusters" => Ok(GeneratedField::SendAllClusters),
                            "loadReportingInterval" | "load_reporting_interval" => Ok(GeneratedField::LoadReportingInterval),
                            "reportEndpointGranularity" | "report_endpoint_granularity" => Ok(GeneratedField::ReportEndpointGranularity),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoadStatsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.load_stats.v3.LoadStatsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoadStatsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut clusters__ = None;
                let mut send_all_clusters__ = None;
                let mut load_reporting_interval__ = None;
                let mut report_endpoint_granularity__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Clusters => {
                            if clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusters"));
                            }
                            clusters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SendAllClusters => {
                            if send_all_clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sendAllClusters"));
                            }
                            send_all_clusters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoadReportingInterval => {
                            if load_reporting_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadReportingInterval"));
                            }
                            load_reporting_interval__ = map_.next_value()?;
                        }
                        GeneratedField::ReportEndpointGranularity => {
                            if report_endpoint_granularity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportEndpointGranularity"));
                            }
                            report_endpoint_granularity__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LoadStatsResponse {
                    clusters: clusters__.unwrap_or_default(),
                    send_all_clusters: send_all_clusters__.unwrap_or_default(),
                    load_reporting_interval: load_reporting_interval__,
                    report_endpoint_granularity: report_endpoint_granularity__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.load_stats.v3.LoadStatsResponse", FIELDS, GeneratedVisitor)
    }
}
