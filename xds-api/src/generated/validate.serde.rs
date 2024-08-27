impl serde::Serialize for AnyRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.required.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.AnyRules", len)?;
        if let Some(v) = self.required.as_ref() {
            struct_ser.serialize_field("required", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("not_in", &self.not_in)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnyRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "required",
            "in",
            "not_in",
            "notIn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Required,
            In,
            NotIn,
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
                            "required" => Ok(GeneratedField::Required),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnyRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.AnyRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnyRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut required__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Required => {
                            if required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("required"));
                            }
                            required__ = map_.next_value()?;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AnyRules {
                    required: required__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("validate.AnyRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BoolRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.BoolRules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BoolRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
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
                            "const" => Ok(GeneratedField::Const),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BoolRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.BoolRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BoolRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BoolRules {
                    r#const: r#const__,
                })
            }
        }
        deserializer.deserialize_struct("validate.BoolRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BytesRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.len.is_some() {
            len += 1;
        }
        if self.min_len.is_some() {
            len += 1;
        }
        if self.max_len.is_some() {
            len += 1;
        }
        if self.pattern.is_some() {
            len += 1;
        }
        if self.prefix.is_some() {
            len += 1;
        }
        if self.suffix.is_some() {
            len += 1;
        }
        if self.contains.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if self.ignore_empty.is_some() {
            len += 1;
        }
        if self.well_known.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.BytesRules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("const", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.len.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("len", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.min_len.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("min_len", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.max_len.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("max_len", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.pattern.as_ref() {
            struct_ser.serialize_field("pattern", v)?;
        }
        if let Some(v) = self.prefix.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("prefix", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.suffix.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("suffix", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.contains.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("contains", pbjson::private::base64::encode(&v).as_str())?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("not_in", &self.not_in.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.ignore_empty.as_ref() {
            struct_ser.serialize_field("ignore_empty", v)?;
        }
        if let Some(v) = self.well_known.as_ref() {
            match v {
                bytes_rules::WellKnown::Ip(v) => {
                    struct_ser.serialize_field("ip", v)?;
                }
                bytes_rules::WellKnown::Ipv4(v) => {
                    struct_ser.serialize_field("ipv4", v)?;
                }
                bytes_rules::WellKnown::Ipv6(v) => {
                    struct_ser.serialize_field("ipv6", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BytesRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "len",
            "min_len",
            "minLen",
            "max_len",
            "maxLen",
            "pattern",
            "prefix",
            "suffix",
            "contains",
            "in",
            "not_in",
            "notIn",
            "ignore_empty",
            "ignoreEmpty",
            "ip",
            "ipv4",
            "ipv6",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Len,
            MinLen,
            MaxLen,
            Pattern,
            Prefix,
            Suffix,
            Contains,
            In,
            NotIn,
            IgnoreEmpty,
            Ip,
            Ipv4,
            Ipv6,
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
                            "const" => Ok(GeneratedField::Const),
                            "len" => Ok(GeneratedField::Len),
                            "minLen" | "min_len" => Ok(GeneratedField::MinLen),
                            "maxLen" | "max_len" => Ok(GeneratedField::MaxLen),
                            "pattern" => Ok(GeneratedField::Pattern),
                            "prefix" => Ok(GeneratedField::Prefix),
                            "suffix" => Ok(GeneratedField::Suffix),
                            "contains" => Ok(GeneratedField::Contains),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "ignoreEmpty" | "ignore_empty" => Ok(GeneratedField::IgnoreEmpty),
                            "ip" => Ok(GeneratedField::Ip),
                            "ipv4" => Ok(GeneratedField::Ipv4),
                            "ipv6" => Ok(GeneratedField::Ipv6),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BytesRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.BytesRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BytesRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut len__ = None;
                let mut min_len__ = None;
                let mut max_len__ = None;
                let mut pattern__ = None;
                let mut prefix__ = None;
                let mut suffix__ = None;
                let mut contains__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut ignore_empty__ = None;
                let mut well_known__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Len => {
                            if len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("len"));
                            }
                            len__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MinLen => {
                            if min_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minLen"));
                            }
                            min_len__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxLen => {
                            if max_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLen"));
                            }
                            max_len__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Pattern => {
                            if pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            pattern__ = map_.next_value()?;
                        }
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Suffix => {
                            if suffix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suffix"));
                            }
                            suffix__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Contains => {
                            if contains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contains"));
                            }
                            contains__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::IgnoreEmpty => {
                            if ignore_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreEmpty"));
                            }
                            ignore_empty__ = map_.next_value()?;
                        }
                        GeneratedField::Ip => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(bytes_rules::WellKnown::Ip);
                        }
                        GeneratedField::Ipv4 => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv4"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(bytes_rules::WellKnown::Ipv4);
                        }
                        GeneratedField::Ipv6 => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv6"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(bytes_rules::WellKnown::Ipv6);
                        }
                    }
                }
                Ok(BytesRules {
                    r#const: r#const__,
                    len: len__,
                    min_len: min_len__,
                    max_len: max_len__,
                    pattern: pattern__,
                    prefix: prefix__,
                    suffix: suffix__,
                    contains: contains__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    ignore_empty: ignore_empty__,
                    well_known: well_known__,
                })
            }
        }
        deserializer.deserialize_struct("validate.BytesRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DoubleRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.lt.is_some() {
            len += 1;
        }
        if self.lte.is_some() {
            len += 1;
        }
        if self.gt.is_some() {
            len += 1;
        }
        if self.gte.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if self.ignore_empty.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.DoubleRules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if let Some(v) = self.lt.as_ref() {
            struct_ser.serialize_field("lt", v)?;
        }
        if let Some(v) = self.lte.as_ref() {
            struct_ser.serialize_field("lte", v)?;
        }
        if let Some(v) = self.gt.as_ref() {
            struct_ser.serialize_field("gt", v)?;
        }
        if let Some(v) = self.gte.as_ref() {
            struct_ser.serialize_field("gte", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("not_in", &self.not_in)?;
        }
        if let Some(v) = self.ignore_empty.as_ref() {
            struct_ser.serialize_field("ignore_empty", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DoubleRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "lt",
            "lte",
            "gt",
            "gte",
            "in",
            "not_in",
            "notIn",
            "ignore_empty",
            "ignoreEmpty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Lt,
            Lte,
            Gt,
            Gte,
            In,
            NotIn,
            IgnoreEmpty,
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
                            "const" => Ok(GeneratedField::Const),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "ignoreEmpty" | "ignore_empty" => Ok(GeneratedField::IgnoreEmpty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DoubleRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.DoubleRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DoubleRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut lt__ = None;
                let mut lte__ = None;
                let mut gt__ = None;
                let mut gte__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut ignore_empty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lt => {
                            if lt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            lt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lte => {
                            if lte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            lte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gt => {
                            if gt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            gt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gte => {
                            if gte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            gte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::IgnoreEmpty => {
                            if ignore_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreEmpty"));
                            }
                            ignore_empty__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DoubleRules {
                    r#const: r#const__,
                    lt: lt__,
                    lte: lte__,
                    gt: gt__,
                    gte: gte__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    ignore_empty: ignore_empty__,
                })
            }
        }
        deserializer.deserialize_struct("validate.DoubleRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DurationRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.required.is_some() {
            len += 1;
        }
        if self.r#const.is_some() {
            len += 1;
        }
        if self.lt.is_some() {
            len += 1;
        }
        if self.lte.is_some() {
            len += 1;
        }
        if self.gt.is_some() {
            len += 1;
        }
        if self.gte.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.DurationRules", len)?;
        if let Some(v) = self.required.as_ref() {
            struct_ser.serialize_field("required", v)?;
        }
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if let Some(v) = self.lt.as_ref() {
            struct_ser.serialize_field("lt", v)?;
        }
        if let Some(v) = self.lte.as_ref() {
            struct_ser.serialize_field("lte", v)?;
        }
        if let Some(v) = self.gt.as_ref() {
            struct_ser.serialize_field("gt", v)?;
        }
        if let Some(v) = self.gte.as_ref() {
            struct_ser.serialize_field("gte", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("not_in", &self.not_in)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DurationRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "required",
            "const",
            "lt",
            "lte",
            "gt",
            "gte",
            "in",
            "not_in",
            "notIn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Required,
            Const,
            Lt,
            Lte,
            Gt,
            Gte,
            In,
            NotIn,
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
                            "required" => Ok(GeneratedField::Required),
                            "const" => Ok(GeneratedField::Const),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DurationRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.DurationRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DurationRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut required__ = None;
                let mut r#const__ = None;
                let mut lt__ = None;
                let mut lte__ = None;
                let mut gt__ = None;
                let mut gte__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Required => {
                            if required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("required"));
                            }
                            required__ = map_.next_value()?;
                        }
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = map_.next_value()?;
                        }
                        GeneratedField::Lt => {
                            if lt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            lt__ = map_.next_value()?;
                        }
                        GeneratedField::Lte => {
                            if lte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            lte__ = map_.next_value()?;
                        }
                        GeneratedField::Gt => {
                            if gt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            gt__ = map_.next_value()?;
                        }
                        GeneratedField::Gte => {
                            if gte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            gte__ = map_.next_value()?;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DurationRules {
                    required: required__,
                    r#const: r#const__,
                    lt: lt__,
                    lte: lte__,
                    gt: gt__,
                    gte: gte__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("validate.DurationRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnumRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.defined_only.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.EnumRules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if let Some(v) = self.defined_only.as_ref() {
            struct_ser.serialize_field("defined_only", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("not_in", &self.not_in)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnumRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "defined_only",
            "definedOnly",
            "in",
            "not_in",
            "notIn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            DefinedOnly,
            In,
            NotIn,
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
                            "const" => Ok(GeneratedField::Const),
                            "definedOnly" | "defined_only" => Ok(GeneratedField::DefinedOnly),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnumRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.EnumRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EnumRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut defined_only__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::DefinedOnly => {
                            if defined_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("definedOnly"));
                            }
                            defined_only__ = map_.next_value()?;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(EnumRules {
                    r#const: r#const__,
                    defined_only: defined_only__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("validate.EnumRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FieldRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.message.is_some() {
            len += 1;
        }
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.FieldRules", len)?;
        if let Some(v) = self.message.as_ref() {
            struct_ser.serialize_field("message", v)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            match v {
                field_rules::Type::Float(v) => {
                    struct_ser.serialize_field("float", v)?;
                }
                field_rules::Type::Double(v) => {
                    struct_ser.serialize_field("double", v)?;
                }
                field_rules::Type::Int32(v) => {
                    struct_ser.serialize_field("int32", v)?;
                }
                field_rules::Type::Int64(v) => {
                    struct_ser.serialize_field("int64", v)?;
                }
                field_rules::Type::Uint32(v) => {
                    struct_ser.serialize_field("uint32", v)?;
                }
                field_rules::Type::Uint64(v) => {
                    struct_ser.serialize_field("uint64", v)?;
                }
                field_rules::Type::Sint32(v) => {
                    struct_ser.serialize_field("sint32", v)?;
                }
                field_rules::Type::Sint64(v) => {
                    struct_ser.serialize_field("sint64", v)?;
                }
                field_rules::Type::Fixed32(v) => {
                    struct_ser.serialize_field("fixed32", v)?;
                }
                field_rules::Type::Fixed64(v) => {
                    struct_ser.serialize_field("fixed64", v)?;
                }
                field_rules::Type::Sfixed32(v) => {
                    struct_ser.serialize_field("sfixed32", v)?;
                }
                field_rules::Type::Sfixed64(v) => {
                    struct_ser.serialize_field("sfixed64", v)?;
                }
                field_rules::Type::Bool(v) => {
                    struct_ser.serialize_field("bool", v)?;
                }
                field_rules::Type::String(v) => {
                    struct_ser.serialize_field("string", v)?;
                }
                field_rules::Type::Bytes(v) => {
                    struct_ser.serialize_field("bytes", v)?;
                }
                field_rules::Type::Enum(v) => {
                    struct_ser.serialize_field("enum", v)?;
                }
                field_rules::Type::Repeated(v) => {
                    struct_ser.serialize_field("repeated", v)?;
                }
                field_rules::Type::Map(v) => {
                    struct_ser.serialize_field("map", v)?;
                }
                field_rules::Type::Any(v) => {
                    struct_ser.serialize_field("any", v)?;
                }
                field_rules::Type::Duration(v) => {
                    struct_ser.serialize_field("duration", v)?;
                }
                field_rules::Type::Timestamp(v) => {
                    struct_ser.serialize_field("timestamp", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FieldRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
            "float",
            "double",
            "int32",
            "int64",
            "uint32",
            "uint64",
            "sint32",
            "sint64",
            "fixed32",
            "fixed64",
            "sfixed32",
            "sfixed64",
            "bool",
            "string",
            "bytes",
            "enum",
            "repeated",
            "map",
            "any",
            "duration",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
            Float,
            Double,
            Int32,
            Int64,
            Uint32,
            Uint64,
            Sint32,
            Sint64,
            Fixed32,
            Fixed64,
            Sfixed32,
            Sfixed64,
            Bool,
            String,
            Bytes,
            Enum,
            Repeated,
            Map,
            Any,
            Duration,
            Timestamp,
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
                            "message" => Ok(GeneratedField::Message),
                            "float" => Ok(GeneratedField::Float),
                            "double" => Ok(GeneratedField::Double),
                            "int32" => Ok(GeneratedField::Int32),
                            "int64" => Ok(GeneratedField::Int64),
                            "uint32" => Ok(GeneratedField::Uint32),
                            "uint64" => Ok(GeneratedField::Uint64),
                            "sint32" => Ok(GeneratedField::Sint32),
                            "sint64" => Ok(GeneratedField::Sint64),
                            "fixed32" => Ok(GeneratedField::Fixed32),
                            "fixed64" => Ok(GeneratedField::Fixed64),
                            "sfixed32" => Ok(GeneratedField::Sfixed32),
                            "sfixed64" => Ok(GeneratedField::Sfixed64),
                            "bool" => Ok(GeneratedField::Bool),
                            "string" => Ok(GeneratedField::String),
                            "bytes" => Ok(GeneratedField::Bytes),
                            "enum" => Ok(GeneratedField::Enum),
                            "repeated" => Ok(GeneratedField::Repeated),
                            "map" => Ok(GeneratedField::Map),
                            "any" => Ok(GeneratedField::Any),
                            "duration" => Ok(GeneratedField::Duration),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.FieldRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FieldRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = map_.next_value()?;
                        }
                        GeneratedField::Float => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("float"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Float)
;
                        }
                        GeneratedField::Double => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("double"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Double)
;
                        }
                        GeneratedField::Int32 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int32"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Int32)
;
                        }
                        GeneratedField::Int64 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int64"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Int64)
;
                        }
                        GeneratedField::Uint32 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uint32"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Uint32)
;
                        }
                        GeneratedField::Uint64 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uint64"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Uint64)
;
                        }
                        GeneratedField::Sint32 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sint32"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Sint32)
;
                        }
                        GeneratedField::Sint64 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sint64"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Sint64)
;
                        }
                        GeneratedField::Fixed32 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixed32"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Fixed32)
;
                        }
                        GeneratedField::Fixed64 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixed64"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Fixed64)
;
                        }
                        GeneratedField::Sfixed32 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sfixed32"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Sfixed32)
;
                        }
                        GeneratedField::Sfixed64 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sfixed64"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Sfixed64)
;
                        }
                        GeneratedField::Bool => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bool"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Bool)
;
                        }
                        GeneratedField::String => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("string"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::String)
