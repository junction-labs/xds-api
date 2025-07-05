impl serde::Serialize for ClusterSpecifierPlugin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.extension.is_some() {
            len += 1;
        }
        if self.is_optional {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.ClusterSpecifierPlugin", len)?;
        if let Some(v) = self.extension.as_ref() {
            struct_ser.serialize_field("extension", v)?;
        }
        if self.is_optional {
            struct_ser.serialize_field("is_optional", &self.is_optional)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterSpecifierPlugin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "extension",
            "is_optional",
            "isOptional",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Extension,
            IsOptional,
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
                            "extension" => Ok(GeneratedField::Extension),
                            "isOptional" | "is_optional" => Ok(GeneratedField::IsOptional),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterSpecifierPlugin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.ClusterSpecifierPlugin")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClusterSpecifierPlugin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut extension__ = None;
                let mut is_optional__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Extension => {
                            if extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            extension__ = map_.next_value()?;
                        }
                        GeneratedField::IsOptional => {
                            if is_optional__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isOptional"));
                            }
                            is_optional__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ClusterSpecifierPlugin {
                    extension: extension__,
                    is_optional: is_optional__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.ClusterSpecifierPlugin", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CorsPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.allow_origin_string_match.is_empty() {
            len += 1;
        }
        if !self.allow_methods.is_empty() {
            len += 1;
        }
        if !self.allow_headers.is_empty() {
            len += 1;
        }
        if !self.expose_headers.is_empty() {
            len += 1;
        }
        if !self.max_age.is_empty() {
            len += 1;
        }
        if self.allow_credentials.is_some() {
            len += 1;
        }
        if self.shadow_enabled.is_some() {
            len += 1;
        }
        if self.allow_private_network_access.is_some() {
            len += 1;
        }
        if self.forward_not_matching_preflights.is_some() {
            len += 1;
        }
        if self.enabled_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.CorsPolicy", len)?;
        if !self.allow_origin_string_match.is_empty() {
            struct_ser.serialize_field("allow_origin_string_match", &self.allow_origin_string_match)?;
        }
        if !self.allow_methods.is_empty() {
            struct_ser.serialize_field("allow_methods", &self.allow_methods)?;
        }
        if !self.allow_headers.is_empty() {
            struct_ser.serialize_field("allow_headers", &self.allow_headers)?;
        }
        if !self.expose_headers.is_empty() {
            struct_ser.serialize_field("expose_headers", &self.expose_headers)?;
        }
        if !self.max_age.is_empty() {
            struct_ser.serialize_field("max_age", &self.max_age)?;
        }
        if let Some(v) = self.allow_credentials.as_ref() {
            struct_ser.serialize_field("allow_credentials", v)?;
        }
        if let Some(v) = self.shadow_enabled.as_ref() {
            struct_ser.serialize_field("shadow_enabled", v)?;
        }
        if let Some(v) = self.allow_private_network_access.as_ref() {
            struct_ser.serialize_field("allow_private_network_access", v)?;
        }
        if let Some(v) = self.forward_not_matching_preflights.as_ref() {
            struct_ser.serialize_field("forward_not_matching_preflights", v)?;
        }
        if let Some(v) = self.enabled_specifier.as_ref() {
            match v {
                cors_policy::EnabledSpecifier::FilterEnabled(v) => {
                    struct_ser.serialize_field("filter_enabled", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CorsPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allow_origin_string_match",
            "allowOriginStringMatch",
            "allow_methods",
            "allowMethods",
            "allow_headers",
            "allowHeaders",
            "expose_headers",
            "exposeHeaders",
            "max_age",
            "maxAge",
            "allow_credentials",
            "allowCredentials",
            "shadow_enabled",
            "shadowEnabled",
            "allow_private_network_access",
            "allowPrivateNetworkAccess",
            "forward_not_matching_preflights",
            "forwardNotMatchingPreflights",
            "filter_enabled",
            "filterEnabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowOriginStringMatch,
            AllowMethods,
            AllowHeaders,
            ExposeHeaders,
            MaxAge,
            AllowCredentials,
            ShadowEnabled,
            AllowPrivateNetworkAccess,
            ForwardNotMatchingPreflights,
            FilterEnabled,
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
                            "allowOriginStringMatch" | "allow_origin_string_match" => Ok(GeneratedField::AllowOriginStringMatch),
                            "allowMethods" | "allow_methods" => Ok(GeneratedField::AllowMethods),
                            "allowHeaders" | "allow_headers" => Ok(GeneratedField::AllowHeaders),
                            "exposeHeaders" | "expose_headers" => Ok(GeneratedField::ExposeHeaders),
                            "maxAge" | "max_age" => Ok(GeneratedField::MaxAge),
                            "allowCredentials" | "allow_credentials" => Ok(GeneratedField::AllowCredentials),
                            "shadowEnabled" | "shadow_enabled" => Ok(GeneratedField::ShadowEnabled),
                            "allowPrivateNetworkAccess" | "allow_private_network_access" => Ok(GeneratedField::AllowPrivateNetworkAccess),
                            "forwardNotMatchingPreflights" | "forward_not_matching_preflights" => Ok(GeneratedField::ForwardNotMatchingPreflights),
                            "filterEnabled" | "filter_enabled" => Ok(GeneratedField::FilterEnabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CorsPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.CorsPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CorsPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allow_origin_string_match__ = None;
                let mut allow_methods__ = None;
                let mut allow_headers__ = None;
                let mut expose_headers__ = None;
                let mut max_age__ = None;
                let mut allow_credentials__ = None;
                let mut shadow_enabled__ = None;
                let mut allow_private_network_access__ = None;
                let mut forward_not_matching_preflights__ = None;
                let mut enabled_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllowOriginStringMatch => {
                            if allow_origin_string_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowOriginStringMatch"));
                            }
                            allow_origin_string_match__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowMethods => {
                            if allow_methods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowMethods"));
                            }
                            allow_methods__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowHeaders => {
                            if allow_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowHeaders"));
                            }
                            allow_headers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExposeHeaders => {
                            if expose_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exposeHeaders"));
                            }
                            expose_headers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxAge => {
                            if max_age__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAge"));
                            }
                            max_age__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowCredentials => {
                            if allow_credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowCredentials"));
                            }
                            allow_credentials__ = map_.next_value()?;
                        }
                        GeneratedField::ShadowEnabled => {
                            if shadow_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shadowEnabled"));
                            }
                            shadow_enabled__ = map_.next_value()?;
                        }
                        GeneratedField::AllowPrivateNetworkAccess => {
                            if allow_private_network_access__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowPrivateNetworkAccess"));
                            }
                            allow_private_network_access__ = map_.next_value()?;
                        }
                        GeneratedField::ForwardNotMatchingPreflights => {
                            if forward_not_matching_preflights__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forwardNotMatchingPreflights"));
                            }
                            forward_not_matching_preflights__ = map_.next_value()?;
                        }
                        GeneratedField::FilterEnabled => {
                            if enabled_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterEnabled"));
                            }
                            enabled_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(cors_policy::EnabledSpecifier::FilterEnabled)
;
                        }
                    }
                }
                Ok(CorsPolicy {
                    allow_origin_string_match: allow_origin_string_match__.unwrap_or_default(),
                    allow_methods: allow_methods__.unwrap_or_default(),
                    allow_headers: allow_headers__.unwrap_or_default(),
                    expose_headers: expose_headers__.unwrap_or_default(),
                    max_age: max_age__.unwrap_or_default(),
                    allow_credentials: allow_credentials__,
                    shadow_enabled: shadow_enabled__,
                    allow_private_network_access: allow_private_network_access__,
                    forward_not_matching_preflights: forward_not_matching_preflights__,
                    enabled_specifier: enabled_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.CorsPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Decorator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.operation.is_empty() {
            len += 1;
        }
        if self.propagate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.Decorator", len)?;
        if !self.operation.is_empty() {
            struct_ser.serialize_field("operation", &self.operation)?;
        }
        if let Some(v) = self.propagate.as_ref() {
            struct_ser.serialize_field("propagate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Decorator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operation",
            "propagate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operation,
            Propagate,
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
                            "operation" => Ok(GeneratedField::Operation),
                            "propagate" => Ok(GeneratedField::Propagate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Decorator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.Decorator")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Decorator, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operation__ = None;
                let mut propagate__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Operation => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Propagate => {
                            if propagate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("propagate"));
                            }
                            propagate__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Decorator {
                    operation: operation__.unwrap_or_default(),
                    propagate: propagate__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.Decorator", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DirectResponseAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        if self.body.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.DirectResponseAction", len)?;
        if self.status != 0 {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if let Some(v) = self.body.as_ref() {
            struct_ser.serialize_field("body", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DirectResponseAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "body",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            Body,
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
                            "status" => Ok(GeneratedField::Status),
                            "body" => Ok(GeneratedField::Body),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DirectResponseAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.DirectResponseAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DirectResponseAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut body__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Body => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("body"));
                            }
                            body__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DirectResponseAction {
                    status: status__.unwrap_or_default(),
                    body: body__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.DirectResponseAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilterAction {
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.FilterAction", len)?;
        if let Some(v) = self.action.as_ref() {
            struct_ser.serialize_field("action", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "action",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Action,
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
                            "action" => Ok(GeneratedField::Action),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.FilterAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FilterAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FilterAction {
                    action: action__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.FilterAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilterConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config.is_some() {
            len += 1;
        }
        if self.is_optional {
            len += 1;
        }
        if self.disabled {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.FilterConfig", len)?;
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if self.is_optional {
            struct_ser.serialize_field("is_optional", &self.is_optional)?;
        }
        if self.disabled {
            struct_ser.serialize_field("disabled", &self.disabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config",
            "is_optional",
            "isOptional",
            "disabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Config,
            IsOptional,
            Disabled,
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
                            "config" => Ok(GeneratedField::Config),
                            "isOptional" | "is_optional" => Ok(GeneratedField::IsOptional),
                            "disabled" => Ok(GeneratedField::Disabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.FilterConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FilterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config__ = None;
                let mut is_optional__ = None;
                let mut disabled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::IsOptional => {
                            if is_optional__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isOptional"));
                            }
                            is_optional__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Disabled => {
                            if disabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            disabled__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FilterConfig {
                    config: config__,
                    is_optional: is_optional__.unwrap_or_default(),
                    disabled: disabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.FilterConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HeaderMatcher {
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
        if self.invert_match {
            len += 1;
        }
        if self.treat_missing_header_as_empty {
            len += 1;
        }
        if self.header_match_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.HeaderMatcher", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.invert_match {
            struct_ser.serialize_field("invert_match", &self.invert_match)?;
        }
        if self.treat_missing_header_as_empty {
            struct_ser.serialize_field("treat_missing_header_as_empty", &self.treat_missing_header_as_empty)?;
        }
        if let Some(v) = self.header_match_specifier.as_ref() {
            match v {
                header_matcher::HeaderMatchSpecifier::ExactMatch(v) => {
                    struct_ser.serialize_field("exact_match", v)?;
                }
                header_matcher::HeaderMatchSpecifier::SafeRegexMatch(v) => {
                    struct_ser.serialize_field("safe_regex_match", v)?;
                }
                header_matcher::HeaderMatchSpecifier::RangeMatch(v) => {
                    struct_ser.serialize_field("range_match", v)?;
                }
                header_matcher::HeaderMatchSpecifier::PresentMatch(v) => {
                    struct_ser.serialize_field("present_match", v)?;
                }
                header_matcher::HeaderMatchSpecifier::PrefixMatch(v) => {
                    struct_ser.serialize_field("prefix_match", v)?;
                }
                header_matcher::HeaderMatchSpecifier::SuffixMatch(v) => {
                    struct_ser.serialize_field("suffix_match", v)?;
                }
                header_matcher::HeaderMatchSpecifier::ContainsMatch(v) => {
                    struct_ser.serialize_field("contains_match", v)?;
                }
                header_matcher::HeaderMatchSpecifier::StringMatch(v) => {
                    struct_ser.serialize_field("string_match", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeaderMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "invert_match",
            "invertMatch",
            "treat_missing_header_as_empty",
            "treatMissingHeaderAsEmpty",
            "exact_match",
            "exactMatch",
            "safe_regex_match",
            "safeRegexMatch",
            "range_match",
            "rangeMatch",
            "present_match",
            "presentMatch",
            "prefix_match",
            "prefixMatch",
            "suffix_match",
            "suffixMatch",
            "contains_match",
            "containsMatch",
            "string_match",
            "stringMatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            InvertMatch,
            TreatMissingHeaderAsEmpty,
            ExactMatch,
            SafeRegexMatch,
            RangeMatch,
            PresentMatch,
            PrefixMatch,
            SuffixMatch,
            ContainsMatch,
            StringMatch,
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
                            "invertMatch" | "invert_match" => Ok(GeneratedField::InvertMatch),
                            "treatMissingHeaderAsEmpty" | "treat_missing_header_as_empty" => Ok(GeneratedField::TreatMissingHeaderAsEmpty),
                            "exactMatch" | "exact_match" => Ok(GeneratedField::ExactMatch),
                            "safeRegexMatch" | "safe_regex_match" => Ok(GeneratedField::SafeRegexMatch),
                            "rangeMatch" | "range_match" => Ok(GeneratedField::RangeMatch),
                            "presentMatch" | "present_match" => Ok(GeneratedField::PresentMatch),
                            "prefixMatch" | "prefix_match" => Ok(GeneratedField::PrefixMatch),
                            "suffixMatch" | "suffix_match" => Ok(GeneratedField::SuffixMatch),
                            "containsMatch" | "contains_match" => Ok(GeneratedField::ContainsMatch),
                            "stringMatch" | "string_match" => Ok(GeneratedField::StringMatch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeaderMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.HeaderMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HeaderMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut invert_match__ = None;
                let mut treat_missing_header_as_empty__ = None;
                let mut header_match_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InvertMatch => {
                            if invert_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("invertMatch"));
                            }
                            invert_match__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TreatMissingHeaderAsEmpty => {
                            if treat_missing_header_as_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("treatMissingHeaderAsEmpty"));
                            }
                            treat_missing_header_as_empty__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExactMatch => {
                            if header_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exactMatch"));
                            }
                            header_match_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(header_matcher::HeaderMatchSpecifier::ExactMatch);
                        }
                        GeneratedField::SafeRegexMatch => {
                            if header_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("safeRegexMatch"));
                            }
                            header_match_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(header_matcher::HeaderMatchSpecifier::SafeRegexMatch)
;
                        }
                        GeneratedField::RangeMatch => {
                            if header_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rangeMatch"));
                            }
                            header_match_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(header_matcher::HeaderMatchSpecifier::RangeMatch)
;
                        }
                        GeneratedField::PresentMatch => {
                            if header_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presentMatch"));
                            }
                            header_match_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(header_matcher::HeaderMatchSpecifier::PresentMatch);
                        }
                        GeneratedField::PrefixMatch => {
                            if header_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefixMatch"));
                            }
                            header_match_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(header_matcher::HeaderMatchSpecifier::PrefixMatch);
                        }
                        GeneratedField::SuffixMatch => {
                            if header_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suffixMatch"));
                            }
                            header_match_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(header_matcher::HeaderMatchSpecifier::SuffixMatch);
                        }
                        GeneratedField::ContainsMatch => {
                            if header_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("containsMatch"));
                            }
                            header_match_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(header_matcher::HeaderMatchSpecifier::ContainsMatch);
                        }
                        GeneratedField::StringMatch => {
                            if header_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringMatch"));
                            }
                            header_match_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(header_matcher::HeaderMatchSpecifier::StringMatch)
;
                        }
                    }
                }
                Ok(HeaderMatcher {
                    name: name__.unwrap_or_default(),
                    invert_match: invert_match__.unwrap_or_default(),
                    treat_missing_header_as_empty: treat_missing_header_as_empty__.unwrap_or_default(),
                    header_match_specifier: header_match_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.HeaderMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HedgePolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.initial_requests.is_some() {
            len += 1;
        }
        if self.additional_request_chance.is_some() {
            len += 1;
        }
        if self.hedge_on_per_try_timeout {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.HedgePolicy", len)?;
        if let Some(v) = self.initial_requests.as_ref() {
            struct_ser.serialize_field("initial_requests", v)?;
        }
        if let Some(v) = self.additional_request_chance.as_ref() {
            struct_ser.serialize_field("additional_request_chance", v)?;
        }
        if self.hedge_on_per_try_timeout {
            struct_ser.serialize_field("hedge_on_per_try_timeout", &self.hedge_on_per_try_timeout)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HedgePolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "initial_requests",
            "initialRequests",
            "additional_request_chance",
            "additionalRequestChance",
            "hedge_on_per_try_timeout",
            "hedgeOnPerTryTimeout",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InitialRequests,
            AdditionalRequestChance,
            HedgeOnPerTryTimeout,
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
                            "initialRequests" | "initial_requests" => Ok(GeneratedField::InitialRequests),
                            "additionalRequestChance" | "additional_request_chance" => Ok(GeneratedField::AdditionalRequestChance),
                            "hedgeOnPerTryTimeout" | "hedge_on_per_try_timeout" => Ok(GeneratedField::HedgeOnPerTryTimeout),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HedgePolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.HedgePolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HedgePolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut initial_requests__ = None;
                let mut additional_request_chance__ = None;
                let mut hedge_on_per_try_timeout__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InitialRequests => {
                            if initial_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialRequests"));
                            }
                            initial_requests__ = map_.next_value()?;
                        }
                        GeneratedField::AdditionalRequestChance => {
                            if additional_request_chance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalRequestChance"));
                            }
                            additional_request_chance__ = map_.next_value()?;
                        }
                        GeneratedField::HedgeOnPerTryTimeout => {
                            if hedge_on_per_try_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hedgeOnPerTryTimeout"));
                            }
                            hedge_on_per_try_timeout__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HedgePolicy {
                    initial_requests: initial_requests__,
                    additional_request_chance: additional_request_chance__,
                    hedge_on_per_try_timeout: hedge_on_per_try_timeout__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.HedgePolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InternalRedirectPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_internal_redirects.is_some() {
            len += 1;
        }
        if !self.redirect_response_codes.is_empty() {
            len += 1;
        }
        if !self.predicates.is_empty() {
            len += 1;
        }
        if self.allow_cross_scheme_redirect {
            len += 1;
        }
        if !self.response_headers_to_copy.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.InternalRedirectPolicy", len)?;
        if let Some(v) = self.max_internal_redirects.as_ref() {
            struct_ser.serialize_field("max_internal_redirects", v)?;
        }
        if !self.redirect_response_codes.is_empty() {
            struct_ser.serialize_field("redirect_response_codes", &self.redirect_response_codes)?;
        }
        if !self.predicates.is_empty() {
            struct_ser.serialize_field("predicates", &self.predicates)?;
        }
        if self.allow_cross_scheme_redirect {
            struct_ser.serialize_field("allow_cross_scheme_redirect", &self.allow_cross_scheme_redirect)?;
        }
        if !self.response_headers_to_copy.is_empty() {
            struct_ser.serialize_field("response_headers_to_copy", &self.response_headers_to_copy)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InternalRedirectPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_internal_redirects",
            "maxInternalRedirects",
            "redirect_response_codes",
            "redirectResponseCodes",
            "predicates",
            "allow_cross_scheme_redirect",
            "allowCrossSchemeRedirect",
            "response_headers_to_copy",
            "responseHeadersToCopy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxInternalRedirects,
            RedirectResponseCodes,
            Predicates,
            AllowCrossSchemeRedirect,
            ResponseHeadersToCopy,
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
                            "maxInternalRedirects" | "max_internal_redirects" => Ok(GeneratedField::MaxInternalRedirects),
                            "redirectResponseCodes" | "redirect_response_codes" => Ok(GeneratedField::RedirectResponseCodes),
                            "predicates" => Ok(GeneratedField::Predicates),
                            "allowCrossSchemeRedirect" | "allow_cross_scheme_redirect" => Ok(GeneratedField::AllowCrossSchemeRedirect),
                            "responseHeadersToCopy" | "response_headers_to_copy" => Ok(GeneratedField::ResponseHeadersToCopy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InternalRedirectPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.InternalRedirectPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InternalRedirectPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_internal_redirects__ = None;
                let mut redirect_response_codes__ = None;
                let mut predicates__ = None;
                let mut allow_cross_scheme_redirect__ = None;
                let mut response_headers_to_copy__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxInternalRedirects => {
                            if max_internal_redirects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxInternalRedirects"));
                            }
                            max_internal_redirects__ = map_.next_value()?;
                        }
                        GeneratedField::RedirectResponseCodes => {
                            if redirect_response_codes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redirectResponseCodes"));
                            }
                            redirect_response_codes__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Predicates => {
                            if predicates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicates"));
                            }
                            predicates__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowCrossSchemeRedirect => {
                            if allow_cross_scheme_redirect__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowCrossSchemeRedirect"));
                            }
                            allow_cross_scheme_redirect__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToCopy => {
                            if response_headers_to_copy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToCopy"));
                            }
                            response_headers_to_copy__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InternalRedirectPolicy {
                    max_internal_redirects: max_internal_redirects__,
                    redirect_response_codes: redirect_response_codes__.unwrap_or_default(),
                    predicates: predicates__.unwrap_or_default(),
                    allow_cross_scheme_redirect: allow_cross_scheme_redirect__.unwrap_or_default(),
                    response_headers_to_copy: response_headers_to_copy__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.InternalRedirectPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NonForwardingAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.route.v3.NonForwardingAction", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NonForwardingAction {
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
            type Value = NonForwardingAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.NonForwardingAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NonForwardingAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(NonForwardingAction {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.NonForwardingAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryParameterMatcher {
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
        if self.query_parameter_match_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.QueryParameterMatcher", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.query_parameter_match_specifier.as_ref() {
            match v {
                query_parameter_matcher::QueryParameterMatchSpecifier::StringMatch(v) => {
                    struct_ser.serialize_field("string_match", v)?;
                }
                query_parameter_matcher::QueryParameterMatchSpecifier::PresentMatch(v) => {
                    struct_ser.serialize_field("present_match", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParameterMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "string_match",
            "stringMatch",
            "present_match",
            "presentMatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            StringMatch,
            PresentMatch,
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
                            "stringMatch" | "string_match" => Ok(GeneratedField::StringMatch),
                            "presentMatch" | "present_match" => Ok(GeneratedField::PresentMatch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParameterMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.QueryParameterMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryParameterMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut query_parameter_match_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StringMatch => {
                            if query_parameter_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringMatch"));
                            }
                            query_parameter_match_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(query_parameter_matcher::QueryParameterMatchSpecifier::StringMatch)
;
                        }
                        GeneratedField::PresentMatch => {
                            if query_parameter_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presentMatch"));
                            }
                            query_parameter_match_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(query_parameter_matcher::QueryParameterMatchSpecifier::PresentMatch);
                        }
                    }
                }
                Ok(QueryParameterMatcher {
                    name: name__.unwrap_or_default(),
                    query_parameter_match_specifier: query_parameter_match_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.QueryParameterMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RateLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stage.is_some() {
            len += 1;
        }
        if !self.disable_key.is_empty() {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        if self.limit.is_some() {
            len += 1;
        }
        if self.hits_addend.is_some() {
            len += 1;
        }
        if self.apply_on_stream_done {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RateLimit", len)?;
        if let Some(v) = self.stage.as_ref() {
            struct_ser.serialize_field("stage", v)?;
        }
        if !self.disable_key.is_empty() {
            struct_ser.serialize_field("disable_key", &self.disable_key)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        if let Some(v) = self.limit.as_ref() {
            struct_ser.serialize_field("limit", v)?;
        }
        if let Some(v) = self.hits_addend.as_ref() {
            struct_ser.serialize_field("hits_addend", v)?;
        }
        if self.apply_on_stream_done {
            struct_ser.serialize_field("apply_on_stream_done", &self.apply_on_stream_done)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stage",
            "disable_key",
            "disableKey",
            "actions",
            "limit",
            "hits_addend",
            "hitsAddend",
            "apply_on_stream_done",
            "applyOnStreamDone",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Stage,
            DisableKey,
            Actions,
            Limit,
            HitsAddend,
            ApplyOnStreamDone,
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
                            "stage" => Ok(GeneratedField::Stage),
                            "disableKey" | "disable_key" => Ok(GeneratedField::DisableKey),
                            "actions" => Ok(GeneratedField::Actions),
                            "limit" => Ok(GeneratedField::Limit),
                            "hitsAddend" | "hits_addend" => Ok(GeneratedField::HitsAddend),
                            "applyOnStreamDone" | "apply_on_stream_done" => Ok(GeneratedField::ApplyOnStreamDone),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RateLimit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RateLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stage__ = None;
                let mut disable_key__ = None;
                let mut actions__ = None;
                let mut limit__ = None;
                let mut hits_addend__ = None;
                let mut apply_on_stream_done__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Stage => {
                            if stage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stage"));
                            }
                            stage__ = map_.next_value()?;
                        }
                        GeneratedField::DisableKey => {
                            if disable_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableKey"));
                            }
                            disable_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = map_.next_value()?;
                        }
                        GeneratedField::HitsAddend => {
                            if hits_addend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hitsAddend"));
                            }
                            hits_addend__ = map_.next_value()?;
                        }
                        GeneratedField::ApplyOnStreamDone => {
                            if apply_on_stream_done__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applyOnStreamDone"));
                            }
                            apply_on_stream_done__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RateLimit {
                    stage: stage__,
                    disable_key: disable_key__.unwrap_or_default(),
                    actions: actions__.unwrap_or_default(),
                    limit: limit__,
                    hits_addend: hits_addend__,
                    apply_on_stream_done: apply_on_stream_done__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RateLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::Action {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RateLimit.Action", len)?;
        if let Some(v) = self.action_specifier.as_ref() {
            match v {
                rate_limit::action::ActionSpecifier::SourceCluster(v) => {
                    struct_ser.serialize_field("source_cluster", v)?;
                }
                rate_limit::action::ActionSpecifier::DestinationCluster(v) => {
                    struct_ser.serialize_field("destination_cluster", v)?;
                }
                rate_limit::action::ActionSpecifier::RequestHeaders(v) => {
                    struct_ser.serialize_field("request_headers", v)?;
                }
                rate_limit::action::ActionSpecifier::QueryParameters(v) => {
                    struct_ser.serialize_field("query_parameters", v)?;
                }
                rate_limit::action::ActionSpecifier::RemoteAddress(v) => {
                    struct_ser.serialize_field("remote_address", v)?;
                }
                rate_limit::action::ActionSpecifier::GenericKey(v) => {
                    struct_ser.serialize_field("generic_key", v)?;
                }
                rate_limit::action::ActionSpecifier::HeaderValueMatch(v) => {
                    struct_ser.serialize_field("header_value_match", v)?;
                }
                rate_limit::action::ActionSpecifier::DynamicMetadata(v) => {
                    struct_ser.serialize_field("dynamic_metadata", v)?;
                }
                rate_limit::action::ActionSpecifier::Metadata(v) => {
                    struct_ser.serialize_field("metadata", v)?;
                }
                rate_limit::action::ActionSpecifier::Extension(v) => {
                    struct_ser.serialize_field("extension", v)?;
                }
                rate_limit::action::ActionSpecifier::MaskedRemoteAddress(v) => {
                    struct_ser.serialize_field("masked_remote_address", v)?;
                }
                rate_limit::action::ActionSpecifier::QueryParameterValueMatch(v) => {
                    struct_ser.serialize_field("query_parameter_value_match", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::Action {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_cluster",
            "sourceCluster",
            "destination_cluster",
            "destinationCluster",
            "request_headers",
            "requestHeaders",
            "query_parameters",
            "queryParameters",
            "remote_address",
            "remoteAddress",
            "generic_key",
            "genericKey",
            "header_value_match",
            "headerValueMatch",
            "dynamic_metadata",
            "dynamicMetadata",
            "metadata",
            "extension",
            "masked_remote_address",
            "maskedRemoteAddress",
            "query_parameter_value_match",
            "queryParameterValueMatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceCluster,
            DestinationCluster,
            RequestHeaders,
            QueryParameters,
            RemoteAddress,
            GenericKey,
            HeaderValueMatch,
            DynamicMetadata,
            Metadata,
            Extension,
            MaskedRemoteAddress,
            QueryParameterValueMatch,
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
                            "sourceCluster" | "source_cluster" => Ok(GeneratedField::SourceCluster),
                            "destinationCluster" | "destination_cluster" => Ok(GeneratedField::DestinationCluster),
                            "requestHeaders" | "request_headers" => Ok(GeneratedField::RequestHeaders),
                            "queryParameters" | "query_parameters" => Ok(GeneratedField::QueryParameters),
                            "remoteAddress" | "remote_address" => Ok(GeneratedField::RemoteAddress),
                            "genericKey" | "generic_key" => Ok(GeneratedField::GenericKey),
                            "headerValueMatch" | "header_value_match" => Ok(GeneratedField::HeaderValueMatch),
                            "dynamicMetadata" | "dynamic_metadata" => Ok(GeneratedField::DynamicMetadata),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "extension" => Ok(GeneratedField::Extension),
                            "maskedRemoteAddress" | "masked_remote_address" => Ok(GeneratedField::MaskedRemoteAddress),
                            "queryParameterValueMatch" | "query_parameter_value_match" => Ok(GeneratedField::QueryParameterValueMatch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::Action;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RateLimit.Action")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit::Action, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SourceCluster => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceCluster"));
                            }
                            action_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::SourceCluster)
;
                        }
                        GeneratedField::DestinationCluster => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationCluster"));
                            }
                            action_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::DestinationCluster)
;
                        }
                        GeneratedField::RequestHeaders => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeaders"));
                            }
                            action_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::RequestHeaders)
;
                        }
                        GeneratedField::QueryParameters => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryParameters"));
                            }
                            action_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::QueryParameters)
;
                        }
                        GeneratedField::RemoteAddress => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteAddress"));
                            }
                            action_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::RemoteAddress)
;
                        }
                        GeneratedField::GenericKey => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genericKey"));
                            }
                            action_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::GenericKey)
;
                        }
                        GeneratedField::HeaderValueMatch => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerValueMatch"));
                            }
                            action_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::HeaderValueMatch)
;
                        }
                        GeneratedField::DynamicMetadata => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicMetadata"));
                            }
                            action_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::DynamicMetadata)
;
                        }
                        GeneratedField::Metadata => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            action_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::Metadata)
