impl serde::Serialize for ActiveRawUdpListenerConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.listener.v3.ActiveRawUdpListenerConfig", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActiveRawUdpListenerConfig {
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
            type Value = ActiveRawUdpListenerConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.ActiveRawUdpListenerConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ActiveRawUdpListenerConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ActiveRawUdpListenerConfig {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.ActiveRawUdpListenerConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AdditionalAddress {
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
        if self.socket_options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.listener.v3.AdditionalAddress", len)?;
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.socket_options.as_ref() {
            struct_ser.serialize_field("socket_options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AdditionalAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "socket_options",
            "socketOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            SocketOptions,
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
                            "socketOptions" | "socket_options" => Ok(GeneratedField::SocketOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AdditionalAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.AdditionalAddress")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AdditionalAddress, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut socket_options__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                            socket_options__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AdditionalAddress {
                    address: address__,
                    socket_options: socket_options__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.AdditionalAddress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApiListener {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.api_listener.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.listener.v3.ApiListener", len)?;
        if let Some(v) = self.api_listener.as_ref() {
            struct_ser.serialize_field("api_listener", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApiListener {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "api_listener",
            "apiListener",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ApiListener,
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
                            "apiListener" | "api_listener" => Ok(GeneratedField::ApiListener),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiListener;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.ApiListener")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ApiListener, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut api_listener__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ApiListener => {
                            if api_listener__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiListener"));
                            }
                            api_listener__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ApiListener {
                    api_listener: api_listener__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.ApiListener", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApiListenerManager {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.listener.v3.ApiListenerManager", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApiListenerManager {
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
            type Value = ApiListenerManager;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.ApiListenerManager")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ApiListenerManager, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ApiListenerManager {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.ApiListenerManager", FIELDS, GeneratedVisitor)
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
        if self.config_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.listener.v3.Filter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                filter::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typed_config", v)?;
                }
                filter::ConfigType::ConfigDiscovery(v) => {
                    struct_ser.serialize_field("config_discovery", v)?;
                }
            }
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
                formatter.write_str("struct envoy.config.listener.v3.Filter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Filter, V::Error>
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
                            config_type__ = map_.next_value::<::std::option::Option<_>>()?.map(filter::ConfigType::TypedConfig)
;
                        }
                        GeneratedField::ConfigDiscovery => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configDiscovery"));
                            }
                            config_type__ = map_.next_value::<::std::option::Option<_>>()?.map(filter::ConfigType::ConfigDiscovery)
;
                        }
                    }
                }
                Ok(Filter {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.Filter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilterChain {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.filter_chain_match.is_some() {
            len += 1;
        }
        if !self.filters.is_empty() {
            len += 1;
        }
        if self.use_proxy_proto.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.transport_socket.is_some() {
            len += 1;
        }
        if self.transport_socket_connect_timeout.is_some() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.listener.v3.FilterChain", len)?;
        if let Some(v) = self.filter_chain_match.as_ref() {
            struct_ser.serialize_field("filter_chain_match", v)?;
        }
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        if let Some(v) = self.use_proxy_proto.as_ref() {
            struct_ser.serialize_field("use_proxy_proto", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.transport_socket.as_ref() {
            struct_ser.serialize_field("transport_socket", v)?;
        }
        if let Some(v) = self.transport_socket_connect_timeout.as_ref() {
            struct_ser.serialize_field("transport_socket_connect_timeout", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterChain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filter_chain_match",
            "filterChainMatch",
            "filters",
            "use_proxy_proto",
            "useProxyProto",
            "metadata",
            "transport_socket",
            "transportSocket",
            "transport_socket_connect_timeout",
            "transportSocketConnectTimeout",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilterChainMatch,
            Filters,
            UseProxyProto,
            Metadata,
            TransportSocket,
            TransportSocketConnectTimeout,
            Name,
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
                            "filterChainMatch" | "filter_chain_match" => Ok(GeneratedField::FilterChainMatch),
                            "filters" => Ok(GeneratedField::Filters),
                            "useProxyProto" | "use_proxy_proto" => Ok(GeneratedField::UseProxyProto),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "transportSocket" | "transport_socket" => Ok(GeneratedField::TransportSocket),
                            "transportSocketConnectTimeout" | "transport_socket_connect_timeout" => Ok(GeneratedField::TransportSocketConnectTimeout),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterChain;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.FilterChain")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FilterChain, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter_chain_match__ = None;
                let mut filters__ = None;
                let mut use_proxy_proto__ = None;
                let mut metadata__ = None;
                let mut transport_socket__ = None;
                let mut transport_socket_connect_timeout__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FilterChainMatch => {
                            if filter_chain_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterChainMatch"));
                            }
                            filter_chain_match__ = map_.next_value()?;
                        }
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UseProxyProto => {
                            if use_proxy_proto__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useProxyProto"));
                            }
                            use_proxy_proto__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::TransportSocket => {
                            if transport_socket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportSocket"));
                            }
                            transport_socket__ = map_.next_value()?;
                        }
                        GeneratedField::TransportSocketConnectTimeout => {
                            if transport_socket_connect_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportSocketConnectTimeout"));
                            }
                            transport_socket_connect_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FilterChain {
                    filter_chain_match: filter_chain_match__,
                    filters: filters__.unwrap_or_default(),
                    use_proxy_proto: use_proxy_proto__,
                    metadata: metadata__,
                    transport_socket: transport_socket__,
                    transport_socket_connect_timeout: transport_socket_connect_timeout__,
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.FilterChain", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilterChainMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.destination_port.is_some() {
            len += 1;
        }
        if !self.prefix_ranges.is_empty() {
            len += 1;
        }
        if !self.address_suffix.is_empty() {
            len += 1;
        }
        if self.suffix_len.is_some() {
            len += 1;
        }
        if !self.direct_source_prefix_ranges.is_empty() {
            len += 1;
        }
        if self.source_type != 0 {
            len += 1;
        }
        if !self.source_prefix_ranges.is_empty() {
            len += 1;
        }
        if !self.source_ports.is_empty() {
            len += 1;
        }
        if !self.server_names.is_empty() {
            len += 1;
        }
        if !self.transport_protocol.is_empty() {
            len += 1;
        }
        if !self.application_protocols.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.listener.v3.FilterChainMatch", len)?;
        if let Some(v) = self.destination_port.as_ref() {
            struct_ser.serialize_field("destination_port", v)?;
        }
        if !self.prefix_ranges.is_empty() {
            struct_ser.serialize_field("prefix_ranges", &self.prefix_ranges)?;
        }
        if !self.address_suffix.is_empty() {
            struct_ser.serialize_field("address_suffix", &self.address_suffix)?;
        }
        if let Some(v) = self.suffix_len.as_ref() {
            struct_ser.serialize_field("suffix_len", v)?;
        }
        if !self.direct_source_prefix_ranges.is_empty() {
            struct_ser.serialize_field("direct_source_prefix_ranges", &self.direct_source_prefix_ranges)?;
        }
        if self.source_type != 0 {
            let v = filter_chain_match::ConnectionSourceType::try_from(self.source_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.source_type)))?;
            struct_ser.serialize_field("source_type", &v)?;
        }
        if !self.source_prefix_ranges.is_empty() {
            struct_ser.serialize_field("source_prefix_ranges", &self.source_prefix_ranges)?;
        }
        if !self.source_ports.is_empty() {
            struct_ser.serialize_field("source_ports", &self.source_ports)?;
        }
        if !self.server_names.is_empty() {
            struct_ser.serialize_field("server_names", &self.server_names)?;
        }
        if !self.transport_protocol.is_empty() {
            struct_ser.serialize_field("transport_protocol", &self.transport_protocol)?;
        }
        if !self.application_protocols.is_empty() {
            struct_ser.serialize_field("application_protocols", &self.application_protocols)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterChainMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "destination_port",
            "destinationPort",
            "prefix_ranges",
            "prefixRanges",
            "address_suffix",
            "addressSuffix",
            "suffix_len",
            "suffixLen",
            "direct_source_prefix_ranges",
            "directSourcePrefixRanges",
            "source_type",
            "sourceType",
            "source_prefix_ranges",
            "sourcePrefixRanges",
            "source_ports",
            "sourcePorts",
            "server_names",
            "serverNames",
            "transport_protocol",
            "transportProtocol",
            "application_protocols",
            "applicationProtocols",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DestinationPort,
            PrefixRanges,
            AddressSuffix,
            SuffixLen,
            DirectSourcePrefixRanges,
            SourceType,
            SourcePrefixRanges,
            SourcePorts,
            ServerNames,
            TransportProtocol,
            ApplicationProtocols,
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
                            "destinationPort" | "destination_port" => Ok(GeneratedField::DestinationPort),
                            "prefixRanges" | "prefix_ranges" => Ok(GeneratedField::PrefixRanges),
                            "addressSuffix" | "address_suffix" => Ok(GeneratedField::AddressSuffix),
                            "suffixLen" | "suffix_len" => Ok(GeneratedField::SuffixLen),
                            "directSourcePrefixRanges" | "direct_source_prefix_ranges" => Ok(GeneratedField::DirectSourcePrefixRanges),
                            "sourceType" | "source_type" => Ok(GeneratedField::SourceType),
                            "sourcePrefixRanges" | "source_prefix_ranges" => Ok(GeneratedField::SourcePrefixRanges),
                            "sourcePorts" | "source_ports" => Ok(GeneratedField::SourcePorts),
                            "serverNames" | "server_names" => Ok(GeneratedField::ServerNames),
                            "transportProtocol" | "transport_protocol" => Ok(GeneratedField::TransportProtocol),
                            "applicationProtocols" | "application_protocols" => Ok(GeneratedField::ApplicationProtocols),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterChainMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.FilterChainMatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FilterChainMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut destination_port__ = None;
                let mut prefix_ranges__ = None;
                let mut address_suffix__ = None;
                let mut suffix_len__ = None;
                let mut direct_source_prefix_ranges__ = None;
                let mut source_type__ = None;
                let mut source_prefix_ranges__ = None;
                let mut source_ports__ = None;
                let mut server_names__ = None;
                let mut transport_protocol__ = None;
                let mut application_protocols__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DestinationPort => {
                            if destination_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationPort"));
                            }
                            destination_port__ = map_.next_value()?;
                        }
                        GeneratedField::PrefixRanges => {
                            if prefix_ranges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefixRanges"));
                            }
                            prefix_ranges__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AddressSuffix => {
                            if address_suffix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressSuffix"));
                            }
                            address_suffix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SuffixLen => {
                            if suffix_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suffixLen"));
                            }
                            suffix_len__ = map_.next_value()?;
                        }
                        GeneratedField::DirectSourcePrefixRanges => {
                            if direct_source_prefix_ranges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directSourcePrefixRanges"));
                            }
                            direct_source_prefix_ranges__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourceType => {
                            if source_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceType"));
                            }
                            source_type__ = Some(map_.next_value::<filter_chain_match::ConnectionSourceType>()? as i32);
                        }
                        GeneratedField::SourcePrefixRanges => {
                            if source_prefix_ranges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourcePrefixRanges"));
                            }
                            source_prefix_ranges__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourcePorts => {
                            if source_ports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourcePorts"));
                            }
                            source_ports__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::ServerNames => {
                            if server_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverNames"));
                            }
                            server_names__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TransportProtocol => {
                            if transport_protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportProtocol"));
                            }
                            transport_protocol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ApplicationProtocols => {
                            if application_protocols__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applicationProtocols"));
                            }
                            application_protocols__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FilterChainMatch {
                    destination_port: destination_port__,
                    prefix_ranges: prefix_ranges__.unwrap_or_default(),
                    address_suffix: address_suffix__.unwrap_or_default(),
                    suffix_len: suffix_len__,
                    direct_source_prefix_ranges: direct_source_prefix_ranges__.unwrap_or_default(),
                    source_type: source_type__.unwrap_or_default(),
                    source_prefix_ranges: source_prefix_ranges__.unwrap_or_default(),
                    source_ports: source_ports__.unwrap_or_default(),
                    server_names: server_names__.unwrap_or_default(),
                    transport_protocol: transport_protocol__.unwrap_or_default(),
                    application_protocols: application_protocols__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.FilterChainMatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for filter_chain_match::ConnectionSourceType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Any => "ANY",
            Self::SameIpOrLoopback => "SAME_IP_OR_LOOPBACK",
            Self::External => "EXTERNAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for filter_chain_match::ConnectionSourceType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ANY",
            "SAME_IP_OR_LOOPBACK",
            "EXTERNAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = filter_chain_match::ConnectionSourceType;

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
                    "ANY" => Ok(filter_chain_match::ConnectionSourceType::Any),
                    "SAME_IP_OR_LOOPBACK" => Ok(filter_chain_match::ConnectionSourceType::SameIpOrLoopback),
                    "EXTERNAL" => Ok(filter_chain_match::ConnectionSourceType::External),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Listener {
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
        if self.address.is_some() {
            len += 1;
        }
        if !self.additional_addresses.is_empty() {
            len += 1;
        }
        if !self.stat_prefix.is_empty() {
            len += 1;
        }
        if !self.filter_chains.is_empty() {
            len += 1;
        }
        if self.fcds_config.is_some() {
            len += 1;
        }
        if self.filter_chain_matcher.is_some() {
            len += 1;
        }
        if self.use_original_dst.is_some() {
            len += 1;
        }
        if self.default_filter_chain.is_some() {
            len += 1;
        }
        if self.per_connection_buffer_limit_bytes.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.deprecated_v1.is_some() {
            len += 1;
        }
        if self.drain_type != 0 {
            len += 1;
        }
        if !self.listener_filters.is_empty() {
            len += 1;
        }
        if self.listener_filters_timeout.is_some() {
            len += 1;
        }
        if self.continue_on_listener_filters_timeout {
            len += 1;
        }
        if self.transparent.is_some() {
            len += 1;
        }
        if self.freebind.is_some() {
            len += 1;
        }
        if !self.socket_options.is_empty() {
            len += 1;
        }
        if self.tcp_fast_open_queue_length.is_some() {
            len += 1;
        }
        if self.traffic_direction != 0 {
            len += 1;
        }
        if self.udp_listener_config.is_some() {
            len += 1;
        }
        if self.api_listener.is_some() {
            len += 1;
        }
        if self.connection_balance_config.is_some() {
            len += 1;
        }
        if self.reuse_port {
            len += 1;
        }
        if self.enable_reuse_port.is_some() {
            len += 1;
        }
        if !self.access_log.is_empty() {
            len += 1;
        }
        if self.tcp_backlog_size.is_some() {
            len += 1;
        }
        if self.max_connections_to_accept_per_socket_event.is_some() {
            len += 1;
        }
        if self.bind_to_port.is_some() {
            len += 1;
        }
        if self.enable_mptcp {
            len += 1;
        }
        if self.ignore_global_conn_limit {
            len += 1;
        }
        if self.bypass_overload_manager {
            len += 1;
        }
        if self.listener_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.listener.v3.Listener", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if !self.additional_addresses.is_empty() {
            struct_ser.serialize_field("additional_addresses", &self.additional_addresses)?;
        }
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("stat_prefix", &self.stat_prefix)?;
        }
        if !self.filter_chains.is_empty() {
            struct_ser.serialize_field("filter_chains", &self.filter_chains)?;
        }
        if let Some(v) = self.fcds_config.as_ref() {
            struct_ser.serialize_field("fcds_config", v)?;
        }
        if let Some(v) = self.filter_chain_matcher.as_ref() {
            struct_ser.serialize_field("filter_chain_matcher", v)?;
        }
        if let Some(v) = self.use_original_dst.as_ref() {
            struct_ser.serialize_field("use_original_dst", v)?;
        }
        if let Some(v) = self.default_filter_chain.as_ref() {
            struct_ser.serialize_field("default_filter_chain", v)?;
        }
        if let Some(v) = self.per_connection_buffer_limit_bytes.as_ref() {
            struct_ser.serialize_field("per_connection_buffer_limit_bytes", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.deprecated_v1.as_ref() {
            struct_ser.serialize_field("deprecated_v1", v)?;
        }
        if self.drain_type != 0 {
            let v = listener::DrainType::try_from(self.drain_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.drain_type)))?;
            struct_ser.serialize_field("drain_type", &v)?;
        }
        if !self.listener_filters.is_empty() {
            struct_ser.serialize_field("listener_filters", &self.listener_filters)?;
        }
        if let Some(v) = self.listener_filters_timeout.as_ref() {
            struct_ser.serialize_field("listener_filters_timeout", v)?;
        }
        if self.continue_on_listener_filters_timeout {
            struct_ser.serialize_field("continue_on_listener_filters_timeout", &self.continue_on_listener_filters_timeout)?;
        }
        if let Some(v) = self.transparent.as_ref() {
            struct_ser.serialize_field("transparent", v)?;
        }
        if let Some(v) = self.freebind.as_ref() {
            struct_ser.serialize_field("freebind", v)?;
        }
        if !self.socket_options.is_empty() {
            struct_ser.serialize_field("socket_options", &self.socket_options)?;
        }
        if let Some(v) = self.tcp_fast_open_queue_length.as_ref() {
            struct_ser.serialize_field("tcp_fast_open_queue_length", v)?;
        }
        if self.traffic_direction != 0 {
            let v = super::super::core::v3::TrafficDirection::try_from(self.traffic_direction)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.traffic_direction)))?;
            struct_ser.serialize_field("traffic_direction", &v)?;
        }
        if let Some(v) = self.udp_listener_config.as_ref() {
            struct_ser.serialize_field("udp_listener_config", v)?;
        }
        if let Some(v) = self.api_listener.as_ref() {
            struct_ser.serialize_field("api_listener", v)?;
        }
        if let Some(v) = self.connection_balance_config.as_ref() {
            struct_ser.serialize_field("connection_balance_config", v)?;
        }
        if self.reuse_port {
            struct_ser.serialize_field("reuse_port", &self.reuse_port)?;
        }
        if let Some(v) = self.enable_reuse_port.as_ref() {
            struct_ser.serialize_field("enable_reuse_port", v)?;
        }
        if !self.access_log.is_empty() {
            struct_ser.serialize_field("access_log", &self.access_log)?;
        }
        if let Some(v) = self.tcp_backlog_size.as_ref() {
            struct_ser.serialize_field("tcp_backlog_size", v)?;
        }
        if let Some(v) = self.max_connections_to_accept_per_socket_event.as_ref() {
            struct_ser.serialize_field("max_connections_to_accept_per_socket_event", v)?;
        }
        if let Some(v) = self.bind_to_port.as_ref() {
            struct_ser.serialize_field("bind_to_port", v)?;
        }
        if self.enable_mptcp {
            struct_ser.serialize_field("enable_mptcp", &self.enable_mptcp)?;
        }
        if self.ignore_global_conn_limit {
            struct_ser.serialize_field("ignore_global_conn_limit", &self.ignore_global_conn_limit)?;
        }
        if self.bypass_overload_manager {
            struct_ser.serialize_field("bypass_overload_manager", &self.bypass_overload_manager)?;
        }
        if let Some(v) = self.listener_specifier.as_ref() {
            match v {
                listener::ListenerSpecifier::InternalListener(v) => {
                    struct_ser.serialize_field("internal_listener", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Listener {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "address",
            "additional_addresses",
            "additionalAddresses",
            "stat_prefix",
            "statPrefix",
            "filter_chains",
            "filterChains",
            "fcds_config",
            "fcdsConfig",
            "filter_chain_matcher",
            "filterChainMatcher",
            "use_original_dst",
            "useOriginalDst",
            "default_filter_chain",
            "defaultFilterChain",
            "per_connection_buffer_limit_bytes",
            "perConnectionBufferLimitBytes",
            "metadata",
            "deprecated_v1",
            "deprecatedV1",
            "drain_type",
            "drainType",
            "listener_filters",
            "listenerFilters",
            "listener_filters_timeout",
            "listenerFiltersTimeout",
            "continue_on_listener_filters_timeout",
            "continueOnListenerFiltersTimeout",
            "transparent",
            "freebind",
            "socket_options",
            "socketOptions",
            "tcp_fast_open_queue_length",
            "tcpFastOpenQueueLength",
            "traffic_direction",
            "trafficDirection",
            "udp_listener_config",
            "udpListenerConfig",
            "api_listener",
            "apiListener",
            "connection_balance_config",
            "connectionBalanceConfig",
            "reuse_port",
            "reusePort",
            "enable_reuse_port",
            "enableReusePort",
            "access_log",
            "accessLog",
            "tcp_backlog_size",
            "tcpBacklogSize",
            "max_connections_to_accept_per_socket_event",
            "maxConnectionsToAcceptPerSocketEvent",
            "bind_to_port",
            "bindToPort",
            "enable_mptcp",
            "enableMptcp",
            "ignore_global_conn_limit",
            "ignoreGlobalConnLimit",
            "bypass_overload_manager",
            "bypassOverloadManager",
            "internal_listener",
            "internalListener",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Address,
            AdditionalAddresses,
            StatPrefix,
            FilterChains,
            FcdsConfig,
            FilterChainMatcher,
            UseOriginalDst,
            DefaultFilterChain,
            PerConnectionBufferLimitBytes,
            Metadata,
            DeprecatedV1,
            DrainType,
            ListenerFilters,
            ListenerFiltersTimeout,
            ContinueOnListenerFiltersTimeout,
            Transparent,
            Freebind,
            SocketOptions,
            TcpFastOpenQueueLength,
            TrafficDirection,
            UdpListenerConfig,
            ApiListener,
            ConnectionBalanceConfig,
            ReusePort,
            EnableReusePort,
            AccessLog,
            TcpBacklogSize,
            MaxConnectionsToAcceptPerSocketEvent,
            BindToPort,
            EnableMptcp,
            IgnoreGlobalConnLimit,
            BypassOverloadManager,
            InternalListener,
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
                            "address" => Ok(GeneratedField::Address),
                            "additionalAddresses" | "additional_addresses" => Ok(GeneratedField::AdditionalAddresses),
                            "statPrefix" | "stat_prefix" => Ok(GeneratedField::StatPrefix),
                            "filterChains" | "filter_chains" => Ok(GeneratedField::FilterChains),
                            "fcdsConfig" | "fcds_config" => Ok(GeneratedField::FcdsConfig),
                            "filterChainMatcher" | "filter_chain_matcher" => Ok(GeneratedField::FilterChainMatcher),
                            "useOriginalDst" | "use_original_dst" => Ok(GeneratedField::UseOriginalDst),
                            "defaultFilterChain" | "default_filter_chain" => Ok(GeneratedField::DefaultFilterChain),
                            "perConnectionBufferLimitBytes" | "per_connection_buffer_limit_bytes" => Ok(GeneratedField::PerConnectionBufferLimitBytes),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "deprecatedV1" | "deprecated_v1" => Ok(GeneratedField::DeprecatedV1),
                            "drainType" | "drain_type" => Ok(GeneratedField::DrainType),
                            "listenerFilters" | "listener_filters" => Ok(GeneratedField::ListenerFilters),
                            "listenerFiltersTimeout" | "listener_filters_timeout" => Ok(GeneratedField::ListenerFiltersTimeout),
                            "continueOnListenerFiltersTimeout" | "continue_on_listener_filters_timeout" => Ok(GeneratedField::ContinueOnListenerFiltersTimeout),
                            "transparent" => Ok(GeneratedField::Transparent),
                            "freebind" => Ok(GeneratedField::Freebind),
                            "socketOptions" | "socket_options" => Ok(GeneratedField::SocketOptions),
                            "tcpFastOpenQueueLength" | "tcp_fast_open_queue_length" => Ok(GeneratedField::TcpFastOpenQueueLength),
                            "trafficDirection" | "traffic_direction" => Ok(GeneratedField::TrafficDirection),
                            "udpListenerConfig" | "udp_listener_config" => Ok(GeneratedField::UdpListenerConfig),
                            "apiListener" | "api_listener" => Ok(GeneratedField::ApiListener),
                            "connectionBalanceConfig" | "connection_balance_config" => Ok(GeneratedField::ConnectionBalanceConfig),
                            "reusePort" | "reuse_port" => Ok(GeneratedField::ReusePort),
                            "enableReusePort" | "enable_reuse_port" => Ok(GeneratedField::EnableReusePort),
                            "accessLog" | "access_log" => Ok(GeneratedField::AccessLog),
                            "tcpBacklogSize" | "tcp_backlog_size" => Ok(GeneratedField::TcpBacklogSize),
                            "maxConnectionsToAcceptPerSocketEvent" | "max_connections_to_accept_per_socket_event" => Ok(GeneratedField::MaxConnectionsToAcceptPerSocketEvent),
                            "bindToPort" | "bind_to_port" => Ok(GeneratedField::BindToPort),
                            "enableMptcp" | "enable_mptcp" => Ok(GeneratedField::EnableMptcp),
                            "ignoreGlobalConnLimit" | "ignore_global_conn_limit" => Ok(GeneratedField::IgnoreGlobalConnLimit),
                            "bypassOverloadManager" | "bypass_overload_manager" => Ok(GeneratedField::BypassOverloadManager),
                            "internalListener" | "internal_listener" => Ok(GeneratedField::InternalListener),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Listener;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.Listener")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Listener, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut address__ = None;
                let mut additional_addresses__ = None;
                let mut stat_prefix__ = None;
                let mut filter_chains__ = None;
                let mut fcds_config__ = None;
                let mut filter_chain_matcher__ = None;
                let mut use_original_dst__ = None;
                let mut default_filter_chain__ = None;
                let mut per_connection_buffer_limit_bytes__ = None;
                let mut metadata__ = None;
                let mut deprecated_v1__ = None;
                let mut drain_type__ = None;
                let mut listener_filters__ = None;
                let mut listener_filters_timeout__ = None;
                let mut continue_on_listener_filters_timeout__ = None;
                let mut transparent__ = None;
                let mut freebind__ = None;
                let mut socket_options__ = None;
                let mut tcp_fast_open_queue_length__ = None;
                let mut traffic_direction__ = None;
                let mut udp_listener_config__ = None;
                let mut api_listener__ = None;
                let mut connection_balance_config__ = None;
                let mut reuse_port__ = None;
                let mut enable_reuse_port__ = None;
                let mut access_log__ = None;
                let mut tcp_backlog_size__ = None;
                let mut max_connections_to_accept_per_socket_event__ = None;
                let mut bind_to_port__ = None;
                let mut enable_mptcp__ = None;
                let mut ignore_global_conn_limit__ = None;
                let mut bypass_overload_manager__ = None;
                let mut listener_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                        GeneratedField::AdditionalAddresses => {
                            if additional_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalAddresses"));
                            }
                            additional_addresses__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FilterChains => {
                            if filter_chains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterChains"));
                            }
                            filter_chains__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FcdsConfig => {
                            if fcds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fcdsConfig"));
                            }
                            fcds_config__ = map_.next_value()?;
                        }
                        GeneratedField::FilterChainMatcher => {
                            if filter_chain_matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterChainMatcher"));
                            }
                            filter_chain_matcher__ = map_.next_value()?;
                        }
                        GeneratedField::UseOriginalDst => {
                            if use_original_dst__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useOriginalDst"));
                            }
                            use_original_dst__ = map_.next_value()?;
                        }
                        GeneratedField::DefaultFilterChain => {
                            if default_filter_chain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultFilterChain"));
                            }
                            default_filter_chain__ = map_.next_value()?;
                        }
                        GeneratedField::PerConnectionBufferLimitBytes => {
                            if per_connection_buffer_limit_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perConnectionBufferLimitBytes"));
                            }
                            per_connection_buffer_limit_bytes__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::DeprecatedV1 => {
                            if deprecated_v1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deprecatedV1"));
                            }
                            deprecated_v1__ = map_.next_value()?;
                        }
                        GeneratedField::DrainType => {
                            if drain_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drainType"));
                            }
                            drain_type__ = Some(map_.next_value::<listener::DrainType>()? as i32);
                        }
                        GeneratedField::ListenerFilters => {
                            if listener_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listenerFilters"));
                            }
                            listener_filters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ListenerFiltersTimeout => {
                            if listener_filters_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listenerFiltersTimeout"));
                            }
                            listener_filters_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::ContinueOnListenerFiltersTimeout => {
                            if continue_on_listener_filters_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("continueOnListenerFiltersTimeout"));
                            }
                            continue_on_listener_filters_timeout__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Transparent => {
                            if transparent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transparent"));
                            }
                            transparent__ = map_.next_value()?;
                        }
                        GeneratedField::Freebind => {
                            if freebind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("freebind"));
                            }
                            freebind__ = map_.next_value()?;
                        }
                        GeneratedField::SocketOptions => {
                            if socket_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("socketOptions"));
                            }
                            socket_options__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TcpFastOpenQueueLength => {
                            if tcp_fast_open_queue_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tcpFastOpenQueueLength"));
                            }
                            tcp_fast_open_queue_length__ = map_.next_value()?;
                        }
                        GeneratedField::TrafficDirection => {
                            if traffic_direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trafficDirection"));
                            }
                            traffic_direction__ = Some(map_.next_value::<super::super::core::v3::TrafficDirection>()? as i32);
                        }
                        GeneratedField::UdpListenerConfig => {
                            if udp_listener_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("udpListenerConfig"));
                            }
                            udp_listener_config__ = map_.next_value()?;
                        }
                        GeneratedField::ApiListener => {
                            if api_listener__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiListener"));
                            }
                            api_listener__ = map_.next_value()?;
                        }
                        GeneratedField::ConnectionBalanceConfig => {
                            if connection_balance_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionBalanceConfig"));
                            }
                            connection_balance_config__ = map_.next_value()?;
                        }
                        GeneratedField::ReusePort => {
                            if reuse_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reusePort"));
                            }
                            reuse_port__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EnableReusePort => {
                            if enable_reuse_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableReusePort"));
                            }
                            enable_reuse_port__ = map_.next_value()?;
                        }
                        GeneratedField::AccessLog => {
                            if access_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessLog"));
                            }
                            access_log__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TcpBacklogSize => {
                            if tcp_backlog_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tcpBacklogSize"));
                            }
                            tcp_backlog_size__ = map_.next_value()?;
                        }
                        GeneratedField::MaxConnectionsToAcceptPerSocketEvent => {
                            if max_connections_to_accept_per_socket_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxConnectionsToAcceptPerSocketEvent"));
                            }
                            max_connections_to_accept_per_socket_event__ = map_.next_value()?;
                        }
                        GeneratedField::BindToPort => {
                            if bind_to_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bindToPort"));
                            }
                            bind_to_port__ = map_.next_value()?;
                        }
                        GeneratedField::EnableMptcp => {
                            if enable_mptcp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableMptcp"));
                            }
                            enable_mptcp__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IgnoreGlobalConnLimit => {
                            if ignore_global_conn_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreGlobalConnLimit"));
                            }
                            ignore_global_conn_limit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BypassOverloadManager => {
                            if bypass_overload_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bypassOverloadManager"));
                            }
                            bypass_overload_manager__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InternalListener => {
                            if listener_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("internalListener"));
                            }
                            listener_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(listener::ListenerSpecifier::InternalListener)
