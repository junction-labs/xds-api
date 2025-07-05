impl serde::Serialize for Action {
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
        if self.action != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v3.Action", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.action != 0 {
            let v = rbac::Action::try_from(self.action)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.action)))?;
            struct_ser.serialize_field("action", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Action {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "action",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            "name" => Ok(GeneratedField::Name),
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
            type Value = Action;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v3.Action")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Action, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut action__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = Some(map_.next_value::<rbac::Action>()? as i32);
                        }
                    }
                }
                Ok(Action {
                    name: name__.unwrap_or_default(),
                    action: action__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v3.Action", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MetadataSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Dynamic => "DYNAMIC",
            Self::Route => "ROUTE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MetadataSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DYNAMIC",
            "ROUTE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MetadataSource;

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
                    "DYNAMIC" => Ok(MetadataSource::Dynamic),
                    "ROUTE" => Ok(MetadataSource::Route),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Permission {
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v3.Permission", len)?;
        if let Some(v) = self.rule.as_ref() {
            match v {
                permission::Rule::AndRules(v) => {
                    struct_ser.serialize_field("and_rules", v)?;
                }
                permission::Rule::OrRules(v) => {
                    struct_ser.serialize_field("or_rules", v)?;
                }
                permission::Rule::Any(v) => {
                    struct_ser.serialize_field("any", v)?;
                }
                permission::Rule::Header(v) => {
                    struct_ser.serialize_field("header", v)?;
                }
                permission::Rule::UrlPath(v) => {
                    struct_ser.serialize_field("url_path", v)?;
                }
                permission::Rule::DestinationIp(v) => {
                    struct_ser.serialize_field("destination_ip", v)?;
                }
                permission::Rule::DestinationPort(v) => {
                    struct_ser.serialize_field("destination_port", v)?;
                }
                permission::Rule::DestinationPortRange(v) => {
                    struct_ser.serialize_field("destination_port_range", v)?;
                }
                permission::Rule::Metadata(v) => {
                    struct_ser.serialize_field("metadata", v)?;
                }
                permission::Rule::NotRule(v) => {
                    struct_ser.serialize_field("not_rule", v)?;
                }
                permission::Rule::RequestedServerName(v) => {
                    struct_ser.serialize_field("requested_server_name", v)?;
                }
                permission::Rule::Matcher(v) => {
                    struct_ser.serialize_field("matcher", v)?;
                }
                permission::Rule::UriTemplate(v) => {
                    struct_ser.serialize_field("uri_template", v)?;
                }
                permission::Rule::SourcedMetadata(v) => {
                    struct_ser.serialize_field("sourced_metadata", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Permission {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "and_rules",
            "andRules",
            "or_rules",
            "orRules",
            "any",
            "header",
            "url_path",
            "urlPath",
            "destination_ip",
            "destinationIp",
            "destination_port",
            "destinationPort",
            "destination_port_range",
            "destinationPortRange",
            "metadata",
            "not_rule",
            "notRule",
            "requested_server_name",
            "requestedServerName",
            "matcher",
            "uri_template",
            "uriTemplate",
            "sourced_metadata",
            "sourcedMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AndRules,
            OrRules,
            Any,
            Header,
            UrlPath,
            DestinationIp,
            DestinationPort,
            DestinationPortRange,
            Metadata,
            NotRule,
            RequestedServerName,
            Matcher,
            UriTemplate,
            SourcedMetadata,
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
                            "andRules" | "and_rules" => Ok(GeneratedField::AndRules),
                            "orRules" | "or_rules" => Ok(GeneratedField::OrRules),
                            "any" => Ok(GeneratedField::Any),
                            "header" => Ok(GeneratedField::Header),
                            "urlPath" | "url_path" => Ok(GeneratedField::UrlPath),
                            "destinationIp" | "destination_ip" => Ok(GeneratedField::DestinationIp),
                            "destinationPort" | "destination_port" => Ok(GeneratedField::DestinationPort),
                            "destinationPortRange" | "destination_port_range" => Ok(GeneratedField::DestinationPortRange),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "notRule" | "not_rule" => Ok(GeneratedField::NotRule),
                            "requestedServerName" | "requested_server_name" => Ok(GeneratedField::RequestedServerName),
                            "matcher" => Ok(GeneratedField::Matcher),
                            "uriTemplate" | "uri_template" => Ok(GeneratedField::UriTemplate),
                            "sourcedMetadata" | "sourced_metadata" => Ok(GeneratedField::SourcedMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Permission;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v3.Permission")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Permission, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AndRules => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("andRules"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(permission::Rule::AndRules)
;
                        }
                        GeneratedField::OrRules => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orRules"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(permission::Rule::OrRules)
;
                        }
                        GeneratedField::Any => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("any"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(permission::Rule::Any);
                        }
                        GeneratedField::Header => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(permission::Rule::Header)
;
                        }
                        GeneratedField::UrlPath => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("urlPath"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(permission::Rule::UrlPath)
;
                        }
                        GeneratedField::DestinationIp => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationIp"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(permission::Rule::DestinationIp)
;
                        }
                        GeneratedField::DestinationPort => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationPort"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| permission::Rule::DestinationPort(x.0));
                        }
                        GeneratedField::DestinationPortRange => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationPortRange"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(permission::Rule::DestinationPortRange)
;
                        }
                        GeneratedField::Metadata => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(permission::Rule::Metadata)
;
                        }
                        GeneratedField::NotRule => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notRule"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(permission::Rule::NotRule)
;
                        }
                        GeneratedField::RequestedServerName => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestedServerName"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(permission::Rule::RequestedServerName)
;
                        }
                        GeneratedField::Matcher => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matcher"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(permission::Rule::Matcher)
