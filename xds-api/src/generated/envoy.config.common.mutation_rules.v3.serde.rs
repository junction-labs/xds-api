impl serde::Serialize for HeaderMutation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.common.mutation_rules.v3.HeaderMutation", len)?;
        if let Some(v) = self.action.as_ref() {
            match v {
                header_mutation::Action::Remove(v) => {
                    struct_ser.serialize_field("remove", v)?;
                }
                header_mutation::Action::Append(v) => {
                    struct_ser.serialize_field("append", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeaderMutation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remove",
            "append",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Remove,
            Append,
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
                            "remove" => Ok(GeneratedField::Remove),
                            "append" => Ok(GeneratedField::Append),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeaderMutation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.common.mutation_rules.v3.HeaderMutation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HeaderMutation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Remove => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remove"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(header_mutation::Action::Remove);
                        }
                        GeneratedField::Append => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("append"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(header_mutation::Action::Append)
;
                        }
                    }
                }
                Ok(HeaderMutation {
                    action: action__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.common.mutation_rules.v3.HeaderMutation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HeaderMutationRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.allow_all_routing.is_some() {
            len += 1;
        }
        if self.allow_envoy.is_some() {
            len += 1;
        }
        if self.disallow_system.is_some() {
            len += 1;
        }
        if self.disallow_all.is_some() {
            len += 1;
        }
        if self.allow_expression.is_some() {
            len += 1;
        }
        if self.disallow_expression.is_some() {
            len += 1;
        }
        if self.disallow_is_error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.common.mutation_rules.v3.HeaderMutationRules", len)?;
        if let Some(v) = self.allow_all_routing.as_ref() {
            struct_ser.serialize_field("allow_all_routing", v)?;
        }
        if let Some(v) = self.allow_envoy.as_ref() {
            struct_ser.serialize_field("allow_envoy", v)?;
        }
        if let Some(v) = self.disallow_system.as_ref() {
            struct_ser.serialize_field("disallow_system", v)?;
        }
        if let Some(v) = self.disallow_all.as_ref() {
            struct_ser.serialize_field("disallow_all", v)?;
        }
        if let Some(v) = self.allow_expression.as_ref() {
            struct_ser.serialize_field("allow_expression", v)?;
        }
        if let Some(v) = self.disallow_expression.as_ref() {
            struct_ser.serialize_field("disallow_expression", v)?;
        }
        if let Some(v) = self.disallow_is_error.as_ref() {
            struct_ser.serialize_field("disallow_is_error", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeaderMutationRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allow_all_routing",
            "allowAllRouting",
            "allow_envoy",
            "allowEnvoy",
            "disallow_system",
            "disallowSystem",
            "disallow_all",
            "disallowAll",
            "allow_expression",
            "allowExpression",
            "disallow_expression",
            "disallowExpression",
            "disallow_is_error",
            "disallowIsError",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowAllRouting,
            AllowEnvoy,
            DisallowSystem,
            DisallowAll,
            AllowExpression,
            DisallowExpression,
            DisallowIsError,
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
                            "allowAllRouting" | "allow_all_routing" => Ok(GeneratedField::AllowAllRouting),
                            "allowEnvoy" | "allow_envoy" => Ok(GeneratedField::AllowEnvoy),
                            "disallowSystem" | "disallow_system" => Ok(GeneratedField::DisallowSystem),
                            "disallowAll" | "disallow_all" => Ok(GeneratedField::DisallowAll),
                            "allowExpression" | "allow_expression" => Ok(GeneratedField::AllowExpression),
                            "disallowExpression" | "disallow_expression" => Ok(GeneratedField::DisallowExpression),
                            "disallowIsError" | "disallow_is_error" => Ok(GeneratedField::DisallowIsError),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeaderMutationRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.common.mutation_rules.v3.HeaderMutationRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HeaderMutationRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allow_all_routing__ = None;
                let mut allow_envoy__ = None;
                let mut disallow_system__ = None;
                let mut disallow_all__ = None;
                let mut allow_expression__ = None;
                let mut disallow_expression__ = None;
                let mut disallow_is_error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllowAllRouting => {
                            if allow_all_routing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowAllRouting"));
                            }
                            allow_all_routing__ = map_.next_value()?;
                        }
                        GeneratedField::AllowEnvoy => {
                            if allow_envoy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowEnvoy"));
                            }
                            allow_envoy__ = map_.next_value()?;
                        }
                        GeneratedField::DisallowSystem => {
                            if disallow_system__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disallowSystem"));
                            }
                            disallow_system__ = map_.next_value()?;
                        }
                        GeneratedField::DisallowAll => {
                            if disallow_all__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disallowAll"));
                            }
                            disallow_all__ = map_.next_value()?;
                        }
                        GeneratedField::AllowExpression => {
                            if allow_expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowExpression"));
                            }
                            allow_expression__ = map_.next_value()?;
                        }
                        GeneratedField::DisallowExpression => {
                            if disallow_expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disallowExpression"));
                            }
                            disallow_expression__ = map_.next_value()?;
                        }
                        GeneratedField::DisallowIsError => {
                            if disallow_is_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disallowIsError"));
                            }
                            disallow_is_error__ = map_.next_value()?;
                        }
                    }
                }
                Ok(HeaderMutationRules {
                    allow_all_routing: allow_all_routing__,
                    allow_envoy: allow_envoy__,
                    disallow_system: disallow_system__,
                    disallow_all: disallow_all__,
                    allow_expression: allow_expression__,
                    disallow_expression: disallow_expression__,
                    disallow_is_error: disallow_is_error__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.common.mutation_rules.v3.HeaderMutationRules", FIELDS, GeneratedVisitor)
    }
}
