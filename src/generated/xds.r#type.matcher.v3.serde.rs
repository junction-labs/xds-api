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
        let mut struct_ser = serializer.serialize_struct("xds.r#type.matcher.v3.ListStringMatcher", len)?;
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
                formatter.write_str("struct xds.r#type.matcher.v3.ListStringMatcher")
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
        deserializer.deserialize_struct("xds.r#type.matcher.v3.ListStringMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Matcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.on_no_match.is_some() {
            len += 1;
        }
        if self.matcher_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.r#type.matcher.v3.Matcher", len)?;
        if let Some(v) = self.on_no_match.as_ref() {
            struct_ser.serialize_field("on_no_match", v)?;
        }
        if let Some(v) = self.matcher_type.as_ref() {
            match v {
                matcher::MatcherType::MatcherList(v) => {
                    struct_ser.serialize_field("matcher_list", v)?;
                }
                matcher::MatcherType::MatcherTree(v) => {
                    struct_ser.serialize_field("matcher_tree", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Matcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "on_no_match",
            "onNoMatch",
            "matcher_list",
            "matcherList",
            "matcher_tree",
            "matcherTree",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OnNoMatch,
            MatcherList,
            MatcherTree,
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
                            "onNoMatch" | "on_no_match" => Ok(GeneratedField::OnNoMatch),
                            "matcherList" | "matcher_list" => Ok(GeneratedField::MatcherList),
                            "matcherTree" | "matcher_tree" => Ok(GeneratedField::MatcherTree),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Matcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.r#type.matcher.v3.Matcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Matcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut on_no_match__ = None;
                let mut matcher_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OnNoMatch => {
                            if on_no_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onNoMatch"));
                            }
                            on_no_match__ = map_.next_value()?;
                        }
                        GeneratedField::MatcherList => {
                            if matcher_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matcherList"));
                            }
                            matcher_type__ = map_.next_value::<::std::option::Option<_>>()?.map(matcher::MatcherType::MatcherList)
;
                        }
                        GeneratedField::MatcherTree => {
                            if matcher_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matcherTree"));
                            }
                            matcher_type__ = map_.next_value::<::std::option::Option<_>>()?.map(matcher::MatcherType::MatcherTree)
;
                        }
                    }
                }
                Ok(Matcher {
                    on_no_match: on_no_match__,
                    matcher_type: matcher_type__,
                })
            }
        }
        deserializer.deserialize_struct("xds.r#type.matcher.v3.Matcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for matcher::MatcherList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.matchers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.r#type.matcher.v3.Matcher.MatcherList", len)?;
        if !self.matchers.is_empty() {
            struct_ser.serialize_field("matchers", &self.matchers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for matcher::MatcherList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "matchers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Matchers,
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
                            "matchers" => Ok(GeneratedField::Matchers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = matcher::MatcherList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.r#type.matcher.v3.Matcher.MatcherList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<matcher::MatcherList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut matchers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Matchers => {
                            if matchers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchers"));
                            }
                            matchers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(matcher::MatcherList {
                    matchers: matchers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xds.r#type.matcher.v3.Matcher.MatcherList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for matcher::matcher_list::FieldMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.predicate.is_some() {
            len += 1;
        }
        if self.on_match.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.r#type.matcher.v3.Matcher.MatcherList.FieldMatcher", len)?;
        if let Some(v) = self.predicate.as_ref() {
            struct_ser.serialize_field("predicate", v)?;
        }
        if let Some(v) = self.on_match.as_ref() {
            struct_ser.serialize_field("on_match", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for matcher::matcher_list::FieldMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "predicate",
            "on_match",
            "onMatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Predicate,
            OnMatch,
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
                            "predicate" => Ok(GeneratedField::Predicate),
                            "onMatch" | "on_match" => Ok(GeneratedField::OnMatch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = matcher::matcher_list::FieldMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.r#type.matcher.v3.Matcher.MatcherList.FieldMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<matcher::matcher_list::FieldMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut predicate__ = None;
                let mut on_match__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Predicate => {
                            if predicate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate__ = map_.next_value()?;
                        }
                        GeneratedField::OnMatch => {
                            if on_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onMatch"));
                            }
                            on_match__ = map_.next_value()?;
                        }
                    }
                }
                Ok(matcher::matcher_list::FieldMatcher {
                    predicate: predicate__,
                    on_match: on_match__,
                })
            }
        }
        deserializer.deserialize_struct("xds.r#type.matcher.v3.Matcher.MatcherList.FieldMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for matcher::matcher_list::Predicate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.match_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.r#type.matcher.v3.Matcher.MatcherList.Predicate", len)?;
        if let Some(v) = self.match_type.as_ref() {
            match v {
                matcher::matcher_list::predicate::MatchType::SinglePredicate(v) => {
                    struct_ser.serialize_field("single_predicate", v)?;
                }
                matcher::matcher_list::predicate::MatchType::OrMatcher(v) => {
                    struct_ser.serialize_field("or_matcher", v)?;
                }
                matcher::matcher_list::predicate::MatchType::AndMatcher(v) => {
                    struct_ser.serialize_field("and_matcher", v)?;
                }
                matcher::matcher_list::predicate::MatchType::NotMatcher(v) => {
                    struct_ser.serialize_field("not_matcher", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for matcher::matcher_list::Predicate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "single_predicate",
            "singlePredicate",
            "or_matcher",
            "orMatcher",
            "and_matcher",
            "andMatcher",
            "not_matcher",
            "notMatcher",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SinglePredicate,
            OrMatcher,
            AndMatcher,
            NotMatcher,
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
                            "singlePredicate" | "single_predicate" => Ok(GeneratedField::SinglePredicate),
                            "orMatcher" | "or_matcher" => Ok(GeneratedField::OrMatcher),
                            "andMatcher" | "and_matcher" => Ok(GeneratedField::AndMatcher),
                            "notMatcher" | "not_matcher" => Ok(GeneratedField::NotMatcher),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = matcher::matcher_list::Predicate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.r#type.matcher.v3.Matcher.MatcherList.Predicate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<matcher::matcher_list::Predicate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut match_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SinglePredicate => {
                            if match_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("singlePredicate"));
                            }
                            match_type__ = map_.next_value::<::std::option::Option<_>>()?.map(matcher::matcher_list::predicate::MatchType::SinglePredicate)
;
                        }
                        GeneratedField::OrMatcher => {
                            if match_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orMatcher"));
                            }
                            match_type__ = map_.next_value::<::std::option::Option<_>>()?.map(matcher::matcher_list::predicate::MatchType::OrMatcher)
;
                        }
                        GeneratedField::AndMatcher => {
                            if match_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("andMatcher"));
                            }
                            match_type__ = map_.next_value::<::std::option::Option<_>>()?.map(matcher::matcher_list::predicate::MatchType::AndMatcher)
