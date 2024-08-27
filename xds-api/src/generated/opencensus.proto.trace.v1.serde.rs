impl serde::Serialize for AttributeValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.AttributeValue", len)?;
        if let Some(v) = self.value.as_ref() {
            match v {
                attribute_value::Value::StringValue(v) => {
                    struct_ser.serialize_field("string_value", v)?;
                }
                attribute_value::Value::IntValue(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("int_value", ToString::to_string(&v).as_str())?;
                }
                attribute_value::Value::BoolValue(v) => {
                    struct_ser.serialize_field("bool_value", v)?;
                }
                attribute_value::Value::DoubleValue(v) => {
                    struct_ser.serialize_field("double_value", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttributeValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "string_value",
            "stringValue",
            "int_value",
            "intValue",
            "bool_value",
            "boolValue",
            "double_value",
            "doubleValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StringValue,
            IntValue,
            BoolValue,
            DoubleValue,
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
                            "stringValue" | "string_value" => Ok(GeneratedField::StringValue),
                            "intValue" | "int_value" => Ok(GeneratedField::IntValue),
                            "boolValue" | "bool_value" => Ok(GeneratedField::BoolValue),
                            "doubleValue" | "double_value" => Ok(GeneratedField::DoubleValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttributeValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.AttributeValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AttributeValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StringValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(attribute_value::Value::StringValue)
;
                        }
                        GeneratedField::IntValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| attribute_value::Value::IntValue(x.0));
                        }
                        GeneratedField::BoolValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(attribute_value::Value::BoolValue);
                        }
                        GeneratedField::DoubleValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doubleValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| attribute_value::Value::DoubleValue(x.0));
                        }
                    }
                }
                Ok(AttributeValue {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.AttributeValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConstantSampler {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.decision != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.ConstantSampler", len)?;
        if self.decision != 0 {
            let v = constant_sampler::ConstantDecision::try_from(self.decision)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.decision)))?;
            struct_ser.serialize_field("decision", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConstantSampler {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "decision",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Decision,
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
                            "decision" => Ok(GeneratedField::Decision),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConstantSampler;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.ConstantSampler")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConstantSampler, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut decision__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Decision => {
                            if decision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decision"));
                            }
                            decision__ = Some(map_.next_value::<constant_sampler::ConstantDecision>()? as i32);
                        }
                    }
                }
                Ok(ConstantSampler {
                    decision: decision__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.ConstantSampler", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for constant_sampler::ConstantDecision {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::AlwaysOff => "ALWAYS_OFF",
            Self::AlwaysOn => "ALWAYS_ON",
            Self::AlwaysParent => "ALWAYS_PARENT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for constant_sampler::ConstantDecision {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ALWAYS_OFF",
            "ALWAYS_ON",
            "ALWAYS_PARENT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = constant_sampler::ConstantDecision;

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
                    "ALWAYS_OFF" => Ok(constant_sampler::ConstantDecision::AlwaysOff),
                    "ALWAYS_ON" => Ok(constant_sampler::ConstantDecision::AlwaysOn),
                    "ALWAYS_PARENT" => Ok(constant_sampler::ConstantDecision::AlwaysParent),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Module {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.module.is_some() {
            len += 1;
        }
        if self.build_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.Module", len)?;
        if let Some(v) = self.module.as_ref() {
            struct_ser.serialize_field("module", v)?;
        }
        if let Some(v) = self.build_id.as_ref() {
            struct_ser.serialize_field("build_id", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Module {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "module",
            "build_id",
            "buildId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Module,
            BuildId,
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
                            "module" => Ok(GeneratedField::Module),
                            "buildId" | "build_id" => Ok(GeneratedField::BuildId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Module;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.Module")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Module, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut module__ = None;
                let mut build_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Module => {
                            if module__.is_some() {
                                return Err(serde::de::Error::duplicate_field("module"));
                            }
                            module__ = map_.next_value()?;
                        }
                        GeneratedField::BuildId => {
                            if build_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buildId"));
                            }
                            build_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Module {
                    module: module__,
                    build_id: build_id__,
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.Module", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProbabilitySampler {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sampling_probability != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.ProbabilitySampler", len)?;
        if self.sampling_probability != 0. {
            struct_ser.serialize_field("samplingProbability", &self.sampling_probability)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProbabilitySampler {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "samplingProbability",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SamplingProbability,
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
                            "samplingProbability" => Ok(GeneratedField::SamplingProbability),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProbabilitySampler;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.ProbabilitySampler")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProbabilitySampler, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sampling_probability__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SamplingProbability => {
                            if sampling_probability__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samplingProbability"));
                            }
                            sampling_probability__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ProbabilitySampler {
                    sampling_probability: sampling_probability__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.ProbabilitySampler", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RateLimitingSampler {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.qps != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.RateLimitingSampler", len)?;
        if self.qps != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("qps", ToString::to_string(&self.qps).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimitingSampler {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "qps",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Qps,
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
                            "qps" => Ok(GeneratedField::Qps),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimitingSampler;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.RateLimitingSampler")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RateLimitingSampler, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut qps__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Qps => {
                            if qps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("qps"));
                            }
                            qps__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RateLimitingSampler {
                    qps: qps__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.RateLimitingSampler", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Span {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.trace_id.is_empty() {
            len += 1;
        }
        if !self.span_id.is_empty() {
            len += 1;
        }
        if self.tracestate.is_some() {
            len += 1;
        }
        if !self.parent_span_id.is_empty() {
            len += 1;
        }
        if self.name.is_some() {
            len += 1;
        }
        if self.kind != 0 {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        if self.attributes.is_some() {
            len += 1;
        }
        if self.stack_trace.is_some() {
            len += 1;
        }
        if self.time_events.is_some() {
            len += 1;
        }
        if self.links.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        if self.same_process_as_parent_span.is_some() {
            len += 1;
        }
        if self.child_span_count.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.Span", len)?;
        if !self.trace_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("trace_id", pbjson::private::base64::encode(&self.trace_id).as_str())?;
        }
        if !self.span_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("span_id", pbjson::private::base64::encode(&self.span_id).as_str())?;
        }
        if let Some(v) = self.tracestate.as_ref() {
            struct_ser.serialize_field("tracestate", v)?;
        }
        if !self.parent_span_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("parent_span_id", pbjson::private::base64::encode(&self.parent_span_id).as_str())?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if self.kind != 0 {
            let v = span::SpanKind::try_from(self.kind)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("start_time", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("end_time", v)?;
        }
        if let Some(v) = self.attributes.as_ref() {
            struct_ser.serialize_field("attributes", v)?;
        }
        if let Some(v) = self.stack_trace.as_ref() {
            struct_ser.serialize_field("stack_trace", v)?;
        }
        if let Some(v) = self.time_events.as_ref() {
            struct_ser.serialize_field("time_events", v)?;
        }
        if let Some(v) = self.links.as_ref() {
            struct_ser.serialize_field("links", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if let Some(v) = self.same_process_as_parent_span.as_ref() {
            struct_ser.serialize_field("same_process_as_parent_span", v)?;
        }
        if let Some(v) = self.child_span_count.as_ref() {
            struct_ser.serialize_field("child_span_count", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Span {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trace_id",
            "traceId",
            "span_id",
            "spanId",
            "tracestate",
            "parent_span_id",
            "parentSpanId",
            "name",
            "kind",
            "start_time",
            "startTime",
            "end_time",
            "endTime",
            "attributes",
            "stack_trace",
            "stackTrace",
            "time_events",
            "timeEvents",
            "links",
            "status",
            "resource",
            "same_process_as_parent_span",
            "sameProcessAsParentSpan",
            "child_span_count",
            "childSpanCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TraceId,
            SpanId,
            Tracestate,
            ParentSpanId,
            Name,
            Kind,
            StartTime,
            EndTime,
            Attributes,
            StackTrace,
            TimeEvents,
            Links,
            Status,
            Resource,
            SameProcessAsParentSpan,
            ChildSpanCount,
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
                            "traceId" | "trace_id" => Ok(GeneratedField::TraceId),
                            "spanId" | "span_id" => Ok(GeneratedField::SpanId),
                            "tracestate" => Ok(GeneratedField::Tracestate),
                            "parentSpanId" | "parent_span_id" => Ok(GeneratedField::ParentSpanId),
                            "name" => Ok(GeneratedField::Name),
                            "kind" => Ok(GeneratedField::Kind),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            "attributes" => Ok(GeneratedField::Attributes),
                            "stackTrace" | "stack_trace" => Ok(GeneratedField::StackTrace),
                            "timeEvents" | "time_events" => Ok(GeneratedField::TimeEvents),
                            "links" => Ok(GeneratedField::Links),
                            "status" => Ok(GeneratedField::Status),
                            "resource" => Ok(GeneratedField::Resource),
                            "sameProcessAsParentSpan" | "same_process_as_parent_span" => Ok(GeneratedField::SameProcessAsParentSpan),
                            "childSpanCount" | "child_span_count" => Ok(GeneratedField::ChildSpanCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Span;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.Span")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Span, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trace_id__ = None;
                let mut span_id__ = None;
                let mut tracestate__ = None;
                let mut parent_span_id__ = None;
                let mut name__ = None;
                let mut kind__ = None;
                let mut start_time__ = None;
                let mut end_time__ = None;
                let mut attributes__ = None;
                let mut stack_trace__ = None;
                let mut time_events__ = None;
                let mut links__ = None;
                let mut status__ = None;
                let mut resource__ = None;
                let mut same_process_as_parent_span__ = None;
                let mut child_span_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TraceId => {
                            if trace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceId"));
                            }
                            trace_id__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SpanId => {
                            if span_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spanId"));
                            }
                            span_id__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Tracestate => {
                            if tracestate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracestate"));
                            }
                            tracestate__ = map_.next_value()?;
                        }
                        GeneratedField::ParentSpanId => {
                            if parent_span_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentSpanId"));
                            }
                            parent_span_id__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value::<span::SpanKind>()? as i32);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ = map_.next_value()?;
                        }
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = map_.next_value()?;
                        }
                        GeneratedField::StackTrace => {
                            if stack_trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stackTrace"));
                            }
                            stack_trace__ = map_.next_value()?;
                        }
                        GeneratedField::TimeEvents => {
                            if time_events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeEvents"));
                            }
                            time_events__ = map_.next_value()?;
                        }
                        GeneratedField::Links => {
                            if links__.is_some() {
                                return Err(serde::de::Error::duplicate_field("links"));
                            }
                            links__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map_.next_value()?;
                        }
                        GeneratedField::SameProcessAsParentSpan => {
                            if same_process_as_parent_span__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sameProcessAsParentSpan"));
                            }
                            same_process_as_parent_span__ = map_.next_value()?;
                        }
                        GeneratedField::ChildSpanCount => {
                            if child_span_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("childSpanCount"));
                            }
                            child_span_count__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Span {
                    trace_id: trace_id__.unwrap_or_default(),
                    span_id: span_id__.unwrap_or_default(),
                    tracestate: tracestate__,
                    parent_span_id: parent_span_id__.unwrap_or_default(),
                    name: name__,
                    kind: kind__.unwrap_or_default(),
                    start_time: start_time__,
                    end_time: end_time__,
                    attributes: attributes__,
                    stack_trace: stack_trace__,
                    time_events: time_events__,
                    links: links__,
                    status: status__,
                    resource: resource__,
                    same_process_as_parent_span: same_process_as_parent_span__,
                    child_span_count: child_span_count__,
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.Span", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for span::Attributes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attribute_map.is_empty() {
            len += 1;
        }
        if self.dropped_attributes_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.Span.Attributes", len)?;
        if !self.attribute_map.is_empty() {
            struct_ser.serialize_field("attribute_map", &self.attribute_map)?;
        }
        if self.dropped_attributes_count != 0 {
            struct_ser.serialize_field("dropped_attributes_count", &self.dropped_attributes_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for span::Attributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute_map",
            "attributeMap",
            "dropped_attributes_count",
            "droppedAttributesCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttributeMap,
            DroppedAttributesCount,
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
                            "attributeMap" | "attribute_map" => Ok(GeneratedField::AttributeMap),
                            "droppedAttributesCount" | "dropped_attributes_count" => Ok(GeneratedField::DroppedAttributesCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = span::Attributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.Span.Attributes")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<span::Attributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute_map__ = None;
                let mut dropped_attributes_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttributeMap => {
                            if attribute_map__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeMap"));
                            }
                            attribute_map__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::DroppedAttributesCount => {
                            if dropped_attributes_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("droppedAttributesCount"));
                            }
                            dropped_attributes_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(span::Attributes {
                    attribute_map: attribute_map__.unwrap_or_default(),
                    dropped_attributes_count: dropped_attributes_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.Span.Attributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for span::Link {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.trace_id.is_empty() {
            len += 1;
        }
        if !self.span_id.is_empty() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if self.attributes.is_some() {
            len += 1;
        }
        if self.tracestate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.Span.Link", len)?;
        if !self.trace_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("trace_id", pbjson::private::base64::encode(&self.trace_id).as_str())?;
        }
        if !self.span_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("span_id", pbjson::private::base64::encode(&self.span_id).as_str())?;
        }
        if self.r#type != 0 {
            let v = span::link::Type::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.attributes.as_ref() {
            struct_ser.serialize_field("attributes", v)?;
        }
        if let Some(v) = self.tracestate.as_ref() {
            struct_ser.serialize_field("tracestate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for span::Link {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trace_id",
            "traceId",
            "span_id",
            "spanId",
            "type",
            "attributes",
            "tracestate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TraceId,
            SpanId,
            Type,
            Attributes,
            Tracestate,
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
                            "traceId" | "trace_id" => Ok(GeneratedField::TraceId),
                            "spanId" | "span_id" => Ok(GeneratedField::SpanId),
                            "type" => Ok(GeneratedField::Type),
                            "attributes" => Ok(GeneratedField::Attributes),
                            "tracestate" => Ok(GeneratedField::Tracestate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = span::Link;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.Span.Link")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<span::Link, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trace_id__ = None;
                let mut span_id__ = None;
                let mut r#type__ = None;
                let mut attributes__ = None;
                let mut tracestate__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TraceId => {
                            if trace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceId"));
                            }
                            trace_id__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SpanId => {
                            if span_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spanId"));
                            }
                            span_id__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<span::link::Type>()? as i32);
                        }
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = map_.next_value()?;
                        }
                        GeneratedField::Tracestate => {
                            if tracestate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracestate"));
                            }
                            tracestate__ = map_.next_value()?;
                        }
                    }
                }
                Ok(span::Link {
                    trace_id: trace_id__.unwrap_or_default(),
                    span_id: span_id__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    attributes: attributes__,
                    tracestate: tracestate__,
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.Span.Link", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for span::link::Type {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TYPE_UNSPECIFIED",
            Self::ChildLinkedSpan => "CHILD_LINKED_SPAN",
            Self::ParentLinkedSpan => "PARENT_LINKED_SPAN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for span::link::Type {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TYPE_UNSPECIFIED",
            "CHILD_LINKED_SPAN",
            "PARENT_LINKED_SPAN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = span::link::Type;

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
                    "TYPE_UNSPECIFIED" => Ok(span::link::Type::Unspecified),
                    "CHILD_LINKED_SPAN" => Ok(span::link::Type::ChildLinkedSpan),
                    "PARENT_LINKED_SPAN" => Ok(span::link::Type::ParentLinkedSpan),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for span::Links {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.link.is_empty() {
            len += 1;
        }
        if self.dropped_links_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.Span.Links", len)?;
        if !self.link.is_empty() {
            struct_ser.serialize_field("link", &self.link)?;
        }
        if self.dropped_links_count != 0 {
            struct_ser.serialize_field("dropped_links_count", &self.dropped_links_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for span::Links {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "link",
            "dropped_links_count",
            "droppedLinksCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Link,
            DroppedLinksCount,
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
                            "link" => Ok(GeneratedField::Link),
                            "droppedLinksCount" | "dropped_links_count" => Ok(GeneratedField::DroppedLinksCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = span::Links;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.Span.Links")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<span::Links, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut link__ = None;
                let mut dropped_links_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Link => {
                            if link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("link"));
                            }
                            link__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DroppedLinksCount => {
                            if dropped_links_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("droppedLinksCount"));
                            }
                            dropped_links_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(span::Links {
                    link: link__.unwrap_or_default(),
                    dropped_links_count: dropped_links_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.Span.Links", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for span::SpanKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SPAN_KIND_UNSPECIFIED",
            Self::Server => "SERVER",
            Self::Client => "CLIENT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for span::SpanKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SPAN_KIND_UNSPECIFIED",
            "SERVER",
            "CLIENT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = span::SpanKind;

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
                    "SPAN_KIND_UNSPECIFIED" => Ok(span::SpanKind::Unspecified),
                    "SERVER" => Ok(span::SpanKind::Server),
                    "CLIENT" => Ok(span::SpanKind::Client),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for span::TimeEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.time.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.Span.TimeEvent", len)?;
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                span::time_event::Value::Annotation(v) => {
                    struct_ser.serialize_field("annotation", v)?;
                }
                span::time_event::Value::MessageEvent(v) => {
                    struct_ser.serialize_field("message_event", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for span::TimeEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "time",
            "annotation",
            "message_event",
            "messageEvent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Time,
            Annotation,
            MessageEvent,
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
                            "time" => Ok(GeneratedField::Time),
                            "annotation" => Ok(GeneratedField::Annotation),
                            "messageEvent" | "message_event" => Ok(GeneratedField::MessageEvent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = span::TimeEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.Span.TimeEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<span::TimeEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut time__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                        GeneratedField::Annotation => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotation"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(span::time_event::Value::Annotation)
;
                        }
                        GeneratedField::MessageEvent => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageEvent"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(span::time_event::Value::MessageEvent)
;
                        }
                    }
                }
                Ok(span::TimeEvent {
                    time: time__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.Span.TimeEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for span::time_event::Annotation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.description.is_some() {
            len += 1;
        }
        if self.attributes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.Span.TimeEvent.Annotation", len)?;
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.attributes.as_ref() {
            struct_ser.serialize_field("attributes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for span::time_event::Annotation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
            "attributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
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
                            "description" => Ok(GeneratedField::Description),
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
            type Value = span::time_event::Annotation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.Span.TimeEvent.Annotation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<span::time_event::Annotation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                let mut attributes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = map_.next_value()?;
                        }
                    }
                }
                Ok(span::time_event::Annotation {
                    description: description__,
                    attributes: attributes__,
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.Span.TimeEvent.Annotation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for span::time_event::MessageEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        if self.id != 0 {
            len += 1;
        }
        if self.uncompressed_size != 0 {
            len += 1;
        }
        if self.compressed_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.Span.TimeEvent.MessageEvent", len)?;
        if self.r#type != 0 {
            let v = span::time_event::message_event::Type::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if self.uncompressed_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("uncompressed_size", ToString::to_string(&self.uncompressed_size).as_str())?;
        }
        if self.compressed_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("compressed_size", ToString::to_string(&self.compressed_size).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for span::time_event::MessageEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "id",
            "uncompressed_size",
            "uncompressedSize",
            "compressed_size",
            "compressedSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Id,
            UncompressedSize,
            CompressedSize,
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
                            "type" => Ok(GeneratedField::Type),
                            "id" => Ok(GeneratedField::Id),
                            "uncompressedSize" | "uncompressed_size" => Ok(GeneratedField::UncompressedSize),
                            "compressedSize" | "compressed_size" => Ok(GeneratedField::CompressedSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = span::time_event::MessageEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.Span.TimeEvent.MessageEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<span::time_event::MessageEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut id__ = None;
                let mut uncompressed_size__ = None;
                let mut compressed_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<span::time_event::message_event::Type>()? as i32);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UncompressedSize => {
                            if uncompressed_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uncompressedSize"));
                            }
                            uncompressed_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CompressedSize => {
                            if compressed_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("compressedSize"));
                            }
                            compressed_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(span::time_event::MessageEvent {
                    r#type: r#type__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    uncompressed_size: uncompressed_size__.unwrap_or_default(),
                    compressed_size: compressed_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.Span.TimeEvent.MessageEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for span::time_event::message_event::Type {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TYPE_UNSPECIFIED",
            Self::Sent => "SENT",
            Self::Received => "RECEIVED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for span::time_event::message_event::Type {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TYPE_UNSPECIFIED",
            "SENT",
            "RECEIVED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = span::time_event::message_event::Type;

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
                    "TYPE_UNSPECIFIED" => Ok(span::time_event::message_event::Type::Unspecified),
                    "SENT" => Ok(span::time_event::message_event::Type::Sent),
                    "RECEIVED" => Ok(span::time_event::message_event::Type::Received),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for span::TimeEvents {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.time_event.is_empty() {
            len += 1;
        }
        if self.dropped_annotations_count != 0 {
            len += 1;
        }
        if self.dropped_message_events_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.Span.TimeEvents", len)?;
        if !self.time_event.is_empty() {
            struct_ser.serialize_field("time_event", &self.time_event)?;
        }
        if self.dropped_annotations_count != 0 {
            struct_ser.serialize_field("dropped_annotations_count", &self.dropped_annotations_count)?;
        }
        if self.dropped_message_events_count != 0 {
            struct_ser.serialize_field("dropped_message_events_count", &self.dropped_message_events_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for span::TimeEvents {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "time_event",
            "timeEvent",
            "dropped_annotations_count",
            "droppedAnnotationsCount",
            "dropped_message_events_count",
            "droppedMessageEventsCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TimeEvent,
            DroppedAnnotationsCount,
            DroppedMessageEventsCount,
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
                            "timeEvent" | "time_event" => Ok(GeneratedField::TimeEvent),
                            "droppedAnnotationsCount" | "dropped_annotations_count" => Ok(GeneratedField::DroppedAnnotationsCount),
                            "droppedMessageEventsCount" | "dropped_message_events_count" => Ok(GeneratedField::DroppedMessageEventsCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = span::TimeEvents;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.Span.TimeEvents")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<span::TimeEvents, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut time_event__ = None;
                let mut dropped_annotations_count__ = None;
                let mut dropped_message_events_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TimeEvent => {
                            if time_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeEvent"));
                            }
                            time_event__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DroppedAnnotationsCount => {
                            if dropped_annotations_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("droppedAnnotationsCount"));
                            }
                            dropped_annotations_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DroppedMessageEventsCount => {
                            if dropped_message_events_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("droppedMessageEventsCount"));
                            }
                            dropped_message_events_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(span::TimeEvents {
                    time_event: time_event__.unwrap_or_default(),
                    dropped_annotations_count: dropped_annotations_count__.unwrap_or_default(),
                    dropped_message_events_count: dropped_message_events_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.Span.TimeEvents", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for span::Tracestate {
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
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.Span.Tracestate", len)?;
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for span::Tracestate {
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
            type Value = span::Tracestate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.Span.Tracestate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<span::Tracestate, V::Error>
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
                Ok(span::Tracestate {
                    entries: entries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.Span.Tracestate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for span::tracestate::Entry {
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
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.Span.Tracestate.Entry", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for span::tracestate::Entry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
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
                            "key" => Ok(GeneratedField::Key),
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
            type Value = span::tracestate::Entry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.Span.Tracestate.Entry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<span::tracestate::Entry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(span::tracestate::Entry {
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.Span.Tracestate.Entry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StackTrace {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stack_frames.is_some() {
            len += 1;
        }
        if self.stack_trace_hash_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.StackTrace", len)?;
        if let Some(v) = self.stack_frames.as_ref() {
            struct_ser.serialize_field("stack_frames", v)?;
        }
        if self.stack_trace_hash_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("stack_trace_hash_id", ToString::to_string(&self.stack_trace_hash_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StackTrace {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stack_frames",
            "stackFrames",
            "stack_trace_hash_id",
            "stackTraceHashId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StackFrames,
            StackTraceHashId,
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
                            "stackFrames" | "stack_frames" => Ok(GeneratedField::StackFrames),
                            "stackTraceHashId" | "stack_trace_hash_id" => Ok(GeneratedField::StackTraceHashId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StackTrace;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.StackTrace")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StackTrace, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stack_frames__ = None;
                let mut stack_trace_hash_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StackFrames => {
                            if stack_frames__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stackFrames"));
                            }
                            stack_frames__ = map_.next_value()?;
                        }
                        GeneratedField::StackTraceHashId => {
                            if stack_trace_hash_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stackTraceHashId"));
                            }
                            stack_trace_hash_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(StackTrace {
                    stack_frames: stack_frames__,
                    stack_trace_hash_id: stack_trace_hash_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.StackTrace", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for stack_trace::StackFrame {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.function_name.is_some() {
            len += 1;
        }
        if self.original_function_name.is_some() {
            len += 1;
        }
        if self.file_name.is_some() {
            len += 1;
        }
        if self.line_number != 0 {
            len += 1;
        }
        if self.column_number != 0 {
            len += 1;
        }
        if self.load_module.is_some() {
            len += 1;
        }
        if self.source_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.StackTrace.StackFrame", len)?;
        if let Some(v) = self.function_name.as_ref() {
            struct_ser.serialize_field("function_name", v)?;
        }
        if let Some(v) = self.original_function_name.as_ref() {
            struct_ser.serialize_field("original_function_name", v)?;
        }
        if let Some(v) = self.file_name.as_ref() {
            struct_ser.serialize_field("file_name", v)?;
        }
        if self.line_number != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("line_number", ToString::to_string(&self.line_number).as_str())?;
        }
        if self.column_number != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("column_number", ToString::to_string(&self.column_number).as_str())?;
        }
        if let Some(v) = self.load_module.as_ref() {
            struct_ser.serialize_field("load_module", v)?;
        }
        if let Some(v) = self.source_version.as_ref() {
            struct_ser.serialize_field("source_version", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for stack_trace::StackFrame {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "function_name",
            "functionName",
            "original_function_name",
            "originalFunctionName",
            "file_name",
            "fileName",
            "line_number",
            "lineNumber",
            "column_number",
            "columnNumber",
            "load_module",
            "loadModule",
            "source_version",
            "sourceVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FunctionName,
            OriginalFunctionName,
            FileName,
            LineNumber,
            ColumnNumber,
            LoadModule,
            SourceVersion,
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
                            "functionName" | "function_name" => Ok(GeneratedField::FunctionName),
                            "originalFunctionName" | "original_function_name" => Ok(GeneratedField::OriginalFunctionName),
                            "fileName" | "file_name" => Ok(GeneratedField::FileName),
                            "lineNumber" | "line_number" => Ok(GeneratedField::LineNumber),
                            "columnNumber" | "column_number" => Ok(GeneratedField::ColumnNumber),
                            "loadModule" | "load_module" => Ok(GeneratedField::LoadModule),
                            "sourceVersion" | "source_version" => Ok(GeneratedField::SourceVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = stack_trace::StackFrame;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.StackTrace.StackFrame")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<stack_trace::StackFrame, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut function_name__ = None;
                let mut original_function_name__ = None;
                let mut file_name__ = None;
                let mut line_number__ = None;
                let mut column_number__ = None;
                let mut load_module__ = None;
                let mut source_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FunctionName => {
                            if function_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functionName"));
                            }
                            function_name__ = map_.next_value()?;
                        }
                        GeneratedField::OriginalFunctionName => {
                            if original_function_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originalFunctionName"));
                            }
                            original_function_name__ = map_.next_value()?;
                        }
                        GeneratedField::FileName => {
                            if file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            file_name__ = map_.next_value()?;
                        }
                        GeneratedField::LineNumber => {
                            if line_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lineNumber"));
                            }
                            line_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ColumnNumber => {
                            if column_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnNumber"));
                            }
                            column_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LoadModule => {
                            if load_module__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadModule"));
                            }
                            load_module__ = map_.next_value()?;
                        }
                        GeneratedField::SourceVersion => {
                            if source_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceVersion"));
                            }
                            source_version__ = map_.next_value()?;
                        }
                    }
                }
                Ok(stack_trace::StackFrame {
                    function_name: function_name__,
                    original_function_name: original_function_name__,
                    file_name: file_name__,
                    line_number: line_number__.unwrap_or_default(),
                    column_number: column_number__.unwrap_or_default(),
                    load_module: load_module__,
                    source_version: source_version__,
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.StackTrace.StackFrame", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for stack_trace::StackFrames {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.frame.is_empty() {
            len += 1;
        }
        if self.dropped_frames_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.StackTrace.StackFrames", len)?;
        if !self.frame.is_empty() {
            struct_ser.serialize_field("frame", &self.frame)?;
        }
        if self.dropped_frames_count != 0 {
            struct_ser.serialize_field("dropped_frames_count", &self.dropped_frames_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for stack_trace::StackFrames {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "frame",
            "dropped_frames_count",
            "droppedFramesCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Frame,
            DroppedFramesCount,
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
                            "frame" => Ok(GeneratedField::Frame),
                            "droppedFramesCount" | "dropped_frames_count" => Ok(GeneratedField::DroppedFramesCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = stack_trace::StackFrames;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.StackTrace.StackFrames")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<stack_trace::StackFrames, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut frame__ = None;
                let mut dropped_frames_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Frame => {
                            if frame__.is_some() {
                                return Err(serde::de::Error::duplicate_field("frame"));
                            }
                            frame__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DroppedFramesCount => {
                            if dropped_frames_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("droppedFramesCount"));
                            }
                            dropped_frames_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(stack_trace::StackFrames {
                    frame: frame__.unwrap_or_default(),
                    dropped_frames_count: dropped_frames_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.StackTrace.StackFrames", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Status {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.Status", len)?;
        if self.code != 0 {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Status {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Message,
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
                            "code" => Ok(GeneratedField::Code),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Status;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.Status")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Status, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Status {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.Status", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TraceConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_number_of_attributes != 0 {
            len += 1;
        }
        if self.max_number_of_annotations != 0 {
            len += 1;
        }
        if self.max_number_of_message_events != 0 {
            len += 1;
        }
        if self.max_number_of_links != 0 {
            len += 1;
        }
        if self.sampler.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.TraceConfig", len)?;
        if self.max_number_of_attributes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("max_number_of_attributes", ToString::to_string(&self.max_number_of_attributes).as_str())?;
        }
        if self.max_number_of_annotations != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("max_number_of_annotations", ToString::to_string(&self.max_number_of_annotations).as_str())?;
        }
        if self.max_number_of_message_events != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("max_number_of_message_events", ToString::to_string(&self.max_number_of_message_events).as_str())?;
        }
        if self.max_number_of_links != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("max_number_of_links", ToString::to_string(&self.max_number_of_links).as_str())?;
        }
        if let Some(v) = self.sampler.as_ref() {
            match v {
                trace_config::Sampler::ProbabilitySampler(v) => {
                    struct_ser.serialize_field("probability_sampler", v)?;
                }
                trace_config::Sampler::ConstantSampler(v) => {
                    struct_ser.serialize_field("constant_sampler", v)?;
                }
                trace_config::Sampler::RateLimitingSampler(v) => {
                    struct_ser.serialize_field("rate_limiting_sampler", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TraceConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_number_of_attributes",
            "maxNumberOfAttributes",
            "max_number_of_annotations",
            "maxNumberOfAnnotations",
            "max_number_of_message_events",
            "maxNumberOfMessageEvents",
            "max_number_of_links",
            "maxNumberOfLinks",
            "probability_sampler",
            "probabilitySampler",
            "constant_sampler",
            "constantSampler",
            "rate_limiting_sampler",
            "rateLimitingSampler",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxNumberOfAttributes,
            MaxNumberOfAnnotations,
            MaxNumberOfMessageEvents,
            MaxNumberOfLinks,
            ProbabilitySampler,
            ConstantSampler,
            RateLimitingSampler,
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
                            "maxNumberOfAttributes" | "max_number_of_attributes" => Ok(GeneratedField::MaxNumberOfAttributes),
                            "maxNumberOfAnnotations" | "max_number_of_annotations" => Ok(GeneratedField::MaxNumberOfAnnotations),
                            "maxNumberOfMessageEvents" | "max_number_of_message_events" => Ok(GeneratedField::MaxNumberOfMessageEvents),
                            "maxNumberOfLinks" | "max_number_of_links" => Ok(GeneratedField::MaxNumberOfLinks),
                            "probabilitySampler" | "probability_sampler" => Ok(GeneratedField::ProbabilitySampler),
                            "constantSampler" | "constant_sampler" => Ok(GeneratedField::ConstantSampler),
                            "rateLimitingSampler" | "rate_limiting_sampler" => Ok(GeneratedField::RateLimitingSampler),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TraceConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.TraceConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TraceConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_number_of_attributes__ = None;
                let mut max_number_of_annotations__ = None;
                let mut max_number_of_message_events__ = None;
                let mut max_number_of_links__ = None;
                let mut sampler__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxNumberOfAttributes => {
                            if max_number_of_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxNumberOfAttributes"));
                            }
                            max_number_of_attributes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxNumberOfAnnotations => {
                            if max_number_of_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxNumberOfAnnotations"));
                            }
                            max_number_of_annotations__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxNumberOfMessageEvents => {
                            if max_number_of_message_events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxNumberOfMessageEvents"));
                            }
                            max_number_of_message_events__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxNumberOfLinks => {
                            if max_number_of_links__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxNumberOfLinks"));
                            }
                            max_number_of_links__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProbabilitySampler => {
                            if sampler__.is_some() {
                                return Err(serde::de::Error::duplicate_field("probabilitySampler"));
                            }
                            sampler__ = map_.next_value::<::std::option::Option<_>>()?.map(trace_config::Sampler::ProbabilitySampler)
;
                        }
                        GeneratedField::ConstantSampler => {
                            if sampler__.is_some() {
                                return Err(serde::de::Error::duplicate_field("constantSampler"));
                            }
                            sampler__ = map_.next_value::<::std::option::Option<_>>()?.map(trace_config::Sampler::ConstantSampler)
;
                        }
                        GeneratedField::RateLimitingSampler => {
                            if sampler__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimitingSampler"));
                            }
                            sampler__ = map_.next_value::<::std::option::Option<_>>()?.map(trace_config::Sampler::RateLimitingSampler)
;
                        }
                    }
                }
                Ok(TraceConfig {
                    max_number_of_attributes: max_number_of_attributes__.unwrap_or_default(),
                    max_number_of_annotations: max_number_of_annotations__.unwrap_or_default(),
                    max_number_of_message_events: max_number_of_message_events__.unwrap_or_default(),
                    max_number_of_links: max_number_of_links__.unwrap_or_default(),
                    sampler: sampler__,
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.TraceConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TruncatableString {
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
        if self.truncated_byte_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("opencensus.proto.trace.v1.TruncatableString", len)?;
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if self.truncated_byte_count != 0 {
            struct_ser.serialize_field("truncated_byte_count", &self.truncated_byte_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TruncatableString {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
            "truncated_byte_count",
            "truncatedByteCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            TruncatedByteCount,
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
                            "truncatedByteCount" | "truncated_byte_count" => Ok(GeneratedField::TruncatedByteCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TruncatableString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct opencensus.proto.trace.v1.TruncatableString")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TruncatableString, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut truncated_byte_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TruncatedByteCount => {
                            if truncated_byte_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("truncatedByteCount"));
                            }
                            truncated_byte_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TruncatableString {
                    value: value__.unwrap_or_default(),
                    truncated_byte_count: truncated_byte_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("opencensus.proto.trace.v1.TruncatableString", FIELDS, GeneratedVisitor)
    }
}
