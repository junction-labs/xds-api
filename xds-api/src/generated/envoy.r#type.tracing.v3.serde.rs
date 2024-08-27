impl serde::Serialize for CustomTag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tag.is_empty() {
            len += 1;
        }
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.tracing.v3.CustomTag", len)?;
        if !self.tag.is_empty() {
            struct_ser.serialize_field("tag", &self.tag)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            match v {
                custom_tag::Type::Literal(v) => {
                    struct_ser.serialize_field("literal", v)?;
                }
                custom_tag::Type::Environment(v) => {
                    struct_ser.serialize_field("environment", v)?;
                }
                custom_tag::Type::RequestHeader(v) => {
                    struct_ser.serialize_field("request_header", v)?;
                }
                custom_tag::Type::Metadata(v) => {
                    struct_ser.serialize_field("metadata", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CustomTag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tag",
            "literal",
            "environment",
            "request_header",
            "requestHeader",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tag,
            Literal,
            Environment,
            RequestHeader,
            Metadata,
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
                            "tag" => Ok(GeneratedField::Tag),
                            "literal" => Ok(GeneratedField::Literal),
                            "environment" => Ok(GeneratedField::Environment),
                            "requestHeader" | "request_header" => Ok(GeneratedField::RequestHeader),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CustomTag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.tracing.v3.CustomTag")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CustomTag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tag__ = None;
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tag => {
                            if tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            tag__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Literal => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("literal"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(custom_tag::Type::Literal)
;
                        }
                        GeneratedField::Environment => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("environment"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(custom_tag::Type::Environment)
;
                        }
                        GeneratedField::RequestHeader => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeader"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(custom_tag::Type::RequestHeader)
;
                        }
                        GeneratedField::Metadata => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(custom_tag::Type::Metadata)
;
                        }
                    }
                }
                Ok(CustomTag {
                    tag: tag__.unwrap_or_default(),
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.tracing.v3.CustomTag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for custom_tag::Environment {
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
        if !self.default_value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.tracing.v3.CustomTag.Environment", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.default_value.is_empty() {
            struct_ser.serialize_field("default_value", &self.default_value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for custom_tag::Environment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "default_value",
            "defaultValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            DefaultValue,
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
                            "defaultValue" | "default_value" => Ok(GeneratedField::DefaultValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = custom_tag::Environment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.tracing.v3.CustomTag.Environment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<custom_tag::Environment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut default_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefaultValue => {
                            if default_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValue"));
                            }
                            default_value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(custom_tag::Environment {
                    name: name__.unwrap_or_default(),
                    default_value: default_value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.tracing.v3.CustomTag.Environment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for custom_tag::Header {
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
        if !self.default_value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.tracing.v3.CustomTag.Header", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.default_value.is_empty() {
            struct_ser.serialize_field("default_value", &self.default_value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for custom_tag::Header {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "default_value",
            "defaultValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            DefaultValue,
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
                            "defaultValue" | "default_value" => Ok(GeneratedField::DefaultValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = custom_tag::Header;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.tracing.v3.CustomTag.Header")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<custom_tag::Header, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut default_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefaultValue => {
                            if default_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValue"));
                            }
                            default_value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(custom_tag::Header {
                    name: name__.unwrap_or_default(),
                    default_value: default_value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.tracing.v3.CustomTag.Header", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for custom_tag::Literal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.tracing.v3.CustomTag.Literal", len)?;
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for custom_tag::Literal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
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
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = custom_tag::Literal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.tracing.v3.CustomTag.Literal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<custom_tag::Literal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(custom_tag::Literal {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.tracing.v3.CustomTag.Literal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for custom_tag::Metadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.kind.is_some() {
            len += 1;
        }
        if self.metadata_key.is_some() {
            len += 1;
        }
        if !self.default_value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.tracing.v3.CustomTag.Metadata", len)?;
        if let Some(v) = self.kind.as_ref() {
            struct_ser.serialize_field("kind", v)?;
        }
        if let Some(v) = self.metadata_key.as_ref() {
            struct_ser.serialize_field("metadata_key", v)?;
        }
        if !self.default_value.is_empty() {
            struct_ser.serialize_field("default_value", &self.default_value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for custom_tag::Metadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "metadata_key",
            "metadataKey",
            "default_value",
            "defaultValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            MetadataKey,
            DefaultValue,
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
                            "kind" => Ok(GeneratedField::Kind),
                            "metadataKey" | "metadata_key" => Ok(GeneratedField::MetadataKey),
                            "defaultValue" | "default_value" => Ok(GeneratedField::DefaultValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = custom_tag::Metadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.tracing.v3.CustomTag.Metadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<custom_tag::Metadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut metadata_key__ = None;
                let mut default_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = map_.next_value()?;
                        }
                        GeneratedField::MetadataKey => {
                            if metadata_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataKey"));
                            }
                            metadata_key__ = map_.next_value()?;
                        }
                        GeneratedField::DefaultValue => {
                            if default_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValue"));
                            }
                            default_value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(custom_tag::Metadata {
                    kind: kind__,
                    metadata_key: metadata_key__,
                    default_value: default_value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.tracing.v3.CustomTag.Metadata", FIELDS, GeneratedVisitor)
    }
}
