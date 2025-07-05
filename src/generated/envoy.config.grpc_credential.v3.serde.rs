impl serde::Serialize for FileBasedMetadataConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.secret_data.is_some() {
            len += 1;
        }
        if !self.header_key.is_empty() {
            len += 1;
        }
        if !self.header_prefix.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.grpc_credential.v3.FileBasedMetadataConfig", len)?;
        if let Some(v) = self.secret_data.as_ref() {
            struct_ser.serialize_field("secret_data", v)?;
        }
        if !self.header_key.is_empty() {
            struct_ser.serialize_field("header_key", &self.header_key)?;
        }
        if !self.header_prefix.is_empty() {
            struct_ser.serialize_field("header_prefix", &self.header_prefix)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileBasedMetadataConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "secret_data",
            "secretData",
            "header_key",
            "headerKey",
            "header_prefix",
            "headerPrefix",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SecretData,
            HeaderKey,
            HeaderPrefix,
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
                            "secretData" | "secret_data" => Ok(GeneratedField::SecretData),
                            "headerKey" | "header_key" => Ok(GeneratedField::HeaderKey),
                            "headerPrefix" | "header_prefix" => Ok(GeneratedField::HeaderPrefix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileBasedMetadataConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.grpc_credential.v3.FileBasedMetadataConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FileBasedMetadataConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut secret_data__ = None;
                let mut header_key__ = None;
                let mut header_prefix__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SecretData => {
                            if secret_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secretData"));
                            }
                            secret_data__ = map_.next_value()?;
                        }
                        GeneratedField::HeaderKey => {
                            if header_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerKey"));
                            }
                            header_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HeaderPrefix => {
                            if header_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerPrefix"));
                            }
                            header_prefix__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FileBasedMetadataConfig {
                    secret_data: secret_data__,
                    header_key: header_key__.unwrap_or_default(),
                    header_prefix: header_prefix__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.grpc_credential.v3.FileBasedMetadataConfig", FIELDS, GeneratedVisitor)
    }
}