;
                        }
                        GeneratedField::Bytes => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bytes"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Bytes)
;
                        }
                        GeneratedField::Enum => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enum"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Enum)
;
                        }
                        GeneratedField::Repeated => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("repeated"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Repeated)
;
                        }
                        GeneratedField::Map => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("map"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Map)
;
                        }
                        GeneratedField::Any => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("any"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Any)
;
                        }
                        GeneratedField::Duration => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Duration)
;
                        }
                        GeneratedField::Timestamp => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Timestamp)
;
                        }
                    }
                }
                Ok(FieldRules {
                    message: message__,
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("validate.FieldRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Fixed32Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.lt.is_some() {
            len += 1;
        }
        if self.lte.is_some() {
            len += 1;
        }
        if self.gt.is_some() {
            len += 1;
        }
        if self.gte.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if self.ignore_empty.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.Fixed32Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if let Some(v) = self.lt.as_ref() {
            struct_ser.serialize_field("lt", v)?;
        }
        if let Some(v) = self.lte.as_ref() {
            struct_ser.serialize_field("lte", v)?;
        }
        if let Some(v) = self.gt.as_ref() {
            struct_ser.serialize_field("gt", v)?;
        }
        if let Some(v) = self.gte.as_ref() {
            struct_ser.serialize_field("gte", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("not_in", &self.not_in)?;
        }
        if let Some(v) = self.ignore_empty.as_ref() {
            struct_ser.serialize_field("ignore_empty", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Fixed32Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "lt",
            "lte",
            "gt",
            "gte",
            "in",
            "not_in",
            "notIn",
            "ignore_empty",
            "ignoreEmpty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Lt,
            Lte,
            Gt,
            Gte,
            In,
            NotIn,
            IgnoreEmpty,
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
                            "const" => Ok(GeneratedField::Const),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "ignoreEmpty" | "ignore_empty" => Ok(GeneratedField::IgnoreEmpty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Fixed32Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.Fixed32Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Fixed32Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut lt__ = None;
                let mut lte__ = None;
                let mut gt__ = None;
                let mut gte__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut ignore_empty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lt => {
                            if lt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            lt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lte => {
                            if lte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            lte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gt => {
                            if gt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            gt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gte => {
                            if gte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            gte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::IgnoreEmpty => {
                            if ignore_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreEmpty"));
                            }
                            ignore_empty__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Fixed32Rules {
                    r#const: r#const__,
                    lt: lt__,
                    lte: lte__,
                    gt: gt__,
                    gte: gte__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    ignore_empty: ignore_empty__,
                })
            }
        }
        deserializer.deserialize_struct("validate.Fixed32Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Fixed64Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.lt.is_some() {
            len += 1;
        }
        if self.lte.is_some() {
            len += 1;
        }
        if self.gt.is_some() {
            len += 1;
        }
        if self.gte.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if self.ignore_empty.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.Fixed64Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("const", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.lt.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.lte.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lte", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.gt.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("gt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.gte.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("gte", ToString::to_string(&v).as_str())?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("not_in", &self.not_in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.ignore_empty.as_ref() {
            struct_ser.serialize_field("ignore_empty", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Fixed64Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "lt",
            "lte",
            "gt",
            "gte",
            "in",
            "not_in",
            "notIn",
            "ignore_empty",
            "ignoreEmpty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Lt,
            Lte,
            Gt,
            Gte,
            In,
            NotIn,
            IgnoreEmpty,
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
                            "const" => Ok(GeneratedField::Const),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "ignoreEmpty" | "ignore_empty" => Ok(GeneratedField::IgnoreEmpty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Fixed64Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.Fixed64Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Fixed64Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut lt__ = None;
                let mut lte__ = None;
                let mut gt__ = None;
                let mut gte__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut ignore_empty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lt => {
                            if lt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            lt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lte => {
                            if lte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            lte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gt => {
                            if gt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            gt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gte => {
                            if gte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            gte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::IgnoreEmpty => {
                            if ignore_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreEmpty"));
                            }
                            ignore_empty__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Fixed64Rules {
                    r#const: r#const__,
                    lt: lt__,
                    lte: lte__,
                    gt: gt__,
                    gte: gte__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    ignore_empty: ignore_empty__,
                })
            }
        }
        deserializer.deserialize_struct("validate.Fixed64Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FloatRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.lt.is_some() {
            len += 1;
        }
        if self.lte.is_some() {
            len += 1;
        }
        if self.gt.is_some() {
            len += 1;
        }
        if self.gte.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if self.ignore_empty.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.FloatRules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if let Some(v) = self.lt.as_ref() {
            struct_ser.serialize_field("lt", v)?;
        }
        if let Some(v) = self.lte.as_ref() {
            struct_ser.serialize_field("lte", v)?;
        }
        if let Some(v) = self.gt.as_ref() {
            struct_ser.serialize_field("gt", v)?;
        }
        if let Some(v) = self.gte.as_ref() {
            struct_ser.serialize_field("gte", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("not_in", &self.not_in)?;
        }
        if let Some(v) = self.ignore_empty.as_ref() {
            struct_ser.serialize_field("ignore_empty", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FloatRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "lt",
            "lte",
            "gt",
            "gte",
            "in",
            "not_in",
            "notIn",
            "ignore_empty",
            "ignoreEmpty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Lt,
            Lte,
            Gt,
            Gte,
            In,
            NotIn,
            IgnoreEmpty,
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
                            "const" => Ok(GeneratedField::Const),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "ignoreEmpty" | "ignore_empty" => Ok(GeneratedField::IgnoreEmpty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FloatRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.FloatRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FloatRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut lt__ = None;
                let mut lte__ = None;
                let mut gt__ = None;
                let mut gte__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut ignore_empty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lt => {
                            if lt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            lt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lte => {
                            if lte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            lte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gt => {
                            if gt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            gt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gte => {
                            if gte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            gte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::IgnoreEmpty => {
                            if ignore_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreEmpty"));
                            }
                            ignore_empty__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FloatRules {
                    r#const: r#const__,
                    lt: lt__,
                    lte: lte__,
                    gt: gt__,
                    gte: gte__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    ignore_empty: ignore_empty__,
                })
            }
        }
        deserializer.deserialize_struct("validate.FloatRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Int32Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.lt.is_some() {
            len += 1;
        }
        if self.lte.is_some() {
            len += 1;
        }
        if self.gt.is_some() {
            len += 1;
        }
        if self.gte.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if self.ignore_empty.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.Int32Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if let Some(v) = self.lt.as_ref() {
            struct_ser.serialize_field("lt", v)?;
        }
        if let Some(v) = self.lte.as_ref() {
            struct_ser.serialize_field("lte", v)?;
        }
        if let Some(v) = self.gt.as_ref() {
            struct_ser.serialize_field("gt", v)?;
        }
        if let Some(v) = self.gte.as_ref() {
            struct_ser.serialize_field("gte", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("not_in", &self.not_in)?;
        }
        if let Some(v) = self.ignore_empty.as_ref() {
            struct_ser.serialize_field("ignore_empty", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Int32Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "lt",
            "lte",
            "gt",
            "gte",
            "in",
            "not_in",
            "notIn",
            "ignore_empty",
            "ignoreEmpty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Lt,
            Lte,
            Gt,
            Gte,
            In,
            NotIn,
            IgnoreEmpty,
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
                            "const" => Ok(GeneratedField::Const),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "ignoreEmpty" | "ignore_empty" => Ok(GeneratedField::IgnoreEmpty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Int32Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.Int32Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Int32Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut lt__ = None;
                let mut lte__ = None;
                let mut gt__ = None;
                let mut gte__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut ignore_empty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lt => {
                            if lt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            lt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lte => {
                            if lte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            lte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gt => {
                            if gt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            gt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gte => {
                            if gte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            gte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::IgnoreEmpty => {
                            if ignore_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreEmpty"));
                            }
                            ignore_empty__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Int32Rules {
                    r#const: r#const__,
                    lt: lt__,
                    lte: lte__,
                    gt: gt__,
                    gte: gte__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    ignore_empty: ignore_empty__,
                })
            }
        }
        deserializer.deserialize_struct("validate.Int32Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Int64Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.lt.is_some() {
            len += 1;
        }
        if self.lte.is_some() {
            len += 1;
        }
        if self.gt.is_some() {
            len += 1;
        }
        if self.gte.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if self.ignore_empty.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.Int64Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("const", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.lt.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.lte.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lte", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.gt.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("gt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.gte.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("gte", ToString::to_string(&v).as_str())?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("not_in", &self.not_in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.ignore_empty.as_ref() {
            struct_ser.serialize_field("ignore_empty", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Int64Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "lt",
            "lte",
            "gt",
            "gte",
            "in",
            "not_in",
            "notIn",
            "ignore_empty",
            "ignoreEmpty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Lt,
            Lte,
            Gt,
            Gte,
            In,
            NotIn,
            IgnoreEmpty,
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
                            "const" => Ok(GeneratedField::Const),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "ignoreEmpty" | "ignore_empty" => Ok(GeneratedField::IgnoreEmpty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Int64Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.Int64Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Int64Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut lt__ = None;
                let mut lte__ = None;
                let mut gt__ = None;
                let mut gte__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut ignore_empty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lt => {
                            if lt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            lt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lte => {
                            if lte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            lte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gt => {
                            if gt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            gt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gte => {
                            if gte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            gte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::IgnoreEmpty => {
                            if ignore_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreEmpty"));
                            }
                            ignore_empty__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Int64Rules {
                    r#const: r#const__,
                    lt: lt__,
                    lte: lte__,
                    gt: gt__,
                    gte: gte__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    ignore_empty: ignore_empty__,
                })
            }
        }
        deserializer.deserialize_struct("validate.Int64Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KnownRegex {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::HttpHeaderName => "HTTP_HEADER_NAME",
            Self::HttpHeaderValue => "HTTP_HEADER_VALUE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for KnownRegex {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "HTTP_HEADER_NAME",
            "HTTP_HEADER_VALUE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KnownRegex;

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
                    "UNKNOWN" => Ok(KnownRegex::Unknown),
                    "HTTP_HEADER_NAME" => Ok(KnownRegex::HttpHeaderName),
                    "HTTP_HEADER_VALUE" => Ok(KnownRegex::HttpHeaderValue),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MapRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min_pairs.is_some() {
            len += 1;
        }
        if self.max_pairs.is_some() {
            len += 1;
        }
        if self.no_sparse.is_some() {
            len += 1;
        }
        if self.keys.is_some() {
            len += 1;
        }
        if self.values.is_some() {
            len += 1;
        }
        if self.ignore_empty.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.MapRules", len)?;
        if let Some(v) = self.min_pairs.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("min_pairs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.max_pairs.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("max_pairs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.no_sparse.as_ref() {
            struct_ser.serialize_field("no_sparse", v)?;
        }
        if let Some(v) = self.keys.as_ref() {
            struct_ser.serialize_field("keys", v)?;
        }
        if let Some(v) = self.values.as_ref() {
            struct_ser.serialize_field("values", v)?;
        }
        if let Some(v) = self.ignore_empty.as_ref() {
            struct_ser.serialize_field("ignore_empty", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MapRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min_pairs",
            "minPairs",
            "max_pairs",
            "maxPairs",
            "no_sparse",
            "noSparse",
            "keys",
            "values",
            "ignore_empty",
            "ignoreEmpty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinPairs,
            MaxPairs,
            NoSparse,
            Keys,
            Values,
            IgnoreEmpty,
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
                            "minPairs" | "min_pairs" => Ok(GeneratedField::MinPairs),
                            "maxPairs" | "max_pairs" => Ok(GeneratedField::MaxPairs),
                            "noSparse" | "no_sparse" => Ok(GeneratedField::NoSparse),
                            "keys" => Ok(GeneratedField::Keys),
                            "values" => Ok(GeneratedField::Values),
                            "ignoreEmpty" | "ignore_empty" => Ok(GeneratedField::IgnoreEmpty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MapRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.MapRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MapRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min_pairs__ = None;
                let mut max_pairs__ = None;
                let mut no_sparse__ = None;
                let mut keys__ = None;
                let mut values__ = None;
                let mut ignore_empty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinPairs => {
                            if min_pairs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minPairs"));
                            }
                            min_pairs__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxPairs => {
                            if max_pairs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPairs"));
                            }
                            max_pairs__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::NoSparse => {
                            if no_sparse__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noSparse"));
                            }
                            no_sparse__ = map_.next_value()?;
                        }
                        GeneratedField::Keys => {
                            if keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keys"));
                            }
                            keys__ = map_.next_value()?;
                        }
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = map_.next_value()?;
                        }
                        GeneratedField::IgnoreEmpty => {
                            if ignore_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreEmpty"));
                            }
                            ignore_empty__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MapRules {
                    min_pairs: min_pairs__,
                    max_pairs: max_pairs__,
                    no_sparse: no_sparse__,
                    keys: keys__,
                    values: values__,
                    ignore_empty: ignore_empty__,
                })
            }
        }
        deserializer.deserialize_struct("validate.MapRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MessageRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.skip.is_some() {
            len += 1;
        }
        if self.required.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.MessageRules", len)?;
        if let Some(v) = self.skip.as_ref() {
            struct_ser.serialize_field("skip", v)?;
        }
        if let Some(v) = self.required.as_ref() {
            struct_ser.serialize_field("required", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MessageRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "skip",
            "required",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Skip,
            Required,
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
                            "skip" => Ok(GeneratedField::Skip),
                            "required" => Ok(GeneratedField::Required),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MessageRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.MessageRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MessageRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut skip__ = None;
                let mut required__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Skip => {
                            if skip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skip"));
                            }
                            skip__ = map_.next_value()?;
                        }
                        GeneratedField::Required => {
                            if required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("required"));
                            }
                            required__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MessageRules {
                    skip: skip__,
                    required: required__,
                })
            }
        }
        deserializer.deserialize_struct("validate.MessageRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RepeatedRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min_items.is_some() {
            len += 1;
        }
        if self.max_items.is_some() {
            len += 1;
        }
        if self.unique.is_some() {
            len += 1;
        }
        if self.items.is_some() {
            len += 1;
        }
        if self.ignore_empty.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.RepeatedRules", len)?;
        if let Some(v) = self.min_items.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("min_items", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.max_items.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("max_items", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.unique.as_ref() {
            struct_ser.serialize_field("unique", v)?;
        }
        if let Some(v) = self.items.as_ref() {
            struct_ser.serialize_field("items", v)?;
        }
        if let Some(v) = self.ignore_empty.as_ref() {
            struct_ser.serialize_field("ignore_empty", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RepeatedRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min_items",
            "minItems",
            "max_items",
            "maxItems",
            "unique",
            "items",
            "ignore_empty",
            "ignoreEmpty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinItems,
            MaxItems,
            Unique,
            Items,
            IgnoreEmpty,
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
                            "minItems" | "min_items" => Ok(GeneratedField::MinItems),
                            "maxItems" | "max_items" => Ok(GeneratedField::MaxItems),
                            "unique" => Ok(GeneratedField::Unique),
                            "items" => Ok(GeneratedField::Items),
                            "ignoreEmpty" | "ignore_empty" => Ok(GeneratedField::IgnoreEmpty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RepeatedRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.RepeatedRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RepeatedRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min_items__ = None;
                let mut max_items__ = None;
                let mut unique__ = None;
                let mut items__ = None;
                let mut ignore_empty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinItems => {
                            if min_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minItems"));
                            }
                            min_items__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxItems => {
                            if max_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxItems"));
                            }
                            max_items__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Unique => {
                            if unique__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unique"));
                            }
                            unique__ = map_.next_value()?;
                        }
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = map_.next_value()?;
                        }
                        GeneratedField::IgnoreEmpty => {
                            if ignore_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreEmpty"));
                            }
                            ignore_empty__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RepeatedRules {
                    min_items: min_items__,
                    max_items: max_items__,
                    unique: unique__,
                    items: items__,
                    ignore_empty: ignore_empty__,
                })
            }
        }
        deserializer.deserialize_struct("validate.RepeatedRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SFixed32Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.lt.is_some() {
            len += 1;
        }
        if self.lte.is_some() {
            len += 1;
        }
        if self.gt.is_some() {
            len += 1;
        }
        if self.gte.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if self.ignore_empty.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.SFixed32Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if let Some(v) = self.lt.as_ref() {
            struct_ser.serialize_field("lt", v)?;
        }
        if let Some(v) = self.lte.as_ref() {
            struct_ser.serialize_field("lte", v)?;
        }
        if let Some(v) = self.gt.as_ref() {
            struct_ser.serialize_field("gt", v)?;
        }
        if let Some(v) = self.gte.as_ref() {
            struct_ser.serialize_field("gte", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("not_in", &self.not_in)?;
        }
        if let Some(v) = self.ignore_empty.as_ref() {
            struct_ser.serialize_field("ignore_empty", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SFixed32Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "lt",
            "lte",
            "gt",
            "gte",
            "in",
            "not_in",
            "notIn",
            "ignore_empty",
            "ignoreEmpty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Lt,
            Lte,
            Gt,
            Gte,
            In,
            NotIn,
            IgnoreEmpty,
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
                            "const" => Ok(GeneratedField::Const),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "ignoreEmpty" | "ignore_empty" => Ok(GeneratedField::IgnoreEmpty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SFixed32Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.SFixed32Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SFixed32Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut lt__ = None;
                let mut lte__ = None;
                let mut gt__ = None;
                let mut gte__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut ignore_empty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lt => {
                            if lt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            lt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lte => {
                            if lte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            lte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gt => {
                            if gt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            gt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gte => {
                            if gte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            gte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::IgnoreEmpty => {
                            if ignore_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreEmpty"));
                            }
                            ignore_empty__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SFixed32Rules {
                    r#const: r#const__,
                    lt: lt__,
                    lte: lte__,
                    gt: gt__,
                    gte: gte__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    ignore_empty: ignore_empty__,
                })
            }
        }
        deserializer.deserialize_struct("validate.SFixed32Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SFixed64Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.lt.is_some() {
            len += 1;
        }
        if self.lte.is_some() {
            len += 1;
        }
        if self.gt.is_some() {
            len += 1;
        }
        if self.gte.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if self.ignore_empty.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.SFixed64Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("const", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.lt.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.lte.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lte", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.gt.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("gt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.gte.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("gte", ToString::to_string(&v).as_str())?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("not_in", &self.not_in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.ignore_empty.as_ref() {
            struct_ser.serialize_field("ignore_empty", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SFixed64Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "lt",
            "lte",
            "gt",
            "gte",
            "in",
            "not_in",
            "notIn",
            "ignore_empty",
            "ignoreEmpty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Lt,
            Lte,
            Gt,
            Gte,
            In,
            NotIn,
            IgnoreEmpty,
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
                            "const" => Ok(GeneratedField::Const),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "ignoreEmpty" | "ignore_empty" => Ok(GeneratedField::IgnoreEmpty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SFixed64Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.SFixed64Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SFixed64Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut lt__ = None;
                let mut lte__ = None;
                let mut gt__ = None;
                let mut gte__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut ignore_empty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lt => {
                            if lt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            lt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lte => {
                            if lte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            lte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gt => {
                            if gt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            gt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gte => {
                            if gte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            gte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::IgnoreEmpty => {
                            if ignore_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreEmpty"));
                            }
                            ignore_empty__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SFixed64Rules {
                    r#const: r#const__,
                    lt: lt__,
                    lte: lte__,
                    gt: gt__,
                    gte: gte__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    ignore_empty: ignore_empty__,
                })
            }
        }
        deserializer.deserialize_struct("validate.SFixed64Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SInt32Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.lt.is_some() {
            len += 1;
        }
        if self.lte.is_some() {
            len += 1;
        }
        if self.gt.is_some() {
            len += 1;
        }
        if self.gte.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if self.ignore_empty.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.SInt32Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if let Some(v) = self.lt.as_ref() {
            struct_ser.serialize_field("lt", v)?;
        }
        if let Some(v) = self.lte.as_ref() {
            struct_ser.serialize_field("lte", v)?;
        }
        if let Some(v) = self.gt.as_ref() {
            struct_ser.serialize_field("gt", v)?;
        }
        if let Some(v) = self.gte.as_ref() {
            struct_ser.serialize_field("gte", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("not_in", &self.not_in)?;
        }
        if let Some(v) = self.ignore_empty.as_ref() {
            struct_ser.serialize_field("ignore_empty", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SInt32Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "lt",
            "lte",
            "gt",
            "gte",
            "in",
            "not_in",
            "notIn",
            "ignore_empty",
            "ignoreEmpty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Lt,
            Lte,
            Gt,
            Gte,
            In,
            NotIn,
            IgnoreEmpty,
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
                            "const" => Ok(GeneratedField::Const),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "ignoreEmpty" | "ignore_empty" => Ok(GeneratedField::IgnoreEmpty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SInt32Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.SInt32Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SInt32Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut lt__ = None;
                let mut lte__ = None;
                let mut gt__ = None;
                let mut gte__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut ignore_empty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lt => {
                            if lt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            lt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lte => {
                            if lte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            lte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gt => {
                            if gt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            gt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gte => {
                            if gte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            gte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::IgnoreEmpty => {
                            if ignore_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreEmpty"));
                            }
                            ignore_empty__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SInt32Rules {
                    r#const: r#const__,
                    lt: lt__,
                    lte: lte__,
                    gt: gt__,
                    gte: gte__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    ignore_empty: ignore_empty__,
                })
            }
        }
        deserializer.deserialize_struct("validate.SInt32Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SInt64Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.lt.is_some() {
            len += 1;
        }
        if self.lte.is_some() {
            len += 1;
        }
        if self.gt.is_some() {
            len += 1;
        }
        if self.gte.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if self.ignore_empty.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.SInt64Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("const", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.lt.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.lte.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lte", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.gt.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("gt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.gte.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("gte", ToString::to_string(&v).as_str())?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("not_in", &self.not_in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.ignore_empty.as_ref() {
            struct_ser.serialize_field("ignore_empty", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SInt64Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "lt",
            "lte",
            "gt",
            "gte",
            "in",
            "not_in",
            "notIn",
            "ignore_empty",
            "ignoreEmpty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Lt,
            Lte,
            Gt,
            Gte,
            In,
            NotIn,
            IgnoreEmpty,
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
                            "const" => Ok(GeneratedField::Const),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "ignoreEmpty" | "ignore_empty" => Ok(GeneratedField::IgnoreEmpty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SInt64Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.SInt64Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SInt64Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut lt__ = None;
                let mut lte__ = None;
                let mut gt__ = None;
                let mut gte__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut ignore_empty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lt => {
                            if lt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            lt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lte => {
                            if lte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            lte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gt => {
                            if gt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            gt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gte => {
                            if gte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            gte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::IgnoreEmpty => {
                            if ignore_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreEmpty"));
                            }
                            ignore_empty__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SInt64Rules {
                    r#const: r#const__,
                    lt: lt__,
                    lte: lte__,
                    gt: gt__,
                    gte: gte__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    ignore_empty: ignore_empty__,
                })
            }
        }
        deserializer.deserialize_struct("validate.SInt64Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StringRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.len.is_some() {
            len += 1;
        }
        if self.min_len.is_some() {
            len += 1;
        }
        if self.max_len.is_some() {
            len += 1;
        }
        if self.len_bytes.is_some() {
            len += 1;
        }
        if self.min_bytes.is_some() {
            len += 1;
        }
        if self.max_bytes.is_some() {
            len += 1;
        }
        if self.pattern.is_some() {
            len += 1;
        }
        if self.prefix.is_some() {
            len += 1;
        }
        if self.suffix.is_some() {
            len += 1;
        }
        if self.contains.is_some() {
            len += 1;
        }
        if self.not_contains.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if self.strict.is_some() {
            len += 1;
        }
        if self.ignore_empty.is_some() {
            len += 1;
        }
        if self.well_known.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.StringRules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if let Some(v) = self.len.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("len", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.min_len.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("min_len", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.max_len.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("max_len", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.len_bytes.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("len_bytes", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.min_bytes.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("min_bytes", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.max_bytes.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("max_bytes", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.pattern.as_ref() {
            struct_ser.serialize_field("pattern", v)?;
        }
        if let Some(v) = self.prefix.as_ref() {
            struct_ser.serialize_field("prefix", v)?;
        }
        if let Some(v) = self.suffix.as_ref() {
            struct_ser.serialize_field("suffix", v)?;
        }
        if let Some(v) = self.contains.as_ref() {
            struct_ser.serialize_field("contains", v)?;
        }
        if let Some(v) = self.not_contains.as_ref() {
            struct_ser.serialize_field("not_contains", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("not_in", &self.not_in)?;
        }
        if let Some(v) = self.strict.as_ref() {
            struct_ser.serialize_field("strict", v)?;
        }
        if let Some(v) = self.ignore_empty.as_ref() {
            struct_ser.serialize_field("ignore_empty", v)?;
        }
        if let Some(v) = self.well_known.as_ref() {
            match v {
                string_rules::WellKnown::Email(v) => {
                    struct_ser.serialize_field("email", v)?;
                }
                string_rules::WellKnown::Hostname(v) => {
                    struct_ser.serialize_field("hostname", v)?;
                }
                string_rules::WellKnown::Ip(v) => {
                    struct_ser.serialize_field("ip", v)?;
                }
                string_rules::WellKnown::Ipv4(v) => {
                    struct_ser.serialize_field("ipv4", v)?;
                }
                string_rules::WellKnown::Ipv6(v) => {
                    struct_ser.serialize_field("ipv6", v)?;
                }
                string_rules::WellKnown::Uri(v) => {
                    struct_ser.serialize_field("uri", v)?;
                }
                string_rules::WellKnown::UriRef(v) => {
                    struct_ser.serialize_field("uri_ref", v)?;
                }
                string_rules::WellKnown::Address(v) => {
                    struct_ser.serialize_field("address", v)?;
                }
                string_rules::WellKnown::Uuid(v) => {
                    struct_ser.serialize_field("uuid", v)?;
                }
                string_rules::WellKnown::WellKnownRegex(v) => {
                    let v = KnownRegex::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("well_known_regex", &v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StringRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "len",
            "min_len",
            "minLen",
            "max_len",
            "maxLen",
            "len_bytes",
            "lenBytes",
            "min_bytes",
            "minBytes",
            "max_bytes",
            "maxBytes",
            "pattern",
            "prefix",
            "suffix",
            "contains",
            "not_contains",
            "notContains",
            "in",
            "not_in",
            "notIn",
            "strict",
            "ignore_empty",
            "ignoreEmpty",
            "email",
            "hostname",
            "ip",
            "ipv4",
            "ipv6",
            "uri",
            "uri_ref",
            "uriRef",
            "address",
            "uuid",
            "well_known_regex",
            "wellKnownRegex",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Len,
            MinLen,
            MaxLen,
            LenBytes,
            MinBytes,
            MaxBytes,
            Pattern,
            Prefix,
            Suffix,
            Contains,
            NotContains,
            In,
            NotIn,
            Strict,
            IgnoreEmpty,
            Email,
            Hostname,
            Ip,
            Ipv4,
            Ipv6,
            Uri,
            UriRef,
            Address,
            Uuid,
            WellKnownRegex,
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
                            "const" => Ok(GeneratedField::Const),
                            "len" => Ok(GeneratedField::Len),
                            "minLen" | "min_len" => Ok(GeneratedField::MinLen),
                            "maxLen" | "max_len" => Ok(GeneratedField::MaxLen),
                            "lenBytes" | "len_bytes" => Ok(GeneratedField::LenBytes),
                            "minBytes" | "min_bytes" => Ok(GeneratedField::MinBytes),
                            "maxBytes" | "max_bytes" => Ok(GeneratedField::MaxBytes),
                            "pattern" => Ok(GeneratedField::Pattern),
                            "prefix" => Ok(GeneratedField::Prefix),
                            "suffix" => Ok(GeneratedField::Suffix),
                            "contains" => Ok(GeneratedField::Contains),
                            "notContains" | "not_contains" => Ok(GeneratedField::NotContains),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "strict" => Ok(GeneratedField::Strict),
                            "ignoreEmpty" | "ignore_empty" => Ok(GeneratedField::IgnoreEmpty),
                            "email" => Ok(GeneratedField::Email),
                            "hostname" => Ok(GeneratedField::Hostname),
                            "ip" => Ok(GeneratedField::Ip),
                            "ipv4" => Ok(GeneratedField::Ipv4),
                            "ipv6" => Ok(GeneratedField::Ipv6),
                            "uri" => Ok(GeneratedField::Uri),
                            "uriRef" | "uri_ref" => Ok(GeneratedField::UriRef),
                            "address" => Ok(GeneratedField::Address),
                            "uuid" => Ok(GeneratedField::Uuid),
                            "wellKnownRegex" | "well_known_regex" => Ok(GeneratedField::WellKnownRegex),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StringRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.StringRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StringRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut len__ = None;
                let mut min_len__ = None;
                let mut max_len__ = None;
                let mut len_bytes__ = None;
                let mut min_bytes__ = None;
                let mut max_bytes__ = None;
                let mut pattern__ = None;
                let mut prefix__ = None;
                let mut suffix__ = None;
                let mut contains__ = None;
                let mut not_contains__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut strict__ = None;
                let mut ignore_empty__ = None;
                let mut well_known__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = map_.next_value()?;
                        }
                        GeneratedField::Len => {
                            if len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("len"));
                            }
                            len__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MinLen => {
                            if min_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minLen"));
                            }
                            min_len__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxLen => {
                            if max_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLen"));
                            }
                            max_len__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::LenBytes => {
                            if len_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lenBytes"));
                            }
                            len_bytes__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MinBytes => {
                            if min_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minBytes"));
                            }
                            min_bytes__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxBytes => {
                            if max_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxBytes"));
                            }
                            max_bytes__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Pattern => {
                            if pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            pattern__ = map_.next_value()?;
                        }
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = map_.next_value()?;
                        }
                        GeneratedField::Suffix => {
                            if suffix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suffix"));
                            }
                            suffix__ = map_.next_value()?;
                        }
                        GeneratedField::Contains => {
                            if contains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contains"));
                            }
                            contains__ = map_.next_value()?;
                        }
                        GeneratedField::NotContains => {
                            if not_contains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notContains"));
                            }
                            not_contains__ = map_.next_value()?;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Strict => {
                            if strict__.is_some() {
                                return Err(serde::de::Error::duplicate_field("strict"));
                            }
                            strict__ = map_.next_value()?;
                        }
                        GeneratedField::IgnoreEmpty => {
                            if ignore_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreEmpty"));
                            }
                            ignore_empty__ = map_.next_value()?;
                        }
                        GeneratedField::Email => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Email);
                        }
                        GeneratedField::Hostname => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostname"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Hostname);
                        }
                        GeneratedField::Ip => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Ip);
                        }
                        GeneratedField::Ipv4 => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv4"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Ipv4);
                        }
                        GeneratedField::Ipv6 => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv6"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Ipv6);
                        }
                        GeneratedField::Uri => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Uri);
                        }
                        GeneratedField::UriRef => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uriRef"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::UriRef);
                        }
                        GeneratedField::Address => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Address);
                        }
                        GeneratedField::Uuid => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Uuid);
                        }
                        GeneratedField::WellKnownRegex => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wellKnownRegex"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<KnownRegex>>()?.map(|x| string_rules::WellKnown::WellKnownRegex(x as i32));
                        }
                    }
                }
                Ok(StringRules {
                    r#const: r#const__,
                    len: len__,
                    min_len: min_len__,
                    max_len: max_len__,
                    len_bytes: len_bytes__,
                    min_bytes: min_bytes__,
                    max_bytes: max_bytes__,
                    pattern: pattern__,
                    prefix: prefix__,
                    suffix: suffix__,
                    contains: contains__,
                    not_contains: not_contains__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    strict: strict__,
                    ignore_empty: ignore_empty__,
                    well_known: well_known__,
                })
            }
        }
        deserializer.deserialize_struct("validate.StringRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimestampRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.required.is_some() {
            len += 1;
        }
        if self.r#const.is_some() {
            len += 1;
        }
        if self.lt.is_some() {
            len += 1;
        }
        if self.lte.is_some() {
            len += 1;
        }
        if self.gt.is_some() {
            len += 1;
        }
        if self.gte.is_some() {
            len += 1;
        }
        if self.lt_now.is_some() {
            len += 1;
        }
        if self.gt_now.is_some() {
            len += 1;
        }
        if self.within.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.TimestampRules", len)?;
        if let Some(v) = self.required.as_ref() {
            struct_ser.serialize_field("required", v)?;
        }
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if let Some(v) = self.lt.as_ref() {
            struct_ser.serialize_field("lt", v)?;
        }
        if let Some(v) = self.lte.as_ref() {
            struct_ser.serialize_field("lte", v)?;
        }
        if let Some(v) = self.gt.as_ref() {
            struct_ser.serialize_field("gt", v)?;
        }
        if let Some(v) = self.gte.as_ref() {
            struct_ser.serialize_field("gte", v)?;
        }
        if let Some(v) = self.lt_now.as_ref() {
            struct_ser.serialize_field("lt_now", v)?;
        }
        if let Some(v) = self.gt_now.as_ref() {
            struct_ser.serialize_field("gt_now", v)?;
        }
        if let Some(v) = self.within.as_ref() {
            struct_ser.serialize_field("within", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimestampRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "required",
            "const",
            "lt",
            "lte",
            "gt",
            "gte",
            "lt_now",
            "ltNow",
            "gt_now",
            "gtNow",
            "within",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Required,
            Const,
            Lt,
            Lte,
            Gt,
            Gte,
            LtNow,
            GtNow,
            Within,
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
                            "required" => Ok(GeneratedField::Required),
                            "const" => Ok(GeneratedField::Const),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            "ltNow" | "lt_now" => Ok(GeneratedField::LtNow),
                            "gtNow" | "gt_now" => Ok(GeneratedField::GtNow),
                            "within" => Ok(GeneratedField::Within),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimestampRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.TimestampRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TimestampRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut required__ = None;
                let mut r#const__ = None;
                let mut lt__ = None;
                let mut lte__ = None;
                let mut gt__ = None;
                let mut gte__ = None;
                let mut lt_now__ = None;
                let mut gt_now__ = None;
                let mut within__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Required => {
                            if required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("required"));
                            }
                            required__ = map_.next_value()?;
                        }
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = map_.next_value()?;
                        }
                        GeneratedField::Lt => {
                            if lt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            lt__ = map_.next_value()?;
                        }
                        GeneratedField::Lte => {
                            if lte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            lte__ = map_.next_value()?;
                        }
                        GeneratedField::Gt => {
                            if gt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            gt__ = map_.next_value()?;
                        }
                        GeneratedField::Gte => {
                            if gte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            gte__ = map_.next_value()?;
                        }
                        GeneratedField::LtNow => {
                            if lt_now__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ltNow"));
                            }
                            lt_now__ = map_.next_value()?;
                        }
                        GeneratedField::GtNow => {
                            if gt_now__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gtNow"));
                            }
                            gt_now__ = map_.next_value()?;
                        }
                        GeneratedField::Within => {
                            if within__.is_some() {
                                return Err(serde::de::Error::duplicate_field("within"));
                            }
                            within__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TimestampRules {
                    required: required__,
                    r#const: r#const__,
                    lt: lt__,
                    lte: lte__,
                    gt: gt__,
                    gte: gte__,
                    lt_now: lt_now__,
                    gt_now: gt_now__,
                    within: within__,
                })
            }
        }
        deserializer.deserialize_struct("validate.TimestampRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UInt32Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.lt.is_some() {
            len += 1;
        }
        if self.lte.is_some() {
            len += 1;
        }
        if self.gt.is_some() {
            len += 1;
        }
        if self.gte.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if self.ignore_empty.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.UInt32Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if let Some(v) = self.lt.as_ref() {
            struct_ser.serialize_field("lt", v)?;
        }
        if let Some(v) = self.lte.as_ref() {
            struct_ser.serialize_field("lte", v)?;
        }
        if let Some(v) = self.gt.as_ref() {
            struct_ser.serialize_field("gt", v)?;
        }
        if let Some(v) = self.gte.as_ref() {
            struct_ser.serialize_field("gte", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("not_in", &self.not_in)?;
        }
        if let Some(v) = self.ignore_empty.as_ref() {
            struct_ser.serialize_field("ignore_empty", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UInt32Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "lt",
            "lte",
            "gt",
            "gte",
            "in",
            "not_in",
            "notIn",
            "ignore_empty",
            "ignoreEmpty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Lt,
            Lte,
            Gt,
            Gte,
            In,
            NotIn,
            IgnoreEmpty,
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
                            "const" => Ok(GeneratedField::Const),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "ignoreEmpty" | "ignore_empty" => Ok(GeneratedField::IgnoreEmpty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UInt32Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.UInt32Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UInt32Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut lt__ = None;
                let mut lte__ = None;
                let mut gt__ = None;
                let mut gte__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut ignore_empty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lt => {
                            if lt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            lt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lte => {
                            if lte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            lte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gt => {
                            if gt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            gt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gte => {
                            if gte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            gte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::IgnoreEmpty => {
                            if ignore_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreEmpty"));
                            }
                            ignore_empty__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UInt32Rules {
                    r#const: r#const__,
                    lt: lt__,
                    lte: lte__,
                    gt: gt__,
                    gte: gte__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    ignore_empty: ignore_empty__,
                })
            }
        }
        deserializer.deserialize_struct("validate.UInt32Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UInt64Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.lt.is_some() {
            len += 1;
        }
        if self.lte.is_some() {
            len += 1;
        }
        if self.gt.is_some() {
            len += 1;
        }
        if self.gte.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if self.ignore_empty.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("validate.UInt64Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("const", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.lt.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.lte.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lte", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.gt.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("gt", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.gte.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("gte", ToString::to_string(&v).as_str())?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("not_in", &self.not_in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.ignore_empty.as_ref() {
            struct_ser.serialize_field("ignore_empty", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UInt64Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "lt",
            "lte",
            "gt",
            "gte",
            "in",
            "not_in",
            "notIn",
            "ignore_empty",
            "ignoreEmpty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Lt,
            Lte,
            Gt,
            Gte,
            In,
            NotIn,
            IgnoreEmpty,
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
                            "const" => Ok(GeneratedField::Const),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "ignoreEmpty" | "ignore_empty" => Ok(GeneratedField::IgnoreEmpty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UInt64Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct validate.UInt64Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UInt64Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut lt__ = None;
                let mut lte__ = None;
                let mut gt__ = None;
                let mut gte__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut ignore_empty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lt => {
                            if lt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            lt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lte => {
                            if lte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            lte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gt => {
                            if gt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            gt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gte => {
                            if gte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            gte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::IgnoreEmpty => {
                            if ignore_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreEmpty"));
                            }
                            ignore_empty__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UInt64Rules {
                    r#const: r#const__,
                    lt: lt__,
                    lte: lte__,
                    gt: gt__,
                    gte: gte__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    ignore_empty: ignore_empty__,
                })
            }
        }
        deserializer.deserialize_struct("validate.UInt64Rules", FIELDS, GeneratedVisitor)
    }
}