;
                        }
                        GeneratedField::NotMatcher => {
                            if match_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notMatcher"));
                            }
                            match_type__ = map_.next_value::<::std::option::Option<_>>()?.map(matcher::matcher_list::predicate::MatchType::NotMatcher)
;
                        }
                    }
                }
                Ok(matcher::matcher_list::Predicate {
                    match_type: match_type__,
                })
            }
        }
        deserializer.deserialize_struct("xds.r#type.matcher.v3.Matcher.MatcherList.Predicate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for matcher::matcher_list::predicate::PredicateList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.predicate.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.r#type.matcher.v3.Matcher.MatcherList.Predicate.PredicateList", len)?;
        if !self.predicate.is_empty() {
            struct_ser.serialize_field("predicate", &self.predicate)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for matcher::matcher_list::predicate::PredicateList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "predicate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Predicate,
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
                            "predicate" => Ok(GeneratedField::Predicate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = matcher::matcher_list::predicate::PredicateList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.r#type.matcher.v3.Matcher.MatcherList.Predicate.PredicateList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<matcher::matcher_list::predicate::PredicateList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut predicate__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Predicate => {
                            if predicate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(matcher::matcher_list::predicate::PredicateList {
                    predicate: predicate__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xds.r#type.matcher.v3.Matcher.MatcherList.Predicate.PredicateList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for matcher::matcher_list::predicate::SinglePredicate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.input.is_some() {
            len += 1;
        }
        if self.matcher.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.r#type.matcher.v3.Matcher.MatcherList.Predicate.SinglePredicate", len)?;
        if let Some(v) = self.input.as_ref() {
            struct_ser.serialize_field("input", v)?;
        }
        if let Some(v) = self.matcher.as_ref() {
            match v {
                matcher::matcher_list::predicate::single_predicate::Matcher::ValueMatch(v) => {
                    struct_ser.serialize_field("value_match", v)?;
                }
                matcher::matcher_list::predicate::single_predicate::Matcher::CustomMatch(v) => {
                    struct_ser.serialize_field("custom_match", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for matcher::matcher_list::predicate::SinglePredicate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "input",
            "value_match",
            "valueMatch",
            "custom_match",
            "customMatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Input,
            ValueMatch,
            CustomMatch,
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
                            "input" => Ok(GeneratedField::Input),
                            "valueMatch" | "value_match" => Ok(GeneratedField::ValueMatch),
                            "customMatch" | "custom_match" => Ok(GeneratedField::CustomMatch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = matcher::matcher_list::predicate::SinglePredicate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.r#type.matcher.v3.Matcher.MatcherList.Predicate.SinglePredicate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<matcher::matcher_list::predicate::SinglePredicate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut input__ = None;
                let mut matcher__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Input => {
                            if input__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input"));
                            }
                            input__ = map_.next_value()?;
                        }
                        GeneratedField::ValueMatch => {
                            if matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMatch"));
                            }
                            matcher__ = map_.next_value::<::std::option::Option<_>>()?.map(matcher::matcher_list::predicate::single_predicate::Matcher::ValueMatch)
;
                        }
                        GeneratedField::CustomMatch => {
                            if matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customMatch"));
                            }
                            matcher__ = map_.next_value::<::std::option::Option<_>>()?.map(matcher::matcher_list::predicate::single_predicate::Matcher::CustomMatch)
;
                        }
                    }
                }
                Ok(matcher::matcher_list::predicate::SinglePredicate {
                    input: input__,
                    matcher: matcher__,
                })
            }
        }
        deserializer.deserialize_struct("xds.r#type.matcher.v3.Matcher.MatcherList.Predicate.SinglePredicate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for matcher::MatcherTree {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.input.is_some() {
            len += 1;
        }
        if self.tree_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.r#type.matcher.v3.Matcher.MatcherTree", len)?;
        if let Some(v) = self.input.as_ref() {
            struct_ser.serialize_field("input", v)?;
        }
        if let Some(v) = self.tree_type.as_ref() {
            match v {
                matcher::matcher_tree::TreeType::ExactMatchMap(v) => {
                    struct_ser.serialize_field("exact_match_map", v)?;
                }
                matcher::matcher_tree::TreeType::PrefixMatchMap(v) => {
                    struct_ser.serialize_field("prefix_match_map", v)?;
                }
                matcher::matcher_tree::TreeType::CustomMatch(v) => {
                    struct_ser.serialize_field("custom_match", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for matcher::MatcherTree {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "input",
            "exact_match_map",
            "exactMatchMap",
            "prefix_match_map",
            "prefixMatchMap",
            "custom_match",
            "customMatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Input,
            ExactMatchMap,
            PrefixMatchMap,
            CustomMatch,
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
                            "input" => Ok(GeneratedField::Input),
                            "exactMatchMap" | "exact_match_map" => Ok(GeneratedField::ExactMatchMap),
                            "prefixMatchMap" | "prefix_match_map" => Ok(GeneratedField::PrefixMatchMap),
                            "customMatch" | "custom_match" => Ok(GeneratedField::CustomMatch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = matcher::MatcherTree;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.r#type.matcher.v3.Matcher.MatcherTree")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<matcher::MatcherTree, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut input__ = None;
                let mut tree_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Input => {
                            if input__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input"));
                            }
                            input__ = map_.next_value()?;
                        }
                        GeneratedField::ExactMatchMap => {
                            if tree_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exactMatchMap"));
                            }
                            tree_type__ = map_.next_value::<::std::option::Option<_>>()?.map(matcher::matcher_tree::TreeType::ExactMatchMap)
;
                        }
                        GeneratedField::PrefixMatchMap => {
                            if tree_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefixMatchMap"));
                            }
                            tree_type__ = map_.next_value::<::std::option::Option<_>>()?.map(matcher::matcher_tree::TreeType::PrefixMatchMap)
;
                        }
                        GeneratedField::CustomMatch => {
                            if tree_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customMatch"));
                            }
                            tree_type__ = map_.next_value::<::std::option::Option<_>>()?.map(matcher::matcher_tree::TreeType::CustomMatch)
;
                        }
                    }
                }
                Ok(matcher::MatcherTree {
                    input: input__,
                    tree_type: tree_type__,
                })
            }
        }
        deserializer.deserialize_struct("xds.r#type.matcher.v3.Matcher.MatcherTree", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for matcher::matcher_tree::MatchMap {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.map.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.r#type.matcher.v3.Matcher.MatcherTree.MatchMap", len)?;
        if !self.map.is_empty() {
            struct_ser.serialize_field("map", &self.map)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for matcher::matcher_tree::MatchMap {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "map",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Map,
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
                            "map" => Ok(GeneratedField::Map),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = matcher::matcher_tree::MatchMap;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.r#type.matcher.v3.Matcher.MatcherTree.MatchMap")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<matcher::matcher_tree::MatchMap, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut map__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Map => {
                            if map__.is_some() {
                                return Err(serde::de::Error::duplicate_field("map"));
                            }
                            map__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(matcher::matcher_tree::MatchMap {
                    map: map__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xds.r#type.matcher.v3.Matcher.MatcherTree.MatchMap", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for matcher::OnMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.keep_matching {
            len += 1;
        }
        if self.on_match.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.r#type.matcher.v3.Matcher.OnMatch", len)?;
        if self.keep_matching {
            struct_ser.serialize_field("keep_matching", &self.keep_matching)?;
        }
        if let Some(v) = self.on_match.as_ref() {
            match v {
                matcher::on_match::OnMatch::Matcher(v) => {
                    struct_ser.serialize_field("matcher", v)?;
                }
                matcher::on_match::OnMatch::Action(v) => {
                    struct_ser.serialize_field("action", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for matcher::OnMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "keep_matching",
            "keepMatching",
            "matcher",
            "action",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeepMatching,
            Matcher,
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
                            "keepMatching" | "keep_matching" => Ok(GeneratedField::KeepMatching),
                            "matcher" => Ok(GeneratedField::Matcher),
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
            type Value = matcher::OnMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.r#type.matcher.v3.Matcher.OnMatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<matcher::OnMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut keep_matching__ = None;
                let mut on_match__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeepMatching => {
                            if keep_matching__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keepMatching"));
                            }
                            keep_matching__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Matcher => {
                            if on_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matcher"));
                            }
                            on_match__ = map_.next_value::<::std::option::Option<_>>()?.map(matcher::on_match::OnMatch::Matcher)
;
                        }
                        GeneratedField::Action => {
                            if on_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            on_match__ = map_.next_value::<::std::option::Option<_>>()?.map(matcher::on_match::OnMatch::Action)
;
                        }
                    }
                }
                Ok(matcher::OnMatch {
                    keep_matching: keep_matching__.unwrap_or_default(),
                    on_match: on_match__,
                })
            }
        }
        deserializer.deserialize_struct("xds.r#type.matcher.v3.Matcher.OnMatch", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("xds.r#type.matcher.v3.RegexMatcher", len)?;
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
                formatter.write_str("struct xds.r#type.matcher.v3.RegexMatcher")
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
        deserializer.deserialize_struct("xds.r#type.matcher.v3.RegexMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for regex_matcher::GoogleRe2 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("xds.r#type.matcher.v3.RegexMatcher.GoogleRE2", len)?;
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
            type Value = regex_matcher::GoogleRe2;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.r#type.matcher.v3.RegexMatcher.GoogleRE2")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<regex_matcher::GoogleRe2, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(regex_matcher::GoogleRe2 {
                })
            }
        }
        deserializer.deserialize_struct("xds.r#type.matcher.v3.RegexMatcher.GoogleRE2", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("xds.r#type.matcher.v3.StringMatcher", len)?;
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
                formatter.write_str("struct xds.r#type.matcher.v3.StringMatcher")
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
        deserializer.deserialize_struct("xds.r#type.matcher.v3.StringMatcher", FIELDS, GeneratedVisitor)
    }
}