;
                        }
                        GeneratedField::UriTemplate => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uriTemplate"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(permission::Rule::UriTemplate)
;
                        }
                        GeneratedField::SourcedMetadata => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourcedMetadata"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(permission::Rule::SourcedMetadata)
;
                        }
                    }
                }
                Ok(Permission {
                    rule: rule__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v3.Permission", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for permission::Set {
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v3.Permission.Set", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for permission::Set {
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
            type Value = permission::Set;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v3.Permission.Set")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<permission::Set, V::Error>
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
                Ok(permission::Set {
                    rules: rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v3.Permission.Set", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Policy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.permissions.is_empty() {
            len += 1;
        }
        if !self.principals.is_empty() {
            len += 1;
        }
        if self.condition.is_some() {
            len += 1;
        }
        if self.checked_condition.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v3.Policy", len)?;
        if !self.permissions.is_empty() {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        if !self.principals.is_empty() {
            struct_ser.serialize_field("principals", &self.principals)?;
        }
        if let Some(v) = self.condition.as_ref() {
            struct_ser.serialize_field("condition", v)?;
        }
        if let Some(v) = self.checked_condition.as_ref() {
            struct_ser.serialize_field("checked_condition", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Policy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "permissions",
            "principals",
            "condition",
            "checked_condition",
            "checkedCondition",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Permissions,
            Principals,
            Condition,
            CheckedCondition,
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
                            "permissions" => Ok(GeneratedField::Permissions),
                            "principals" => Ok(GeneratedField::Principals),
                            "condition" => Ok(GeneratedField::Condition),
                            "checkedCondition" | "checked_condition" => Ok(GeneratedField::CheckedCondition),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Policy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v3.Policy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Policy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut permissions__ = None;
                let mut principals__ = None;
                let mut condition__ = None;
                let mut checked_condition__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Principals => {
                            if principals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principals"));
                            }
                            principals__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Condition => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            condition__ = map_.next_value()?;
                        }
                        GeneratedField::CheckedCondition => {
                            if checked_condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkedCondition"));
                            }
                            checked_condition__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Policy {
                    permissions: permissions__.unwrap_or_default(),
                    principals: principals__.unwrap_or_default(),
                    condition: condition__,
                    checked_condition: checked_condition__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v3.Policy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Principal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v3.Principal", len)?;
        if let Some(v) = self.identifier.as_ref() {
            match v {
                principal::Identifier::AndIds(v) => {
                    struct_ser.serialize_field("and_ids", v)?;
                }
                principal::Identifier::OrIds(v) => {
                    struct_ser.serialize_field("or_ids", v)?;
                }
                principal::Identifier::Any(v) => {
                    struct_ser.serialize_field("any", v)?;
                }
                principal::Identifier::Authenticated(v) => {
                    struct_ser.serialize_field("authenticated", v)?;
                }
                principal::Identifier::SourceIp(v) => {
                    struct_ser.serialize_field("source_ip", v)?;
                }
                principal::Identifier::DirectRemoteIp(v) => {
                    struct_ser.serialize_field("direct_remote_ip", v)?;
                }
                principal::Identifier::RemoteIp(v) => {
                    struct_ser.serialize_field("remote_ip", v)?;
                }
                principal::Identifier::Header(v) => {
                    struct_ser.serialize_field("header", v)?;
                }
                principal::Identifier::UrlPath(v) => {
                    struct_ser.serialize_field("url_path", v)?;
                }
                principal::Identifier::Metadata(v) => {
                    struct_ser.serialize_field("metadata", v)?;
                }
                principal::Identifier::FilterState(v) => {
                    struct_ser.serialize_field("filter_state", v)?;
                }
                principal::Identifier::NotId(v) => {
                    struct_ser.serialize_field("not_id", v)?;
                }
                principal::Identifier::SourcedMetadata(v) => {
                    struct_ser.serialize_field("sourced_metadata", v)?;
                }
                principal::Identifier::Custom(v) => {
                    struct_ser.serialize_field("custom", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Principal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "and_ids",
            "andIds",
            "or_ids",
            "orIds",
            "any",
            "authenticated",
            "source_ip",
            "sourceIp",
            "direct_remote_ip",
            "directRemoteIp",
            "remote_ip",
            "remoteIp",
            "header",
            "url_path",
            "urlPath",
            "metadata",
            "filter_state",
            "filterState",
            "not_id",
            "notId",
            "sourced_metadata",
            "sourcedMetadata",
            "custom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AndIds,
            OrIds,
            Any,
            Authenticated,
            SourceIp,
            DirectRemoteIp,
            RemoteIp,
            Header,
            UrlPath,
            Metadata,
            FilterState,
            NotId,
            SourcedMetadata,
            Custom,
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
                            "andIds" | "and_ids" => Ok(GeneratedField::AndIds),
                            "orIds" | "or_ids" => Ok(GeneratedField::OrIds),
                            "any" => Ok(GeneratedField::Any),
                            "authenticated" => Ok(GeneratedField::Authenticated),
                            "sourceIp" | "source_ip" => Ok(GeneratedField::SourceIp),
                            "directRemoteIp" | "direct_remote_ip" => Ok(GeneratedField::DirectRemoteIp),
                            "remoteIp" | "remote_ip" => Ok(GeneratedField::RemoteIp),
                            "header" => Ok(GeneratedField::Header),
                            "urlPath" | "url_path" => Ok(GeneratedField::UrlPath),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "filterState" | "filter_state" => Ok(GeneratedField::FilterState),
                            "notId" | "not_id" => Ok(GeneratedField::NotId),
                            "sourcedMetadata" | "sourced_metadata" => Ok(GeneratedField::SourcedMetadata),
                            "custom" => Ok(GeneratedField::Custom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Principal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v3.Principal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Principal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AndIds => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("andIds"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::AndIds)
;
                        }
                        GeneratedField::OrIds => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orIds"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::OrIds)
;
                        }
                        GeneratedField::Any => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("any"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::Any);
                        }
                        GeneratedField::Authenticated => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authenticated"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::Authenticated)
;
                        }
                        GeneratedField::SourceIp => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceIp"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::SourceIp)
;
                        }
                        GeneratedField::DirectRemoteIp => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directRemoteIp"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::DirectRemoteIp)
;
                        }
                        GeneratedField::RemoteIp => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteIp"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::RemoteIp)
;
                        }
                        GeneratedField::Header => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::Header)
;
                        }
                        GeneratedField::UrlPath => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("urlPath"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::UrlPath)
;
                        }
                        GeneratedField::Metadata => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::Metadata)