;
                        }
                        GeneratedField::Extension => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            action_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::Extension)
;
                        }
                        GeneratedField::MaskedRemoteAddress => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maskedRemoteAddress"));
                            }
                            action_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::MaskedRemoteAddress)
;
                        }
                        GeneratedField::QueryParameterValueMatch => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryParameterValueMatch"));
                            }
                            action_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::QueryParameterValueMatch)
;
                        }
                    }
                }
                Ok(rate_limit::Action {
                    action_specifier: action_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RateLimit.Action", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::DestinationCluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.route.v3.RateLimit.Action.DestinationCluster", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::action::DestinationCluster {
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
            type Value = rate_limit::action::DestinationCluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RateLimit.Action.DestinationCluster")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit::action::DestinationCluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(rate_limit::action::DestinationCluster {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RateLimit.Action.DestinationCluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::DynamicMetaData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.descriptor_key.is_empty() {
            len += 1;
        }
        if self.metadata_key.is_some() {
            len += 1;
        }
        if !self.default_value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RateLimit.Action.DynamicMetaData", len)?;
        if !self.descriptor_key.is_empty() {
            struct_ser.serialize_field("descriptor_key", &self.descriptor_key)?;
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
impl<'de> serde::Deserialize<'de> for rate_limit::action::DynamicMetaData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "descriptor_key",
            "descriptorKey",
            "metadata_key",
            "metadataKey",
            "default_value",
            "defaultValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DescriptorKey,
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
                            "descriptorKey" | "descriptor_key" => Ok(GeneratedField::DescriptorKey),
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
            type Value = rate_limit::action::DynamicMetaData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RateLimit.Action.DynamicMetaData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit::action::DynamicMetaData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut descriptor_key__ = None;
                let mut metadata_key__ = None;
                let mut default_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DescriptorKey => {
                            if descriptor_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorKey"));
                            }
                            descriptor_key__ = Some(map_.next_value()?);
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
                Ok(rate_limit::action::DynamicMetaData {
                    descriptor_key: descriptor_key__.unwrap_or_default(),
                    metadata_key: metadata_key__,
                    default_value: default_value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RateLimit.Action.DynamicMetaData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::GenericKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.descriptor_value.is_empty() {
            len += 1;
        }
        if !self.descriptor_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RateLimit.Action.GenericKey", len)?;
        if !self.descriptor_value.is_empty() {
            struct_ser.serialize_field("descriptor_value", &self.descriptor_value)?;
        }
        if !self.descriptor_key.is_empty() {
            struct_ser.serialize_field("descriptor_key", &self.descriptor_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::action::GenericKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "descriptor_value",
            "descriptorValue",
            "descriptor_key",
            "descriptorKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DescriptorValue,
            DescriptorKey,
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
                            "descriptorValue" | "descriptor_value" => Ok(GeneratedField::DescriptorValue),
                            "descriptorKey" | "descriptor_key" => Ok(GeneratedField::DescriptorKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::action::GenericKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RateLimit.Action.GenericKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit::action::GenericKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut descriptor_value__ = None;
                let mut descriptor_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DescriptorValue => {
                            if descriptor_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorValue"));
                            }
                            descriptor_value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DescriptorKey => {
                            if descriptor_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorKey"));
                            }
                            descriptor_key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(rate_limit::action::GenericKey {
                    descriptor_value: descriptor_value__.unwrap_or_default(),
                    descriptor_key: descriptor_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RateLimit.Action.GenericKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::HeaderValueMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.descriptor_key.is_empty() {
            len += 1;
        }
        if !self.descriptor_value.is_empty() {
            len += 1;
        }
        if self.expect_match.is_some() {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RateLimit.Action.HeaderValueMatch", len)?;
        if !self.descriptor_key.is_empty() {
            struct_ser.serialize_field("descriptor_key", &self.descriptor_key)?;
        }
        if !self.descriptor_value.is_empty() {
            struct_ser.serialize_field("descriptor_value", &self.descriptor_value)?;
        }
        if let Some(v) = self.expect_match.as_ref() {
            struct_ser.serialize_field("expect_match", v)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::action::HeaderValueMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "descriptor_key",
            "descriptorKey",
            "descriptor_value",
            "descriptorValue",
            "expect_match",
            "expectMatch",
            "headers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DescriptorKey,
            DescriptorValue,
            ExpectMatch,
            Headers,
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
                            "descriptorKey" | "descriptor_key" => Ok(GeneratedField::DescriptorKey),
                            "descriptorValue" | "descriptor_value" => Ok(GeneratedField::DescriptorValue),
                            "expectMatch" | "expect_match" => Ok(GeneratedField::ExpectMatch),
                            "headers" => Ok(GeneratedField::Headers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::action::HeaderValueMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RateLimit.Action.HeaderValueMatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit::action::HeaderValueMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut descriptor_key__ = None;
                let mut descriptor_value__ = None;
                let mut expect_match__ = None;
                let mut headers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DescriptorKey => {
                            if descriptor_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorKey"));
                            }
                            descriptor_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DescriptorValue => {
                            if descriptor_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorValue"));
                            }
                            descriptor_value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExpectMatch => {
                            if expect_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expectMatch"));
                            }
                            expect_match__ = map_.next_value()?;
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(rate_limit::action::HeaderValueMatch {
                    descriptor_key: descriptor_key__.unwrap_or_default(),
                    descriptor_value: descriptor_value__.unwrap_or_default(),
                    expect_match: expect_match__,
                    headers: headers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RateLimit.Action.HeaderValueMatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::MaskedRemoteAddress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.v4_prefix_mask_len.is_some() {
            len += 1;
        }
        if self.v6_prefix_mask_len.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RateLimit.Action.MaskedRemoteAddress", len)?;
        if let Some(v) = self.v4_prefix_mask_len.as_ref() {
            struct_ser.serialize_field("v4_prefix_mask_len", v)?;
        }
        if let Some(v) = self.v6_prefix_mask_len.as_ref() {
            struct_ser.serialize_field("v6_prefix_mask_len", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::action::MaskedRemoteAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "v4_prefix_mask_len",
            "v4PrefixMaskLen",
            "v6_prefix_mask_len",
            "v6PrefixMaskLen",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            V4PrefixMaskLen,
            V6PrefixMaskLen,
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
                            "v4PrefixMaskLen" | "v4_prefix_mask_len" => Ok(GeneratedField::V4PrefixMaskLen),
                            "v6PrefixMaskLen" | "v6_prefix_mask_len" => Ok(GeneratedField::V6PrefixMaskLen),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::action::MaskedRemoteAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RateLimit.Action.MaskedRemoteAddress")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit::action::MaskedRemoteAddress, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut v4_prefix_mask_len__ = None;
                let mut v6_prefix_mask_len__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::V4PrefixMaskLen => {
                            if v4_prefix_mask_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("v4PrefixMaskLen"));
                            }
                            v4_prefix_mask_len__ = map_.next_value()?;
                        }
                        GeneratedField::V6PrefixMaskLen => {
                            if v6_prefix_mask_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("v6PrefixMaskLen"));
                            }
                            v6_prefix_mask_len__ = map_.next_value()?;
                        }
                    }
                }
                Ok(rate_limit::action::MaskedRemoteAddress {
                    v4_prefix_mask_len: v4_prefix_mask_len__,
                    v6_prefix_mask_len: v6_prefix_mask_len__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RateLimit.Action.MaskedRemoteAddress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::MetaData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.descriptor_key.is_empty() {
            len += 1;
        }
        if self.metadata_key.is_some() {
            len += 1;
        }
        if !self.default_value.is_empty() {
            len += 1;
        }
        if self.source != 0 {
            len += 1;
        }
        if self.skip_if_absent {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RateLimit.Action.MetaData", len)?;
        if !self.descriptor_key.is_empty() {
            struct_ser.serialize_field("descriptor_key", &self.descriptor_key)?;
        }
        if let Some(v) = self.metadata_key.as_ref() {
            struct_ser.serialize_field("metadata_key", v)?;
        }
        if !self.default_value.is_empty() {
            struct_ser.serialize_field("default_value", &self.default_value)?;
        }
        if self.source != 0 {
            let v = rate_limit::action::meta_data::Source::try_from(self.source)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.source)))?;
            struct_ser.serialize_field("source", &v)?;
        }
        if self.skip_if_absent {
            struct_ser.serialize_field("skip_if_absent", &self.skip_if_absent)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::action::MetaData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "descriptor_key",
            "descriptorKey",
            "metadata_key",
            "metadataKey",
            "default_value",
            "defaultValue",
            "source",
            "skip_if_absent",
            "skipIfAbsent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DescriptorKey,
            MetadataKey,
            DefaultValue,
            Source,
            SkipIfAbsent,
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
                            "descriptorKey" | "descriptor_key" => Ok(GeneratedField::DescriptorKey),
                            "metadataKey" | "metadata_key" => Ok(GeneratedField::MetadataKey),
                            "defaultValue" | "default_value" => Ok(GeneratedField::DefaultValue),
                            "source" => Ok(GeneratedField::Source),
                            "skipIfAbsent" | "skip_if_absent" => Ok(GeneratedField::SkipIfAbsent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::action::MetaData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RateLimit.Action.MetaData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit::action::MetaData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut descriptor_key__ = None;
                let mut metadata_key__ = None;
                let mut default_value__ = None;
                let mut source__ = None;
                let mut skip_if_absent__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DescriptorKey => {
                            if descriptor_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorKey"));
                            }
                            descriptor_key__ = Some(map_.next_value()?);
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
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map_.next_value::<rate_limit::action::meta_data::Source>()? as i32);
                        }
                        GeneratedField::SkipIfAbsent => {
                            if skip_if_absent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipIfAbsent"));
                            }
                            skip_if_absent__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(rate_limit::action::MetaData {
                    descriptor_key: descriptor_key__.unwrap_or_default(),
                    metadata_key: metadata_key__,
                    default_value: default_value__.unwrap_or_default(),
                    source: source__.unwrap_or_default(),
                    skip_if_absent: skip_if_absent__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RateLimit.Action.MetaData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::meta_data::Source {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Dynamic => "DYNAMIC",
            Self::RouteEntry => "ROUTE_ENTRY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::action::meta_data::Source {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DYNAMIC",
            "ROUTE_ENTRY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::action::meta_data::Source;

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
                    "DYNAMIC" => Ok(rate_limit::action::meta_data::Source::Dynamic),
                    "ROUTE_ENTRY" => Ok(rate_limit::action::meta_data::Source::RouteEntry),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::QueryParameterValueMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.descriptor_key.is_empty() {
            len += 1;
        }
        if !self.descriptor_value.is_empty() {
            len += 1;
        }
        if self.expect_match.is_some() {
            len += 1;
        }
        if !self.query_parameters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RateLimit.Action.QueryParameterValueMatch", len)?;
        if !self.descriptor_key.is_empty() {
            struct_ser.serialize_field("descriptor_key", &self.descriptor_key)?;
        }
        if !self.descriptor_value.is_empty() {
            struct_ser.serialize_field("descriptor_value", &self.descriptor_value)?;
        }
        if let Some(v) = self.expect_match.as_ref() {
            struct_ser.serialize_field("expect_match", v)?;
        }
        if !self.query_parameters.is_empty() {
            struct_ser.serialize_field("query_parameters", &self.query_parameters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::action::QueryParameterValueMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "descriptor_key",
            "descriptorKey",
            "descriptor_value",
            "descriptorValue",
            "expect_match",
            "expectMatch",
            "query_parameters",
            "queryParameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DescriptorKey,
            DescriptorValue,
            ExpectMatch,
            QueryParameters,
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
                            "descriptorKey" | "descriptor_key" => Ok(GeneratedField::DescriptorKey),
                            "descriptorValue" | "descriptor_value" => Ok(GeneratedField::DescriptorValue),
                            "expectMatch" | "expect_match" => Ok(GeneratedField::ExpectMatch),
                            "queryParameters" | "query_parameters" => Ok(GeneratedField::QueryParameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::action::QueryParameterValueMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RateLimit.Action.QueryParameterValueMatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit::action::QueryParameterValueMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut descriptor_key__ = None;
                let mut descriptor_value__ = None;
                let mut expect_match__ = None;
                let mut query_parameters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DescriptorKey => {
                            if descriptor_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorKey"));
                            }
                            descriptor_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DescriptorValue => {
                            if descriptor_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorValue"));
                            }
                            descriptor_value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExpectMatch => {
                            if expect_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expectMatch"));
                            }
                            expect_match__ = map_.next_value()?;
                        }
                        GeneratedField::QueryParameters => {
                            if query_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryParameters"));
                            }
                            query_parameters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(rate_limit::action::QueryParameterValueMatch {
                    descriptor_key: descriptor_key__.unwrap_or_default(),
                    descriptor_value: descriptor_value__.unwrap_or_default(),
                    expect_match: expect_match__,
                    query_parameters: query_parameters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RateLimit.Action.QueryParameterValueMatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::QueryParameters {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.query_parameter_name.is_empty() {
            len += 1;
        }
        if !self.descriptor_key.is_empty() {
            len += 1;
        }
        if self.skip_if_absent {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RateLimit.Action.QueryParameters", len)?;
        if !self.query_parameter_name.is_empty() {
            struct_ser.serialize_field("query_parameter_name", &self.query_parameter_name)?;
        }
        if !self.descriptor_key.is_empty() {
            struct_ser.serialize_field("descriptor_key", &self.descriptor_key)?;
        }
        if self.skip_if_absent {
            struct_ser.serialize_field("skip_if_absent", &self.skip_if_absent)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::action::QueryParameters {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query_parameter_name",
            "queryParameterName",
            "descriptor_key",
            "descriptorKey",
            "skip_if_absent",
            "skipIfAbsent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueryParameterName,
            DescriptorKey,
            SkipIfAbsent,
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
                            "queryParameterName" | "query_parameter_name" => Ok(GeneratedField::QueryParameterName),
                            "descriptorKey" | "descriptor_key" => Ok(GeneratedField::DescriptorKey),
                            "skipIfAbsent" | "skip_if_absent" => Ok(GeneratedField::SkipIfAbsent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::action::QueryParameters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RateLimit.Action.QueryParameters")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit::action::QueryParameters, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query_parameter_name__ = None;
                let mut descriptor_key__ = None;
                let mut skip_if_absent__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueryParameterName => {
                            if query_parameter_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryParameterName"));
                            }
                            query_parameter_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DescriptorKey => {
                            if descriptor_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorKey"));
                            }
                            descriptor_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SkipIfAbsent => {
                            if skip_if_absent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipIfAbsent"));
                            }
                            skip_if_absent__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(rate_limit::action::QueryParameters {
                    query_parameter_name: query_parameter_name__.unwrap_or_default(),
                    descriptor_key: descriptor_key__.unwrap_or_default(),
                    skip_if_absent: skip_if_absent__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RateLimit.Action.QueryParameters", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::RemoteAddress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.route.v3.RateLimit.Action.RemoteAddress", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::action::RemoteAddress {
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
            type Value = rate_limit::action::RemoteAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RateLimit.Action.RemoteAddress")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit::action::RemoteAddress, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(rate_limit::action::RemoteAddress {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RateLimit.Action.RemoteAddress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::RequestHeaders {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.header_name.is_empty() {
            len += 1;
        }
        if !self.descriptor_key.is_empty() {
            len += 1;
        }
        if self.skip_if_absent {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RateLimit.Action.RequestHeaders", len)?;
        if !self.header_name.is_empty() {
            struct_ser.serialize_field("header_name", &self.header_name)?;
        }
        if !self.descriptor_key.is_empty() {
            struct_ser.serialize_field("descriptor_key", &self.descriptor_key)?;
        }
        if self.skip_if_absent {
            struct_ser.serialize_field("skip_if_absent", &self.skip_if_absent)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::action::RequestHeaders {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header_name",
            "headerName",
            "descriptor_key",
            "descriptorKey",
            "skip_if_absent",
            "skipIfAbsent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HeaderName,
            DescriptorKey,
            SkipIfAbsent,
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
                            "headerName" | "header_name" => Ok(GeneratedField::HeaderName),
                            "descriptorKey" | "descriptor_key" => Ok(GeneratedField::DescriptorKey),
                            "skipIfAbsent" | "skip_if_absent" => Ok(GeneratedField::SkipIfAbsent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::action::RequestHeaders;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RateLimit.Action.RequestHeaders")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit::action::RequestHeaders, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header_name__ = None;
                let mut descriptor_key__ = None;
                let mut skip_if_absent__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HeaderName => {
                            if header_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerName"));
                            }
                            header_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DescriptorKey => {
                            if descriptor_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorKey"));
                            }
                            descriptor_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SkipIfAbsent => {
                            if skip_if_absent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipIfAbsent"));
                            }
                            skip_if_absent__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(rate_limit::action::RequestHeaders {
                    header_name: header_name__.unwrap_or_default(),
                    descriptor_key: descriptor_key__.unwrap_or_default(),
                    skip_if_absent: skip_if_absent__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RateLimit.Action.RequestHeaders", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::SourceCluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.route.v3.RateLimit.Action.SourceCluster", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::action::SourceCluster {
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
            type Value = rate_limit::action::SourceCluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RateLimit.Action.SourceCluster")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit::action::SourceCluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(rate_limit::action::SourceCluster {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RateLimit.Action.SourceCluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::HitsAddend {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.number.is_some() {
            len += 1;
        }
        if !self.format.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RateLimit.HitsAddend", len)?;
        if let Some(v) = self.number.as_ref() {
            struct_ser.serialize_field("number", v)?;
        }
        if !self.format.is_empty() {
            struct_ser.serialize_field("format", &self.format)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::HitsAddend {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "number",
            "format",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Number,
            Format,
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
                            "number" => Ok(GeneratedField::Number),
                            "format" => Ok(GeneratedField::Format),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::HitsAddend;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RateLimit.HitsAddend")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit::HitsAddend, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut number__ = None;
                let mut format__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Number => {
                            if number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            number__ = map_.next_value()?;
                        }
                        GeneratedField::Format => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            format__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(rate_limit::HitsAddend {
                    number: number__,
                    format: format__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RateLimit.HitsAddend", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::Override {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.override_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RateLimit.Override", len)?;
        if let Some(v) = self.override_specifier.as_ref() {
            match v {
                rate_limit::r#override::OverrideSpecifier::DynamicMetadata(v) => {
                    struct_ser.serialize_field("dynamic_metadata", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::Override {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dynamic_metadata",
            "dynamicMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DynamicMetadata,
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
                            "dynamicMetadata" | "dynamic_metadata" => Ok(GeneratedField::DynamicMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::Override;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RateLimit.Override")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit::Override, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut override_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DynamicMetadata => {
                            if override_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicMetadata"));
                            }
                            override_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(rate_limit::r#override::OverrideSpecifier::DynamicMetadata)
;
                        }
                    }
                }
                Ok(rate_limit::Override {
                    override_specifier: override_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RateLimit.Override", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::r#override::DynamicMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RateLimit.Override.DynamicMetadata", len)?;
        if let Some(v) = self.metadata_key.as_ref() {
            struct_ser.serialize_field("metadata_key", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::r#override::DynamicMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata_key",
            "metadataKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MetadataKey,
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
                            "metadataKey" | "metadata_key" => Ok(GeneratedField::MetadataKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::r#override::DynamicMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RateLimit.Override.DynamicMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit::r#override::DynamicMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MetadataKey => {
                            if metadata_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataKey"));
                            }
                            metadata_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(rate_limit::r#override::DynamicMetadata {
                    metadata_key: metadata_key__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RateLimit.Override.DynamicMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RedirectAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.host_redirect.is_empty() {
            len += 1;
        }
        if self.port_redirect != 0 {
            len += 1;
        }
        if self.response_code != 0 {
            len += 1;
        }
        if self.strip_query {
            len += 1;
        }
        if self.scheme_rewrite_specifier.is_some() {
            len += 1;
        }
        if self.path_rewrite_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RedirectAction", len)?;
        if !self.host_redirect.is_empty() {
            struct_ser.serialize_field("host_redirect", &self.host_redirect)?;
        }
        if self.port_redirect != 0 {
            struct_ser.serialize_field("port_redirect", &self.port_redirect)?;
        }
        if self.response_code != 0 {
            let v = redirect_action::RedirectResponseCode::try_from(self.response_code)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.response_code)))?;
            struct_ser.serialize_field("response_code", &v)?;
        }
        if self.strip_query {
            struct_ser.serialize_field("strip_query", &self.strip_query)?;
        }
        if let Some(v) = self.scheme_rewrite_specifier.as_ref() {
            match v {
                redirect_action::SchemeRewriteSpecifier::HttpsRedirect(v) => {
                    struct_ser.serialize_field("https_redirect", v)?;
                }
                redirect_action::SchemeRewriteSpecifier::SchemeRedirect(v) => {
                    struct_ser.serialize_field("scheme_redirect", v)?;
                }
            }
        }
        if let Some(v) = self.path_rewrite_specifier.as_ref() {
            match v {
                redirect_action::PathRewriteSpecifier::PathRedirect(v) => {
                    struct_ser.serialize_field("path_redirect", v)?;
                }
                redirect_action::PathRewriteSpecifier::PrefixRewrite(v) => {
                    struct_ser.serialize_field("prefix_rewrite", v)?;
                }
                redirect_action::PathRewriteSpecifier::RegexRewrite(v) => {
                    struct_ser.serialize_field("regex_rewrite", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RedirectAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host_redirect",
            "hostRedirect",
            "port_redirect",
            "portRedirect",
            "response_code",
            "responseCode",
            "strip_query",
            "stripQuery",
            "https_redirect",
            "httpsRedirect",
            "scheme_redirect",
            "schemeRedirect",
            "path_redirect",
            "pathRedirect",
            "prefix_rewrite",
            "prefixRewrite",
            "regex_rewrite",
            "regexRewrite",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostRedirect,
            PortRedirect,
            ResponseCode,
            StripQuery,
            HttpsRedirect,
            SchemeRedirect,
            PathRedirect,
            PrefixRewrite,
            RegexRewrite,
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
                            "hostRedirect" | "host_redirect" => Ok(GeneratedField::HostRedirect),
                            "portRedirect" | "port_redirect" => Ok(GeneratedField::PortRedirect),
                            "responseCode" | "response_code" => Ok(GeneratedField::ResponseCode),
                            "stripQuery" | "strip_query" => Ok(GeneratedField::StripQuery),
                            "httpsRedirect" | "https_redirect" => Ok(GeneratedField::HttpsRedirect),
                            "schemeRedirect" | "scheme_redirect" => Ok(GeneratedField::SchemeRedirect),
                            "pathRedirect" | "path_redirect" => Ok(GeneratedField::PathRedirect),
                            "prefixRewrite" | "prefix_rewrite" => Ok(GeneratedField::PrefixRewrite),
                            "regexRewrite" | "regex_rewrite" => Ok(GeneratedField::RegexRewrite),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RedirectAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RedirectAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RedirectAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host_redirect__ = None;
                let mut port_redirect__ = None;
                let mut response_code__ = None;
                let mut strip_query__ = None;
                let mut scheme_rewrite_specifier__ = None;
                let mut path_rewrite_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HostRedirect => {
                            if host_redirect__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostRedirect"));
                            }
                            host_redirect__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PortRedirect => {
                            if port_redirect__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portRedirect"));
                            }
                            port_redirect__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResponseCode => {
                            if response_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseCode"));
                            }
                            response_code__ = Some(map_.next_value::<redirect_action::RedirectResponseCode>()? as i32);
                        }
                        GeneratedField::StripQuery => {
                            if strip_query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripQuery"));
                            }
                            strip_query__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HttpsRedirect => {
                            if scheme_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpsRedirect"));
                            }
                            scheme_rewrite_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(redirect_action::SchemeRewriteSpecifier::HttpsRedirect);
                        }
                        GeneratedField::SchemeRedirect => {
                            if scheme_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemeRedirect"));
                            }
                            scheme_rewrite_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(redirect_action::SchemeRewriteSpecifier::SchemeRedirect);
                        }
                        GeneratedField::PathRedirect => {
                            if path_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathRedirect"));
                            }
                            path_rewrite_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(redirect_action::PathRewriteSpecifier::PathRedirect);
                        }
                        GeneratedField::PrefixRewrite => {
                            if path_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefixRewrite"));
                            }
                            path_rewrite_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(redirect_action::PathRewriteSpecifier::PrefixRewrite);
                        }
                        GeneratedField::RegexRewrite => {
                            if path_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regexRewrite"));
                            }
                            path_rewrite_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(redirect_action::PathRewriteSpecifier::RegexRewrite)
;
                        }
                    }
                }
                Ok(RedirectAction {
                    host_redirect: host_redirect__.unwrap_or_default(),
                    port_redirect: port_redirect__.unwrap_or_default(),
                    response_code: response_code__.unwrap_or_default(),
                    strip_query: strip_query__.unwrap_or_default(),
                    scheme_rewrite_specifier: scheme_rewrite_specifier__,
                    path_rewrite_specifier: path_rewrite_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RedirectAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for redirect_action::RedirectResponseCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::MovedPermanently => "MOVED_PERMANENTLY",
            Self::Found => "FOUND",
            Self::SeeOther => "SEE_OTHER",
            Self::TemporaryRedirect => "TEMPORARY_REDIRECT",
            Self::PermanentRedirect => "PERMANENT_REDIRECT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for redirect_action::RedirectResponseCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MOVED_PERMANENTLY",
            "FOUND",
            "SEE_OTHER",
            "TEMPORARY_REDIRECT",
            "PERMANENT_REDIRECT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = redirect_action::RedirectResponseCode;

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
                    "MOVED_PERMANENTLY" => Ok(redirect_action::RedirectResponseCode::MovedPermanently),
                    "FOUND" => Ok(redirect_action::RedirectResponseCode::Found),
                    "SEE_OTHER" => Ok(redirect_action::RedirectResponseCode::SeeOther),
                    "TEMPORARY_REDIRECT" => Ok(redirect_action::RedirectResponseCode::TemporaryRedirect),
                    "PERMANENT_REDIRECT" => Ok(redirect_action::RedirectResponseCode::PermanentRedirect),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RetryPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.retry_on.is_empty() {
            len += 1;
        }
        if self.num_retries.is_some() {
            len += 1;
        }
        if self.per_try_timeout.is_some() {
            len += 1;
        }
        if self.per_try_idle_timeout.is_some() {
            len += 1;
        }
        if self.retry_priority.is_some() {
            len += 1;
        }
        if !self.retry_host_predicate.is_empty() {
            len += 1;
        }
        if !self.retry_options_predicates.is_empty() {
            len += 1;
        }
        if self.host_selection_retry_max_attempts != 0 {
            len += 1;
        }
        if !self.retriable_status_codes.is_empty() {
            len += 1;
        }
        if self.retry_back_off.is_some() {
            len += 1;
        }
        if self.rate_limited_retry_back_off.is_some() {
            len += 1;
        }
        if !self.retriable_headers.is_empty() {
            len += 1;
        }
        if !self.retriable_request_headers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RetryPolicy", len)?;
        if !self.retry_on.is_empty() {
            struct_ser.serialize_field("retry_on", &self.retry_on)?;
        }
        if let Some(v) = self.num_retries.as_ref() {
            struct_ser.serialize_field("num_retries", v)?;
        }
        if let Some(v) = self.per_try_timeout.as_ref() {
            struct_ser.serialize_field("per_try_timeout", v)?;
        }
        if let Some(v) = self.per_try_idle_timeout.as_ref() {
            struct_ser.serialize_field("per_try_idle_timeout", v)?;
        }
        if let Some(v) = self.retry_priority.as_ref() {
            struct_ser.serialize_field("retry_priority", v)?;
        }
        if !self.retry_host_predicate.is_empty() {
            struct_ser.serialize_field("retry_host_predicate", &self.retry_host_predicate)?;
        }
        if !self.retry_options_predicates.is_empty() {
            struct_ser.serialize_field("retry_options_predicates", &self.retry_options_predicates)?;
        }
        if self.host_selection_retry_max_attempts != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("host_selection_retry_max_attempts", ToString::to_string(&self.host_selection_retry_max_attempts).as_str())?;
        }
        if !self.retriable_status_codes.is_empty() {
            struct_ser.serialize_field("retriable_status_codes", &self.retriable_status_codes)?;
        }
        if let Some(v) = self.retry_back_off.as_ref() {
            struct_ser.serialize_field("retry_back_off", v)?;
        }
        if let Some(v) = self.rate_limited_retry_back_off.as_ref() {
            struct_ser.serialize_field("rate_limited_retry_back_off", v)?;
        }
        if !self.retriable_headers.is_empty() {
            struct_ser.serialize_field("retriable_headers", &self.retriable_headers)?;
        }
        if !self.retriable_request_headers.is_empty() {
            struct_ser.serialize_field("retriable_request_headers", &self.retriable_request_headers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RetryPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "retry_on",
            "retryOn",
            "num_retries",
            "numRetries",
            "per_try_timeout",
            "perTryTimeout",
            "per_try_idle_timeout",
            "perTryIdleTimeout",
            "retry_priority",
            "retryPriority",
            "retry_host_predicate",
            "retryHostPredicate",
            "retry_options_predicates",
            "retryOptionsPredicates",
            "host_selection_retry_max_attempts",
            "hostSelectionRetryMaxAttempts",
            "retriable_status_codes",
            "retriableStatusCodes",
            "retry_back_off",
            "retryBackOff",
            "rate_limited_retry_back_off",
            "rateLimitedRetryBackOff",
            "retriable_headers",
            "retriableHeaders",
            "retriable_request_headers",
            "retriableRequestHeaders",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RetryOn,
            NumRetries,
            PerTryTimeout,
            PerTryIdleTimeout,
            RetryPriority,
            RetryHostPredicate,
            RetryOptionsPredicates,
            HostSelectionRetryMaxAttempts,
            RetriableStatusCodes,
            RetryBackOff,
            RateLimitedRetryBackOff,
            RetriableHeaders,
            RetriableRequestHeaders,
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
                            "retryOn" | "retry_on" => Ok(GeneratedField::RetryOn),
                            "numRetries" | "num_retries" => Ok(GeneratedField::NumRetries),
                            "perTryTimeout" | "per_try_timeout" => Ok(GeneratedField::PerTryTimeout),
                            "perTryIdleTimeout" | "per_try_idle_timeout" => Ok(GeneratedField::PerTryIdleTimeout),
                            "retryPriority" | "retry_priority" => Ok(GeneratedField::RetryPriority),
                            "retryHostPredicate" | "retry_host_predicate" => Ok(GeneratedField::RetryHostPredicate),
                            "retryOptionsPredicates" | "retry_options_predicates" => Ok(GeneratedField::RetryOptionsPredicates),
                            "hostSelectionRetryMaxAttempts" | "host_selection_retry_max_attempts" => Ok(GeneratedField::HostSelectionRetryMaxAttempts),
                            "retriableStatusCodes" | "retriable_status_codes" => Ok(GeneratedField::RetriableStatusCodes),
                            "retryBackOff" | "retry_back_off" => Ok(GeneratedField::RetryBackOff),
                            "rateLimitedRetryBackOff" | "rate_limited_retry_back_off" => Ok(GeneratedField::RateLimitedRetryBackOff),
                            "retriableHeaders" | "retriable_headers" => Ok(GeneratedField::RetriableHeaders),
                            "retriableRequestHeaders" | "retriable_request_headers" => Ok(GeneratedField::RetriableRequestHeaders),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RetryPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RetryPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RetryPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut retry_on__ = None;
                let mut num_retries__ = None;
                let mut per_try_timeout__ = None;
                let mut per_try_idle_timeout__ = None;
                let mut retry_priority__ = None;
                let mut retry_host_predicate__ = None;
                let mut retry_options_predicates__ = None;
                let mut host_selection_retry_max_attempts__ = None;
                let mut retriable_status_codes__ = None;
                let mut retry_back_off__ = None;
                let mut rate_limited_retry_back_off__ = None;
                let mut retriable_headers__ = None;
                let mut retriable_request_headers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RetryOn => {
                            if retry_on__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryOn"));
                            }
                            retry_on__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NumRetries => {
                            if num_retries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numRetries"));
                            }
                            num_retries__ = map_.next_value()?;
                        }
                        GeneratedField::PerTryTimeout => {
                            if per_try_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perTryTimeout"));
                            }
                            per_try_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::PerTryIdleTimeout => {
                            if per_try_idle_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perTryIdleTimeout"));
                            }
                            per_try_idle_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::RetryPriority => {
                            if retry_priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryPriority"));
                            }
                            retry_priority__ = map_.next_value()?;
                        }
                        GeneratedField::RetryHostPredicate => {
                            if retry_host_predicate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryHostPredicate"));
                            }
                            retry_host_predicate__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetryOptionsPredicates => {
                            if retry_options_predicates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryOptionsPredicates"));
                            }
                            retry_options_predicates__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HostSelectionRetryMaxAttempts => {
                            if host_selection_retry_max_attempts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostSelectionRetryMaxAttempts"));
                            }
                            host_selection_retry_max_attempts__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RetriableStatusCodes => {
                            if retriable_status_codes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retriableStatusCodes"));
                            }
                            retriable_status_codes__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::RetryBackOff => {
                            if retry_back_off__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryBackOff"));
                            }
                            retry_back_off__ = map_.next_value()?;
                        }
                        GeneratedField::RateLimitedRetryBackOff => {
                            if rate_limited_retry_back_off__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimitedRetryBackOff"));
                            }
                            rate_limited_retry_back_off__ = map_.next_value()?;
                        }
                        GeneratedField::RetriableHeaders => {
                            if retriable_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retriableHeaders"));
                            }
                            retriable_headers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetriableRequestHeaders => {
                            if retriable_request_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retriableRequestHeaders"));
                            }
                            retriable_request_headers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RetryPolicy {
                    retry_on: retry_on__.unwrap_or_default(),
                    num_retries: num_retries__,
                    per_try_timeout: per_try_timeout__,
                    per_try_idle_timeout: per_try_idle_timeout__,
                    retry_priority: retry_priority__,
                    retry_host_predicate: retry_host_predicate__.unwrap_or_default(),
                    retry_options_predicates: retry_options_predicates__.unwrap_or_default(),
                    host_selection_retry_max_attempts: host_selection_retry_max_attempts__.unwrap_or_default(),
                    retriable_status_codes: retriable_status_codes__.unwrap_or_default(),
                    retry_back_off: retry_back_off__,
                    rate_limited_retry_back_off: rate_limited_retry_back_off__,
                    retriable_headers: retriable_headers__.unwrap_or_default(),
                    retriable_request_headers: retriable_request_headers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RetryPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for retry_policy::RateLimitedRetryBackOff {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reset_headers.is_empty() {
            len += 1;
        }
        if self.max_interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RetryPolicy.RateLimitedRetryBackOff", len)?;
        if !self.reset_headers.is_empty() {
            struct_ser.serialize_field("reset_headers", &self.reset_headers)?;
        }
        if let Some(v) = self.max_interval.as_ref() {
            struct_ser.serialize_field("max_interval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for retry_policy::RateLimitedRetryBackOff {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reset_headers",
            "resetHeaders",
            "max_interval",
            "maxInterval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResetHeaders,
            MaxInterval,
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
                            "resetHeaders" | "reset_headers" => Ok(GeneratedField::ResetHeaders),
                            "maxInterval" | "max_interval" => Ok(GeneratedField::MaxInterval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = retry_policy::RateLimitedRetryBackOff;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RetryPolicy.RateLimitedRetryBackOff")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<retry_policy::RateLimitedRetryBackOff, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reset_headers__ = None;
                let mut max_interval__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResetHeaders => {
                            if reset_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resetHeaders"));
                            }
                            reset_headers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxInterval => {
                            if max_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxInterval"));
                            }
                            max_interval__ = map_.next_value()?;
                        }
                    }
                }
                Ok(retry_policy::RateLimitedRetryBackOff {
                    reset_headers: reset_headers__.unwrap_or_default(),
                    max_interval: max_interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RetryPolicy.RateLimitedRetryBackOff", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for retry_policy::ResetHeader {
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
        if self.format != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RetryPolicy.ResetHeader", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.format != 0 {
            let v = retry_policy::ResetHeaderFormat::try_from(self.format)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.format)))?;
            struct_ser.serialize_field("format", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for retry_policy::ResetHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "format",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Format,
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
                            "format" => Ok(GeneratedField::Format),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = retry_policy::ResetHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RetryPolicy.ResetHeader")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<retry_policy::ResetHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut format__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Format => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            format__ = Some(map_.next_value::<retry_policy::ResetHeaderFormat>()? as i32);
                        }
                    }
                }
                Ok(retry_policy::ResetHeader {
                    name: name__.unwrap_or_default(),
                    format: format__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RetryPolicy.ResetHeader", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for retry_policy::ResetHeaderFormat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Seconds => "SECONDS",
            Self::UnixTimestamp => "UNIX_TIMESTAMP",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for retry_policy::ResetHeaderFormat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SECONDS",
            "UNIX_TIMESTAMP",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = retry_policy::ResetHeaderFormat;

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
                    "SECONDS" => Ok(retry_policy::ResetHeaderFormat::Seconds),
                    "UNIX_TIMESTAMP" => Ok(retry_policy::ResetHeaderFormat::UnixTimestamp),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for retry_policy::RetryBackOff {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base_interval.is_some() {
            len += 1;
        }
        if self.max_interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RetryPolicy.RetryBackOff", len)?;
        if let Some(v) = self.base_interval.as_ref() {
            struct_ser.serialize_field("base_interval", v)?;
        }
        if let Some(v) = self.max_interval.as_ref() {
            struct_ser.serialize_field("max_interval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for retry_policy::RetryBackOff {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_interval",
            "baseInterval",
            "max_interval",
            "maxInterval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseInterval,
            MaxInterval,
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
                            "baseInterval" | "base_interval" => Ok(GeneratedField::BaseInterval),
                            "maxInterval" | "max_interval" => Ok(GeneratedField::MaxInterval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = retry_policy::RetryBackOff;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RetryPolicy.RetryBackOff")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<retry_policy::RetryBackOff, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_interval__ = None;
                let mut max_interval__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseInterval => {
                            if base_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseInterval"));
                            }
                            base_interval__ = map_.next_value()?;
                        }
                        GeneratedField::MaxInterval => {
                            if max_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxInterval"));
                            }
                            max_interval__ = map_.next_value()?;
                        }
                    }
                }
                Ok(retry_policy::RetryBackOff {
                    base_interval: base_interval__,
                    max_interval: max_interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RetryPolicy.RetryBackOff", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for retry_policy::RetryHostPredicate {
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RetryPolicy.RetryHostPredicate", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                retry_policy::retry_host_predicate::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typed_config", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for retry_policy::RetryHostPredicate {
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
            type Value = retry_policy::RetryHostPredicate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RetryPolicy.RetryHostPredicate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<retry_policy::RetryHostPredicate, V::Error>
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
                            config_type__ = map_.next_value::<::std::option::Option<_>>()?.map(retry_policy::retry_host_predicate::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(retry_policy::RetryHostPredicate {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RetryPolicy.RetryHostPredicate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for retry_policy::RetryPriority {
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RetryPolicy.RetryPriority", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                retry_policy::retry_priority::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typed_config", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for retry_policy::RetryPriority {
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
            type Value = retry_policy::RetryPriority;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RetryPolicy.RetryPriority")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<retry_policy::RetryPriority, V::Error>
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
                            config_type__ = map_.next_value::<::std::option::Option<_>>()?.map(retry_policy::retry_priority::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(retry_policy::RetryPriority {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RetryPolicy.RetryPriority", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Route {
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
        if self.r#match.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.decorator.is_some() {
            len += 1;
        }
        if !self.typed_per_filter_config.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_remove.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_remove.is_empty() {
            len += 1;
        }
        if self.tracing.is_some() {
            len += 1;
        }
        if self.per_request_buffer_limit_bytes.is_some() {
            len += 1;
        }
        if !self.stat_prefix.is_empty() {
            len += 1;
        }
        if self.action.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.Route", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.r#match.as_ref() {
            struct_ser.serialize_field("match", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.decorator.as_ref() {
            struct_ser.serialize_field("decorator", v)?;
        }
        if !self.typed_per_filter_config.is_empty() {
            struct_ser.serialize_field("typed_per_filter_config", &self.typed_per_filter_config)?;
        }
        if !self.request_headers_to_add.is_empty() {
            struct_ser.serialize_field("request_headers_to_add", &self.request_headers_to_add)?;
        }
        if !self.request_headers_to_remove.is_empty() {
            struct_ser.serialize_field("request_headers_to_remove", &self.request_headers_to_remove)?;
        }
        if !self.response_headers_to_add.is_empty() {
            struct_ser.serialize_field("response_headers_to_add", &self.response_headers_to_add)?;
        }
        if !self.response_headers_to_remove.is_empty() {
            struct_ser.serialize_field("response_headers_to_remove", &self.response_headers_to_remove)?;
        }
        if let Some(v) = self.tracing.as_ref() {
            struct_ser.serialize_field("tracing", v)?;
        }
        if let Some(v) = self.per_request_buffer_limit_bytes.as_ref() {
            struct_ser.serialize_field("per_request_buffer_limit_bytes", v)?;
        }
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("stat_prefix", &self.stat_prefix)?;
        }
        if let Some(v) = self.action.as_ref() {
            match v {
                route::Action::Route(v) => {
                    struct_ser.serialize_field("route", v)?;
                }
                route::Action::Redirect(v) => {
                    struct_ser.serialize_field("redirect", v)?;
                }
                route::Action::DirectResponse(v) => {
                    struct_ser.serialize_field("direct_response", v)?;
                }
                route::Action::FilterAction(v) => {
                    struct_ser.serialize_field("filter_action", v)?;
                }
                route::Action::NonForwardingAction(v) => {
                    struct_ser.serialize_field("non_forwarding_action", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Route {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "match",
            "metadata",
            "decorator",
            "typed_per_filter_config",
            "typedPerFilterConfig",
            "request_headers_to_add",
            "requestHeadersToAdd",
            "request_headers_to_remove",
            "requestHeadersToRemove",
            "response_headers_to_add",
            "responseHeadersToAdd",
            "response_headers_to_remove",
            "responseHeadersToRemove",
            "tracing",
            "per_request_buffer_limit_bytes",
            "perRequestBufferLimitBytes",
            "stat_prefix",
            "statPrefix",
            "route",
            "redirect",
            "direct_response",
            "directResponse",
            "filter_action",
            "filterAction",
            "non_forwarding_action",
            "nonForwardingAction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Match,
            Metadata,
            Decorator,
            TypedPerFilterConfig,
            RequestHeadersToAdd,
            RequestHeadersToRemove,
            ResponseHeadersToAdd,
            ResponseHeadersToRemove,
            Tracing,
            PerRequestBufferLimitBytes,
            StatPrefix,
            Route,
            Redirect,
            DirectResponse,
            FilterAction,
            NonForwardingAction,
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
                            "match" => Ok(GeneratedField::Match),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "decorator" => Ok(GeneratedField::Decorator),
                            "typedPerFilterConfig" | "typed_per_filter_config" => Ok(GeneratedField::TypedPerFilterConfig),
                            "requestHeadersToAdd" | "request_headers_to_add" => Ok(GeneratedField::RequestHeadersToAdd),
                            "requestHeadersToRemove" | "request_headers_to_remove" => Ok(GeneratedField::RequestHeadersToRemove),
                            "responseHeadersToAdd" | "response_headers_to_add" => Ok(GeneratedField::ResponseHeadersToAdd),
                            "responseHeadersToRemove" | "response_headers_to_remove" => Ok(GeneratedField::ResponseHeadersToRemove),
                            "tracing" => Ok(GeneratedField::Tracing),
                            "perRequestBufferLimitBytes" | "per_request_buffer_limit_bytes" => Ok(GeneratedField::PerRequestBufferLimitBytes),
                            "statPrefix" | "stat_prefix" => Ok(GeneratedField::StatPrefix),
                            "route" => Ok(GeneratedField::Route),
                            "redirect" => Ok(GeneratedField::Redirect),
                            "directResponse" | "direct_response" => Ok(GeneratedField::DirectResponse),
                            "filterAction" | "filter_action" => Ok(GeneratedField::FilterAction),
                            "nonForwardingAction" | "non_forwarding_action" => Ok(GeneratedField::NonForwardingAction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Route;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.Route")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Route, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#match__ = None;
                let mut metadata__ = None;
                let mut decorator__ = None;
                let mut typed_per_filter_config__ = None;
                let mut request_headers_to_add__ = None;
                let mut request_headers_to_remove__ = None;
                let mut response_headers_to_add__ = None;
                let mut response_headers_to_remove__ = None;
                let mut tracing__ = None;
                let mut per_request_buffer_limit_bytes__ = None;
                let mut stat_prefix__ = None;
                let mut action__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Match => {
                            if r#match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            r#match__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Decorator => {
                            if decorator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decorator"));
                            }
                            decorator__ = map_.next_value()?;
                        }
                        GeneratedField::TypedPerFilterConfig => {
                            if typed_per_filter_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedPerFilterConfig"));
                            }
                            typed_per_filter_config__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::RequestHeadersToAdd => {
                            if request_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToAdd"));
                            }
                            request_headers_to_add__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestHeadersToRemove => {
                            if request_headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToRemove"));
                            }
                            request_headers_to_remove__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToAdd => {
                            if response_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToAdd"));
                            }
                            response_headers_to_add__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToRemove => {
                            if response_headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToRemove"));
                            }
                            response_headers_to_remove__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tracing => {
                            if tracing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracing"));
                            }
                            tracing__ = map_.next_value()?;
                        }
                        GeneratedField::PerRequestBufferLimitBytes => {
                            if per_request_buffer_limit_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perRequestBufferLimitBytes"));
                            }
                            per_request_buffer_limit_bytes__ = map_.next_value()?;
                        }
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Route => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("route"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(route::Action::Route)
;
                        }
                        GeneratedField::Redirect => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redirect"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(route::Action::Redirect)
;
                        }
                        GeneratedField::DirectResponse => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directResponse"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(route::Action::DirectResponse)
;
                        }
                        GeneratedField::FilterAction => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterAction"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(route::Action::FilterAction)
;
                        }
                        GeneratedField::NonForwardingAction => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonForwardingAction"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(route::Action::NonForwardingAction)
;
                        }
                    }
                }
                Ok(Route {
                    name: name__.unwrap_or_default(),
                    r#match: r#match__,
                    metadata: metadata__,
                    decorator: decorator__,
                    typed_per_filter_config: typed_per_filter_config__.unwrap_or_default(),
                    request_headers_to_add: request_headers_to_add__.unwrap_or_default(),
                    request_headers_to_remove: request_headers_to_remove__.unwrap_or_default(),
                    response_headers_to_add: response_headers_to_add__.unwrap_or_default(),
                    response_headers_to_remove: response_headers_to_remove__.unwrap_or_default(),
                    tracing: tracing__,
                    per_request_buffer_limit_bytes: per_request_buffer_limit_bytes__,
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    action: action__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.Route", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cluster_not_found_response_code != 0 {
            len += 1;
        }
        if self.metadata_match.is_some() {
            len += 1;
        }
        if !self.prefix_rewrite.is_empty() {
            len += 1;
        }
        if self.regex_rewrite.is_some() {
            len += 1;
        }
        if self.path_rewrite_policy.is_some() {
            len += 1;
        }
        if self.append_x_forwarded_host {
            len += 1;
        }
        if self.timeout.is_some() {
            len += 1;
        }
        if self.idle_timeout.is_some() {
            len += 1;
        }
        if self.early_data_policy.is_some() {
            len += 1;
        }
        if self.retry_policy.is_some() {
            len += 1;
        }
        if self.retry_policy_typed_config.is_some() {
            len += 1;
        }
        if !self.request_mirror_policies.is_empty() {
            len += 1;
        }
        if self.priority != 0 {
            len += 1;
        }
        if !self.rate_limits.is_empty() {
            len += 1;
        }
        if self.include_vh_rate_limits.is_some() {
            len += 1;
        }
        if !self.hash_policy.is_empty() {
            len += 1;
        }
        if self.cors.is_some() {
            len += 1;
        }
        if self.max_grpc_timeout.is_some() {
            len += 1;
        }
        if self.grpc_timeout_offset.is_some() {
            len += 1;
        }
        if !self.upgrade_configs.is_empty() {
            len += 1;
        }
        if self.internal_redirect_policy.is_some() {
            len += 1;
        }
        if self.internal_redirect_action != 0 {
            len += 1;
        }
        if self.max_internal_redirects.is_some() {
            len += 1;
        }
        if self.hedge_policy.is_some() {
            len += 1;
        }
        if self.max_stream_duration.is_some() {
            len += 1;
        }
        if self.cluster_specifier.is_some() {
            len += 1;
        }
        if self.host_rewrite_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteAction", len)?;
        if self.cluster_not_found_response_code != 0 {
            let v = route_action::ClusterNotFoundResponseCode::try_from(self.cluster_not_found_response_code)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.cluster_not_found_response_code)))?;
            struct_ser.serialize_field("cluster_not_found_response_code", &v)?;
        }
        if let Some(v) = self.metadata_match.as_ref() {
            struct_ser.serialize_field("metadata_match", v)?;
        }
        if !self.prefix_rewrite.is_empty() {
            struct_ser.serialize_field("prefix_rewrite", &self.prefix_rewrite)?;
        }
        if let Some(v) = self.regex_rewrite.as_ref() {
            struct_ser.serialize_field("regex_rewrite", v)?;
        }
        if let Some(v) = self.path_rewrite_policy.as_ref() {
            struct_ser.serialize_field("path_rewrite_policy", v)?;
        }
        if self.append_x_forwarded_host {
            struct_ser.serialize_field("append_x_forwarded_host", &self.append_x_forwarded_host)?;
        }
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        if let Some(v) = self.idle_timeout.as_ref() {
            struct_ser.serialize_field("idle_timeout", v)?;
        }
        if let Some(v) = self.early_data_policy.as_ref() {
            struct_ser.serialize_field("early_data_policy", v)?;
        }
        if let Some(v) = self.retry_policy.as_ref() {
            struct_ser.serialize_field("retry_policy", v)?;
        }
        if let Some(v) = self.retry_policy_typed_config.as_ref() {
            struct_ser.serialize_field("retry_policy_typed_config", v)?;
        }
        if !self.request_mirror_policies.is_empty() {
            struct_ser.serialize_field("request_mirror_policies", &self.request_mirror_policies)?;
        }
        if self.priority != 0 {
            let v = super::super::core::v3::RoutingPriority::try_from(self.priority)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.priority)))?;
            struct_ser.serialize_field("priority", &v)?;
        }
        if !self.rate_limits.is_empty() {
            struct_ser.serialize_field("rate_limits", &self.rate_limits)?;
        }
        if let Some(v) = self.include_vh_rate_limits.as_ref() {
            struct_ser.serialize_field("include_vh_rate_limits", v)?;
        }
        if !self.hash_policy.is_empty() {
            struct_ser.serialize_field("hash_policy", &self.hash_policy)?;
        }
        if let Some(v) = self.cors.as_ref() {
            struct_ser.serialize_field("cors", v)?;
        }
        if let Some(v) = self.max_grpc_timeout.as_ref() {
            struct_ser.serialize_field("max_grpc_timeout", v)?;
        }
        if let Some(v) = self.grpc_timeout_offset.as_ref() {
            struct_ser.serialize_field("grpc_timeout_offset", v)?;
        }
        if !self.upgrade_configs.is_empty() {
            struct_ser.serialize_field("upgrade_configs", &self.upgrade_configs)?;
        }
        if let Some(v) = self.internal_redirect_policy.as_ref() {
            struct_ser.serialize_field("internal_redirect_policy", v)?;
        }
        if self.internal_redirect_action != 0 {
            let v = route_action::InternalRedirectAction::try_from(self.internal_redirect_action)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.internal_redirect_action)))?;
            struct_ser.serialize_field("internal_redirect_action", &v)?;
        }
        if let Some(v) = self.max_internal_redirects.as_ref() {
            struct_ser.serialize_field("max_internal_redirects", v)?;
        }
        if let Some(v) = self.hedge_policy.as_ref() {
            struct_ser.serialize_field("hedge_policy", v)?;
        }
        if let Some(v) = self.max_stream_duration.as_ref() {
            struct_ser.serialize_field("max_stream_duration", v)?;
        }
        if let Some(v) = self.cluster_specifier.as_ref() {
            match v {
                route_action::ClusterSpecifier::Cluster(v) => {
                    struct_ser.serialize_field("cluster", v)?;
                }
                route_action::ClusterSpecifier::ClusterHeader(v) => {
                    struct_ser.serialize_field("cluster_header", v)?;
                }
                route_action::ClusterSpecifier::WeightedClusters(v) => {
                    struct_ser.serialize_field("weighted_clusters", v)?;
                }
                route_action::ClusterSpecifier::ClusterSpecifierPlugin(v) => {
                    struct_ser.serialize_field("cluster_specifier_plugin", v)?;
                }
                route_action::ClusterSpecifier::InlineClusterSpecifierPlugin(v) => {
                    struct_ser.serialize_field("inline_cluster_specifier_plugin", v)?;
                }
            }
        }
        if let Some(v) = self.host_rewrite_specifier.as_ref() {
            match v {
                route_action::HostRewriteSpecifier::HostRewriteLiteral(v) => {
                    struct_ser.serialize_field("host_rewrite_literal", v)?;
                }
                route_action::HostRewriteSpecifier::AutoHostRewrite(v) => {
                    struct_ser.serialize_field("auto_host_rewrite", v)?;
                }
                route_action::HostRewriteSpecifier::HostRewriteHeader(v) => {
                    struct_ser.serialize_field("host_rewrite_header", v)?;
                }
                route_action::HostRewriteSpecifier::HostRewritePathRegex(v) => {
                    struct_ser.serialize_field("host_rewrite_path_regex", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_not_found_response_code",
            "clusterNotFoundResponseCode",
            "metadata_match",
            "metadataMatch",
            "prefix_rewrite",
            "prefixRewrite",
            "regex_rewrite",
            "regexRewrite",
            "path_rewrite_policy",
            "pathRewritePolicy",
            "append_x_forwarded_host",
            "appendXForwardedHost",
            "timeout",
            "idle_timeout",
            "idleTimeout",
            "early_data_policy",
            "earlyDataPolicy",
            "retry_policy",
            "retryPolicy",
            "retry_policy_typed_config",
            "retryPolicyTypedConfig",
            "request_mirror_policies",
            "requestMirrorPolicies",
            "priority",
            "rate_limits",
            "rateLimits",
            "include_vh_rate_limits",
            "includeVhRateLimits",
            "hash_policy",
            "hashPolicy",
            "cors",
            "max_grpc_timeout",
            "maxGrpcTimeout",
            "grpc_timeout_offset",
            "grpcTimeoutOffset",
            "upgrade_configs",
            "upgradeConfigs",
            "internal_redirect_policy",
            "internalRedirectPolicy",
            "internal_redirect_action",
            "internalRedirectAction",
            "max_internal_redirects",
            "maxInternalRedirects",
            "hedge_policy",
            "hedgePolicy",
            "max_stream_duration",
            "maxStreamDuration",
            "cluster",
            "cluster_header",
            "clusterHeader",
            "weighted_clusters",
            "weightedClusters",
            "cluster_specifier_plugin",
            "clusterSpecifierPlugin",
            "inline_cluster_specifier_plugin",
            "inlineClusterSpecifierPlugin",
            "host_rewrite_literal",
            "hostRewriteLiteral",
            "auto_host_rewrite",
            "autoHostRewrite",
            "host_rewrite_header",
            "hostRewriteHeader",
            "host_rewrite_path_regex",
            "hostRewritePathRegex",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterNotFoundResponseCode,
            MetadataMatch,
            PrefixRewrite,
            RegexRewrite,
            PathRewritePolicy,
            AppendXForwardedHost,
            Timeout,
            IdleTimeout,
            EarlyDataPolicy,
            RetryPolicy,
            RetryPolicyTypedConfig,
            RequestMirrorPolicies,
            Priority,
            RateLimits,
            IncludeVhRateLimits,
            HashPolicy,
            Cors,
            MaxGrpcTimeout,
            GrpcTimeoutOffset,
            UpgradeConfigs,
            InternalRedirectPolicy,
            InternalRedirectAction,
            MaxInternalRedirects,
            HedgePolicy,
            MaxStreamDuration,
            Cluster,
            ClusterHeader,
            WeightedClusters,
            ClusterSpecifierPlugin,
            InlineClusterSpecifierPlugin,
            HostRewriteLiteral,
            AutoHostRewrite,
            HostRewriteHeader,
            HostRewritePathRegex,
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
                            "clusterNotFoundResponseCode" | "cluster_not_found_response_code" => Ok(GeneratedField::ClusterNotFoundResponseCode),
                            "metadataMatch" | "metadata_match" => Ok(GeneratedField::MetadataMatch),
                            "prefixRewrite" | "prefix_rewrite" => Ok(GeneratedField::PrefixRewrite),
                            "regexRewrite" | "regex_rewrite" => Ok(GeneratedField::RegexRewrite),
                            "pathRewritePolicy" | "path_rewrite_policy" => Ok(GeneratedField::PathRewritePolicy),
                            "appendXForwardedHost" | "append_x_forwarded_host" => Ok(GeneratedField::AppendXForwardedHost),
                            "timeout" => Ok(GeneratedField::Timeout),
                            "idleTimeout" | "idle_timeout" => Ok(GeneratedField::IdleTimeout),
                            "earlyDataPolicy" | "early_data_policy" => Ok(GeneratedField::EarlyDataPolicy),
                            "retryPolicy" | "retry_policy" => Ok(GeneratedField::RetryPolicy),
                            "retryPolicyTypedConfig" | "retry_policy_typed_config" => Ok(GeneratedField::RetryPolicyTypedConfig),
                            "requestMirrorPolicies" | "request_mirror_policies" => Ok(GeneratedField::RequestMirrorPolicies),
                            "priority" => Ok(GeneratedField::Priority),
                            "rateLimits" | "rate_limits" => Ok(GeneratedField::RateLimits),
                            "includeVhRateLimits" | "include_vh_rate_limits" => Ok(GeneratedField::IncludeVhRateLimits),
                            "hashPolicy" | "hash_policy" => Ok(GeneratedField::HashPolicy),
                            "cors" => Ok(GeneratedField::Cors),
                            "maxGrpcTimeout" | "max_grpc_timeout" => Ok(GeneratedField::MaxGrpcTimeout),
                            "grpcTimeoutOffset" | "grpc_timeout_offset" => Ok(GeneratedField::GrpcTimeoutOffset),
                            "upgradeConfigs" | "upgrade_configs" => Ok(GeneratedField::UpgradeConfigs),
                            "internalRedirectPolicy" | "internal_redirect_policy" => Ok(GeneratedField::InternalRedirectPolicy),
                            "internalRedirectAction" | "internal_redirect_action" => Ok(GeneratedField::InternalRedirectAction),
                            "maxInternalRedirects" | "max_internal_redirects" => Ok(GeneratedField::MaxInternalRedirects),
                            "hedgePolicy" | "hedge_policy" => Ok(GeneratedField::HedgePolicy),
                            "maxStreamDuration" | "max_stream_duration" => Ok(GeneratedField::MaxStreamDuration),
                            "cluster" => Ok(GeneratedField::Cluster),
                            "clusterHeader" | "cluster_header" => Ok(GeneratedField::ClusterHeader),
                            "weightedClusters" | "weighted_clusters" => Ok(GeneratedField::WeightedClusters),
                            "clusterSpecifierPlugin" | "cluster_specifier_plugin" => Ok(GeneratedField::ClusterSpecifierPlugin),
                            "inlineClusterSpecifierPlugin" | "inline_cluster_specifier_plugin" => Ok(GeneratedField::InlineClusterSpecifierPlugin),
                            "hostRewriteLiteral" | "host_rewrite_literal" => Ok(GeneratedField::HostRewriteLiteral),
                            "autoHostRewrite" | "auto_host_rewrite" => Ok(GeneratedField::AutoHostRewrite),
                            "hostRewriteHeader" | "host_rewrite_header" => Ok(GeneratedField::HostRewriteHeader),
                            "hostRewritePathRegex" | "host_rewrite_path_regex" => Ok(GeneratedField::HostRewritePathRegex),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RouteAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_not_found_response_code__ = None;
                let mut metadata_match__ = None;
                let mut prefix_rewrite__ = None;
                let mut regex_rewrite__ = None;
                let mut path_rewrite_policy__ = None;
                let mut append_x_forwarded_host__ = None;
                let mut timeout__ = None;
                let mut idle_timeout__ = None;
                let mut early_data_policy__ = None;
                let mut retry_policy__ = None;
                let mut retry_policy_typed_config__ = None;
                let mut request_mirror_policies__ = None;
                let mut priority__ = None;
                let mut rate_limits__ = None;
                let mut include_vh_rate_limits__ = None;
                let mut hash_policy__ = None;
                let mut cors__ = None;
                let mut max_grpc_timeout__ = None;
                let mut grpc_timeout_offset__ = None;
                let mut upgrade_configs__ = None;
                let mut internal_redirect_policy__ = None;
                let mut internal_redirect_action__ = None;
                let mut max_internal_redirects__ = None;
                let mut hedge_policy__ = None;
                let mut max_stream_duration__ = None;
                let mut cluster_specifier__ = None;
                let mut host_rewrite_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClusterNotFoundResponseCode => {
                            if cluster_not_found_response_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterNotFoundResponseCode"));
                            }
                            cluster_not_found_response_code__ = Some(map_.next_value::<route_action::ClusterNotFoundResponseCode>()? as i32);
                        }
                        GeneratedField::MetadataMatch => {
                            if metadata_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataMatch"));
                            }
                            metadata_match__ = map_.next_value()?;
                        }
                        GeneratedField::PrefixRewrite => {
                            if prefix_rewrite__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefixRewrite"));
                            }
                            prefix_rewrite__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RegexRewrite => {
                            if regex_rewrite__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regexRewrite"));
                            }
                            regex_rewrite__ = map_.next_value()?;
                        }
                        GeneratedField::PathRewritePolicy => {
                            if path_rewrite_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathRewritePolicy"));
                            }
                            path_rewrite_policy__ = map_.next_value()?;
                        }
                        GeneratedField::AppendXForwardedHost => {
                            if append_x_forwarded_host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appendXForwardedHost"));
                            }
                            append_x_forwarded_host__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = map_.next_value()?;
                        }
                        GeneratedField::IdleTimeout => {
                            if idle_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idleTimeout"));
                            }
                            idle_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::EarlyDataPolicy => {
                            if early_data_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("earlyDataPolicy"));
                            }
                            early_data_policy__ = map_.next_value()?;
                        }
                        GeneratedField::RetryPolicy => {
                            if retry_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryPolicy"));
                            }
                            retry_policy__ = map_.next_value()?;
                        }
                        GeneratedField::RetryPolicyTypedConfig => {
                            if retry_policy_typed_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryPolicyTypedConfig"));
                            }
                            retry_policy_typed_config__ = map_.next_value()?;
                        }
                        GeneratedField::RequestMirrorPolicies => {
                            if request_mirror_policies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestMirrorPolicies"));
                            }
                            request_mirror_policies__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = Some(map_.next_value::<super::super::core::v3::RoutingPriority>()? as i32);
                        }
                        GeneratedField::RateLimits => {
                            if rate_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimits"));
                            }
                            rate_limits__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IncludeVhRateLimits => {
                            if include_vh_rate_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeVhRateLimits"));
                            }
                            include_vh_rate_limits__ = map_.next_value()?;
                        }
                        GeneratedField::HashPolicy => {
                            if hash_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashPolicy"));
                            }
                            hash_policy__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cors => {
                            if cors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cors"));
                            }
                            cors__ = map_.next_value()?;
                        }
                        GeneratedField::MaxGrpcTimeout => {
                            if max_grpc_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxGrpcTimeout"));
                            }
                            max_grpc_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::GrpcTimeoutOffset => {
                            if grpc_timeout_offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcTimeoutOffset"));
                            }
                            grpc_timeout_offset__ = map_.next_value()?;
                        }
                        GeneratedField::UpgradeConfigs => {
                            if upgrade_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upgradeConfigs"));
                            }
                            upgrade_configs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InternalRedirectPolicy => {
                            if internal_redirect_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("internalRedirectPolicy"));
                            }
                            internal_redirect_policy__ = map_.next_value()?;
                        }
                        GeneratedField::InternalRedirectAction => {
                            if internal_redirect_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("internalRedirectAction"));
                            }
                            internal_redirect_action__ = Some(map_.next_value::<route_action::InternalRedirectAction>()? as i32);
                        }
                        GeneratedField::MaxInternalRedirects => {
                            if max_internal_redirects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxInternalRedirects"));
                            }
                            max_internal_redirects__ = map_.next_value()?;
                        }
                        GeneratedField::HedgePolicy => {
                            if hedge_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hedgePolicy"));
                            }
                            hedge_policy__ = map_.next_value()?;
                        }
                        GeneratedField::MaxStreamDuration => {
                            if max_stream_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxStreamDuration"));
                            }
                            max_stream_duration__ = map_.next_value()?;
                        }
                        GeneratedField::Cluster => {
                            if cluster_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_action::ClusterSpecifier::Cluster);
                        }
                        GeneratedField::ClusterHeader => {
                            if cluster_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterHeader"));
                            }
                            cluster_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_action::ClusterSpecifier::ClusterHeader);
                        }
                        GeneratedField::WeightedClusters => {
                            if cluster_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weightedClusters"));
                            }
                            cluster_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_action::ClusterSpecifier::WeightedClusters)
;
                        }
                        GeneratedField::ClusterSpecifierPlugin => {
                            if cluster_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterSpecifierPlugin"));
                            }
                            cluster_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_action::ClusterSpecifier::ClusterSpecifierPlugin);
                        }
                        GeneratedField::InlineClusterSpecifierPlugin => {
                            if cluster_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inlineClusterSpecifierPlugin"));
                            }
                            cluster_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_action::ClusterSpecifier::InlineClusterSpecifierPlugin)
;
                        }
                        GeneratedField::HostRewriteLiteral => {
                            if host_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostRewriteLiteral"));
                            }
                            host_rewrite_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_action::HostRewriteSpecifier::HostRewriteLiteral);
                        }
                        GeneratedField::AutoHostRewrite => {
                            if host_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoHostRewrite"));
                            }
                            host_rewrite_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_action::HostRewriteSpecifier::AutoHostRewrite)
;
                        }
                        GeneratedField::HostRewriteHeader => {
                            if host_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostRewriteHeader"));
                            }
                            host_rewrite_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_action::HostRewriteSpecifier::HostRewriteHeader);
                        }
                        GeneratedField::HostRewritePathRegex => {
                            if host_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostRewritePathRegex"));
                            }
                            host_rewrite_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_action::HostRewriteSpecifier::HostRewritePathRegex)
