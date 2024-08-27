impl serde::Serialize for RateLimitServiceConfig {
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
        if self.transport_api_version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.ratelimit.v3.RateLimitServiceConfig", len)?;
        if let Some(v) = self.grpc_service.as_ref() {
            struct_ser.serialize_field("grpc_service", v)?;
        }
        if self.transport_api_version != 0 {
            let v = super::super::core::v3::ApiVersion::try_from(self.transport_api_version)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.transport_api_version)))?;
            struct_ser.serialize_field("transport_api_version", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimitServiceConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "grpc_service",
            "grpcService",
            "transport_api_version",
            "transportApiVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GrpcService,
            TransportApiVersion,
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
                            "transportApiVersion" | "transport_api_version" => Ok(GeneratedField::TransportApiVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimitServiceConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.ratelimit.v3.RateLimitServiceConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RateLimitServiceConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut grpc_service__ = None;
                let mut transport_api_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GrpcService => {
                            if grpc_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcService"));
                            }
                            grpc_service__ = map_.next_value()?;
                        }
                        GeneratedField::TransportApiVersion => {
                            if transport_api_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportApiVersion"));
                            }
                            transport_api_version__ = Some(map_.next_value::<super::super::core::v3::ApiVersion>()? as i32);
                        }
                    }
                }
                Ok(RateLimitServiceConfig {
                    grpc_service: grpc_service__,
                    transport_api_version: transport_api_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.ratelimit.v3.RateLimitServiceConfig", FIELDS, GeneratedVisitor)
    }
}