;
                        }
                    }
                }
                Ok(Listener {
                    name: name__.unwrap_or_default(),
                    address: address__,
                    additional_addresses: additional_addresses__.unwrap_or_default(),
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    filter_chains: filter_chains__.unwrap_or_default(),
                    fcds_config: fcds_config__,
                    filter_chain_matcher: filter_chain_matcher__,
                    use_original_dst: use_original_dst__,
                    default_filter_chain: default_filter_chain__,
                    per_connection_buffer_limit_bytes: per_connection_buffer_limit_bytes__,
                    metadata: metadata__,
                    deprecated_v1: deprecated_v1__,
                    drain_type: drain_type__.unwrap_or_default(),
                    listener_filters: listener_filters__.unwrap_or_default(),
                    listener_filters_timeout: listener_filters_timeout__,
                    continue_on_listener_filters_timeout: continue_on_listener_filters_timeout__.unwrap_or_default(),
                    transparent: transparent__,
                    freebind: freebind__,
                    socket_options: socket_options__.unwrap_or_default(),
                    tcp_fast_open_queue_length: tcp_fast_open_queue_length__,
                    traffic_direction: traffic_direction__.unwrap_or_default(),
                    udp_listener_config: udp_listener_config__,
                    api_listener: api_listener__,
                    connection_balance_config: connection_balance_config__,
                    reuse_port: reuse_port__.unwrap_or_default(),
                    enable_reuse_port: enable_reuse_port__,
                    access_log: access_log__.unwrap_or_default(),
                    tcp_backlog_size: tcp_backlog_size__,
                    max_connections_to_accept_per_socket_event: max_connections_to_accept_per_socket_event__,
                    bind_to_port: bind_to_port__,
                    enable_mptcp: enable_mptcp__.unwrap_or_default(),
                    ignore_global_conn_limit: ignore_global_conn_limit__.unwrap_or_default(),
                    bypass_overload_manager: bypass_overload_manager__.unwrap_or_default(),
                    listener_specifier: listener_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.Listener", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for listener::ConnectionBalanceConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.balance_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.listener.v3.Listener.ConnectionBalanceConfig", len)?;
        if let Some(v) = self.balance_type.as_ref() {
            match v {
                listener::connection_balance_config::BalanceType::ExactBalance(v) => {
                    struct_ser.serialize_field("exact_balance", v)?;
                }
                listener::connection_balance_config::BalanceType::ExtendBalance(v) => {
                    struct_ser.serialize_field("extend_balance", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for listener::ConnectionBalanceConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "exact_balance",
            "exactBalance",
            "extend_balance",
            "extendBalance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExactBalance,
            ExtendBalance,
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
                            "exactBalance" | "exact_balance" => Ok(GeneratedField::ExactBalance),
                            "extendBalance" | "extend_balance" => Ok(GeneratedField::ExtendBalance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = listener::ConnectionBalanceConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.Listener.ConnectionBalanceConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<listener::ConnectionBalanceConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut balance_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExactBalance => {
                            if balance_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exactBalance"));
                            }
                            balance_type__ = map_.next_value::<::std::option::Option<_>>()?.map(listener::connection_balance_config::BalanceType::ExactBalance)
;
                        }
                        GeneratedField::ExtendBalance => {
                            if balance_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extendBalance"));
                            }
                            balance_type__ = map_.next_value::<::std::option::Option<_>>()?.map(listener::connection_balance_config::BalanceType::ExtendBalance)
;
                        }
                    }
                }
                Ok(listener::ConnectionBalanceConfig {
                    balance_type: balance_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.Listener.ConnectionBalanceConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for listener::connection_balance_config::ExactBalance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.listener.v3.Listener.ConnectionBalanceConfig.ExactBalance", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for listener::connection_balance_config::ExactBalance {
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
            type Value = listener::connection_balance_config::ExactBalance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.Listener.ConnectionBalanceConfig.ExactBalance")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<listener::connection_balance_config::ExactBalance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(listener::connection_balance_config::ExactBalance {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.Listener.ConnectionBalanceConfig.ExactBalance", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for listener::DeprecatedV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bind_to_port.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.listener.v3.Listener.DeprecatedV1", len)?;
        if let Some(v) = self.bind_to_port.as_ref() {
            struct_ser.serialize_field("bind_to_port", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for listener::DeprecatedV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bind_to_port",
            "bindToPort",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BindToPort,
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
                            "bindToPort" | "bind_to_port" => Ok(GeneratedField::BindToPort),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = listener::DeprecatedV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.Listener.DeprecatedV1")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<listener::DeprecatedV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bind_to_port__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BindToPort => {
                            if bind_to_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bindToPort"));
                            }
                            bind_to_port__ = map_.next_value()?;
                        }
                    }
                }
                Ok(listener::DeprecatedV1 {
                    bind_to_port: bind_to_port__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.Listener.DeprecatedV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for listener::DrainType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Default => "DEFAULT",
            Self::ModifyOnly => "MODIFY_ONLY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for listener::DrainType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEFAULT",
            "MODIFY_ONLY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = listener::DrainType;

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
                    "DEFAULT" => Ok(listener::DrainType::Default),
                    "MODIFY_ONLY" => Ok(listener::DrainType::ModifyOnly),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for listener::FcdsConfig {
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
        if self.config_source.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.listener.v3.Listener.FcdsConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_source.as_ref() {
            struct_ser.serialize_field("config_source", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for listener::FcdsConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "config_source",
            "configSource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ConfigSource,
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
                            "configSource" | "config_source" => Ok(GeneratedField::ConfigSource),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = listener::FcdsConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.Listener.FcdsConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<listener::FcdsConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut config_source__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConfigSource => {
                            if config_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configSource"));
                            }
                            config_source__ = map_.next_value()?;
                        }
                    }
                }
                Ok(listener::FcdsConfig {
                    name: name__.unwrap_or_default(),
                    config_source: config_source__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.Listener.FcdsConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for listener::InternalListenerConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.listener.v3.Listener.InternalListenerConfig", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for listener::InternalListenerConfig {
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
            type Value = listener::InternalListenerConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.Listener.InternalListenerConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<listener::InternalListenerConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(listener::InternalListenerConfig {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.Listener.InternalListenerConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListenerCollection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.listener.v3.ListenerCollection", len)?;
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListenerCollection {
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
            type Value = ListenerCollection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.ListenerCollection")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListenerCollection, V::Error>
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
                            entries__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListenerCollection {
                    entries: entries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.ListenerCollection", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListenerFilter {
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
        if self.filter_disabled.is_some() {
            len += 1;
        }
        if self.config_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.listener.v3.ListenerFilter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.filter_disabled.as_ref() {
            struct_ser.serialize_field("filter_disabled", v)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                listener_filter::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typed_config", v)?;
                }
                listener_filter::ConfigType::ConfigDiscovery(v) => {
                    struct_ser.serialize_field("config_discovery", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListenerFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "filter_disabled",
            "filterDisabled",
            "typed_config",
            "typedConfig",
            "config_discovery",
            "configDiscovery",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            FilterDisabled,
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
                            "filterDisabled" | "filter_disabled" => Ok(GeneratedField::FilterDisabled),
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
            type Value = ListenerFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.ListenerFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListenerFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut filter_disabled__ = None;
                let mut config_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FilterDisabled => {
                            if filter_disabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterDisabled"));
                            }
                            filter_disabled__ = map_.next_value()?;
                        }
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map_.next_value::<::std::option::Option<_>>()?.map(listener_filter::ConfigType::TypedConfig)
;
                        }
                        GeneratedField::ConfigDiscovery => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configDiscovery"));
                            }
                            config_type__ = map_.next_value::<::std::option::Option<_>>()?.map(listener_filter::ConfigType::ConfigDiscovery)
;
                        }
                    }
                }
                Ok(ListenerFilter {
                    name: name__.unwrap_or_default(),
                    filter_disabled: filter_disabled__,
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.ListenerFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListenerFilterChainMatchPredicate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.listener.v3.ListenerFilterChainMatchPredicate", len)?;
        if let Some(v) = self.rule.as_ref() {
            match v {
                listener_filter_chain_match_predicate::Rule::OrMatch(v) => {
                    struct_ser.serialize_field("or_match", v)?;
                }
                listener_filter_chain_match_predicate::Rule::AndMatch(v) => {
                    struct_ser.serialize_field("and_match", v)?;
                }
                listener_filter_chain_match_predicate::Rule::NotMatch(v) => {
                    struct_ser.serialize_field("not_match", v)?;
                }
                listener_filter_chain_match_predicate::Rule::AnyMatch(v) => {
                    struct_ser.serialize_field("any_match", v)?;
                }
                listener_filter_chain_match_predicate::Rule::DestinationPortRange(v) => {
                    struct_ser.serialize_field("destination_port_range", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListenerFilterChainMatchPredicate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "or_match",
            "orMatch",
            "and_match",
            "andMatch",
            "not_match",
            "notMatch",
            "any_match",
            "anyMatch",
            "destination_port_range",
            "destinationPortRange",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrMatch,
            AndMatch,
            NotMatch,
            AnyMatch,
            DestinationPortRange,
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
                            "orMatch" | "or_match" => Ok(GeneratedField::OrMatch),
                            "andMatch" | "and_match" => Ok(GeneratedField::AndMatch),
                            "notMatch" | "not_match" => Ok(GeneratedField::NotMatch),
                            "anyMatch" | "any_match" => Ok(GeneratedField::AnyMatch),
                            "destinationPortRange" | "destination_port_range" => Ok(GeneratedField::DestinationPortRange),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListenerFilterChainMatchPredicate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.ListenerFilterChainMatchPredicate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListenerFilterChainMatchPredicate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrMatch => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orMatch"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(listener_filter_chain_match_predicate::Rule::OrMatch)
;
                        }
                        GeneratedField::AndMatch => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("andMatch"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(listener_filter_chain_match_predicate::Rule::AndMatch)
;
                        }
                        GeneratedField::NotMatch => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notMatch"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(listener_filter_chain_match_predicate::Rule::NotMatch)
;
                        }
                        GeneratedField::AnyMatch => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("anyMatch"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(listener_filter_chain_match_predicate::Rule::AnyMatch);
                        }
                        GeneratedField::DestinationPortRange => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationPortRange"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(listener_filter_chain_match_predicate::Rule::DestinationPortRange)
;
                        }
                    }
                }
                Ok(ListenerFilterChainMatchPredicate {
                    rule: rule__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.ListenerFilterChainMatchPredicate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for listener_filter_chain_match_predicate::MatchSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.listener.v3.ListenerFilterChainMatchPredicate.MatchSet", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for listener_filter_chain_match_predicate::MatchSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
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
                            "rules" => Ok(GeneratedField::Rules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = listener_filter_chain_match_predicate::MatchSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.ListenerFilterChainMatchPredicate.MatchSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<listener_filter_chain_match_predicate::MatchSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(listener_filter_chain_match_predicate::MatchSet {
                    rules: rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.ListenerFilterChainMatchPredicate.MatchSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListenerManager {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.listener.v3.ListenerManager", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListenerManager {
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
            type Value = ListenerManager;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.ListenerManager")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListenerManager, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListenerManager {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.ListenerManager", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuicProtocolOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.quic_protocol_options.is_some() {
            len += 1;
        }
        if self.idle_timeout.is_some() {
            len += 1;
        }
        if self.crypto_handshake_timeout.is_some() {
            len += 1;
        }
        if self.enabled.is_some() {
            len += 1;
        }
        if self.packets_to_read_to_connection_count_ratio.is_some() {
            len += 1;
        }
        if self.crypto_stream_config.is_some() {
            len += 1;
        }
        if self.proof_source_config.is_some() {
            len += 1;
        }
        if self.connection_id_generator_config.is_some() {
            len += 1;
        }
        if self.server_preferred_address_config.is_some() {
            len += 1;
        }
        if self.send_disable_active_migration.is_some() {
            len += 1;
        }
        if self.connection_debug_visitor_config.is_some() {
            len += 1;
        }
        if !self.save_cmsg_config.is_empty() {
            len += 1;
        }
        if self.reject_new_connections {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.listener.v3.QuicProtocolOptions", len)?;
        if let Some(v) = self.quic_protocol_options.as_ref() {
            struct_ser.serialize_field("quic_protocol_options", v)?;
        }
        if let Some(v) = self.idle_timeout.as_ref() {
            struct_ser.serialize_field("idle_timeout", v)?;
        }
        if let Some(v) = self.crypto_handshake_timeout.as_ref() {
            struct_ser.serialize_field("crypto_handshake_timeout", v)?;
        }
        if let Some(v) = self.enabled.as_ref() {
            struct_ser.serialize_field("enabled", v)?;
        }
        if let Some(v) = self.packets_to_read_to_connection_count_ratio.as_ref() {
            struct_ser.serialize_field("packets_to_read_to_connection_count_ratio", v)?;
        }
        if let Some(v) = self.crypto_stream_config.as_ref() {
            struct_ser.serialize_field("crypto_stream_config", v)?;
        }
        if let Some(v) = self.proof_source_config.as_ref() {
            struct_ser.serialize_field("proof_source_config", v)?;
        }
        if let Some(v) = self.connection_id_generator_config.as_ref() {
            struct_ser.serialize_field("connection_id_generator_config", v)?;
        }
        if let Some(v) = self.server_preferred_address_config.as_ref() {
            struct_ser.serialize_field("server_preferred_address_config", v)?;
        }
        if let Some(v) = self.send_disable_active_migration.as_ref() {
            struct_ser.serialize_field("send_disable_active_migration", v)?;
        }
        if let Some(v) = self.connection_debug_visitor_config.as_ref() {
            struct_ser.serialize_field("connection_debug_visitor_config", v)?;
        }
        if !self.save_cmsg_config.is_empty() {
            struct_ser.serialize_field("save_cmsg_config", &self.save_cmsg_config)?;
        }
        if self.reject_new_connections {
            struct_ser.serialize_field("reject_new_connections", &self.reject_new_connections)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuicProtocolOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "quic_protocol_options",
            "quicProtocolOptions",
            "idle_timeout",
            "idleTimeout",
            "crypto_handshake_timeout",
            "cryptoHandshakeTimeout",
            "enabled",
            "packets_to_read_to_connection_count_ratio",
            "packetsToReadToConnectionCountRatio",
            "crypto_stream_config",
            "cryptoStreamConfig",
            "proof_source_config",
            "proofSourceConfig",
            "connection_id_generator_config",
            "connectionIdGeneratorConfig",
            "server_preferred_address_config",
            "serverPreferredAddressConfig",
            "send_disable_active_migration",
            "sendDisableActiveMigration",
            "connection_debug_visitor_config",
            "connectionDebugVisitorConfig",
            "save_cmsg_config",
            "saveCmsgConfig",
            "reject_new_connections",
            "rejectNewConnections",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QuicProtocolOptions,
            IdleTimeout,
            CryptoHandshakeTimeout,
            Enabled,
            PacketsToReadToConnectionCountRatio,
            CryptoStreamConfig,
            ProofSourceConfig,
            ConnectionIdGeneratorConfig,
            ServerPreferredAddressConfig,
            SendDisableActiveMigration,
            ConnectionDebugVisitorConfig,
            SaveCmsgConfig,
            RejectNewConnections,
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
                            "quicProtocolOptions" | "quic_protocol_options" => Ok(GeneratedField::QuicProtocolOptions),
                            "idleTimeout" | "idle_timeout" => Ok(GeneratedField::IdleTimeout),
                            "cryptoHandshakeTimeout" | "crypto_handshake_timeout" => Ok(GeneratedField::CryptoHandshakeTimeout),
                            "enabled" => Ok(GeneratedField::Enabled),
                            "packetsToReadToConnectionCountRatio" | "packets_to_read_to_connection_count_ratio" => Ok(GeneratedField::PacketsToReadToConnectionCountRatio),
                            "cryptoStreamConfig" | "crypto_stream_config" => Ok(GeneratedField::CryptoStreamConfig),
                            "proofSourceConfig" | "proof_source_config" => Ok(GeneratedField::ProofSourceConfig),
                            "connectionIdGeneratorConfig" | "connection_id_generator_config" => Ok(GeneratedField::ConnectionIdGeneratorConfig),
                            "serverPreferredAddressConfig" | "server_preferred_address_config" => Ok(GeneratedField::ServerPreferredAddressConfig),
                            "sendDisableActiveMigration" | "send_disable_active_migration" => Ok(GeneratedField::SendDisableActiveMigration),
                            "connectionDebugVisitorConfig" | "connection_debug_visitor_config" => Ok(GeneratedField::ConnectionDebugVisitorConfig),
                            "saveCmsgConfig" | "save_cmsg_config" => Ok(GeneratedField::SaveCmsgConfig),
                            "rejectNewConnections" | "reject_new_connections" => Ok(GeneratedField::RejectNewConnections),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuicProtocolOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.QuicProtocolOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuicProtocolOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut quic_protocol_options__ = None;
                let mut idle_timeout__ = None;
                let mut crypto_handshake_timeout__ = None;
                let mut enabled__ = None;
                let mut packets_to_read_to_connection_count_ratio__ = None;
                let mut crypto_stream_config__ = None;
                let mut proof_source_config__ = None;
                let mut connection_id_generator_config__ = None;
                let mut server_preferred_address_config__ = None;
                let mut send_disable_active_migration__ = None;
                let mut connection_debug_visitor_config__ = None;
                let mut save_cmsg_config__ = None;
                let mut reject_new_connections__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QuicProtocolOptions => {
                            if quic_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quicProtocolOptions"));
                            }
                            quic_protocol_options__ = map_.next_value()?;
                        }
                        GeneratedField::IdleTimeout => {
                            if idle_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idleTimeout"));
                            }
                            idle_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::CryptoHandshakeTimeout => {
                            if crypto_handshake_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cryptoHandshakeTimeout"));
                            }
                            crypto_handshake_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = map_.next_value()?;
                        }
                        GeneratedField::PacketsToReadToConnectionCountRatio => {
                            if packets_to_read_to_connection_count_ratio__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packetsToReadToConnectionCountRatio"));
                            }
                            packets_to_read_to_connection_count_ratio__ = map_.next_value()?;
                        }
                        GeneratedField::CryptoStreamConfig => {
                            if crypto_stream_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cryptoStreamConfig"));
                            }
                            crypto_stream_config__ = map_.next_value()?;
                        }
                        GeneratedField::ProofSourceConfig => {
                            if proof_source_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proofSourceConfig"));
                            }
                            proof_source_config__ = map_.next_value()?;
                        }
                        GeneratedField::ConnectionIdGeneratorConfig => {
                            if connection_id_generator_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionIdGeneratorConfig"));
                            }
                            connection_id_generator_config__ = map_.next_value()?;
                        }
                        GeneratedField::ServerPreferredAddressConfig => {
                            if server_preferred_address_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverPreferredAddressConfig"));
                            }
                            server_preferred_address_config__ = map_.next_value()?;
                        }
                        GeneratedField::SendDisableActiveMigration => {
                            if send_disable_active_migration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sendDisableActiveMigration"));
                            }
                            send_disable_active_migration__ = map_.next_value()?;
                        }
                        GeneratedField::ConnectionDebugVisitorConfig => {
                            if connection_debug_visitor_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionDebugVisitorConfig"));
                            }
                            connection_debug_visitor_config__ = map_.next_value()?;
                        }
                        GeneratedField::SaveCmsgConfig => {
                            if save_cmsg_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("saveCmsgConfig"));
                            }
                            save_cmsg_config__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RejectNewConnections => {
                            if reject_new_connections__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rejectNewConnections"));
                            }
                            reject_new_connections__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QuicProtocolOptions {
                    quic_protocol_options: quic_protocol_options__,
                    idle_timeout: idle_timeout__,
                    crypto_handshake_timeout: crypto_handshake_timeout__,
                    enabled: enabled__,
                    packets_to_read_to_connection_count_ratio: packets_to_read_to_connection_count_ratio__,
                    crypto_stream_config: crypto_stream_config__,
                    proof_source_config: proof_source_config__,
                    connection_id_generator_config: connection_id_generator_config__,
                    server_preferred_address_config: server_preferred_address_config__,
                    send_disable_active_migration: send_disable_active_migration__,
                    connection_debug_visitor_config: connection_debug_visitor_config__,
                    save_cmsg_config: save_cmsg_config__.unwrap_or_default(),
                    reject_new_connections: reject_new_connections__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.QuicProtocolOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UdpListenerConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.downstream_socket_config.is_some() {
            len += 1;
        }
        if self.quic_options.is_some() {
            len += 1;
        }
        if self.udp_packet_packet_writer_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.listener.v3.UdpListenerConfig", len)?;
        if let Some(v) = self.downstream_socket_config.as_ref() {
            struct_ser.serialize_field("downstream_socket_config", v)?;
        }
        if let Some(v) = self.quic_options.as_ref() {
            struct_ser.serialize_field("quic_options", v)?;
        }
        if let Some(v) = self.udp_packet_packet_writer_config.as_ref() {
            struct_ser.serialize_field("udp_packet_packet_writer_config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UdpListenerConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "downstream_socket_config",
            "downstreamSocketConfig",
            "quic_options",
            "quicOptions",
            "udp_packet_packet_writer_config",
            "udpPacketPacketWriterConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DownstreamSocketConfig,
            QuicOptions,
            UdpPacketPacketWriterConfig,
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
                            "downstreamSocketConfig" | "downstream_socket_config" => Ok(GeneratedField::DownstreamSocketConfig),
                            "quicOptions" | "quic_options" => Ok(GeneratedField::QuicOptions),
                            "udpPacketPacketWriterConfig" | "udp_packet_packet_writer_config" => Ok(GeneratedField::UdpPacketPacketWriterConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UdpListenerConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.UdpListenerConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UdpListenerConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut downstream_socket_config__ = None;
                let mut quic_options__ = None;
                let mut udp_packet_packet_writer_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DownstreamSocketConfig => {
                            if downstream_socket_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downstreamSocketConfig"));
                            }
                            downstream_socket_config__ = map_.next_value()?;
                        }
                        GeneratedField::QuicOptions => {
                            if quic_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quicOptions"));
                            }
                            quic_options__ = map_.next_value()?;
                        }
                        GeneratedField::UdpPacketPacketWriterConfig => {
                            if udp_packet_packet_writer_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("udpPacketPacketWriterConfig"));
                            }
                            udp_packet_packet_writer_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UdpListenerConfig {
                    downstream_socket_config: downstream_socket_config__,
                    quic_options: quic_options__,
                    udp_packet_packet_writer_config: udp_packet_packet_writer_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.UdpListenerConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidationListenerManager {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.listener.v3.ValidationListenerManager", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidationListenerManager {
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
            type Value = ValidationListenerManager;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v3.ValidationListenerManager")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidationListenerManager, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ValidationListenerManager {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v3.ValidationListenerManager", FIELDS, GeneratedVisitor)
    }
}
