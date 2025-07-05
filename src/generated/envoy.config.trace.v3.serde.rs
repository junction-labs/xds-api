impl serde::Serialize for ClientConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.service_name.is_empty() {
            len += 1;
        }
        if !self.instance_name.is_empty() {
            len += 1;
        }
        if self.max_cache_size.is_some() {
            len += 1;
        }
        if self.backend_token_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.ClientConfig", len)?;
        if !self.service_name.is_empty() {
            struct_ser.serialize_field("service_name", &self.service_name)?;
        }
        if !self.instance_name.is_empty() {
            struct_ser.serialize_field("instance_name", &self.instance_name)?;
        }
        if let Some(v) = self.max_cache_size.as_ref() {
            struct_ser.serialize_field("max_cache_size", v)?;
        }
        if let Some(v) = self.backend_token_specifier.as_ref() {
            match v {
                client_config::BackendTokenSpecifier::BackendToken(v) => {
                    struct_ser.serialize_field("backend_token", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service_name",
            "serviceName",
            "instance_name",
            "instanceName",
            "max_cache_size",
            "maxCacheSize",
            "backend_token",
            "backendToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServiceName,
            InstanceName,
            MaxCacheSize,
            BackendToken,
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
                            "serviceName" | "service_name" => Ok(GeneratedField::ServiceName),
                            "instanceName" | "instance_name" => Ok(GeneratedField::InstanceName),
                            "maxCacheSize" | "max_cache_size" => Ok(GeneratedField::MaxCacheSize),
                            "backendToken" | "backend_token" => Ok(GeneratedField::BackendToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.ClientConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClientConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service_name__ = None;
                let mut instance_name__ = None;
                let mut max_cache_size__ = None;
                let mut backend_token_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ServiceName => {
                            if service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            service_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InstanceName => {
                            if instance_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceName"));
                            }
                            instance_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxCacheSize => {
                            if max_cache_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxCacheSize"));
                            }
                            max_cache_size__ = map_.next_value()?;
                        }
                        GeneratedField::BackendToken => {
                            if backend_token_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backendToken"));
                            }
                            backend_token_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(client_config::BackendTokenSpecifier::BackendToken);
                        }
                    }
                }
                Ok(ClientConfig {
                    service_name: service_name__.unwrap_or_default(),
                    instance_name: instance_name__.unwrap_or_default(),
                    max_cache_size: max_cache_size__,
                    backend_token_specifier: backend_token_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.ClientConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DatadogConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.collector_cluster.is_empty() {
            len += 1;
        }
        if !self.service_name.is_empty() {
            len += 1;
        }
        if !self.collector_hostname.is_empty() {
            len += 1;
        }
        if self.remote_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.DatadogConfig", len)?;
        if !self.collector_cluster.is_empty() {
            struct_ser.serialize_field("collector_cluster", &self.collector_cluster)?;
        }
        if !self.service_name.is_empty() {
            struct_ser.serialize_field("service_name", &self.service_name)?;
        }
        if !self.collector_hostname.is_empty() {
            struct_ser.serialize_field("collector_hostname", &self.collector_hostname)?;
        }
        if let Some(v) = self.remote_config.as_ref() {
            struct_ser.serialize_field("remote_config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DatadogConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "collector_cluster",
            "collectorCluster",
            "service_name",
            "serviceName",
            "collector_hostname",
            "collectorHostname",
            "remote_config",
            "remoteConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CollectorCluster,
            ServiceName,
            CollectorHostname,
            RemoteConfig,
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
                            "collectorCluster" | "collector_cluster" => Ok(GeneratedField::CollectorCluster),
                            "serviceName" | "service_name" => Ok(GeneratedField::ServiceName),
                            "collectorHostname" | "collector_hostname" => Ok(GeneratedField::CollectorHostname),
                            "remoteConfig" | "remote_config" => Ok(GeneratedField::RemoteConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DatadogConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.DatadogConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DatadogConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut collector_cluster__ = None;
                let mut service_name__ = None;
                let mut collector_hostname__ = None;
                let mut remote_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CollectorCluster => {
                            if collector_cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectorCluster"));
                            }
                            collector_cluster__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServiceName => {
                            if service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            service_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CollectorHostname => {
                            if collector_hostname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectorHostname"));
                            }
                            collector_hostname__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RemoteConfig => {
                            if remote_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteConfig"));
                            }
                            remote_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DatadogConfig {
                    collector_cluster: collector_cluster__.unwrap_or_default(),
                    service_name: service_name__.unwrap_or_default(),
                    collector_hostname: collector_hostname__.unwrap_or_default(),
                    remote_config: remote_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.DatadogConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DatadogRemoteConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.polling_interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.DatadogRemoteConfig", len)?;
        if let Some(v) = self.polling_interval.as_ref() {
            struct_ser.serialize_field("polling_interval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DatadogRemoteConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "polling_interval",
            "pollingInterval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PollingInterval,
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
                            "pollingInterval" | "polling_interval" => Ok(GeneratedField::PollingInterval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DatadogRemoteConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.DatadogRemoteConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DatadogRemoteConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut polling_interval__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PollingInterval => {
                            if polling_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pollingInterval"));
                            }
                            polling_interval__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DatadogRemoteConfig {
                    polling_interval: polling_interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.DatadogRemoteConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DynamicOtConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.library.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.DynamicOtConfig", len)?;
        if !self.library.is_empty() {
            struct_ser.serialize_field("library", &self.library)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DynamicOtConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "library",
            "config",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Library,
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
                            "library" => Ok(GeneratedField::Library),
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
            type Value = DynamicOtConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.DynamicOtConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DynamicOtConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut library__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Library => {
                            if library__.is_some() {
                                return Err(serde::de::Error::duplicate_field("library"));
                            }
                            library__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DynamicOtConfig {
                    library: library__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.DynamicOtConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LightstepConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.collector_cluster.is_empty() {
            len += 1;
        }
        if !self.access_token_file.is_empty() {
            len += 1;
        }
        if self.access_token.is_some() {
            len += 1;
        }
        if !self.propagation_modes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.LightstepConfig", len)?;
        if !self.collector_cluster.is_empty() {
            struct_ser.serialize_field("collector_cluster", &self.collector_cluster)?;
        }
        if !self.access_token_file.is_empty() {
            struct_ser.serialize_field("access_token_file", &self.access_token_file)?;
        }
        if let Some(v) = self.access_token.as_ref() {
            struct_ser.serialize_field("access_token", v)?;
        }
        if !self.propagation_modes.is_empty() {
            let v = self.propagation_modes.iter().cloned().map(|v| {
                lightstep_config::PropagationMode::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("propagation_modes", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LightstepConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "collector_cluster",
            "collectorCluster",
            "access_token_file",
            "accessTokenFile",
            "access_token",
            "accessToken",
            "propagation_modes",
            "propagationModes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CollectorCluster,
            AccessTokenFile,
            AccessToken,
            PropagationModes,
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
                            "collectorCluster" | "collector_cluster" => Ok(GeneratedField::CollectorCluster),
                            "accessTokenFile" | "access_token_file" => Ok(GeneratedField::AccessTokenFile),
                            "accessToken" | "access_token" => Ok(GeneratedField::AccessToken),
                            "propagationModes" | "propagation_modes" => Ok(GeneratedField::PropagationModes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LightstepConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.LightstepConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LightstepConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut collector_cluster__ = None;
                let mut access_token_file__ = None;
                let mut access_token__ = None;
                let mut propagation_modes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CollectorCluster => {
                            if collector_cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectorCluster"));
                            }
                            collector_cluster__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AccessTokenFile => {
                            if access_token_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessTokenFile"));
                            }
                            access_token_file__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AccessToken => {
                            if access_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessToken"));
                            }
                            access_token__ = map_.next_value()?;
                        }
                        GeneratedField::PropagationModes => {
                            if propagation_modes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("propagationModes"));
                            }
                            propagation_modes__ = Some(map_.next_value::<Vec<lightstep_config::PropagationMode>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(LightstepConfig {
                    collector_cluster: collector_cluster__.unwrap_or_default(),
                    access_token_file: access_token_file__.unwrap_or_default(),
                    access_token: access_token__,
                    propagation_modes: propagation_modes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.LightstepConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for lightstep_config::PropagationMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Envoy => "ENVOY",
            Self::Lightstep => "LIGHTSTEP",
            Self::B3 => "B3",
            Self::TraceContext => "TRACE_CONTEXT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for lightstep_config::PropagationMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ENVOY",
            "LIGHTSTEP",
            "B3",
            "TRACE_CONTEXT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = lightstep_config::PropagationMode;

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
                    "ENVOY" => Ok(lightstep_config::PropagationMode::Envoy),
                    "LIGHTSTEP" => Ok(lightstep_config::PropagationMode::Lightstep),
                    "B3" => Ok(lightstep_config::PropagationMode::B3),
                    "TRACE_CONTEXT" => Ok(lightstep_config::PropagationMode::TraceContext),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OpenTelemetryConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.grpc_service.is_some() {
            len += 1;
        }
        if self.http_service.is_some() {
            len += 1;
        }
        if !self.service_name.is_empty() {
            len += 1;
        }
        if !self.resource_detectors.is_empty() {
            len += 1;
        }
        if self.sampler.is_some() {
            len += 1;
        }
        if self.max_cache_size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.OpenTelemetryConfig", len)?;
        if let Some(v) = self.grpc_service.as_ref() {
            struct_ser.serialize_field("grpc_service", v)?;
        }
        if let Some(v) = self.http_service.as_ref() {
            struct_ser.serialize_field("http_service", v)?;
        }
        if !self.service_name.is_empty() {
            struct_ser.serialize_field("service_name", &self.service_name)?;
        }
        if !self.resource_detectors.is_empty() {
            struct_ser.serialize_field("resource_detectors", &self.resource_detectors)?;
        }
        if let Some(v) = self.sampler.as_ref() {
            struct_ser.serialize_field("sampler", v)?;
        }
        if let Some(v) = self.max_cache_size.as_ref() {
            struct_ser.serialize_field("max_cache_size", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OpenTelemetryConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "grpc_service",
            "grpcService",
            "http_service",
            "httpService",
            "service_name",
            "serviceName",
            "resource_detectors",
            "resourceDetectors",
            "sampler",
            "max_cache_size",
            "maxCacheSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GrpcService,
            HttpService,
            ServiceName,
            ResourceDetectors,
            Sampler,
            MaxCacheSize,
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
                            "grpcService" | "grpc_service" => Ok(GeneratedField::GrpcService),
                            "httpService" | "http_service" => Ok(GeneratedField::HttpService),
                            "serviceName" | "service_name" => Ok(GeneratedField::ServiceName),
                            "resourceDetectors" | "resource_detectors" => Ok(GeneratedField::ResourceDetectors),
                            "sampler" => Ok(GeneratedField::Sampler),
                            "maxCacheSize" | "max_cache_size" => Ok(GeneratedField::MaxCacheSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OpenTelemetryConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.OpenTelemetryConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OpenTelemetryConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut grpc_service__ = None;
                let mut http_service__ = None;
                let mut service_name__ = None;
                let mut resource_detectors__ = None;
                let mut sampler__ = None;
                let mut max_cache_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GrpcService => {
                            if grpc_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcService"));
                            }
                            grpc_service__ = map_.next_value()?;
                        }
                        GeneratedField::HttpService => {
                            if http_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpService"));
                            }
                            http_service__ = map_.next_value()?;
                        }
                        GeneratedField::ServiceName => {
                            if service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            service_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceDetectors => {
                            if resource_detectors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceDetectors"));
                            }
                            resource_detectors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sampler => {
                            if sampler__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sampler"));
                            }
                            sampler__ = map_.next_value()?;
                        }
                        GeneratedField::MaxCacheSize => {
                            if max_cache_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxCacheSize"));
                            }
                            max_cache_size__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OpenTelemetryConfig {
                    grpc_service: grpc_service__,
                    http_service: http_service__,
                    service_name: service_name__.unwrap_or_default(),
                    resource_detectors: resource_detectors__.unwrap_or_default(),
                    sampler: sampler__,
                    max_cache_size: max_cache_size__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.OpenTelemetryConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SkyWalkingConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.grpc_service.is_some() {
            len += 1;
        }
        if self.client_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.SkyWalkingConfig", len)?;
        if let Some(v) = self.grpc_service.as_ref() {
            struct_ser.serialize_field("grpc_service", v)?;
        }
        if let Some(v) = self.client_config.as_ref() {
            struct_ser.serialize_field("client_config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SkyWalkingConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "grpc_service",
            "grpcService",
            "client_config",
            "clientConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GrpcService,
            ClientConfig,
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
                            "grpcService" | "grpc_service" => Ok(GeneratedField::GrpcService),
                            "clientConfig" | "client_config" => Ok(GeneratedField::ClientConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SkyWalkingConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.SkyWalkingConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SkyWalkingConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut grpc_service__ = None;
                let mut client_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GrpcService => {
                            if grpc_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcService"));
                            }
                            grpc_service__ = map_.next_value()?;
                        }
                        GeneratedField::ClientConfig => {
                            if client_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientConfig"));
                            }
                            client_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SkyWalkingConfig {
                    grpc_service: grpc_service__,
                    client_config: client_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.SkyWalkingConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TraceServiceConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.grpc_service.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.TraceServiceConfig", len)?;
        if let Some(v) = self.grpc_service.as_ref() {
            struct_ser.serialize_field("grpc_service", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TraceServiceConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "grpc_service",
            "grpcService",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GrpcService,
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
                            "grpcService" | "grpc_service" => Ok(GeneratedField::GrpcService),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TraceServiceConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.TraceServiceConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TraceServiceConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut grpc_service__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GrpcService => {
                            if grpc_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcService"));
                            }
                            grpc_service__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TraceServiceConfig {
                    grpc_service: grpc_service__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.TraceServiceConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Tracing {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.http.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.Tracing", len)?;
        if let Some(v) = self.http.as_ref() {
            struct_ser.serialize_field("http", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Tracing {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "http",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Http,
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
                            "http" => Ok(GeneratedField::Http),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Tracing;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.Tracing")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Tracing, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut http__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Http => {
                            if http__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http"));
                            }
                            http__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Tracing {
                    http: http__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.Tracing", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for tracing::Http {
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
        if self.config_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.Tracing.Http", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                tracing::http::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typed_config", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for tracing::Http {
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
            type Value = tracing::Http;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.Tracing.Http")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<tracing::Http, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut config_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map_.next_value::<::std::option::Option<_>>()?.map(tracing::http::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(tracing::Http {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.Tracing.Http", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for XRayConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.daemon_endpoint.is_some() {
            len += 1;
        }
        if !self.segment_name.is_empty() {
            len += 1;
        }
        if self.sampling_rule_manifest.is_some() {
            len += 1;
        }
        if self.segment_fields.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.XRayConfig", len)?;
        if let Some(v) = self.daemon_endpoint.as_ref() {
            struct_ser.serialize_field("daemon_endpoint", v)?;
        }
        if !self.segment_name.is_empty() {
            struct_ser.serialize_field("segment_name", &self.segment_name)?;
        }
        if let Some(v) = self.sampling_rule_manifest.as_ref() {
            struct_ser.serialize_field("sampling_rule_manifest", v)?;
        }
        if let Some(v) = self.segment_fields.as_ref() {
            struct_ser.serialize_field("segment_fields", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for XRayConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "daemon_endpoint",
            "daemonEndpoint",
            "segment_name",
            "segmentName",
            "sampling_rule_manifest",
            "samplingRuleManifest",
            "segment_fields",
            "segmentFields",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DaemonEndpoint,
            SegmentName,
            SamplingRuleManifest,
            SegmentFields,
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
                            "daemonEndpoint" | "daemon_endpoint" => Ok(GeneratedField::DaemonEndpoint),
                            "segmentName" | "segment_name" => Ok(GeneratedField::SegmentName),
                            "samplingRuleManifest" | "sampling_rule_manifest" => Ok(GeneratedField::SamplingRuleManifest),
                            "segmentFields" | "segment_fields" => Ok(GeneratedField::SegmentFields),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = XRayConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.XRayConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<XRayConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut daemon_endpoint__ = None;
                let mut segment_name__ = None;
                let mut sampling_rule_manifest__ = None;
                let mut segment_fields__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DaemonEndpoint => {
                            if daemon_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("daemonEndpoint"));
                            }
                            daemon_endpoint__ = map_.next_value()?;
                        }
                        GeneratedField::SegmentName => {
                            if segment_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("segmentName"));
                            }
                            segment_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SamplingRuleManifest => {
                            if sampling_rule_manifest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samplingRuleManifest"));
                            }
                            sampling_rule_manifest__ = map_.next_value()?;
                        }
                        GeneratedField::SegmentFields => {
                            if segment_fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("segmentFields"));
                            }
                            segment_fields__ = map_.next_value()?;
                        }
                    }
                }
                Ok(XRayConfig {
                    daemon_endpoint: daemon_endpoint__,
                    segment_name: segment_name__.unwrap_or_default(),
                    sampling_rule_manifest: sampling_rule_manifest__,
                    segment_fields: segment_fields__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.XRayConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for x_ray_config::SegmentFields {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.origin.is_empty() {
            len += 1;
        }
        if self.aws.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.XRayConfig.SegmentFields", len)?;
        if !self.origin.is_empty() {
            struct_ser.serialize_field("origin", &self.origin)?;
        }
        if let Some(v) = self.aws.as_ref() {
            struct_ser.serialize_field("aws", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for x_ray_config::SegmentFields {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "origin",
            "aws",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Origin,
            Aws,
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
                            "origin" => Ok(GeneratedField::Origin),
                            "aws" => Ok(GeneratedField::Aws),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = x_ray_config::SegmentFields;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.XRayConfig.SegmentFields")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<x_ray_config::SegmentFields, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut origin__ = None;
                let mut aws__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Origin => {
                            if origin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("origin"));
                            }
                            origin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Aws => {
                            if aws__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aws"));
                            }
                            aws__ = map_.next_value()?;
                        }
                    }
                }
                Ok(x_ray_config::SegmentFields {
                    origin: origin__.unwrap_or_default(),
                    aws: aws__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.XRayConfig.SegmentFields", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZipkinConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.collector_cluster.is_empty() {
            len += 1;
        }
        if !self.collector_endpoint.is_empty() {
            len += 1;
        }
        if self.trace_id_128bit {
            len += 1;
        }
        if self.shared_span_context.is_some() {
            len += 1;
        }
        if self.collector_endpoint_version != 0 {
            len += 1;
        }
        if !self.collector_hostname.is_empty() {
            len += 1;
        }
        if self.split_spans_for_request {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.ZipkinConfig", len)?;
        if !self.collector_cluster.is_empty() {
            struct_ser.serialize_field("collector_cluster", &self.collector_cluster)?;
        }
        if !self.collector_endpoint.is_empty() {
            struct_ser.serialize_field("collector_endpoint", &self.collector_endpoint)?;
        }
        if self.trace_id_128bit {
            struct_ser.serialize_field("trace_id_128bit", &self.trace_id_128bit)?;
        }
        if let Some(v) = self.shared_span_context.as_ref() {
            struct_ser.serialize_field("shared_span_context", v)?;
        }
        if self.collector_endpoint_version != 0 {
            let v = zipkin_config::CollectorEndpointVersion::try_from(self.collector_endpoint_version)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.collector_endpoint_version)))?;
            struct_ser.serialize_field("collector_endpoint_version", &v)?;
        }
        if !self.collector_hostname.is_empty() {
            struct_ser.serialize_field("collector_hostname", &self.collector_hostname)?;
        }
        if self.split_spans_for_request {
            struct_ser.serialize_field("split_spans_for_request", &self.split_spans_for_request)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZipkinConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "collector_cluster",
            "collectorCluster",
            "collector_endpoint",
            "collectorEndpoint",
            "trace_id_128bit",
            "traceId128bit",
            "shared_span_context",
            "sharedSpanContext",
            "collector_endpoint_version",
            "collectorEndpointVersion",
            "collector_hostname",
            "collectorHostname",
            "split_spans_for_request",
            "splitSpansForRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CollectorCluster,
            CollectorEndpoint,
            TraceId128bit,
            SharedSpanContext,
            CollectorEndpointVersion,
            CollectorHostname,
            SplitSpansForRequest,
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
                            "collectorCluster" | "collector_cluster" => Ok(GeneratedField::CollectorCluster),
                            "collectorEndpoint" | "collector_endpoint" => Ok(GeneratedField::CollectorEndpoint),
                            "traceId128bit" | "trace_id_128bit" => Ok(GeneratedField::TraceId128bit),
                            "sharedSpanContext" | "shared_span_context" => Ok(GeneratedField::SharedSpanContext),
                            "collectorEndpointVersion" | "collector_endpoint_version" => Ok(GeneratedField::CollectorEndpointVersion),
                            "collectorHostname" | "collector_hostname" => Ok(GeneratedField::CollectorHostname),
                            "splitSpansForRequest" | "split_spans_for_request" => Ok(GeneratedField::SplitSpansForRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZipkinConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.ZipkinConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZipkinConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut collector_cluster__ = None;
                let mut collector_endpoint__ = None;
                let mut trace_id_128bit__ = None;
                let mut shared_span_context__ = None;
                let mut collector_endpoint_version__ = None;
                let mut collector_hostname__ = None;
                let mut split_spans_for_request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CollectorCluster => {
                            if collector_cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectorCluster"));
                            }
                            collector_cluster__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CollectorEndpoint => {
                            if collector_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectorEndpoint"));
                            }
                            collector_endpoint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TraceId128bit => {
                            if trace_id_128bit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceId128bit"));
                            }
                            trace_id_128bit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SharedSpanContext => {
                            if shared_span_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sharedSpanContext"));
                            }
                            shared_span_context__ = map_.next_value()?;
                        }
                        GeneratedField::CollectorEndpointVersion => {
                            if collector_endpoint_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectorEndpointVersion"));
                            }
                            collector_endpoint_version__ = Some(map_.next_value::<zipkin_config::CollectorEndpointVersion>()? as i32);
                        }
                        GeneratedField::CollectorHostname => {
                            if collector_hostname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectorHostname"));
                            }
                            collector_hostname__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SplitSpansForRequest => {
                            if split_spans_for_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("splitSpansForRequest"));
                            }
                            split_spans_for_request__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ZipkinConfig {
                    collector_cluster: collector_cluster__.unwrap_or_default(),
                    collector_endpoint: collector_endpoint__.unwrap_or_default(),
                    trace_id_128bit: trace_id_128bit__.unwrap_or_default(),
                    shared_span_context: shared_span_context__,
                    collector_endpoint_version: collector_endpoint_version__.unwrap_or_default(),
                    collector_hostname: collector_hostname__.unwrap_or_default(),
                    split_spans_for_request: split_spans_for_request__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.ZipkinConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for zipkin_config::CollectorEndpointVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::DeprecatedAndUnavailableDoNotUse => "DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE",
            Self::HttpJson => "HTTP_JSON",
            Self::HttpProto => "HTTP_PROTO",
            Self::Grpc => "GRPC",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for zipkin_config::CollectorEndpointVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE",
            "HTTP_JSON",
            "HTTP_PROTO",
            "GRPC",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = zipkin_config::CollectorEndpointVersion;

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
                    "DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE" => Ok(zipkin_config::CollectorEndpointVersion::DeprecatedAndUnavailableDoNotUse),
                    "HTTP_JSON" => Ok(zipkin_config::CollectorEndpointVersion::HttpJson),
                    "HTTP_PROTO" => Ok(zipkin_config::CollectorEndpointVersion::HttpProto),
                    "GRPC" => Ok(zipkin_config::CollectorEndpointVersion::Grpc),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
