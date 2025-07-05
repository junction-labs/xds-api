impl serde::Serialize for ProcessingMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request_header_mode != 0 {
            len += 1;
        }
        if self.response_header_mode != 0 {
            len += 1;
        }
        if self.request_body_mode != 0 {
            len += 1;
        }
        if self.response_body_mode != 0 {
            len += 1;
        }
        if self.request_trailer_mode != 0 {
            len += 1;
        }
        if self.response_trailer_mode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ext_proc.v3.ProcessingMode", len)?;
        if self.request_header_mode != 0 {
            let v = processing_mode::HeaderSendMode::try_from(self.request_header_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.request_header_mode)))?;
            struct_ser.serialize_field("request_header_mode", &v)?;
        }
        if self.response_header_mode != 0 {
            let v = processing_mode::HeaderSendMode::try_from(self.response_header_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.response_header_mode)))?;
            struct_ser.serialize_field("response_header_mode", &v)?;
        }
        if self.request_body_mode != 0 {
            let v = processing_mode::BodySendMode::try_from(self.request_body_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.request_body_mode)))?;
            struct_ser.serialize_field("request_body_mode", &v)?;
        }
        if self.response_body_mode != 0 {
            let v = processing_mode::BodySendMode::try_from(self.response_body_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.response_body_mode)))?;
            struct_ser.serialize_field("response_body_mode", &v)?;
        }
        if self.request_trailer_mode != 0 {
            let v = processing_mode::HeaderSendMode::try_from(self.request_trailer_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.request_trailer_mode)))?;
            struct_ser.serialize_field("request_trailer_mode", &v)?;
        }
        if self.response_trailer_mode != 0 {
            let v = processing_mode::HeaderSendMode::try_from(self.response_trailer_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.response_trailer_mode)))?;
            struct_ser.serialize_field("response_trailer_mode", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProcessingMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_header_mode",
            "requestHeaderMode",
            "response_header_mode",
            "responseHeaderMode",
            "request_body_mode",
            "requestBodyMode",
            "response_body_mode",
            "responseBodyMode",
            "request_trailer_mode",
            "requestTrailerMode",
            "response_trailer_mode",
            "responseTrailerMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestHeaderMode,
            ResponseHeaderMode,
            RequestBodyMode,
            ResponseBodyMode,
            RequestTrailerMode,
            ResponseTrailerMode,
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
                            "requestHeaderMode" | "request_header_mode" => Ok(GeneratedField::RequestHeaderMode),
                            "responseHeaderMode" | "response_header_mode" => Ok(GeneratedField::ResponseHeaderMode),
                            "requestBodyMode" | "request_body_mode" => Ok(GeneratedField::RequestBodyMode),
                            "responseBodyMode" | "response_body_mode" => Ok(GeneratedField::ResponseBodyMode),
                            "requestTrailerMode" | "request_trailer_mode" => Ok(GeneratedField::RequestTrailerMode),
                            "responseTrailerMode" | "response_trailer_mode" => Ok(GeneratedField::ResponseTrailerMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProcessingMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ext_proc.v3.ProcessingMode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProcessingMode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_header_mode__ = None;
                let mut response_header_mode__ = None;
                let mut request_body_mode__ = None;
                let mut response_body_mode__ = None;
                let mut request_trailer_mode__ = None;
                let mut response_trailer_mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RequestHeaderMode => {
                            if request_header_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeaderMode"));
                            }
                            request_header_mode__ = Some(map_.next_value::<processing_mode::HeaderSendMode>()? as i32);
                        }
                        GeneratedField::ResponseHeaderMode => {
                            if response_header_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeaderMode"));
                            }
                            response_header_mode__ = Some(map_.next_value::<processing_mode::HeaderSendMode>()? as i32);
                        }
                        GeneratedField::RequestBodyMode => {
                            if request_body_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestBodyMode"));
                            }
                            request_body_mode__ = Some(map_.next_value::<processing_mode::BodySendMode>()? as i32);
                        }
                        GeneratedField::ResponseBodyMode => {
                            if response_body_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseBodyMode"));
                            }
                            response_body_mode__ = Some(map_.next_value::<processing_mode::BodySendMode>()? as i32);
                        }
                        GeneratedField::RequestTrailerMode => {
                            if request_trailer_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestTrailerMode"));
                            }
                            request_trailer_mode__ = Some(map_.next_value::<processing_mode::HeaderSendMode>()? as i32);
                        }
                        GeneratedField::ResponseTrailerMode => {
                            if response_trailer_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseTrailerMode"));
                            }
                            response_trailer_mode__ = Some(map_.next_value::<processing_mode::HeaderSendMode>()? as i32);
                        }
                    }
                }
                Ok(ProcessingMode {
                    request_header_mode: request_header_mode__.unwrap_or_default(),
                    response_header_mode: response_header_mode__.unwrap_or_default(),
                    request_body_mode: request_body_mode__.unwrap_or_default(),
                    response_body_mode: response_body_mode__.unwrap_or_default(),
                    request_trailer_mode: request_trailer_mode__.unwrap_or_default(),
                    response_trailer_mode: response_trailer_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ext_proc.v3.ProcessingMode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for processing_mode::BodySendMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::None => "NONE",
            Self::Streamed => "STREAMED",
            Self::Buffered => "BUFFERED",
            Self::BufferedPartial => "BUFFERED_PARTIAL",
            Self::FullDuplexStreamed => "FULL_DUPLEX_STREAMED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for processing_mode::BodySendMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NONE",
            "STREAMED",
            "BUFFERED",
            "BUFFERED_PARTIAL",
            "FULL_DUPLEX_STREAMED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = processing_mode::BodySendMode;

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
                    "NONE" => Ok(processing_mode::BodySendMode::None),
                    "STREAMED" => Ok(processing_mode::BodySendMode::Streamed),
                    "BUFFERED" => Ok(processing_mode::BodySendMode::Buffered),
                    "BUFFERED_PARTIAL" => Ok(processing_mode::BodySendMode::BufferedPartial),
                    "FULL_DUPLEX_STREAMED" => Ok(processing_mode::BodySendMode::FullDuplexStreamed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for processing_mode::HeaderSendMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Default => "DEFAULT",
            Self::Send => "SEND",
            Self::Skip => "SKIP",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for processing_mode::HeaderSendMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEFAULT",
            "SEND",
            "SKIP",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = processing_mode::HeaderSendMode;

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
                    "DEFAULT" => Ok(processing_mode::HeaderSendMode::Default),
                    "SEND" => Ok(processing_mode::HeaderSendMode::Send),
                    "SKIP" => Ok(processing_mode::HeaderSendMode::Skip),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
