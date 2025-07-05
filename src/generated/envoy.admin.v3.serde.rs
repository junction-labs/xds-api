impl serde::Serialize for ClientResourceStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::Requested => "REQUESTED",
            Self::DoesNotExist => "DOES_NOT_EXIST",
            Self::Acked => "ACKED",
            Self::Nacked => "NACKED",
            Self::ReceivedError => "RECEIVED_ERROR",
            Self::Timeout => "TIMEOUT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ClientResourceStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "REQUESTED",
            "DOES_NOT_EXIST",
            "ACKED",
            "NACKED",
            "RECEIVED_ERROR",
            "TIMEOUT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientResourceStatus;

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
                    "UNKNOWN" => Ok(ClientResourceStatus::Unknown),
                    "REQUESTED" => Ok(ClientResourceStatus::Requested),
                    "DOES_NOT_EXIST" => Ok(ClientResourceStatus::DoesNotExist),
                    "ACKED" => Ok(ClientResourceStatus::Acked),
                    "NACKED" => Ok(ClientResourceStatus::Nacked),
                    "RECEIVED_ERROR" => Ok(ClientResourceStatus::ReceivedError),
                    "TIMEOUT" => Ok(ClientResourceStatus::Timeout),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ClustersConfigDump {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if !self.static_clusters.is_empty() {
            len += 1;
        }
        if !self.dynamic_active_clusters.is_empty() {
            len += 1;
        }
        if !self.dynamic_warming_clusters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ClustersConfigDump", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("version_info", &self.version_info)?;
        }
        if !self.static_clusters.is_empty() {
            struct_ser.serialize_field("static_clusters", &self.static_clusters)?;
        }
        if !self.dynamic_active_clusters.is_empty() {
            struct_ser.serialize_field("dynamic_active_clusters", &self.dynamic_active_clusters)?;
        }
        if !self.dynamic_warming_clusters.is_empty() {
            struct_ser.serialize_field("dynamic_warming_clusters", &self.dynamic_warming_clusters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClustersConfigDump {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "static_clusters",
            "staticClusters",
            "dynamic_active_clusters",
            "dynamicActiveClusters",
            "dynamic_warming_clusters",
            "dynamicWarmingClusters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            StaticClusters,
            DynamicActiveClusters,
            DynamicWarmingClusters,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "staticClusters" | "static_clusters" => Ok(GeneratedField::StaticClusters),
                            "dynamicActiveClusters" | "dynamic_active_clusters" => Ok(GeneratedField::DynamicActiveClusters),
                            "dynamicWarmingClusters" | "dynamic_warming_clusters" => Ok(GeneratedField::DynamicWarmingClusters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClustersConfigDump;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ClustersConfigDump")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClustersConfigDump, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut static_clusters__ = None;
                let mut dynamic_active_clusters__ = None;
                let mut dynamic_warming_clusters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StaticClusters => {
                            if static_clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticClusters"));
                            }
                            static_clusters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DynamicActiveClusters => {
                            if dynamic_active_clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicActiveClusters"));
                            }
                            dynamic_active_clusters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DynamicWarmingClusters => {
                            if dynamic_warming_clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicWarmingClusters"));
                            }
                            dynamic_warming_clusters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ClustersConfigDump {
                    version_info: version_info__.unwrap_or_default(),
                    static_clusters: static_clusters__.unwrap_or_default(),
                    dynamic_active_clusters: dynamic_active_clusters__.unwrap_or_default(),
                    dynamic_warming_clusters: dynamic_warming_clusters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ClustersConfigDump", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for clusters_config_dump::DynamicCluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if self.cluster.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        if self.error_state.is_some() {
            len += 1;
        }
        if self.client_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ClustersConfigDump.DynamicCluster", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("version_info", &self.version_info)?;
        }
        if let Some(v) = self.cluster.as_ref() {
            struct_ser.serialize_field("cluster", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("last_updated", v)?;
        }
        if let Some(v) = self.error_state.as_ref() {
            struct_ser.serialize_field("error_state", v)?;
        }
        if self.client_status != 0 {
            let v = ClientResourceStatus::try_from(self.client_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.client_status)))?;
            struct_ser.serialize_field("client_status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for clusters_config_dump::DynamicCluster {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "cluster",
            "last_updated",
            "lastUpdated",
            "error_state",
            "errorState",
            "client_status",
            "clientStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            Cluster,
            LastUpdated,
            ErrorState,
            ClientStatus,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "cluster" => Ok(GeneratedField::Cluster),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            "errorState" | "error_state" => Ok(GeneratedField::ErrorState),
                            "clientStatus" | "client_status" => Ok(GeneratedField::ClientStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = clusters_config_dump::DynamicCluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ClustersConfigDump.DynamicCluster")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<clusters_config_dump::DynamicCluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut cluster__ = None;
                let mut last_updated__ = None;
                let mut error_state__ = None;
                let mut client_status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cluster => {
                            if cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster__ = map_.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map_.next_value()?;
                        }
                        GeneratedField::ErrorState => {
                            if error_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorState"));
                            }
                            error_state__ = map_.next_value()?;
                        }
                        GeneratedField::ClientStatus => {
                            if client_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientStatus"));
                            }
                            client_status__ = Some(map_.next_value::<ClientResourceStatus>()? as i32);
                        }
                    }
                }
                Ok(clusters_config_dump::DynamicCluster {
                    version_info: version_info__.unwrap_or_default(),
                    cluster: cluster__,
                    last_updated: last_updated__,
                    error_state: error_state__,
                    client_status: client_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ClustersConfigDump.DynamicCluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for clusters_config_dump::StaticCluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cluster.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ClustersConfigDump.StaticCluster", len)?;
        if let Some(v) = self.cluster.as_ref() {
            struct_ser.serialize_field("cluster", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("last_updated", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for clusters_config_dump::StaticCluster {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster",
            "last_updated",
            "lastUpdated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cluster,
            LastUpdated,
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
                            "cluster" => Ok(GeneratedField::Cluster),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = clusters_config_dump::StaticCluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ClustersConfigDump.StaticCluster")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<clusters_config_dump::StaticCluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster__ = None;
                let mut last_updated__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cluster => {
                            if cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster__ = map_.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map_.next_value()?;
                        }
                    }
                }
                Ok(clusters_config_dump::StaticCluster {
                    cluster: cluster__,
                    last_updated: last_updated__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ClustersConfigDump.StaticCluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EcdsConfigDump {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ecds_filters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.EcdsConfigDump", len)?;
        if !self.ecds_filters.is_empty() {
            struct_ser.serialize_field("ecds_filters", &self.ecds_filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EcdsConfigDump {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ecds_filters",
            "ecdsFilters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EcdsFilters,
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
                            "ecdsFilters" | "ecds_filters" => Ok(GeneratedField::EcdsFilters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EcdsConfigDump;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.EcdsConfigDump")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EcdsConfigDump, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ecds_filters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EcdsFilters => {
                            if ecds_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ecdsFilters"));
                            }
                            ecds_filters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EcdsConfigDump {
                    ecds_filters: ecds_filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.EcdsConfigDump", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ecds_config_dump::EcdsFilterConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if self.ecds_filter.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        if self.error_state.is_some() {
            len += 1;
        }
        if self.client_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.EcdsConfigDump.EcdsFilterConfig", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("version_info", &self.version_info)?;
        }
        if let Some(v) = self.ecds_filter.as_ref() {
            struct_ser.serialize_field("ecds_filter", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("last_updated", v)?;
        }
        if let Some(v) = self.error_state.as_ref() {
            struct_ser.serialize_field("error_state", v)?;
        }
        if self.client_status != 0 {
            let v = ClientResourceStatus::try_from(self.client_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.client_status)))?;
            struct_ser.serialize_field("client_status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ecds_config_dump::EcdsFilterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "ecds_filter",
            "ecdsFilter",
            "last_updated",
            "lastUpdated",
            "error_state",
            "errorState",
            "client_status",
            "clientStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            EcdsFilter,
            LastUpdated,
            ErrorState,
            ClientStatus,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "ecdsFilter" | "ecds_filter" => Ok(GeneratedField::EcdsFilter),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            "errorState" | "error_state" => Ok(GeneratedField::ErrorState),
                            "clientStatus" | "client_status" => Ok(GeneratedField::ClientStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ecds_config_dump::EcdsFilterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.EcdsConfigDump.EcdsFilterConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ecds_config_dump::EcdsFilterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut ecds_filter__ = None;
                let mut last_updated__ = None;
                let mut error_state__ = None;
                let mut client_status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EcdsFilter => {
                            if ecds_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ecdsFilter"));
                            }
                            ecds_filter__ = map_.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map_.next_value()?;
                        }
                        GeneratedField::ErrorState => {
                            if error_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorState"));
                            }
                            error_state__ = map_.next_value()?;
                        }
                        GeneratedField::ClientStatus => {
                            if client_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientStatus"));
                            }
                            client_status__ = Some(map_.next_value::<ClientResourceStatus>()? as i32);
                        }
                    }
                }
                Ok(ecds_config_dump::EcdsFilterConfig {
                    version_info: version_info__.unwrap_or_default(),
                    ecds_filter: ecds_filter__,
                    last_updated: last_updated__,
                    error_state: error_state__,
                    client_status: client_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.EcdsConfigDump.EcdsFilterConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EndpointsConfigDump {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.static_endpoint_configs.is_empty() {
            len += 1;
        }
        if !self.dynamic_endpoint_configs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.EndpointsConfigDump", len)?;
        if !self.static_endpoint_configs.is_empty() {
            struct_ser.serialize_field("static_endpoint_configs", &self.static_endpoint_configs)?;
        }
        if !self.dynamic_endpoint_configs.is_empty() {
            struct_ser.serialize_field("dynamic_endpoint_configs", &self.dynamic_endpoint_configs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EndpointsConfigDump {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "static_endpoint_configs",
            "staticEndpointConfigs",
            "dynamic_endpoint_configs",
            "dynamicEndpointConfigs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StaticEndpointConfigs,
            DynamicEndpointConfigs,
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
                            "staticEndpointConfigs" | "static_endpoint_configs" => Ok(GeneratedField::StaticEndpointConfigs),
                            "dynamicEndpointConfigs" | "dynamic_endpoint_configs" => Ok(GeneratedField::DynamicEndpointConfigs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EndpointsConfigDump;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.EndpointsConfigDump")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EndpointsConfigDump, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut static_endpoint_configs__ = None;
                let mut dynamic_endpoint_configs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StaticEndpointConfigs => {
                            if static_endpoint_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticEndpointConfigs"));
                            }
                            static_endpoint_configs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DynamicEndpointConfigs => {
                            if dynamic_endpoint_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicEndpointConfigs"));
                            }
                            dynamic_endpoint_configs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EndpointsConfigDump {
                    static_endpoint_configs: static_endpoint_configs__.unwrap_or_default(),
                    dynamic_endpoint_configs: dynamic_endpoint_configs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.EndpointsConfigDump", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for endpoints_config_dump::DynamicEndpointConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if self.endpoint_config.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        if self.error_state.is_some() {
            len += 1;
        }
        if self.client_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.EndpointsConfigDump.DynamicEndpointConfig", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("version_info", &self.version_info)?;
        }
        if let Some(v) = self.endpoint_config.as_ref() {
            struct_ser.serialize_field("endpoint_config", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("last_updated", v)?;
        }
        if let Some(v) = self.error_state.as_ref() {
            struct_ser.serialize_field("error_state", v)?;
        }
        if self.client_status != 0 {
            let v = ClientResourceStatus::try_from(self.client_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.client_status)))?;
            struct_ser.serialize_field("client_status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for endpoints_config_dump::DynamicEndpointConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "endpoint_config",
            "endpointConfig",
            "last_updated",
            "lastUpdated",
            "error_state",
            "errorState",
            "client_status",
            "clientStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            EndpointConfig,
            LastUpdated,
            ErrorState,
            ClientStatus,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "endpointConfig" | "endpoint_config" => Ok(GeneratedField::EndpointConfig),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            "errorState" | "error_state" => Ok(GeneratedField::ErrorState),
                            "clientStatus" | "client_status" => Ok(GeneratedField::ClientStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = endpoints_config_dump::DynamicEndpointConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.EndpointsConfigDump.DynamicEndpointConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<endpoints_config_dump::DynamicEndpointConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut endpoint_config__ = None;
                let mut last_updated__ = None;
                let mut error_state__ = None;
                let mut client_status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EndpointConfig => {
                            if endpoint_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointConfig"));
                            }
                            endpoint_config__ = map_.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map_.next_value()?;
                        }
                        GeneratedField::ErrorState => {
                            if error_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorState"));
                            }
                            error_state__ = map_.next_value()?;
                        }
                        GeneratedField::ClientStatus => {
                            if client_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientStatus"));
                            }
                            client_status__ = Some(map_.next_value::<ClientResourceStatus>()? as i32);
                        }
                    }
                }
                Ok(endpoints_config_dump::DynamicEndpointConfig {
                    version_info: version_info__.unwrap_or_default(),
                    endpoint_config: endpoint_config__,
                    last_updated: last_updated__,
                    error_state: error_state__,
                    client_status: client_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.EndpointsConfigDump.DynamicEndpointConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for endpoints_config_dump::StaticEndpointConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.endpoint_config.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.EndpointsConfigDump.StaticEndpointConfig", len)?;
        if let Some(v) = self.endpoint_config.as_ref() {
            struct_ser.serialize_field("endpoint_config", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("last_updated", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for endpoints_config_dump::StaticEndpointConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "endpoint_config",
            "endpointConfig",
            "last_updated",
            "lastUpdated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EndpointConfig,
            LastUpdated,
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
                            "endpointConfig" | "endpoint_config" => Ok(GeneratedField::EndpointConfig),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = endpoints_config_dump::StaticEndpointConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.EndpointsConfigDump.StaticEndpointConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<endpoints_config_dump::StaticEndpointConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut endpoint_config__ = None;
                let mut last_updated__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EndpointConfig => {
                            if endpoint_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointConfig"));
                            }
                            endpoint_config__ = map_.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map_.next_value()?;
                        }
                    }
                }
                Ok(endpoints_config_dump::StaticEndpointConfig {
                    endpoint_config: endpoint_config__,
                    last_updated: last_updated__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.EndpointsConfigDump.StaticEndpointConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListenersConfigDump {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if !self.static_listeners.is_empty() {
            len += 1;
        }
        if !self.dynamic_listeners.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ListenersConfigDump", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("version_info", &self.version_info)?;
        }
        if !self.static_listeners.is_empty() {
            struct_ser.serialize_field("static_listeners", &self.static_listeners)?;
        }
        if !self.dynamic_listeners.is_empty() {
            struct_ser.serialize_field("dynamic_listeners", &self.dynamic_listeners)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListenersConfigDump {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "static_listeners",
            "staticListeners",
            "dynamic_listeners",
            "dynamicListeners",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            StaticListeners,
            DynamicListeners,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "staticListeners" | "static_listeners" => Ok(GeneratedField::StaticListeners),
                            "dynamicListeners" | "dynamic_listeners" => Ok(GeneratedField::DynamicListeners),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListenersConfigDump;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ListenersConfigDump")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListenersConfigDump, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut static_listeners__ = None;
                let mut dynamic_listeners__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StaticListeners => {
                            if static_listeners__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticListeners"));
                            }
                            static_listeners__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DynamicListeners => {
                            if dynamic_listeners__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicListeners"));
                            }
                            dynamic_listeners__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListenersConfigDump {
                    version_info: version_info__.unwrap_or_default(),
                    static_listeners: static_listeners__.unwrap_or_default(),
                    dynamic_listeners: dynamic_listeners__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ListenersConfigDump", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for listeners_config_dump::DynamicListener {
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
        if self.active_state.is_some() {
            len += 1;
        }
        if self.warming_state.is_some() {
            len += 1;
        }
        if self.draining_state.is_some() {
            len += 1;
        }
        if self.error_state.is_some() {
            len += 1;
        }
        if self.client_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ListenersConfigDump.DynamicListener", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.active_state.as_ref() {
            struct_ser.serialize_field("active_state", v)?;
        }
        if let Some(v) = self.warming_state.as_ref() {
            struct_ser.serialize_field("warming_state", v)?;
        }
        if let Some(v) = self.draining_state.as_ref() {
            struct_ser.serialize_field("draining_state", v)?;
        }
        if let Some(v) = self.error_state.as_ref() {
            struct_ser.serialize_field("error_state", v)?;
        }
        if self.client_status != 0 {
            let v = ClientResourceStatus::try_from(self.client_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.client_status)))?;
            struct_ser.serialize_field("client_status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for listeners_config_dump::DynamicListener {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "active_state",
            "activeState",
            "warming_state",
            "warmingState",
            "draining_state",
            "drainingState",
            "error_state",
            "errorState",
            "client_status",
            "clientStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ActiveState,
            WarmingState,
            DrainingState,
            ErrorState,
            ClientStatus,
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
                            "activeState" | "active_state" => Ok(GeneratedField::ActiveState),
                            "warmingState" | "warming_state" => Ok(GeneratedField::WarmingState),
                            "drainingState" | "draining_state" => Ok(GeneratedField::DrainingState),
                            "errorState" | "error_state" => Ok(GeneratedField::ErrorState),
                            "clientStatus" | "client_status" => Ok(GeneratedField::ClientStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = listeners_config_dump::DynamicListener;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ListenersConfigDump.DynamicListener")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<listeners_config_dump::DynamicListener, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut active_state__ = None;
                let mut warming_state__ = None;
                let mut draining_state__ = None;
                let mut error_state__ = None;
                let mut client_status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ActiveState => {
                            if active_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activeState"));
                            }
                            active_state__ = map_.next_value()?;
                        }
                        GeneratedField::WarmingState => {
                            if warming_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warmingState"));
                            }
                            warming_state__ = map_.next_value()?;
                        }
                        GeneratedField::DrainingState => {
                            if draining_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drainingState"));
                            }
                            draining_state__ = map_.next_value()?;
                        }
                        GeneratedField::ErrorState => {
                            if error_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorState"));
                            }
                            error_state__ = map_.next_value()?;
                        }
                        GeneratedField::ClientStatus => {
                            if client_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientStatus"));
                            }
                            client_status__ = Some(map_.next_value::<ClientResourceStatus>()? as i32);
                        }
                    }
                }
                Ok(listeners_config_dump::DynamicListener {
                    name: name__.unwrap_or_default(),
                    active_state: active_state__,
                    warming_state: warming_state__,
                    draining_state: draining_state__,
                    error_state: error_state__,
                    client_status: client_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ListenersConfigDump.DynamicListener", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for listeners_config_dump::DynamicListenerState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if self.listener.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ListenersConfigDump.DynamicListenerState", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("version_info", &self.version_info)?;
        }
        if let Some(v) = self.listener.as_ref() {
            struct_ser.serialize_field("listener", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("last_updated", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for listeners_config_dump::DynamicListenerState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "listener",
            "last_updated",
            "lastUpdated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            Listener,
            LastUpdated,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "listener" => Ok(GeneratedField::Listener),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = listeners_config_dump::DynamicListenerState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ListenersConfigDump.DynamicListenerState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<listeners_config_dump::DynamicListenerState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut listener__ = None;
                let mut last_updated__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Listener => {
                            if listener__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listener"));
                            }
                            listener__ = map_.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map_.next_value()?;
                        }
                    }
                }
                Ok(listeners_config_dump::DynamicListenerState {
                    version_info: version_info__.unwrap_or_default(),
                    listener: listener__,
                    last_updated: last_updated__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ListenersConfigDump.DynamicListenerState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for listeners_config_dump::StaticListener {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.listener.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ListenersConfigDump.StaticListener", len)?;
        if let Some(v) = self.listener.as_ref() {
            struct_ser.serialize_field("listener", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("last_updated", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for listeners_config_dump::StaticListener {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "listener",
            "last_updated",
            "lastUpdated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Listener,
            LastUpdated,
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
                            "listener" => Ok(GeneratedField::Listener),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = listeners_config_dump::StaticListener;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ListenersConfigDump.StaticListener")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<listeners_config_dump::StaticListener, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut listener__ = None;
                let mut last_updated__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Listener => {
                            if listener__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listener"));
                            }
                            listener__ = map_.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map_.next_value()?;
                        }
                    }
                }
                Ok(listeners_config_dump::StaticListener {
                    listener: listener__,
                    last_updated: last_updated__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ListenersConfigDump.StaticListener", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoutesConfigDump {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.static_route_configs.is_empty() {
            len += 1;
        }
        if !self.dynamic_route_configs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.RoutesConfigDump", len)?;
        if !self.static_route_configs.is_empty() {
            struct_ser.serialize_field("static_route_configs", &self.static_route_configs)?;
        }
        if !self.dynamic_route_configs.is_empty() {
            struct_ser.serialize_field("dynamic_route_configs", &self.dynamic_route_configs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoutesConfigDump {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "static_route_configs",
            "staticRouteConfigs",
            "dynamic_route_configs",
            "dynamicRouteConfigs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StaticRouteConfigs,
            DynamicRouteConfigs,
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
                            "staticRouteConfigs" | "static_route_configs" => Ok(GeneratedField::StaticRouteConfigs),
                            "dynamicRouteConfigs" | "dynamic_route_configs" => Ok(GeneratedField::DynamicRouteConfigs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoutesConfigDump;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.RoutesConfigDump")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RoutesConfigDump, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut static_route_configs__ = None;
                let mut dynamic_route_configs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StaticRouteConfigs => {
                            if static_route_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticRouteConfigs"));
                            }
                            static_route_configs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DynamicRouteConfigs => {
                            if dynamic_route_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicRouteConfigs"));
                            }
                            dynamic_route_configs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RoutesConfigDump {
                    static_route_configs: static_route_configs__.unwrap_or_default(),
                    dynamic_route_configs: dynamic_route_configs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.RoutesConfigDump", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for routes_config_dump::DynamicRouteConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if self.route_config.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        if self.error_state.is_some() {
            len += 1;
        }
        if self.client_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.RoutesConfigDump.DynamicRouteConfig", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("version_info", &self.version_info)?;
        }
        if let Some(v) = self.route_config.as_ref() {
            struct_ser.serialize_field("route_config", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("last_updated", v)?;
        }
        if let Some(v) = self.error_state.as_ref() {
            struct_ser.serialize_field("error_state", v)?;
        }
        if self.client_status != 0 {
            let v = ClientResourceStatus::try_from(self.client_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.client_status)))?;
            struct_ser.serialize_field("client_status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for routes_config_dump::DynamicRouteConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "route_config",
            "routeConfig",
            "last_updated",
            "lastUpdated",
            "error_state",
            "errorState",
            "client_status",
            "clientStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            RouteConfig,
            LastUpdated,
            ErrorState,
            ClientStatus,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "routeConfig" | "route_config" => Ok(GeneratedField::RouteConfig),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            "errorState" | "error_state" => Ok(GeneratedField::ErrorState),
                            "clientStatus" | "client_status" => Ok(GeneratedField::ClientStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = routes_config_dump::DynamicRouteConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.RoutesConfigDump.DynamicRouteConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<routes_config_dump::DynamicRouteConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut route_config__ = None;
                let mut last_updated__ = None;
                let mut error_state__ = None;
                let mut client_status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RouteConfig => {
                            if route_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeConfig"));
                            }
                            route_config__ = map_.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map_.next_value()?;
                        }
                        GeneratedField::ErrorState => {
                            if error_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorState"));
                            }
                            error_state__ = map_.next_value()?;
                        }
                        GeneratedField::ClientStatus => {
                            if client_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientStatus"));
                            }
                            client_status__ = Some(map_.next_value::<ClientResourceStatus>()? as i32);
                        }
                    }
                }
                Ok(routes_config_dump::DynamicRouteConfig {
                    version_info: version_info__.unwrap_or_default(),
                    route_config: route_config__,
                    last_updated: last_updated__,
                    error_state: error_state__,
                    client_status: client_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.RoutesConfigDump.DynamicRouteConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for routes_config_dump::StaticRouteConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.route_config.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.RoutesConfigDump.StaticRouteConfig", len)?;
        if let Some(v) = self.route_config.as_ref() {
            struct_ser.serialize_field("route_config", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("last_updated", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for routes_config_dump::StaticRouteConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "route_config",
            "routeConfig",
            "last_updated",
            "lastUpdated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RouteConfig,
            LastUpdated,
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
                            "routeConfig" | "route_config" => Ok(GeneratedField::RouteConfig),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = routes_config_dump::StaticRouteConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.RoutesConfigDump.StaticRouteConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<routes_config_dump::StaticRouteConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut route_config__ = None;
                let mut last_updated__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RouteConfig => {
                            if route_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeConfig"));
                            }
                            route_config__ = map_.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map_.next_value()?;
                        }
                    }
                }
                Ok(routes_config_dump::StaticRouteConfig {
                    route_config: route_config__,
                    last_updated: last_updated__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.RoutesConfigDump.StaticRouteConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScopedRoutesConfigDump {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.inline_scoped_route_configs.is_empty() {
            len += 1;
        }
        if !self.dynamic_scoped_route_configs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ScopedRoutesConfigDump", len)?;
        if !self.inline_scoped_route_configs.is_empty() {
            struct_ser.serialize_field("inline_scoped_route_configs", &self.inline_scoped_route_configs)?;
        }
        if !self.dynamic_scoped_route_configs.is_empty() {
            struct_ser.serialize_field("dynamic_scoped_route_configs", &self.dynamic_scoped_route_configs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScopedRoutesConfigDump {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "inline_scoped_route_configs",
            "inlineScopedRouteConfigs",
            "dynamic_scoped_route_configs",
            "dynamicScopedRouteConfigs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InlineScopedRouteConfigs,
            DynamicScopedRouteConfigs,
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
                            "inlineScopedRouteConfigs" | "inline_scoped_route_configs" => Ok(GeneratedField::InlineScopedRouteConfigs),
                            "dynamicScopedRouteConfigs" | "dynamic_scoped_route_configs" => Ok(GeneratedField::DynamicScopedRouteConfigs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScopedRoutesConfigDump;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ScopedRoutesConfigDump")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ScopedRoutesConfigDump, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut inline_scoped_route_configs__ = None;
                let mut dynamic_scoped_route_configs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InlineScopedRouteConfigs => {
                            if inline_scoped_route_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inlineScopedRouteConfigs"));
                            }
                            inline_scoped_route_configs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DynamicScopedRouteConfigs => {
                            if dynamic_scoped_route_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicScopedRouteConfigs"));
                            }
                            dynamic_scoped_route_configs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ScopedRoutesConfigDump {
                    inline_scoped_route_configs: inline_scoped_route_configs__.unwrap_or_default(),
                    dynamic_scoped_route_configs: dynamic_scoped_route_configs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ScopedRoutesConfigDump", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scoped_routes_config_dump::DynamicScopedRouteConfigs {
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
        if !self.version_info.is_empty() {
            len += 1;
        }
        if !self.scoped_route_configs.is_empty() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        if self.error_state.is_some() {
            len += 1;
        }
        if self.client_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ScopedRoutesConfigDump.DynamicScopedRouteConfigs", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("version_info", &self.version_info)?;
        }
        if !self.scoped_route_configs.is_empty() {
            struct_ser.serialize_field("scoped_route_configs", &self.scoped_route_configs)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("last_updated", v)?;
        }
        if let Some(v) = self.error_state.as_ref() {
            struct_ser.serialize_field("error_state", v)?;
        }
        if self.client_status != 0 {
            let v = ClientResourceStatus::try_from(self.client_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.client_status)))?;
            struct_ser.serialize_field("client_status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for scoped_routes_config_dump::DynamicScopedRouteConfigs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "version_info",
            "versionInfo",
            "scoped_route_configs",
            "scopedRouteConfigs",
            "last_updated",
            "lastUpdated",
            "error_state",
            "errorState",
            "client_status",
            "clientStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            VersionInfo,
            ScopedRouteConfigs,
            LastUpdated,
            ErrorState,
            ClientStatus,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "scopedRouteConfigs" | "scoped_route_configs" => Ok(GeneratedField::ScopedRouteConfigs),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            "errorState" | "error_state" => Ok(GeneratedField::ErrorState),
                            "clientStatus" | "client_status" => Ok(GeneratedField::ClientStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scoped_routes_config_dump::DynamicScopedRouteConfigs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ScopedRoutesConfigDump.DynamicScopedRouteConfigs")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<scoped_routes_config_dump::DynamicScopedRouteConfigs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut version_info__ = None;
                let mut scoped_route_configs__ = None;
                let mut last_updated__ = None;
                let mut error_state__ = None;
                let mut client_status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ScopedRouteConfigs => {
                            if scoped_route_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopedRouteConfigs"));
                            }
                            scoped_route_configs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map_.next_value()?;
                        }
                        GeneratedField::ErrorState => {
                            if error_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorState"));
                            }
                            error_state__ = map_.next_value()?;
                        }
                        GeneratedField::ClientStatus => {
                            if client_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientStatus"));
                            }
                            client_status__ = Some(map_.next_value::<ClientResourceStatus>()? as i32);
                        }
                    }
                }
                Ok(scoped_routes_config_dump::DynamicScopedRouteConfigs {
                    name: name__.unwrap_or_default(),
                    version_info: version_info__.unwrap_or_default(),
                    scoped_route_configs: scoped_route_configs__.unwrap_or_default(),
                    last_updated: last_updated__,
                    error_state: error_state__,
                    client_status: client_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ScopedRoutesConfigDump.DynamicScopedRouteConfigs", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scoped_routes_config_dump::InlineScopedRouteConfigs {
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
        if !self.scoped_route_configs.is_empty() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ScopedRoutesConfigDump.InlineScopedRouteConfigs", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.scoped_route_configs.is_empty() {
            struct_ser.serialize_field("scoped_route_configs", &self.scoped_route_configs)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("last_updated", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for scoped_routes_config_dump::InlineScopedRouteConfigs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "scoped_route_configs",
            "scopedRouteConfigs",
            "last_updated",
            "lastUpdated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ScopedRouteConfigs,
            LastUpdated,
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
                            "scopedRouteConfigs" | "scoped_route_configs" => Ok(GeneratedField::ScopedRouteConfigs),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scoped_routes_config_dump::InlineScopedRouteConfigs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ScopedRoutesConfigDump.InlineScopedRouteConfigs")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<scoped_routes_config_dump::InlineScopedRouteConfigs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut scoped_route_configs__ = None;
                let mut last_updated__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ScopedRouteConfigs => {
                            if scoped_route_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopedRouteConfigs"));
                            }
                            scoped_route_configs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map_.next_value()?;
                        }
                    }
                }
                Ok(scoped_routes_config_dump::InlineScopedRouteConfigs {
                    name: name__.unwrap_or_default(),
                    scoped_route_configs: scoped_route_configs__.unwrap_or_default(),
                    last_updated: last_updated__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ScopedRoutesConfigDump.InlineScopedRouteConfigs", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateFailureState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.failed_configuration.is_some() {
            len += 1;
        }
        if self.last_update_attempt.is_some() {
            len += 1;
        }
        if !self.details.is_empty() {
            len += 1;
        }
        if !self.version_info.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.UpdateFailureState", len)?;
        if let Some(v) = self.failed_configuration.as_ref() {
            struct_ser.serialize_field("failed_configuration", v)?;
        }
        if let Some(v) = self.last_update_attempt.as_ref() {
            struct_ser.serialize_field("last_update_attempt", v)?;
        }
        if !self.details.is_empty() {
            struct_ser.serialize_field("details", &self.details)?;
        }
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("version_info", &self.version_info)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateFailureState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "failed_configuration",
            "failedConfiguration",
            "last_update_attempt",
            "lastUpdateAttempt",
            "details",
            "version_info",
            "versionInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FailedConfiguration,
            LastUpdateAttempt,
            Details,
            VersionInfo,
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
                            "failedConfiguration" | "failed_configuration" => Ok(GeneratedField::FailedConfiguration),
                            "lastUpdateAttempt" | "last_update_attempt" => Ok(GeneratedField::LastUpdateAttempt),
                            "details" => Ok(GeneratedField::Details),
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateFailureState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.UpdateFailureState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateFailureState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut failed_configuration__ = None;
                let mut last_update_attempt__ = None;
                let mut details__ = None;
                let mut version_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FailedConfiguration => {
                            if failed_configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failedConfiguration"));
                            }
                            failed_configuration__ = map_.next_value()?;
                        }
                        GeneratedField::LastUpdateAttempt => {
                            if last_update_attempt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdateAttempt"));
                            }
                            last_update_attempt__ = map_.next_value()?;
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateFailureState {
                    failed_configuration: failed_configuration__,
                    last_update_attempt: last_update_attempt__,
                    details: details__.unwrap_or_default(),
                    version_info: version_info__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.UpdateFailureState", FIELDS, GeneratedVisitor)
    }
}