;
                        }
                    }
                }
                Ok(RouteAction {
                    cluster_not_found_response_code: cluster_not_found_response_code__.unwrap_or_default(),
                    metadata_match: metadata_match__,
                    prefix_rewrite: prefix_rewrite__.unwrap_or_default(),
                    regex_rewrite: regex_rewrite__,
                    path_rewrite_policy: path_rewrite_policy__,
                    append_x_forwarded_host: append_x_forwarded_host__.unwrap_or_default(),
                    timeout: timeout__,
                    idle_timeout: idle_timeout__,
                    early_data_policy: early_data_policy__,
                    retry_policy: retry_policy__,
                    retry_policy_typed_config: retry_policy_typed_config__,
                    request_mirror_policies: request_mirror_policies__.unwrap_or_default(),
                    priority: priority__.unwrap_or_default(),
                    rate_limits: rate_limits__.unwrap_or_default(),
                    include_vh_rate_limits: include_vh_rate_limits__,
                    hash_policy: hash_policy__.unwrap_or_default(),
                    cors: cors__,
                    max_grpc_timeout: max_grpc_timeout__,
                    grpc_timeout_offset: grpc_timeout_offset__,
                    upgrade_configs: upgrade_configs__.unwrap_or_default(),
                    internal_redirect_policy: internal_redirect_policy__,
                    internal_redirect_action: internal_redirect_action__.unwrap_or_default(),
                    max_internal_redirects: max_internal_redirects__,
                    hedge_policy: hedge_policy__,
                    max_stream_duration: max_stream_duration__,
                    cluster_specifier: cluster_specifier__,
                    host_rewrite_specifier: host_rewrite_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::ClusterNotFoundResponseCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::ServiceUnavailable => "SERVICE_UNAVAILABLE",
            Self::NotFound => "NOT_FOUND",
            Self::InternalServerError => "INTERNAL_SERVER_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for route_action::ClusterNotFoundResponseCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SERVICE_UNAVAILABLE",
            "NOT_FOUND",
            "INTERNAL_SERVER_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::ClusterNotFoundResponseCode;

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
                    "SERVICE_UNAVAILABLE" => Ok(route_action::ClusterNotFoundResponseCode::ServiceUnavailable),
                    "NOT_FOUND" => Ok(route_action::ClusterNotFoundResponseCode::NotFound),
                    "INTERNAL_SERVER_ERROR" => Ok(route_action::ClusterNotFoundResponseCode::InternalServerError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::HashPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.terminal {
            len += 1;
        }
        if self.policy_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteAction.HashPolicy", len)?;
        if self.terminal {
            struct_ser.serialize_field("terminal", &self.terminal)?;
        }
        if let Some(v) = self.policy_specifier.as_ref() {
            match v {
                route_action::hash_policy::PolicySpecifier::Header(v) => {
                    struct_ser.serialize_field("header", v)?;
                }
                route_action::hash_policy::PolicySpecifier::Cookie(v) => {
                    struct_ser.serialize_field("cookie", v)?;
                }
                route_action::hash_policy::PolicySpecifier::ConnectionProperties(v) => {
                    struct_ser.serialize_field("connection_properties", v)?;
                }
                route_action::hash_policy::PolicySpecifier::QueryParameter(v) => {
                    struct_ser.serialize_field("query_parameter", v)?;
                }
                route_action::hash_policy::PolicySpecifier::FilterState(v) => {
                    struct_ser.serialize_field("filter_state", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::HashPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "terminal",
            "header",
            "cookie",
            "connection_properties",
            "connectionProperties",
            "query_parameter",
            "queryParameter",
            "filter_state",
            "filterState",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Terminal,
            Header,
            Cookie,
            ConnectionProperties,
            QueryParameter,
            FilterState,
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
                            "terminal" => Ok(GeneratedField::Terminal),
                            "header" => Ok(GeneratedField::Header),
                            "cookie" => Ok(GeneratedField::Cookie),
                            "connectionProperties" | "connection_properties" => Ok(GeneratedField::ConnectionProperties),
                            "queryParameter" | "query_parameter" => Ok(GeneratedField::QueryParameter),
                            "filterState" | "filter_state" => Ok(GeneratedField::FilterState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::HashPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteAction.HashPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<route_action::HashPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut terminal__ = None;
                let mut policy_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Terminal => {
                            if terminal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("terminal"));
                            }
                            terminal__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Header => {
                            if policy_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            policy_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_action::hash_policy::PolicySpecifier::Header)
;
                        }
                        GeneratedField::Cookie => {
                            if policy_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cookie"));
                            }
                            policy_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_action::hash_policy::PolicySpecifier::Cookie)
;
                        }
                        GeneratedField::ConnectionProperties => {
                            if policy_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionProperties"));
                            }
                            policy_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_action::hash_policy::PolicySpecifier::ConnectionProperties)
;
                        }
                        GeneratedField::QueryParameter => {
                            if policy_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryParameter"));
                            }
                            policy_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_action::hash_policy::PolicySpecifier::QueryParameter)
;
                        }
                        GeneratedField::FilterState => {
                            if policy_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterState"));
                            }
                            policy_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_action::hash_policy::PolicySpecifier::FilterState)
