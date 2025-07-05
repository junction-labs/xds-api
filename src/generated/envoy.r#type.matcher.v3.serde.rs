impl serde::Serialize for AddressMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ranges.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.AddressMatcher", len)?;
        if !self.ranges.is_empty() {
            struct_ser.serialize_field("ranges", &self.ranges)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddressMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ranges",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ranges,
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
                            "ranges" => Ok(GeneratedField::Ranges),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddressMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.AddressMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddressMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ranges__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ranges => {
                            if ranges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ranges"));
                            }
                            ranges__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddressMatcher {
                    ranges: ranges__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.AddressMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DoubleMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.match_pattern.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.DoubleMatcher", len)?;
        if let Some(v) = self.match_pattern.as_ref() {
            match v {
                double_matcher::MatchPattern::Range(v) => {
                    struct_ser.serialize_field("range", v)?;
                }
                double_matcher::MatchPattern::Exact(v) => {
                    struct_ser.serialize_field("exact", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DoubleMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "range",
            "exact",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Range,
            Exact,
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
                            "range" => Ok(GeneratedField::Range),
                            "exact" => Ok(GeneratedField::Exact),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DoubleMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.DoubleMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DoubleMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut match_pattern__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Range => {
                            if match_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            match_pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(double_matcher::MatchPattern::Range)
;
                        }
                        GeneratedField::Exact => {
                            if match_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exact"));
                            }
                            match_pattern__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| double_matcher::MatchPattern::Exact(x.0));
                        }
                    }
                }
                Ok(DoubleMatcher {
                    match_pattern: match_pattern__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.DoubleMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilterStateMatcher {
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
        if self.matcher.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.FilterStateMatcher", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if let Some(v) = self.matcher.as_ref() {
            match v {
                filter_state_matcher::Matcher::StringMatch(v) => {
                    struct_ser.serialize_field("string_match", v)?;
                }
                filter_state_matcher::Matcher::AddressMatch(v) => {
                    struct_ser.serialize_field("address_match", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterStateMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "string_match",
            "stringMatch",
            "address_match",
            "addressMatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            StringMatch,
            AddressMatch,
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
                            "stringMatch" | "string_match" => Ok(GeneratedField::StringMatch),
                            "addressMatch" | "address_match" => Ok(GeneratedField::AddressMatch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterStateMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.FilterStateMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FilterStateMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut matcher__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StringMatch => {
                            if matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringMatch"));
                            }
                            matcher__ = map_.next_value::<::std::option::Option<_>>()?.map(filter_state_matcher::Matcher::StringMatch)
;
                        }
                        GeneratedField::AddressMatch => {
                            if matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressMatch"));
                            }
                            matcher__ = map_.next_value::<::std::option::Option<_>>()?.map(filter_state_matcher::Matcher::AddressMatch)
;
                        }
                    }
                }
                Ok(FilterStateMatcher {
                    key: key__.unwrap_or_default(),
                    matcher: matcher__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.FilterStateMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpRequestHeaderMatchInput {
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
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.HttpRequestHeaderMatchInput", len)?;
        if !self.header_name.is_empty() {
            struct_ser.serialize_field("header_name", &self.header_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpRequestHeaderMatchInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header_name",
            "headerName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = HttpRequestHeaderMatchInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.HttpRequestHeaderMatchInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpRequestHeaderMatchInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HeaderName => {
                            if header_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerName"));
                            }
                            header_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HttpRequestHeaderMatchInput {
                    header_name: header_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.HttpRequestHeaderMatchInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpRequestQueryParamMatchInput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.query_param.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.HttpRequestQueryParamMatchInput", len)?;
        if !self.query_param.is_empty() {
            struct_ser.serialize_field("query_param", &self.query_param)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpRequestQueryParamMatchInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query_param",
            "queryParam",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueryParam,
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
                            "queryParam" | "query_param" => Ok(GeneratedField::QueryParam),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpRequestQueryParamMatchInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.HttpRequestQueryParamMatchInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpRequestQueryParamMatchInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query_param__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueryParam => {
                            if query_param__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryParam"));
                            }
                            query_param__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HttpRequestQueryParamMatchInput {
                    query_param: query_param__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.HttpRequestQueryParamMatchInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpRequestTrailerMatchInput {
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
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.HttpRequestTrailerMatchInput", len)?;
        if !self.header_name.is_empty() {
            struct_ser.serialize_field("header_name", &self.header_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpRequestTrailerMatchInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header_name",
            "headerName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = HttpRequestTrailerMatchInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.HttpRequestTrailerMatchInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpRequestTrailerMatchInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HeaderName => {
                            if header_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerName"));
                            }
                            header_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HttpRequestTrailerMatchInput {
                    header_name: header_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.HttpRequestTrailerMatchInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpResponseHeaderMatchInput {
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
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.HttpResponseHeaderMatchInput", len)?;
        if !self.header_name.is_empty() {
            struct_ser.serialize_field("header_name", &self.header_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpResponseHeaderMatchInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header_name",
            "headerName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = HttpResponseHeaderMatchInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.HttpResponseHeaderMatchInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpResponseHeaderMatchInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HeaderName => {
                            if header_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerName"));
                            }
                            header_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HttpResponseHeaderMatchInput {
                    header_name: header_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.HttpResponseHeaderMatchInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpResponseStatusCodeClassMatchInput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.HttpResponseStatusCodeClassMatchInput", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpResponseStatusCodeClassMatchInput {
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
            type Value = HttpResponseStatusCodeClassMatchInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.HttpResponseStatusCodeClassMatchInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpResponseStatusCodeClassMatchInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HttpResponseStatusCodeClassMatchInput {
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.HttpResponseStatusCodeClassMatchInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpResponseStatusCodeMatchInput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.HttpResponseStatusCodeMatchInput", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpResponseStatusCodeMatchInput {
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
            type Value = HttpResponseStatusCodeMatchInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.HttpResponseStatusCodeMatchInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpResponseStatusCodeMatchInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HttpResponseStatusCodeMatchInput {
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.HttpResponseStatusCodeMatchInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpResponseTrailerMatchInput {
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
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.HttpResponseTrailerMatchInput", len)?;
        if !self.header_name.is_empty() {
            struct_ser.serialize_field("header_name", &self.header_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpResponseTrailerMatchInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header_name",
            "headerName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = HttpResponseTrailerMatchInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.HttpResponseTrailerMatchInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpResponseTrailerMatchInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HeaderName => {
                            if header_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerName"));
                            }
                            header_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HttpResponseTrailerMatchInput {
                    header_name: header_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.HttpResponseTrailerMatchInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.match_pattern.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.ListMatcher", len)?;
        if let Some(v) = self.match_pattern.as_ref() {
            match v {
                list_matcher::MatchPattern::OneOf(v) => {
                    struct_ser.serialize_field("one_of", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "one_of",
            "oneOf",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OneOf,
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
                            "oneOf" | "one_of" => Ok(GeneratedField::OneOf),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.ListMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut match_pattern__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OneOf => {
                            if match_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oneOf"));
                            }
                            match_pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(list_matcher::MatchPattern::OneOf)
;
                        }
                    }
                }
                Ok(ListMatcher {
                    match_pattern: match_pattern__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.ListMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListStringMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.patterns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.ListStringMatcher", len)?;
        if !self.patterns.is_empty() {
            struct_ser.serialize_field("patterns", &self.patterns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListStringMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "patterns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Patterns,
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
                            "patterns" => Ok(GeneratedField::Patterns),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListStringMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.ListStringMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListStringMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut patterns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Patterns => {
                            if patterns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("patterns"));
                            }
                            patterns__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListStringMatcher {
                    patterns: patterns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.ListStringMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MetadataMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.filter.is_empty() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        if self.invert {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.MetadataMatcher", len)?;
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if self.invert {
            struct_ser.serialize_field("invert", &self.invert)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MetadataMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filter",
            "path",
            "value",
            "invert",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filter,
            Path,
            Value,
            Invert,
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
                            "filter" => Ok(GeneratedField::Filter),
                            "path" => Ok(GeneratedField::Path),
                            "value" => Ok(GeneratedField::Value),
                            "invert" => Ok(GeneratedField::Invert),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MetadataMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.MetadataMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MetadataMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter__ = None;
                let mut path__ = None;
                let mut value__ = None;
                let mut invert__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::Invert => {
                            if invert__.is_some() {
                                return Err(serde::de::Error::duplicate_field("invert"));
                            }
                            invert__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MetadataMatcher {
                    filter: filter__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                    value: value__,
                    invert: invert__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.MetadataMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for metadata_matcher::PathSegment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.segment.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.MetadataMatcher.PathSegment", len)?;
        if let Some(v) = self.segment.as_ref() {
            match v {
                metadata_matcher::path_segment::Segment::Key(v) => {
                    struct_ser.serialize_field("key", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for metadata_matcher::PathSegment {
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
            type Value = metadata_matcher::PathSegment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.MetadataMatcher.PathSegment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<metadata_matcher::PathSegment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut segment__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if segment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            segment__ = map_.next_value::<::std::option::Option<_>>()?.map(metadata_matcher::path_segment::Segment::Key);
                        }
                    }
                }
                Ok(metadata_matcher::PathSegment {
                    segment: segment__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.MetadataMatcher.PathSegment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NodeMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.node_id.is_some() {
            len += 1;
        }
        if !self.node_metadatas.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.NodeMatcher", len)?;
        if let Some(v) = self.node_id.as_ref() {
            struct_ser.serialize_field("node_id", v)?;
        }
        if !self.node_metadatas.is_empty() {
            struct_ser.serialize_field("node_metadatas", &self.node_metadatas)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NodeMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "node_id",
            "nodeId",
            "node_metadatas",
            "nodeMetadatas",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NodeId,
            NodeMetadatas,
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
                            "nodeId" | "node_id" => Ok(GeneratedField::NodeId),
                            "nodeMetadatas" | "node_metadatas" => Ok(GeneratedField::NodeMetadatas),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NodeMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.NodeMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NodeMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut node_id__ = None;
                let mut node_metadatas__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NodeId => {
                            if node_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeId"));
                            }
                            node_id__ = map_.next_value()?;
                        }
                        GeneratedField::NodeMetadatas => {
                            if node_metadatas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeMetadatas"));
                            }
                            node_metadatas__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(NodeMatcher {
                    node_id: node_id__,
                    node_metadatas: node_metadatas__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.NodeMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value_matchers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.OrMatcher", len)?;
        if !self.value_matchers.is_empty() {
            struct_ser.serialize_field("value_matchers", &self.value_matchers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value_matchers",
            "valueMatchers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValueMatchers,
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
                            "valueMatchers" | "value_matchers" => Ok(GeneratedField::ValueMatchers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.OrMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value_matchers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValueMatchers => {
                            if value_matchers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMatchers"));
                            }
                            value_matchers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OrMatcher {
                    value_matchers: value_matchers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.OrMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PathMatcher {
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
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.PathMatcher", len)?;
        if let Some(v) = self.rule.as_ref() {
            match v {
                path_matcher::Rule::Path(v) => {
                    struct_ser.serialize_field("path", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PathMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
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
                            "path" => Ok(GeneratedField::Path),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PathMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.PathMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PathMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            rule__ = map_.next_value::<::std::option::Option<_>>()?.map(path_matcher::Rule::Path)
;
                        }
                    }
                }
                Ok(PathMatcher {
                    rule: rule__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.PathMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegexMatchAndSubstitute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pattern.is_some() {
            len += 1;
        }
        if !self.substitution.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.RegexMatchAndSubstitute", len)?;
        if let Some(v) = self.pattern.as_ref() {
            struct_ser.serialize_field("pattern", v)?;
        }
        if !self.substitution.is_empty() {
            struct_ser.serialize_field("substitution", &self.substitution)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegexMatchAndSubstitute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pattern",
            "substitution",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pattern,
            Substitution,
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
                            "pattern" => Ok(GeneratedField::Pattern),
                            "substitution" => Ok(GeneratedField::Substitution),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegexMatchAndSubstitute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.RegexMatchAndSubstitute")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegexMatchAndSubstitute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pattern__ = None;
                let mut substitution__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pattern => {
                            if pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            pattern__ = map_.next_value()?;
                        }
                        GeneratedField::Substitution => {
                            if substitution__.is_some() {
                                return Err(serde::de::Error::duplicate_field("substitution"));
                            }
                            substitution__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RegexMatchAndSubstitute {
                    pattern: pattern__,
                    substitution: substitution__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.RegexMatchAndSubstitute", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegexMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.regex.is_empty() {
            len += 1;
        }
        if self.engine_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.RegexMatcher", len)?;
        if !self.regex.is_empty() {
            struct_ser.serialize_field("regex", &self.regex)?;
        }
        if let Some(v) = self.engine_type.as_ref() {
            match v {
                regex_matcher::EngineType::GoogleRe2(v) => {
                    struct_ser.serialize_field("google_re2", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegexMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "regex",
            "google_re2",
            "googleRe2",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Regex,
            GoogleRe2,
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
                            "regex" => Ok(GeneratedField::Regex),
                            "googleRe2" | "google_re2" => Ok(GeneratedField::GoogleRe2),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegexMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.RegexMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegexMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut regex__ = None;
                let mut engine_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Regex => {
                            if regex__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regex"));
                            }
                            regex__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GoogleRe2 => {
                            if engine_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("googleRe2"));
                            }
                            engine_type__ = map_.next_value::<::std::option::Option<_>>()?.map(regex_matcher::EngineType::GoogleRe2)
;
                        }
                    }
                }
                Ok(RegexMatcher {
                    regex: regex__.unwrap_or_default(),
                    engine_type: engine_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.RegexMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for regex_matcher::GoogleRe2 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_program_size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.RegexMatcher.GoogleRE2", len)?;
        if let Some(v) = self.max_program_size.as_ref() {
            struct_ser.serialize_field("max_program_size", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for regex_matcher::GoogleRe2 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_program_size",
            "maxProgramSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxProgramSize,
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
                            "maxProgramSize" | "max_program_size" => Ok(GeneratedField::MaxProgramSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = regex_matcher::GoogleRe2;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.RegexMatcher.GoogleRE2")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<regex_matcher::GoogleRe2, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_program_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxProgramSize => {
                            if max_program_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxProgramSize"));
                            }
                            max_program_size__ = map_.next_value()?;
                        }
                    }
                }
                Ok(regex_matcher::GoogleRe2 {
                    max_program_size: max_program_size__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.RegexMatcher.GoogleRE2", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StringMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ignore_case {
            len += 1;
        }
        if self.match_pattern.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.StringMatcher", len)?;
        if self.ignore_case {
            struct_ser.serialize_field("ignore_case", &self.ignore_case)?;
        }
        if let Some(v) = self.match_pattern.as_ref() {
            match v {
                string_matcher::MatchPattern::Exact(v) => {
                    struct_ser.serialize_field("exact", v)?;
                }
                string_matcher::MatchPattern::Prefix(v) => {
                    struct_ser.serialize_field("prefix", v)?;
                }
                string_matcher::MatchPattern::Suffix(v) => {
                    struct_ser.serialize_field("suffix", v)?;
                }
                string_matcher::MatchPattern::SafeRegex(v) => {
                    struct_ser.serialize_field("safe_regex", v)?;
                }
                string_matcher::MatchPattern::Contains(v) => {
                    struct_ser.serialize_field("contains", v)?;
                }
                string_matcher::MatchPattern::Custom(v) => {
                    struct_ser.serialize_field("custom", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StringMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ignore_case",
            "ignoreCase",
            "exact",
            "prefix",
            "suffix",
            "safe_regex",
            "safeRegex",
            "contains",
            "custom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IgnoreCase,
            Exact,
            Prefix,
            Suffix,
            SafeRegex,
            Contains,
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
                            "ignoreCase" | "ignore_case" => Ok(GeneratedField::IgnoreCase),
                            "exact" => Ok(GeneratedField::Exact),
                            "prefix" => Ok(GeneratedField::Prefix),
                            "suffix" => Ok(GeneratedField::Suffix),
                            "safeRegex" | "safe_regex" => Ok(GeneratedField::SafeRegex),
                            "contains" => Ok(GeneratedField::Contains),
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
            type Value = StringMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.StringMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StringMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ignore_case__ = None;
                let mut match_pattern__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IgnoreCase => {
                            if ignore_case__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreCase"));
                            }
                            ignore_case__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Exact => {
                            if match_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exact"));
                            }
                            match_pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(string_matcher::MatchPattern::Exact);
                        }
                        GeneratedField::Prefix => {
                            if match_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            match_pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(string_matcher::MatchPattern::Prefix);
                        }
                        GeneratedField::Suffix => {
                            if match_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suffix"));
                            }
                            match_pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(string_matcher::MatchPattern::Suffix);
                        }
                        GeneratedField::SafeRegex => {
                            if match_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("safeRegex"));
                            }
                            match_pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(string_matcher::MatchPattern::SafeRegex)
;
                        }
                        GeneratedField::Contains => {
                            if match_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contains"));
                            }
                            match_pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(string_matcher::MatchPattern::Contains);
                        }
                        GeneratedField::Custom => {
                            if match_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("custom"));
                            }
                            match_pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(string_matcher::MatchPattern::Custom)
;
                        }
                    }
                }
                Ok(StringMatcher {
                    ignore_case: ignore_case__.unwrap_or_default(),
                    match_pattern: match_pattern__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.StringMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StructMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.StructMatcher", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StructMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
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
                            "path" => Ok(GeneratedField::Path),
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
            type Value = StructMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.StructMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StructMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StructMatcher {
                    path: path__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.StructMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for struct_matcher::PathSegment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.segment.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.StructMatcher.PathSegment", len)?;
        if let Some(v) = self.segment.as_ref() {
            match v {
                struct_matcher::path_segment::Segment::Key(v) => {
                    struct_ser.serialize_field("key", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for struct_matcher::PathSegment {
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
            type Value = struct_matcher::PathSegment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.StructMatcher.PathSegment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<struct_matcher::PathSegment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut segment__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if segment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            segment__ = map_.next_value::<::std::option::Option<_>>()?.map(struct_matcher::path_segment::Segment::Key);
                        }
                    }
                }
                Ok(struct_matcher::PathSegment {
                    segment: segment__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.StructMatcher.PathSegment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValueMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.match_pattern.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.ValueMatcher", len)?;
        if let Some(v) = self.match_pattern.as_ref() {
            match v {
                value_matcher::MatchPattern::NullMatch(v) => {
                    struct_ser.serialize_field("null_match", v)?;
                }
                value_matcher::MatchPattern::DoubleMatch(v) => {
                    struct_ser.serialize_field("double_match", v)?;
                }
                value_matcher::MatchPattern::StringMatch(v) => {
                    struct_ser.serialize_field("string_match", v)?;
                }
                value_matcher::MatchPattern::BoolMatch(v) => {
                    struct_ser.serialize_field("bool_match", v)?;
                }
                value_matcher::MatchPattern::PresentMatch(v) => {
                    struct_ser.serialize_field("present_match", v)?;
                }
                value_matcher::MatchPattern::ListMatch(v) => {
                    struct_ser.serialize_field("list_match", v)?;
                }
                value_matcher::MatchPattern::OrMatch(v) => {
                    struct_ser.serialize_field("or_match", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValueMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "null_match",
            "nullMatch",
            "double_match",
            "doubleMatch",
            "string_match",
            "stringMatch",
            "bool_match",
            "boolMatch",
            "present_match",
            "presentMatch",
            "list_match",
            "listMatch",
            "or_match",
            "orMatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NullMatch,
            DoubleMatch,
            StringMatch,
            BoolMatch,
            PresentMatch,
            ListMatch,
            OrMatch,
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
                            "nullMatch" | "null_match" => Ok(GeneratedField::NullMatch),
                            "doubleMatch" | "double_match" => Ok(GeneratedField::DoubleMatch),
                            "stringMatch" | "string_match" => Ok(GeneratedField::StringMatch),
                            "boolMatch" | "bool_match" => Ok(GeneratedField::BoolMatch),
                            "presentMatch" | "present_match" => Ok(GeneratedField::PresentMatch),
                            "listMatch" | "list_match" => Ok(GeneratedField::ListMatch),
                            "orMatch" | "or_match" => Ok(GeneratedField::OrMatch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValueMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.ValueMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValueMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut match_pattern__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NullMatch => {
                            if match_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nullMatch"));
                            }
                            match_pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(value_matcher::MatchPattern::NullMatch)
;
                        }
                        GeneratedField::DoubleMatch => {
                            if match_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doubleMatch"));
                            }
                            match_pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(value_matcher::MatchPattern::DoubleMatch)
;
                        }
                        GeneratedField::StringMatch => {
                            if match_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringMatch"));
                            }
                            match_pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(value_matcher::MatchPattern::StringMatch)
;
                        }
                        GeneratedField::BoolMatch => {
                            if match_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolMatch"));
                            }
                            match_pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(value_matcher::MatchPattern::BoolMatch);
                        }
                        GeneratedField::PresentMatch => {
                            if match_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presentMatch"));
                            }
                            match_pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(value_matcher::MatchPattern::PresentMatch);
                        }
                        GeneratedField::ListMatch => {
                            if match_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listMatch"));
                            }
                            match_pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(value_matcher::MatchPattern::ListMatch)
;
                        }
                        GeneratedField::OrMatch => {
                            if match_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orMatch"));
                            }
                            match_pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(value_matcher::MatchPattern::OrMatch)
;
                        }
                    }
                }
                Ok(ValueMatcher {
                    match_pattern: match_pattern__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.ValueMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for value_matcher::NullMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.r#type.matcher.v3.ValueMatcher.NullMatch", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for value_matcher::NullMatch {
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
            type Value = value_matcher::NullMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.matcher.v3.ValueMatcher.NullMatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<value_matcher::NullMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(value_matcher::NullMatch {
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.matcher.v3.ValueMatcher.NullMatch", FIELDS, GeneratedVisitor)
    }
}