;
                        }
                        GeneratedField::FilterState => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterState"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::FilterState)
;
                        }
                        GeneratedField::NotId => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notId"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::NotId)
;
                        }
                        GeneratedField::SourcedMetadata => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourcedMetadata"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::SourcedMetadata)
;
                        }
                        GeneratedField::Custom => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("custom"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::Custom)
;
                        }
                    }
                }
                Ok(Principal {
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v3.Principal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for principal::Authenticated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.principal_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v3.Principal.Authenticated", len)?;
        if let Some(v) = self.principal_name.as_ref() {
            struct_ser.serialize_field("principal_name", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for principal::Authenticated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_name",
            "principalName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalName,
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
                            "principalName" | "principal_name" => Ok(GeneratedField::PrincipalName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = principal::Authenticated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v3.Principal.Authenticated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<principal::Authenticated, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalName => {
                            if principal_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalName"));
                            }
                            principal_name__ = map_.next_value()?;
                        }
                    }
                }
                Ok(principal::Authenticated {
                    principal_name: principal_name__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v3.Principal.Authenticated", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for principal::Set {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v3.Principal.Set", len)?;
        if !self.ids.is_empty() {
            struct_ser.serialize_field("ids", &self.ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for principal::Set {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ids",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ids,
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
                            "ids" => Ok(GeneratedField::Ids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = principal::Set;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v3.Principal.Set")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<principal::Set, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ids => {
                            if ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ids"));
                            }
                            ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(principal::Set {
                    ids: ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v3.Principal.Set", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Rbac {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action != 0 {
            len += 1;
        }
        if !self.policies.is_empty() {
            len += 1;
        }
        if self.audit_logging_options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v3.RBAC", len)?;
        if self.action != 0 {
            let v = rbac::Action::try_from(self.action)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.action)))?;
            struct_ser.serialize_field("action", &v)?;
        }
        if !self.policies.is_empty() {
            struct_ser.serialize_field("policies", &self.policies)?;
        }
        if let Some(v) = self.audit_logging_options.as_ref() {
            struct_ser.serialize_field("audit_logging_options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Rbac {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "action",
            "policies",
            "audit_logging_options",
            "auditLoggingOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Action,
            Policies,
            AuditLoggingOptions,
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
                            "policies" => Ok(GeneratedField::Policies),
                            "auditLoggingOptions" | "audit_logging_options" => Ok(GeneratedField::AuditLoggingOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Rbac;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v3.RBAC")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Rbac, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action__ = None;
                let mut policies__ = None;
                let mut audit_logging_options__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = Some(map_.next_value::<rbac::Action>()? as i32);
                        }
                        GeneratedField::Policies => {
                            if policies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policies"));
                            }
                            policies__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::AuditLoggingOptions => {
                            if audit_logging_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("auditLoggingOptions"));
                            }
                            audit_logging_options__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Rbac {
                    action: action__.unwrap_or_default(),
                    policies: policies__.unwrap_or_default(),
                    audit_logging_options: audit_logging_options__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v3.RBAC", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rbac::Action {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Allow => "ALLOW",
            Self::Deny => "DENY",
            Self::Log => "LOG",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for rbac::Action {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ALLOW",
            "DENY",
            "LOG",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rbac::Action;

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
                    "ALLOW" => Ok(rbac::Action::Allow),
                    "DENY" => Ok(rbac::Action::Deny),
                    "LOG" => Ok(rbac::Action::Log),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for rbac::AuditLoggingOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.audit_condition != 0 {
            len += 1;
        }
        if !self.logger_configs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v3.RBAC.AuditLoggingOptions", len)?;
        if self.audit_condition != 0 {
            let v = rbac::audit_logging_options::AuditCondition::try_from(self.audit_condition)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.audit_condition)))?;
            struct_ser.serialize_field("audit_condition", &v)?;
        }
        if !self.logger_configs.is_empty() {
            struct_ser.serialize_field("logger_configs", &self.logger_configs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rbac::AuditLoggingOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "audit_condition",
            "auditCondition",
            "logger_configs",
            "loggerConfigs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuditCondition,
            LoggerConfigs,
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
                            "auditCondition" | "audit_condition" => Ok(GeneratedField::AuditCondition),
                            "loggerConfigs" | "logger_configs" => Ok(GeneratedField::LoggerConfigs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rbac::AuditLoggingOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v3.RBAC.AuditLoggingOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rbac::AuditLoggingOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut audit_condition__ = None;
                let mut logger_configs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuditCondition => {
                            if audit_condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("auditCondition"));
                            }
                            audit_condition__ = Some(map_.next_value::<rbac::audit_logging_options::AuditCondition>()? as i32);
                        }
                        GeneratedField::LoggerConfigs => {
                            if logger_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loggerConfigs"));
                            }
                            logger_configs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(rbac::AuditLoggingOptions {
                    audit_condition: audit_condition__.unwrap_or_default(),
                    logger_configs: logger_configs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v3.RBAC.AuditLoggingOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rbac::audit_logging_options::AuditCondition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::None => "NONE",
            Self::OnDeny => "ON_DENY",
            Self::OnAllow => "ON_ALLOW",
            Self::OnDenyAndAllow => "ON_DENY_AND_ALLOW",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for rbac::audit_logging_options::AuditCondition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NONE",
            "ON_DENY",
            "ON_ALLOW",
            "ON_DENY_AND_ALLOW",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rbac::audit_logging_options::AuditCondition;

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
                    "NONE" => Ok(rbac::audit_logging_options::AuditCondition::None),
                    "ON_DENY" => Ok(rbac::audit_logging_options::AuditCondition::OnDeny),
                    "ON_ALLOW" => Ok(rbac::audit_logging_options::AuditCondition::OnAllow),
                    "ON_DENY_AND_ALLOW" => Ok(rbac::audit_logging_options::AuditCondition::OnDenyAndAllow),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for rbac::audit_logging_options::AuditLoggerConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.audit_logger.is_some() {
            len += 1;
        }
        if self.is_optional {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v3.RBAC.AuditLoggingOptions.AuditLoggerConfig", len)?;
        if let Some(v) = self.audit_logger.as_ref() {
            struct_ser.serialize_field("audit_logger", v)?;
        }
        if self.is_optional {
            struct_ser.serialize_field("is_optional", &self.is_optional)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rbac::audit_logging_options::AuditLoggerConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "audit_logger",
            "auditLogger",
            "is_optional",
            "isOptional",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuditLogger,
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
                            "auditLogger" | "audit_logger" => Ok(GeneratedField::AuditLogger),
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
            type Value = rbac::audit_logging_options::AuditLoggerConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v3.RBAC.AuditLoggingOptions.AuditLoggerConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rbac::audit_logging_options::AuditLoggerConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut audit_logger__ = None;
                let mut is_optional__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuditLogger => {
                            if audit_logger__.is_some() {
                                return Err(serde::de::Error::duplicate_field("auditLogger"));
                            }
                            audit_logger__ = map_.next_value()?;
                        }
                        GeneratedField::IsOptional => {
                            if is_optional__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isOptional"));
                            }
                            is_optional__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(rbac::audit_logging_options::AuditLoggerConfig {
                    audit_logger: audit_logger__,
                    is_optional: is_optional__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v3.RBAC.AuditLoggingOptions.AuditLoggerConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SourcedMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata_matcher.is_some() {
            len += 1;
        }
        if self.metadata_source != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v3.SourcedMetadata", len)?;
        if let Some(v) = self.metadata_matcher.as_ref() {
            struct_ser.serialize_field("metadata_matcher", v)?;
        }
        if self.metadata_source != 0 {
            let v = MetadataSource::try_from(self.metadata_source)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.metadata_source)))?;
            struct_ser.serialize_field("metadata_source", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SourcedMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata_matcher",
            "metadataMatcher",
            "metadata_source",
            "metadataSource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MetadataMatcher,
            MetadataSource,
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
                            "metadataMatcher" | "metadata_matcher" => Ok(GeneratedField::MetadataMatcher),
                            "metadataSource" | "metadata_source" => Ok(GeneratedField::MetadataSource),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SourcedMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v3.SourcedMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SourcedMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata_matcher__ = None;
                let mut metadata_source__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MetadataMatcher => {
                            if metadata_matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataMatcher"));
                            }
                            metadata_matcher__ = map_.next_value()?;
                        }
                        GeneratedField::MetadataSource => {
                            if metadata_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataSource"));
                            }
                            metadata_source__ = Some(map_.next_value::<MetadataSource>()? as i32);
                        }
                    }
                }
                Ok(SourcedMetadata {
                    metadata_matcher: metadata_matcher__,
                    metadata_source: metadata_source__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v3.SourcedMetadata", FIELDS, GeneratedVisitor)
    }
}