;
                        }
                    }
                }
                Ok(route_action::HashPolicy {
                    terminal: terminal__.unwrap_or_default(),
                    policy_specifier: policy_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteAction.HashPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::hash_policy::ConnectionProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source_ip {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteAction.HashPolicy.ConnectionProperties", len)?;
        if self.source_ip {
            struct_ser.serialize_field("source_ip", &self.source_ip)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::hash_policy::ConnectionProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_ip",
            "sourceIp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceIp,
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
                            "sourceIp" | "source_ip" => Ok(GeneratedField::SourceIp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::hash_policy::ConnectionProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteAction.HashPolicy.ConnectionProperties")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<route_action::hash_policy::ConnectionProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source_ip__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SourceIp => {
                            if source_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceIp"));
                            }
                            source_ip__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(route_action::hash_policy::ConnectionProperties {
                    source_ip: source_ip__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteAction.HashPolicy.ConnectionProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::hash_policy::Cookie {
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
        if self.ttl.is_some() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.attributes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteAction.HashPolicy.Cookie", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.ttl.as_ref() {
            struct_ser.serialize_field("ttl", v)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.attributes.is_empty() {
            struct_ser.serialize_field("attributes", &self.attributes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::hash_policy::Cookie {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "ttl",
            "path",
            "attributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Ttl,
            Path,
            Attributes,
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
                            "ttl" => Ok(GeneratedField::Ttl),
                            "path" => Ok(GeneratedField::Path),
                            "attributes" => Ok(GeneratedField::Attributes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::hash_policy::Cookie;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteAction.HashPolicy.Cookie")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<route_action::hash_policy::Cookie, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut ttl__ = None;
                let mut path__ = None;
                let mut attributes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Ttl => {
                            if ttl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ttl"));
                            }
                            ttl__ = map_.next_value()?;
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(route_action::hash_policy::Cookie {
                    name: name__.unwrap_or_default(),
                    ttl: ttl__,
                    path: path__.unwrap_or_default(),
                    attributes: attributes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteAction.HashPolicy.Cookie", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::hash_policy::CookieAttribute {
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
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteAction.HashPolicy.CookieAttribute", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::hash_policy::CookieAttribute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            "name" => Ok(GeneratedField::Name),
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
            type Value = route_action::hash_policy::CookieAttribute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteAction.HashPolicy.CookieAttribute")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<route_action::hash_policy::CookieAttribute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(route_action::hash_policy::CookieAttribute {
                    name: name__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteAction.HashPolicy.CookieAttribute", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::hash_policy::FilterState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteAction.HashPolicy.FilterState", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::hash_policy::FilterState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
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
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::hash_policy::FilterState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteAction.HashPolicy.FilterState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<route_action::hash_policy::FilterState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(route_action::hash_policy::FilterState {
                    key: key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteAction.HashPolicy.FilterState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::hash_policy::Header {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.header_name.is_empty() {
            len += 1;
        }
        if self.regex_rewrite.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteAction.HashPolicy.Header", len)?;
        if !self.header_name.is_empty() {
            struct_ser.serialize_field("header_name", &self.header_name)?;
        }
        if let Some(v) = self.regex_rewrite.as_ref() {
            struct_ser.serialize_field("regex_rewrite", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::hash_policy::Header {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header_name",
            "headerName",
            "regex_rewrite",
            "regexRewrite",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HeaderName,
            RegexRewrite,
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
                            "headerName" | "header_name" => Ok(GeneratedField::HeaderName),
                            "regexRewrite" | "regex_rewrite" => Ok(GeneratedField::RegexRewrite),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::hash_policy::Header;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteAction.HashPolicy.Header")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<route_action::hash_policy::Header, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header_name__ = None;
                let mut regex_rewrite__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HeaderName => {
                            if header_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerName"));
                            }
                            header_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RegexRewrite => {
                            if regex_rewrite__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regexRewrite"));
                            }
                            regex_rewrite__ = map_.next_value()?;
                        }
                    }
                }
                Ok(route_action::hash_policy::Header {
                    header_name: header_name__.unwrap_or_default(),
                    regex_rewrite: regex_rewrite__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteAction.HashPolicy.Header", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::hash_policy::QueryParameter {
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteAction.HashPolicy.QueryParameter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::hash_policy::QueryParameter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = route_action::hash_policy::QueryParameter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteAction.HashPolicy.QueryParameter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<route_action::hash_policy::QueryParameter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(route_action::hash_policy::QueryParameter {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteAction.HashPolicy.QueryParameter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::InternalRedirectAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::PassThroughInternalRedirect => "PASS_THROUGH_INTERNAL_REDIRECT",
            Self::HandleInternalRedirect => "HANDLE_INTERNAL_REDIRECT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for route_action::InternalRedirectAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PASS_THROUGH_INTERNAL_REDIRECT",
            "HANDLE_INTERNAL_REDIRECT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::InternalRedirectAction;

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
                    "PASS_THROUGH_INTERNAL_REDIRECT" => Ok(route_action::InternalRedirectAction::PassThroughInternalRedirect),
                    "HANDLE_INTERNAL_REDIRECT" => Ok(route_action::InternalRedirectAction::HandleInternalRedirect),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::MaxStreamDuration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_stream_duration.is_some() {
            len += 1;
        }
        if self.grpc_timeout_header_max.is_some() {
            len += 1;
        }
        if self.grpc_timeout_header_offset.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteAction.MaxStreamDuration", len)?;
        if let Some(v) = self.max_stream_duration.as_ref() {
            struct_ser.serialize_field("max_stream_duration", v)?;
        }
        if let Some(v) = self.grpc_timeout_header_max.as_ref() {
            struct_ser.serialize_field("grpc_timeout_header_max", v)?;
        }
        if let Some(v) = self.grpc_timeout_header_offset.as_ref() {
            struct_ser.serialize_field("grpc_timeout_header_offset", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::MaxStreamDuration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_stream_duration",
            "maxStreamDuration",
            "grpc_timeout_header_max",
            "grpcTimeoutHeaderMax",
            "grpc_timeout_header_offset",
            "grpcTimeoutHeaderOffset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxStreamDuration,
            GrpcTimeoutHeaderMax,
            GrpcTimeoutHeaderOffset,
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
                            "maxStreamDuration" | "max_stream_duration" => Ok(GeneratedField::MaxStreamDuration),
                            "grpcTimeoutHeaderMax" | "grpc_timeout_header_max" => Ok(GeneratedField::GrpcTimeoutHeaderMax),
                            "grpcTimeoutHeaderOffset" | "grpc_timeout_header_offset" => Ok(GeneratedField::GrpcTimeoutHeaderOffset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::MaxStreamDuration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteAction.MaxStreamDuration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<route_action::MaxStreamDuration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_stream_duration__ = None;
                let mut grpc_timeout_header_max__ = None;
                let mut grpc_timeout_header_offset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxStreamDuration => {
                            if max_stream_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxStreamDuration"));
                            }
                            max_stream_duration__ = map_.next_value()?;
                        }
                        GeneratedField::GrpcTimeoutHeaderMax => {
                            if grpc_timeout_header_max__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcTimeoutHeaderMax"));
                            }
                            grpc_timeout_header_max__ = map_.next_value()?;
                        }
                        GeneratedField::GrpcTimeoutHeaderOffset => {
                            if grpc_timeout_header_offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcTimeoutHeaderOffset"));
                            }
                            grpc_timeout_header_offset__ = map_.next_value()?;
                        }
                    }
                }
                Ok(route_action::MaxStreamDuration {
                    max_stream_duration: max_stream_duration__,
                    grpc_timeout_header_max: grpc_timeout_header_max__,
                    grpc_timeout_header_offset: grpc_timeout_header_offset__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteAction.MaxStreamDuration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::RequestMirrorPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster.is_empty() {
            len += 1;
        }
        if !self.cluster_header.is_empty() {
            len += 1;
        }
        if self.runtime_fraction.is_some() {
            len += 1;
        }
        if self.trace_sampled.is_some() {
            len += 1;
        }
        if self.disable_shadow_host_suffix_append {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteAction.RequestMirrorPolicy", len)?;
        if !self.cluster.is_empty() {
            struct_ser.serialize_field("cluster", &self.cluster)?;
        }
        if !self.cluster_header.is_empty() {
            struct_ser.serialize_field("cluster_header", &self.cluster_header)?;
        }
        if let Some(v) = self.runtime_fraction.as_ref() {
            struct_ser.serialize_field("runtime_fraction", v)?;
        }
        if let Some(v) = self.trace_sampled.as_ref() {
            struct_ser.serialize_field("trace_sampled", v)?;
        }
        if self.disable_shadow_host_suffix_append {
            struct_ser.serialize_field("disable_shadow_host_suffix_append", &self.disable_shadow_host_suffix_append)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::RequestMirrorPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster",
            "cluster_header",
            "clusterHeader",
            "runtime_fraction",
            "runtimeFraction",
            "trace_sampled",
            "traceSampled",
            "disable_shadow_host_suffix_append",
            "disableShadowHostSuffixAppend",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cluster,
            ClusterHeader,
            RuntimeFraction,
            TraceSampled,
            DisableShadowHostSuffixAppend,
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
                            "clusterHeader" | "cluster_header" => Ok(GeneratedField::ClusterHeader),
                            "runtimeFraction" | "runtime_fraction" => Ok(GeneratedField::RuntimeFraction),
                            "traceSampled" | "trace_sampled" => Ok(GeneratedField::TraceSampled),
                            "disableShadowHostSuffixAppend" | "disable_shadow_host_suffix_append" => Ok(GeneratedField::DisableShadowHostSuffixAppend),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::RequestMirrorPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteAction.RequestMirrorPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<route_action::RequestMirrorPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster__ = None;
                let mut cluster_header__ = None;
                let mut runtime_fraction__ = None;
                let mut trace_sampled__ = None;
                let mut disable_shadow_host_suffix_append__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cluster => {
                            if cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClusterHeader => {
                            if cluster_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterHeader"));
                            }
                            cluster_header__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RuntimeFraction => {
                            if runtime_fraction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeFraction"));
                            }
                            runtime_fraction__ = map_.next_value()?;
                        }
                        GeneratedField::TraceSampled => {
                            if trace_sampled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceSampled"));
                            }
                            trace_sampled__ = map_.next_value()?;
                        }
                        GeneratedField::DisableShadowHostSuffixAppend => {
                            if disable_shadow_host_suffix_append__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableShadowHostSuffixAppend"));
                            }
                            disable_shadow_host_suffix_append__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(route_action::RequestMirrorPolicy {
                    cluster: cluster__.unwrap_or_default(),
                    cluster_header: cluster_header__.unwrap_or_default(),
                    runtime_fraction: runtime_fraction__,
                    trace_sampled: trace_sampled__,
                    disable_shadow_host_suffix_append: disable_shadow_host_suffix_append__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteAction.RequestMirrorPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::UpgradeConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.upgrade_type.is_empty() {
            len += 1;
        }
        if self.enabled.is_some() {
            len += 1;
        }
        if self.connect_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteAction.UpgradeConfig", len)?;
        if !self.upgrade_type.is_empty() {
            struct_ser.serialize_field("upgrade_type", &self.upgrade_type)?;
        }
        if let Some(v) = self.enabled.as_ref() {
            struct_ser.serialize_field("enabled", v)?;
        }
        if let Some(v) = self.connect_config.as_ref() {
            struct_ser.serialize_field("connect_config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::UpgradeConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "upgrade_type",
            "upgradeType",
            "enabled",
            "connect_config",
            "connectConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UpgradeType,
            Enabled,
            ConnectConfig,
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
                            "upgradeType" | "upgrade_type" => Ok(GeneratedField::UpgradeType),
                            "enabled" => Ok(GeneratedField::Enabled),
                            "connectConfig" | "connect_config" => Ok(GeneratedField::ConnectConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::UpgradeConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteAction.UpgradeConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<route_action::UpgradeConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut upgrade_type__ = None;
                let mut enabled__ = None;
                let mut connect_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UpgradeType => {
                            if upgrade_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upgradeType"));
                            }
                            upgrade_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = map_.next_value()?;
                        }
                        GeneratedField::ConnectConfig => {
                            if connect_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectConfig"));
                            }
                            connect_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(route_action::UpgradeConfig {
                    upgrade_type: upgrade_type__.unwrap_or_default(),
                    enabled: enabled__,
                    connect_config: connect_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteAction.UpgradeConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::upgrade_config::ConnectConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.proxy_protocol_config.is_some() {
            len += 1;
        }
        if self.allow_post {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteAction.UpgradeConfig.ConnectConfig", len)?;
        if let Some(v) = self.proxy_protocol_config.as_ref() {
            struct_ser.serialize_field("proxy_protocol_config", v)?;
        }
        if self.allow_post {
            struct_ser.serialize_field("allow_post", &self.allow_post)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::upgrade_config::ConnectConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "proxy_protocol_config",
            "proxyProtocolConfig",
            "allow_post",
            "allowPost",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProxyProtocolConfig,
            AllowPost,
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
                            "proxyProtocolConfig" | "proxy_protocol_config" => Ok(GeneratedField::ProxyProtocolConfig),
                            "allowPost" | "allow_post" => Ok(GeneratedField::AllowPost),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::upgrade_config::ConnectConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteAction.UpgradeConfig.ConnectConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<route_action::upgrade_config::ConnectConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut proxy_protocol_config__ = None;
                let mut allow_post__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProxyProtocolConfig => {
                            if proxy_protocol_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proxyProtocolConfig"));
                            }
                            proxy_protocol_config__ = map_.next_value()?;
                        }
                        GeneratedField::AllowPost => {
                            if allow_post__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowPost"));
                            }
                            allow_post__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(route_action::upgrade_config::ConnectConfig {
                    proxy_protocol_config: proxy_protocol_config__,
                    allow_post: allow_post__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteAction.UpgradeConfig.ConnectConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteConfiguration {
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
        if !self.virtual_hosts.is_empty() {
            len += 1;
        }
        if self.vhds.is_some() {
            len += 1;
        }
        if !self.internal_only_headers.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_remove.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_remove.is_empty() {
            len += 1;
        }
        if self.most_specific_header_mutations_wins {
            len += 1;
        }
        if self.validate_clusters.is_some() {
            len += 1;
        }
        if self.max_direct_response_body_size_bytes.is_some() {
            len += 1;
        }
        if !self.cluster_specifier_plugins.is_empty() {
            len += 1;
        }
        if !self.request_mirror_policies.is_empty() {
            len += 1;
        }
        if self.ignore_port_in_host_matching {
            len += 1;
        }
        if self.ignore_path_parameters_in_path_matching {
            len += 1;
        }
        if !self.typed_per_filter_config.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteConfiguration", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.virtual_hosts.is_empty() {
            struct_ser.serialize_field("virtual_hosts", &self.virtual_hosts)?;
        }
        if let Some(v) = self.vhds.as_ref() {
            struct_ser.serialize_field("vhds", v)?;
        }
        if !self.internal_only_headers.is_empty() {
            struct_ser.serialize_field("internal_only_headers", &self.internal_only_headers)?;
        }
        if !self.response_headers_to_add.is_empty() {
            struct_ser.serialize_field("response_headers_to_add", &self.response_headers_to_add)?;
        }
        if !self.response_headers_to_remove.is_empty() {
            struct_ser.serialize_field("response_headers_to_remove", &self.response_headers_to_remove)?;
        }
        if !self.request_headers_to_add.is_empty() {
            struct_ser.serialize_field("request_headers_to_add", &self.request_headers_to_add)?;
        }
        if !self.request_headers_to_remove.is_empty() {
            struct_ser.serialize_field("request_headers_to_remove", &self.request_headers_to_remove)?;
        }
        if self.most_specific_header_mutations_wins {
            struct_ser.serialize_field("most_specific_header_mutations_wins", &self.most_specific_header_mutations_wins)?;
        }
        if let Some(v) = self.validate_clusters.as_ref() {
            struct_ser.serialize_field("validate_clusters", v)?;
        }
        if let Some(v) = self.max_direct_response_body_size_bytes.as_ref() {
            struct_ser.serialize_field("max_direct_response_body_size_bytes", v)?;
        }
        if !self.cluster_specifier_plugins.is_empty() {
            struct_ser.serialize_field("cluster_specifier_plugins", &self.cluster_specifier_plugins)?;
        }
        if !self.request_mirror_policies.is_empty() {
            struct_ser.serialize_field("request_mirror_policies", &self.request_mirror_policies)?;
        }
        if self.ignore_port_in_host_matching {
            struct_ser.serialize_field("ignore_port_in_host_matching", &self.ignore_port_in_host_matching)?;
        }
        if self.ignore_path_parameters_in_path_matching {
            struct_ser.serialize_field("ignore_path_parameters_in_path_matching", &self.ignore_path_parameters_in_path_matching)?;
        }
        if !self.typed_per_filter_config.is_empty() {
            struct_ser.serialize_field("typed_per_filter_config", &self.typed_per_filter_config)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "virtual_hosts",
            "virtualHosts",
            "vhds",
            "internal_only_headers",
            "internalOnlyHeaders",
            "response_headers_to_add",
            "responseHeadersToAdd",
            "response_headers_to_remove",
            "responseHeadersToRemove",
            "request_headers_to_add",
            "requestHeadersToAdd",
            "request_headers_to_remove",
            "requestHeadersToRemove",
            "most_specific_header_mutations_wins",
            "mostSpecificHeaderMutationsWins",
            "validate_clusters",
            "validateClusters",
            "max_direct_response_body_size_bytes",
            "maxDirectResponseBodySizeBytes",
            "cluster_specifier_plugins",
            "clusterSpecifierPlugins",
            "request_mirror_policies",
            "requestMirrorPolicies",
            "ignore_port_in_host_matching",
            "ignorePortInHostMatching",
            "ignore_path_parameters_in_path_matching",
            "ignorePathParametersInPathMatching",
            "typed_per_filter_config",
            "typedPerFilterConfig",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            VirtualHosts,
            Vhds,
            InternalOnlyHeaders,
            ResponseHeadersToAdd,
            ResponseHeadersToRemove,
            RequestHeadersToAdd,
            RequestHeadersToRemove,
            MostSpecificHeaderMutationsWins,
            ValidateClusters,
            MaxDirectResponseBodySizeBytes,
            ClusterSpecifierPlugins,
            RequestMirrorPolicies,
            IgnorePortInHostMatching,
            IgnorePathParametersInPathMatching,
            TypedPerFilterConfig,
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
                            "name" => Ok(GeneratedField::Name),
                            "virtualHosts" | "virtual_hosts" => Ok(GeneratedField::VirtualHosts),
                            "vhds" => Ok(GeneratedField::Vhds),
                            "internalOnlyHeaders" | "internal_only_headers" => Ok(GeneratedField::InternalOnlyHeaders),
                            "responseHeadersToAdd" | "response_headers_to_add" => Ok(GeneratedField::ResponseHeadersToAdd),
                            "responseHeadersToRemove" | "response_headers_to_remove" => Ok(GeneratedField::ResponseHeadersToRemove),
                            "requestHeadersToAdd" | "request_headers_to_add" => Ok(GeneratedField::RequestHeadersToAdd),
                            "requestHeadersToRemove" | "request_headers_to_remove" => Ok(GeneratedField::RequestHeadersToRemove),
                            "mostSpecificHeaderMutationsWins" | "most_specific_header_mutations_wins" => Ok(GeneratedField::MostSpecificHeaderMutationsWins),
                            "validateClusters" | "validate_clusters" => Ok(GeneratedField::ValidateClusters),
                            "maxDirectResponseBodySizeBytes" | "max_direct_response_body_size_bytes" => Ok(GeneratedField::MaxDirectResponseBodySizeBytes),
                            "clusterSpecifierPlugins" | "cluster_specifier_plugins" => Ok(GeneratedField::ClusterSpecifierPlugins),
                            "requestMirrorPolicies" | "request_mirror_policies" => Ok(GeneratedField::RequestMirrorPolicies),
                            "ignorePortInHostMatching" | "ignore_port_in_host_matching" => Ok(GeneratedField::IgnorePortInHostMatching),
                            "ignorePathParametersInPathMatching" | "ignore_path_parameters_in_path_matching" => Ok(GeneratedField::IgnorePathParametersInPathMatching),
                            "typedPerFilterConfig" | "typed_per_filter_config" => Ok(GeneratedField::TypedPerFilterConfig),
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
            type Value = RouteConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteConfiguration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RouteConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut virtual_hosts__ = None;
                let mut vhds__ = None;
                let mut internal_only_headers__ = None;
                let mut response_headers_to_add__ = None;
                let mut response_headers_to_remove__ = None;
                let mut request_headers_to_add__ = None;
                let mut request_headers_to_remove__ = None;
                let mut most_specific_header_mutations_wins__ = None;
                let mut validate_clusters__ = None;
                let mut max_direct_response_body_size_bytes__ = None;
                let mut cluster_specifier_plugins__ = None;
                let mut request_mirror_policies__ = None;
                let mut ignore_port_in_host_matching__ = None;
                let mut ignore_path_parameters_in_path_matching__ = None;
                let mut typed_per_filter_config__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VirtualHosts => {
                            if virtual_hosts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualHosts"));
                            }
                            virtual_hosts__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Vhds => {
                            if vhds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vhds"));
                            }
                            vhds__ = map_.next_value()?;
                        }
                        GeneratedField::InternalOnlyHeaders => {
                            if internal_only_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("internalOnlyHeaders"));
                            }
                            internal_only_headers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToAdd => {
                            if response_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToAdd"));
                            }
                            response_headers_to_add__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToRemove => {
                            if response_headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToRemove"));
                            }
                            response_headers_to_remove__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestHeadersToAdd => {
                            if request_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToAdd"));
                            }
                            request_headers_to_add__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestHeadersToRemove => {
                            if request_headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToRemove"));
                            }
                            request_headers_to_remove__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MostSpecificHeaderMutationsWins => {
                            if most_specific_header_mutations_wins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mostSpecificHeaderMutationsWins"));
                            }
                            most_specific_header_mutations_wins__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValidateClusters => {
                            if validate_clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validateClusters"));
                            }
                            validate_clusters__ = map_.next_value()?;
                        }
                        GeneratedField::MaxDirectResponseBodySizeBytes => {
                            if max_direct_response_body_size_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxDirectResponseBodySizeBytes"));
                            }
                            max_direct_response_body_size_bytes__ = map_.next_value()?;
                        }
                        GeneratedField::ClusterSpecifierPlugins => {
                            if cluster_specifier_plugins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterSpecifierPlugins"));
                            }
                            cluster_specifier_plugins__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestMirrorPolicies => {
                            if request_mirror_policies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestMirrorPolicies"));
                            }
                            request_mirror_policies__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IgnorePortInHostMatching => {
                            if ignore_port_in_host_matching__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignorePortInHostMatching"));
                            }
                            ignore_port_in_host_matching__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IgnorePathParametersInPathMatching => {
                            if ignore_path_parameters_in_path_matching__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignorePathParametersInPathMatching"));
                            }
                            ignore_path_parameters_in_path_matching__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypedPerFilterConfig => {
                            if typed_per_filter_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedPerFilterConfig"));
                            }
                            typed_per_filter_config__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RouteConfiguration {
                    name: name__.unwrap_or_default(),
                    virtual_hosts: virtual_hosts__.unwrap_or_default(),
                    vhds: vhds__,
                    internal_only_headers: internal_only_headers__.unwrap_or_default(),
                    response_headers_to_add: response_headers_to_add__.unwrap_or_default(),
                    response_headers_to_remove: response_headers_to_remove__.unwrap_or_default(),
                    request_headers_to_add: request_headers_to_add__.unwrap_or_default(),
                    request_headers_to_remove: request_headers_to_remove__.unwrap_or_default(),
                    most_specific_header_mutations_wins: most_specific_header_mutations_wins__.unwrap_or_default(),
                    validate_clusters: validate_clusters__,
                    max_direct_response_body_size_bytes: max_direct_response_body_size_bytes__,
                    cluster_specifier_plugins: cluster_specifier_plugins__.unwrap_or_default(),
                    request_mirror_policies: request_mirror_policies__.unwrap_or_default(),
                    ignore_port_in_host_matching: ignore_port_in_host_matching__.unwrap_or_default(),
                    ignore_path_parameters_in_path_matching: ignore_path_parameters_in_path_matching__.unwrap_or_default(),
                    typed_per_filter_config: typed_per_filter_config__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.routes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteList", len)?;
        if !self.routes.is_empty() {
            struct_ser.serialize_field("routes", &self.routes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "routes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Routes,
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
                            "routes" => Ok(GeneratedField::Routes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RouteList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut routes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Routes => {
                            if routes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routes"));
                            }
                            routes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RouteList {
                    routes: routes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.case_sensitive.is_some() {
            len += 1;
        }
        if self.runtime_fraction.is_some() {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        if !self.query_parameters.is_empty() {
            len += 1;
        }
        if self.grpc.is_some() {
            len += 1;
        }
        if self.tls_context.is_some() {
            len += 1;
        }
        if !self.dynamic_metadata.is_empty() {
            len += 1;
        }
        if !self.filter_state.is_empty() {
            len += 1;
        }
        if self.path_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteMatch", len)?;
        if let Some(v) = self.case_sensitive.as_ref() {
            struct_ser.serialize_field("case_sensitive", v)?;
        }
        if let Some(v) = self.runtime_fraction.as_ref() {
            struct_ser.serialize_field("runtime_fraction", v)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if !self.query_parameters.is_empty() {
            struct_ser.serialize_field("query_parameters", &self.query_parameters)?;
        }
        if let Some(v) = self.grpc.as_ref() {
            struct_ser.serialize_field("grpc", v)?;
        }
        if let Some(v) = self.tls_context.as_ref() {
            struct_ser.serialize_field("tls_context", v)?;
        }
        if !self.dynamic_metadata.is_empty() {
            struct_ser.serialize_field("dynamic_metadata", &self.dynamic_metadata)?;
        }
        if !self.filter_state.is_empty() {
            struct_ser.serialize_field("filter_state", &self.filter_state)?;
        }
        if let Some(v) = self.path_specifier.as_ref() {
            match v {
                route_match::PathSpecifier::Prefix(v) => {
                    struct_ser.serialize_field("prefix", v)?;
                }
                route_match::PathSpecifier::Path(v) => {
                    struct_ser.serialize_field("path", v)?;
                }
                route_match::PathSpecifier::SafeRegex(v) => {
                    struct_ser.serialize_field("safe_regex", v)?;
                }
                route_match::PathSpecifier::ConnectMatcher(v) => {
                    struct_ser.serialize_field("connect_matcher", v)?;
                }
                route_match::PathSpecifier::PathSeparatedPrefix(v) => {
                    struct_ser.serialize_field("path_separated_prefix", v)?;
                }
                route_match::PathSpecifier::PathMatchPolicy(v) => {
                    struct_ser.serialize_field("path_match_policy", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "case_sensitive",
            "caseSensitive",
            "runtime_fraction",
            "runtimeFraction",
            "headers",
            "query_parameters",
            "queryParameters",
            "grpc",
            "tls_context",
            "tlsContext",
            "dynamic_metadata",
            "dynamicMetadata",
            "filter_state",
            "filterState",
            "prefix",
            "path",
            "safe_regex",
            "safeRegex",
            "connect_matcher",
            "connectMatcher",
            "path_separated_prefix",
            "pathSeparatedPrefix",
            "path_match_policy",
            "pathMatchPolicy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CaseSensitive,
            RuntimeFraction,
            Headers,
            QueryParameters,
            Grpc,
            TlsContext,
            DynamicMetadata,
            FilterState,
            Prefix,
            Path,
            SafeRegex,
            ConnectMatcher,
            PathSeparatedPrefix,
            PathMatchPolicy,
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
                            "caseSensitive" | "case_sensitive" => Ok(GeneratedField::CaseSensitive),
                            "runtimeFraction" | "runtime_fraction" => Ok(GeneratedField::RuntimeFraction),
                            "headers" => Ok(GeneratedField::Headers),
                            "queryParameters" | "query_parameters" => Ok(GeneratedField::QueryParameters),
                            "grpc" => Ok(GeneratedField::Grpc),
                            "tlsContext" | "tls_context" => Ok(GeneratedField::TlsContext),
                            "dynamicMetadata" | "dynamic_metadata" => Ok(GeneratedField::DynamicMetadata),
                            "filterState" | "filter_state" => Ok(GeneratedField::FilterState),
                            "prefix" => Ok(GeneratedField::Prefix),
                            "path" => Ok(GeneratedField::Path),
                            "safeRegex" | "safe_regex" => Ok(GeneratedField::SafeRegex),
                            "connectMatcher" | "connect_matcher" => Ok(GeneratedField::ConnectMatcher),
                            "pathSeparatedPrefix" | "path_separated_prefix" => Ok(GeneratedField::PathSeparatedPrefix),
                            "pathMatchPolicy" | "path_match_policy" => Ok(GeneratedField::PathMatchPolicy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteMatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RouteMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut case_sensitive__ = None;
                let mut runtime_fraction__ = None;
                let mut headers__ = None;
                let mut query_parameters__ = None;
                let mut grpc__ = None;
                let mut tls_context__ = None;
                let mut dynamic_metadata__ = None;
                let mut filter_state__ = None;
                let mut path_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CaseSensitive => {
                            if case_sensitive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caseSensitive"));
                            }
                            case_sensitive__ = map_.next_value()?;
                        }
                        GeneratedField::RuntimeFraction => {
                            if runtime_fraction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeFraction"));
                            }
                            runtime_fraction__ = map_.next_value()?;
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QueryParameters => {
                            if query_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryParameters"));
                            }
                            query_parameters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Grpc => {
                            if grpc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpc"));
                            }
                            grpc__ = map_.next_value()?;
                        }
                        GeneratedField::TlsContext => {
                            if tls_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsContext"));
                            }
                            tls_context__ = map_.next_value()?;
                        }
                        GeneratedField::DynamicMetadata => {
                            if dynamic_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicMetadata"));
                            }
                            dynamic_metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FilterState => {
                            if filter_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterState"));
                            }
                            filter_state__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Prefix => {
                            if path_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            path_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_match::PathSpecifier::Prefix);
                        }
                        GeneratedField::Path => {
                            if path_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_match::PathSpecifier::Path);
                        }
                        GeneratedField::SafeRegex => {
                            if path_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("safeRegex"));
                            }
                            path_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_match::PathSpecifier::SafeRegex)
;
                        }
                        GeneratedField::ConnectMatcher => {
                            if path_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectMatcher"));
                            }
                            path_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_match::PathSpecifier::ConnectMatcher)
;
                        }
                        GeneratedField::PathSeparatedPrefix => {
                            if path_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathSeparatedPrefix"));
                            }
                            path_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_match::PathSpecifier::PathSeparatedPrefix);
                        }
                        GeneratedField::PathMatchPolicy => {
                            if path_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathMatchPolicy"));
                            }
                            path_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(route_match::PathSpecifier::PathMatchPolicy)
;
                        }
                    }
                }
                Ok(RouteMatch {
                    case_sensitive: case_sensitive__,
                    runtime_fraction: runtime_fraction__,
                    headers: headers__.unwrap_or_default(),
                    query_parameters: query_parameters__.unwrap_or_default(),
                    grpc: grpc__,
                    tls_context: tls_context__,
                    dynamic_metadata: dynamic_metadata__.unwrap_or_default(),
                    filter_state: filter_state__.unwrap_or_default(),
                    path_specifier: path_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteMatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_match::ConnectMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteMatch.ConnectMatcher", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_match::ConnectMatcher {
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
            type Value = route_match::ConnectMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteMatch.ConnectMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<route_match::ConnectMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(route_match::ConnectMatcher {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteMatch.ConnectMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_match::GrpcRouteMatchOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteMatch.GrpcRouteMatchOptions", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_match::GrpcRouteMatchOptions {
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
            type Value = route_match::GrpcRouteMatchOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteMatch.GrpcRouteMatchOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<route_match::GrpcRouteMatchOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(route_match::GrpcRouteMatchOptions {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteMatch.GrpcRouteMatchOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_match::TlsContextMatchOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.presented.is_some() {
            len += 1;
        }
        if self.validated.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.RouteMatch.TlsContextMatchOptions", len)?;
        if let Some(v) = self.presented.as_ref() {
            struct_ser.serialize_field("presented", v)?;
        }
        if let Some(v) = self.validated.as_ref() {
            struct_ser.serialize_field("validated", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_match::TlsContextMatchOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "presented",
            "validated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Presented,
            Validated,
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
                            "presented" => Ok(GeneratedField::Presented),
                            "validated" => Ok(GeneratedField::Validated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_match::TlsContextMatchOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.RouteMatch.TlsContextMatchOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<route_match::TlsContextMatchOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut presented__ = None;
                let mut validated__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Presented => {
                            if presented__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presented"));
                            }
                            presented__ = map_.next_value()?;
                        }
                        GeneratedField::Validated => {
                            if validated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validated"));
                            }
                            validated__ = map_.next_value()?;
                        }
                    }
                }
                Ok(route_match::TlsContextMatchOptions {
                    presented: presented__,
                    validated: validated__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.RouteMatch.TlsContextMatchOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScopedRouteConfiguration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.on_demand {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.route_configuration_name.is_empty() {
            len += 1;
        }
        if self.route_configuration.is_some() {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.ScopedRouteConfiguration", len)?;
        if self.on_demand {
            struct_ser.serialize_field("on_demand", &self.on_demand)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.route_configuration_name.is_empty() {
            struct_ser.serialize_field("route_configuration_name", &self.route_configuration_name)?;
        }
        if let Some(v) = self.route_configuration.as_ref() {
            struct_ser.serialize_field("route_configuration", v)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScopedRouteConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "on_demand",
            "onDemand",
            "name",
            "route_configuration_name",
            "routeConfigurationName",
            "route_configuration",
            "routeConfiguration",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OnDemand,
            Name,
            RouteConfigurationName,
            RouteConfiguration,
            Key,
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
                            "onDemand" | "on_demand" => Ok(GeneratedField::OnDemand),
                            "name" => Ok(GeneratedField::Name),
                            "routeConfigurationName" | "route_configuration_name" => Ok(GeneratedField::RouteConfigurationName),
                            "routeConfiguration" | "route_configuration" => Ok(GeneratedField::RouteConfiguration),
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScopedRouteConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.ScopedRouteConfiguration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ScopedRouteConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut on_demand__ = None;
                let mut name__ = None;
                let mut route_configuration_name__ = None;
                let mut route_configuration__ = None;
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OnDemand => {
                            if on_demand__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onDemand"));
                            }
                            on_demand__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RouteConfigurationName => {
                            if route_configuration_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeConfigurationName"));
                            }
                            route_configuration_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RouteConfiguration => {
                            if route_configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeConfiguration"));
                            }
                            route_configuration__ = map_.next_value()?;
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ScopedRouteConfiguration {
                    on_demand: on_demand__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    route_configuration_name: route_configuration_name__.unwrap_or_default(),
                    route_configuration: route_configuration__,
                    key: key__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.ScopedRouteConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scoped_route_configuration::Key {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fragments.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.ScopedRouteConfiguration.Key", len)?;
        if !self.fragments.is_empty() {
            struct_ser.serialize_field("fragments", &self.fragments)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for scoped_route_configuration::Key {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fragments",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fragments,
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
                            "fragments" => Ok(GeneratedField::Fragments),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scoped_route_configuration::Key;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.ScopedRouteConfiguration.Key")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<scoped_route_configuration::Key, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fragments__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fragments => {
                            if fragments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fragments"));
                            }
                            fragments__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(scoped_route_configuration::Key {
                    fragments: fragments__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.ScopedRouteConfiguration.Key", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scoped_route_configuration::key::Fragment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.ScopedRouteConfiguration.Key.Fragment", len)?;
        if let Some(v) = self.r#type.as_ref() {
            match v {
                scoped_route_configuration::key::fragment::Type::StringKey(v) => {
                    struct_ser.serialize_field("string_key", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for scoped_route_configuration::key::Fragment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "string_key",
            "stringKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StringKey,
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
                            "stringKey" | "string_key" => Ok(GeneratedField::StringKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scoped_route_configuration::key::Fragment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.ScopedRouteConfiguration.Key.Fragment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<scoped_route_configuration::key::Fragment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StringKey => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringKey"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(scoped_route_configuration::key::fragment::Type::StringKey);
                        }
                    }
                }
                Ok(scoped_route_configuration::key::Fragment {
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.ScopedRouteConfiguration.Key.Fragment", FIELDS, GeneratedVisitor)
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
        if self.client_sampling.is_some() {
            len += 1;
        }
        if self.random_sampling.is_some() {
            len += 1;
        }
        if self.overall_sampling.is_some() {
            len += 1;
        }
        if !self.custom_tags.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.Tracing", len)?;
        if let Some(v) = self.client_sampling.as_ref() {
            struct_ser.serialize_field("client_sampling", v)?;
        }
        if let Some(v) = self.random_sampling.as_ref() {
            struct_ser.serialize_field("random_sampling", v)?;
        }
        if let Some(v) = self.overall_sampling.as_ref() {
            struct_ser.serialize_field("overall_sampling", v)?;
        }
        if !self.custom_tags.is_empty() {
            struct_ser.serialize_field("custom_tags", &self.custom_tags)?;
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
            "client_sampling",
            "clientSampling",
            "random_sampling",
            "randomSampling",
            "overall_sampling",
            "overallSampling",
            "custom_tags",
            "customTags",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientSampling,
            RandomSampling,
            OverallSampling,
            CustomTags,
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
                            "clientSampling" | "client_sampling" => Ok(GeneratedField::ClientSampling),
                            "randomSampling" | "random_sampling" => Ok(GeneratedField::RandomSampling),
                            "overallSampling" | "overall_sampling" => Ok(GeneratedField::OverallSampling),
                            "customTags" | "custom_tags" => Ok(GeneratedField::CustomTags),
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
                formatter.write_str("struct envoy.config.route.v3.Tracing")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Tracing, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_sampling__ = None;
                let mut random_sampling__ = None;
                let mut overall_sampling__ = None;
                let mut custom_tags__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientSampling => {
                            if client_sampling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientSampling"));
                            }
                            client_sampling__ = map_.next_value()?;
                        }
                        GeneratedField::RandomSampling => {
                            if random_sampling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("randomSampling"));
                            }
                            random_sampling__ = map_.next_value()?;
                        }
                        GeneratedField::OverallSampling => {
                            if overall_sampling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overallSampling"));
                            }
                            overall_sampling__ = map_.next_value()?;
                        }
                        GeneratedField::CustomTags => {
                            if custom_tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customTags"));
                            }
                            custom_tags__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Tracing {
                    client_sampling: client_sampling__,
                    random_sampling: random_sampling__,
                    overall_sampling: overall_sampling__,
                    custom_tags: custom_tags__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.Tracing", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Vhds {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config_source.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.Vhds", len)?;
        if let Some(v) = self.config_source.as_ref() {
            struct_ser.serialize_field("config_source", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Vhds {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config_source",
            "configSource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = Vhds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.Vhds")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Vhds, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config_source__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConfigSource => {
                            if config_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configSource"));
                            }
                            config_source__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Vhds {
                    config_source: config_source__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.Vhds", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualCluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.headers.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.VirtualCluster", len)?;
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualCluster {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "headers",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Headers,
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
                            "headers" => Ok(GeneratedField::Headers),
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
            type Value = VirtualCluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.VirtualCluster")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VirtualCluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut headers__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(VirtualCluster {
                    headers: headers__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.VirtualCluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualHost {
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
        if !self.domains.is_empty() {
            len += 1;
        }
        if !self.routes.is_empty() {
            len += 1;
        }
        if self.matcher.is_some() {
            len += 1;
        }
        if self.require_tls != 0 {
            len += 1;
        }
        if !self.virtual_clusters.is_empty() {
            len += 1;
        }
        if !self.rate_limits.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_remove.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_remove.is_empty() {
            len += 1;
        }
        if self.cors.is_some() {
            len += 1;
        }
        if !self.typed_per_filter_config.is_empty() {
            len += 1;
        }
        if self.include_request_attempt_count {
            len += 1;
        }
        if self.include_attempt_count_in_response {
            len += 1;
        }
        if self.retry_policy.is_some() {
            len += 1;
        }
        if self.retry_policy_typed_config.is_some() {
            len += 1;
        }
        if self.hedge_policy.is_some() {
            len += 1;
        }
        if self.include_is_timeout_retry_header {
            len += 1;
        }
        if self.per_request_buffer_limit_bytes.is_some() {
            len += 1;
        }
        if !self.request_mirror_policies.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.VirtualHost", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.domains.is_empty() {
            struct_ser.serialize_field("domains", &self.domains)?;
        }
        if !self.routes.is_empty() {
            struct_ser.serialize_field("routes", &self.routes)?;
        }
        if let Some(v) = self.matcher.as_ref() {
            struct_ser.serialize_field("matcher", v)?;
        }
        if self.require_tls != 0 {
            let v = virtual_host::TlsRequirementType::try_from(self.require_tls)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.require_tls)))?;
            struct_ser.serialize_field("require_tls", &v)?;
        }
        if !self.virtual_clusters.is_empty() {
            struct_ser.serialize_field("virtual_clusters", &self.virtual_clusters)?;
        }
        if !self.rate_limits.is_empty() {
            struct_ser.serialize_field("rate_limits", &self.rate_limits)?;
        }
        if !self.request_headers_to_add.is_empty() {
            struct_ser.serialize_field("request_headers_to_add", &self.request_headers_to_add)?;
        }
        if !self.request_headers_to_remove.is_empty() {
            struct_ser.serialize_field("request_headers_to_remove", &self.request_headers_to_remove)?;
        }
        if !self.response_headers_to_add.is_empty() {
            struct_ser.serialize_field("response_headers_to_add", &self.response_headers_to_add)?;
        }
        if !self.response_headers_to_remove.is_empty() {
            struct_ser.serialize_field("response_headers_to_remove", &self.response_headers_to_remove)?;
        }
        if let Some(v) = self.cors.as_ref() {
            struct_ser.serialize_field("cors", v)?;
        }
        if !self.typed_per_filter_config.is_empty() {
            struct_ser.serialize_field("typed_per_filter_config", &self.typed_per_filter_config)?;
        }
        if self.include_request_attempt_count {
            struct_ser.serialize_field("include_request_attempt_count", &self.include_request_attempt_count)?;
        }
        if self.include_attempt_count_in_response {
            struct_ser.serialize_field("include_attempt_count_in_response", &self.include_attempt_count_in_response)?;
        }
        if let Some(v) = self.retry_policy.as_ref() {
            struct_ser.serialize_field("retry_policy", v)?;
        }
        if let Some(v) = self.retry_policy_typed_config.as_ref() {
            struct_ser.serialize_field("retry_policy_typed_config", v)?;
        }
        if let Some(v) = self.hedge_policy.as_ref() {
            struct_ser.serialize_field("hedge_policy", v)?;
        }
        if self.include_is_timeout_retry_header {
            struct_ser.serialize_field("include_is_timeout_retry_header", &self.include_is_timeout_retry_header)?;
        }
        if let Some(v) = self.per_request_buffer_limit_bytes.as_ref() {
            struct_ser.serialize_field("per_request_buffer_limit_bytes", v)?;
        }
        if !self.request_mirror_policies.is_empty() {
            struct_ser.serialize_field("request_mirror_policies", &self.request_mirror_policies)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualHost {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "domains",
            "routes",
            "matcher",
            "require_tls",
            "requireTls",
            "virtual_clusters",
            "virtualClusters",
            "rate_limits",
            "rateLimits",
            "request_headers_to_add",
            "requestHeadersToAdd",
            "request_headers_to_remove",
            "requestHeadersToRemove",
            "response_headers_to_add",
            "responseHeadersToAdd",
            "response_headers_to_remove",
            "responseHeadersToRemove",
            "cors",
            "typed_per_filter_config",
            "typedPerFilterConfig",
            "include_request_attempt_count",
            "includeRequestAttemptCount",
            "include_attempt_count_in_response",
            "includeAttemptCountInResponse",
            "retry_policy",
            "retryPolicy",
            "retry_policy_typed_config",
            "retryPolicyTypedConfig",
            "hedge_policy",
            "hedgePolicy",
            "include_is_timeout_retry_header",
            "includeIsTimeoutRetryHeader",
            "per_request_buffer_limit_bytes",
            "perRequestBufferLimitBytes",
            "request_mirror_policies",
            "requestMirrorPolicies",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Domains,
            Routes,
            Matcher,
            RequireTls,
            VirtualClusters,
            RateLimits,
            RequestHeadersToAdd,
            RequestHeadersToRemove,
            ResponseHeadersToAdd,
            ResponseHeadersToRemove,
            Cors,
            TypedPerFilterConfig,
            IncludeRequestAttemptCount,
            IncludeAttemptCountInResponse,
            RetryPolicy,
            RetryPolicyTypedConfig,
            HedgePolicy,
            IncludeIsTimeoutRetryHeader,
            PerRequestBufferLimitBytes,
            RequestMirrorPolicies,
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
                            "name" => Ok(GeneratedField::Name),
                            "domains" => Ok(GeneratedField::Domains),
                            "routes" => Ok(GeneratedField::Routes),
                            "matcher" => Ok(GeneratedField::Matcher),
                            "requireTls" | "require_tls" => Ok(GeneratedField::RequireTls),
                            "virtualClusters" | "virtual_clusters" => Ok(GeneratedField::VirtualClusters),
                            "rateLimits" | "rate_limits" => Ok(GeneratedField::RateLimits),
                            "requestHeadersToAdd" | "request_headers_to_add" => Ok(GeneratedField::RequestHeadersToAdd),
                            "requestHeadersToRemove" | "request_headers_to_remove" => Ok(GeneratedField::RequestHeadersToRemove),
                            "responseHeadersToAdd" | "response_headers_to_add" => Ok(GeneratedField::ResponseHeadersToAdd),
                            "responseHeadersToRemove" | "response_headers_to_remove" => Ok(GeneratedField::ResponseHeadersToRemove),
                            "cors" => Ok(GeneratedField::Cors),
                            "typedPerFilterConfig" | "typed_per_filter_config" => Ok(GeneratedField::TypedPerFilterConfig),
                            "includeRequestAttemptCount" | "include_request_attempt_count" => Ok(GeneratedField::IncludeRequestAttemptCount),
                            "includeAttemptCountInResponse" | "include_attempt_count_in_response" => Ok(GeneratedField::IncludeAttemptCountInResponse),
                            "retryPolicy" | "retry_policy" => Ok(GeneratedField::RetryPolicy),
                            "retryPolicyTypedConfig" | "retry_policy_typed_config" => Ok(GeneratedField::RetryPolicyTypedConfig),
                            "hedgePolicy" | "hedge_policy" => Ok(GeneratedField::HedgePolicy),
                            "includeIsTimeoutRetryHeader" | "include_is_timeout_retry_header" => Ok(GeneratedField::IncludeIsTimeoutRetryHeader),
                            "perRequestBufferLimitBytes" | "per_request_buffer_limit_bytes" => Ok(GeneratedField::PerRequestBufferLimitBytes),
                            "requestMirrorPolicies" | "request_mirror_policies" => Ok(GeneratedField::RequestMirrorPolicies),
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
            type Value = VirtualHost;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.VirtualHost")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VirtualHost, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut domains__ = None;
                let mut routes__ = None;
                let mut matcher__ = None;
                let mut require_tls__ = None;
                let mut virtual_clusters__ = None;
                let mut rate_limits__ = None;
                let mut request_headers_to_add__ = None;
                let mut request_headers_to_remove__ = None;
                let mut response_headers_to_add__ = None;
                let mut response_headers_to_remove__ = None;
                let mut cors__ = None;
                let mut typed_per_filter_config__ = None;
                let mut include_request_attempt_count__ = None;
                let mut include_attempt_count_in_response__ = None;
                let mut retry_policy__ = None;
                let mut retry_policy_typed_config__ = None;
                let mut hedge_policy__ = None;
                let mut include_is_timeout_retry_header__ = None;
                let mut per_request_buffer_limit_bytes__ = None;
                let mut request_mirror_policies__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domains => {
                            if domains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domains"));
                            }
                            domains__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Routes => {
                            if routes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routes"));
                            }
                            routes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Matcher => {
                            if matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matcher"));
                            }
                            matcher__ = map_.next_value()?;
                        }
                        GeneratedField::RequireTls => {
                            if require_tls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requireTls"));
                            }
                            require_tls__ = Some(map_.next_value::<virtual_host::TlsRequirementType>()? as i32);
                        }
                        GeneratedField::VirtualClusters => {
                            if virtual_clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualClusters"));
                            }
                            virtual_clusters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RateLimits => {
                            if rate_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimits"));
                            }
                            rate_limits__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestHeadersToAdd => {
                            if request_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToAdd"));
                            }
                            request_headers_to_add__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestHeadersToRemove => {
                            if request_headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToRemove"));
                            }
                            request_headers_to_remove__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToAdd => {
                            if response_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToAdd"));
                            }
                            response_headers_to_add__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToRemove => {
                            if response_headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToRemove"));
                            }
                            response_headers_to_remove__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cors => {
                            if cors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cors"));
                            }
                            cors__ = map_.next_value()?;
                        }
                        GeneratedField::TypedPerFilterConfig => {
                            if typed_per_filter_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedPerFilterConfig"));
                            }
                            typed_per_filter_config__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::IncludeRequestAttemptCount => {
                            if include_request_attempt_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeRequestAttemptCount"));
                            }
                            include_request_attempt_count__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IncludeAttemptCountInResponse => {
                            if include_attempt_count_in_response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeAttemptCountInResponse"));
                            }
                            include_attempt_count_in_response__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetryPolicy => {
                            if retry_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryPolicy"));
                            }
                            retry_policy__ = map_.next_value()?;
                        }
                        GeneratedField::RetryPolicyTypedConfig => {
                            if retry_policy_typed_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryPolicyTypedConfig"));
                            }
                            retry_policy_typed_config__ = map_.next_value()?;
                        }
                        GeneratedField::HedgePolicy => {
                            if hedge_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hedgePolicy"));
                            }
                            hedge_policy__ = map_.next_value()?;
                        }
                        GeneratedField::IncludeIsTimeoutRetryHeader => {
                            if include_is_timeout_retry_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeIsTimeoutRetryHeader"));
                            }
                            include_is_timeout_retry_header__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PerRequestBufferLimitBytes => {
                            if per_request_buffer_limit_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perRequestBufferLimitBytes"));
                            }
                            per_request_buffer_limit_bytes__ = map_.next_value()?;
                        }
                        GeneratedField::RequestMirrorPolicies => {
                            if request_mirror_policies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestMirrorPolicies"));
                            }
                            request_mirror_policies__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(VirtualHost {
                    name: name__.unwrap_or_default(),
                    domains: domains__.unwrap_or_default(),
                    routes: routes__.unwrap_or_default(),
                    matcher: matcher__,
                    require_tls: require_tls__.unwrap_or_default(),
                    virtual_clusters: virtual_clusters__.unwrap_or_default(),
                    rate_limits: rate_limits__.unwrap_or_default(),
                    request_headers_to_add: request_headers_to_add__.unwrap_or_default(),
                    request_headers_to_remove: request_headers_to_remove__.unwrap_or_default(),
                    response_headers_to_add: response_headers_to_add__.unwrap_or_default(),
                    response_headers_to_remove: response_headers_to_remove__.unwrap_or_default(),
                    cors: cors__,
                    typed_per_filter_config: typed_per_filter_config__.unwrap_or_default(),
                    include_request_attempt_count: include_request_attempt_count__.unwrap_or_default(),
                    include_attempt_count_in_response: include_attempt_count_in_response__.unwrap_or_default(),
                    retry_policy: retry_policy__,
                    retry_policy_typed_config: retry_policy_typed_config__,
                    hedge_policy: hedge_policy__,
                    include_is_timeout_retry_header: include_is_timeout_retry_header__.unwrap_or_default(),
                    per_request_buffer_limit_bytes: per_request_buffer_limit_bytes__,
                    request_mirror_policies: request_mirror_policies__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.VirtualHost", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for virtual_host::TlsRequirementType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::None => "NONE",
            Self::ExternalOnly => "EXTERNAL_ONLY",
            Self::All => "ALL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for virtual_host::TlsRequirementType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NONE",
            "EXTERNAL_ONLY",
            "ALL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = virtual_host::TlsRequirementType;

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
                    "NONE" => Ok(virtual_host::TlsRequirementType::None),
                    "EXTERNAL_ONLY" => Ok(virtual_host::TlsRequirementType::ExternalOnly),
                    "ALL" => Ok(virtual_host::TlsRequirementType::All),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for WeightedCluster {
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
        if self.total_weight.is_some() {
            len += 1;
        }
        if !self.runtime_key_prefix.is_empty() {
            len += 1;
        }
        if self.random_value_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.WeightedCluster", len)?;
        if !self.clusters.is_empty() {
            struct_ser.serialize_field("clusters", &self.clusters)?;
        }
        if let Some(v) = self.total_weight.as_ref() {
            struct_ser.serialize_field("total_weight", v)?;
        }
        if !self.runtime_key_prefix.is_empty() {
            struct_ser.serialize_field("runtime_key_prefix", &self.runtime_key_prefix)?;
        }
        if let Some(v) = self.random_value_specifier.as_ref() {
            match v {
                weighted_cluster::RandomValueSpecifier::HeaderName(v) => {
                    struct_ser.serialize_field("header_name", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WeightedCluster {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "clusters",
            "total_weight",
            "totalWeight",
            "runtime_key_prefix",
            "runtimeKeyPrefix",
            "header_name",
            "headerName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Clusters,
            TotalWeight,
            RuntimeKeyPrefix,
            HeaderName,
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
                            "totalWeight" | "total_weight" => Ok(GeneratedField::TotalWeight),
                            "runtimeKeyPrefix" | "runtime_key_prefix" => Ok(GeneratedField::RuntimeKeyPrefix),
                            "headerName" | "header_name" => Ok(GeneratedField::HeaderName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WeightedCluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.WeightedCluster")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WeightedCluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut clusters__ = None;
                let mut total_weight__ = None;
                let mut runtime_key_prefix__ = None;
                let mut random_value_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Clusters => {
                            if clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusters"));
                            }
                            clusters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalWeight => {
                            if total_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalWeight"));
                            }
                            total_weight__ = map_.next_value()?;
                        }
                        GeneratedField::RuntimeKeyPrefix => {
                            if runtime_key_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeKeyPrefix"));
                            }
                            runtime_key_prefix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HeaderName => {
                            if random_value_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerName"));
                            }
                            random_value_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(weighted_cluster::RandomValueSpecifier::HeaderName);
                        }
                    }
                }
                Ok(WeightedCluster {
                    clusters: clusters__.unwrap_or_default(),
                    total_weight: total_weight__,
                    runtime_key_prefix: runtime_key_prefix__.unwrap_or_default(),
                    random_value_specifier: random_value_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.WeightedCluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for weighted_cluster::ClusterWeight {
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
        if !self.cluster_header.is_empty() {
            len += 1;
        }
        if self.weight.is_some() {
            len += 1;
        }
        if self.metadata_match.is_some() {
            len += 1;
        }
        if !self.request_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_remove.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_remove.is_empty() {
            len += 1;
        }
        if !self.typed_per_filter_config.is_empty() {
            len += 1;
        }
        if self.host_rewrite_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.route.v3.WeightedCluster.ClusterWeight", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.cluster_header.is_empty() {
            struct_ser.serialize_field("cluster_header", &self.cluster_header)?;
        }
        if let Some(v) = self.weight.as_ref() {
            struct_ser.serialize_field("weight", v)?;
        }
        if let Some(v) = self.metadata_match.as_ref() {
            struct_ser.serialize_field("metadata_match", v)?;
        }
        if !self.request_headers_to_add.is_empty() {
            struct_ser.serialize_field("request_headers_to_add", &self.request_headers_to_add)?;
        }
        if !self.request_headers_to_remove.is_empty() {
            struct_ser.serialize_field("request_headers_to_remove", &self.request_headers_to_remove)?;
        }
        if !self.response_headers_to_add.is_empty() {
            struct_ser.serialize_field("response_headers_to_add", &self.response_headers_to_add)?;
        }
        if !self.response_headers_to_remove.is_empty() {
            struct_ser.serialize_field("response_headers_to_remove", &self.response_headers_to_remove)?;
        }
        if !self.typed_per_filter_config.is_empty() {
            struct_ser.serialize_field("typed_per_filter_config", &self.typed_per_filter_config)?;
        }
        if let Some(v) = self.host_rewrite_specifier.as_ref() {
            match v {
                weighted_cluster::cluster_weight::HostRewriteSpecifier::HostRewriteLiteral(v) => {
                    struct_ser.serialize_field("host_rewrite_literal", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for weighted_cluster::ClusterWeight {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "cluster_header",
            "clusterHeader",
            "weight",
            "metadata_match",
            "metadataMatch",
            "request_headers_to_add",
            "requestHeadersToAdd",
            "request_headers_to_remove",
            "requestHeadersToRemove",
            "response_headers_to_add",
            "responseHeadersToAdd",
            "response_headers_to_remove",
            "responseHeadersToRemove",
            "typed_per_filter_config",
            "typedPerFilterConfig",
            "host_rewrite_literal",
            "hostRewriteLiteral",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ClusterHeader,
            Weight,
            MetadataMatch,
            RequestHeadersToAdd,
            RequestHeadersToRemove,
            ResponseHeadersToAdd,
            ResponseHeadersToRemove,
            TypedPerFilterConfig,
            HostRewriteLiteral,
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
                            "clusterHeader" | "cluster_header" => Ok(GeneratedField::ClusterHeader),
                            "weight" => Ok(GeneratedField::Weight),
                            "metadataMatch" | "metadata_match" => Ok(GeneratedField::MetadataMatch),
                            "requestHeadersToAdd" | "request_headers_to_add" => Ok(GeneratedField::RequestHeadersToAdd),
                            "requestHeadersToRemove" | "request_headers_to_remove" => Ok(GeneratedField::RequestHeadersToRemove),
                            "responseHeadersToAdd" | "response_headers_to_add" => Ok(GeneratedField::ResponseHeadersToAdd),
                            "responseHeadersToRemove" | "response_headers_to_remove" => Ok(GeneratedField::ResponseHeadersToRemove),
                            "typedPerFilterConfig" | "typed_per_filter_config" => Ok(GeneratedField::TypedPerFilterConfig),
                            "hostRewriteLiteral" | "host_rewrite_literal" => Ok(GeneratedField::HostRewriteLiteral),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = weighted_cluster::ClusterWeight;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.route.v3.WeightedCluster.ClusterWeight")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<weighted_cluster::ClusterWeight, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut cluster_header__ = None;
                let mut weight__ = None;
                let mut metadata_match__ = None;
                let mut request_headers_to_add__ = None;
                let mut request_headers_to_remove__ = None;
                let mut response_headers_to_add__ = None;
                let mut response_headers_to_remove__ = None;
                let mut typed_per_filter_config__ = None;
                let mut host_rewrite_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClusterHeader => {
                            if cluster_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterHeader"));
                            }
                            cluster_header__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Weight => {
                            if weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            weight__ = map_.next_value()?;
                        }
                        GeneratedField::MetadataMatch => {
                            if metadata_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataMatch"));
                            }
                            metadata_match__ = map_.next_value()?;
                        }
                        GeneratedField::RequestHeadersToAdd => {
                            if request_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToAdd"));
                            }
                            request_headers_to_add__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestHeadersToRemove => {
                            if request_headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToRemove"));
                            }
                            request_headers_to_remove__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToAdd => {
                            if response_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToAdd"));
                            }
                            response_headers_to_add__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToRemove => {
                            if response_headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToRemove"));
                            }
                            response_headers_to_remove__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypedPerFilterConfig => {
                            if typed_per_filter_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedPerFilterConfig"));
                            }
                            typed_per_filter_config__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::HostRewriteLiteral => {
                            if host_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostRewriteLiteral"));
                            }
                            host_rewrite_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(weighted_cluster::cluster_weight::HostRewriteSpecifier::HostRewriteLiteral);
                        }
                    }
                }
                Ok(weighted_cluster::ClusterWeight {
                    name: name__.unwrap_or_default(),
                    cluster_header: cluster_header__.unwrap_or_default(),
                    weight: weight__,
                    metadata_match: metadata_match__,
                    request_headers_to_add: request_headers_to_add__.unwrap_or_default(),
                    request_headers_to_remove: request_headers_to_remove__.unwrap_or_default(),
                    response_headers_to_add: response_headers_to_add__.unwrap_or_default(),
                    response_headers_to_remove: response_headers_to_remove__.unwrap_or_default(),
                    typed_per_filter_config: typed_per_filter_config__.unwrap_or_default(),
                    host_rewrite_specifier: host_rewrite_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.route.v3.WeightedCluster.ClusterWeight", FIELDS, GeneratedVisitor)
    }
}
