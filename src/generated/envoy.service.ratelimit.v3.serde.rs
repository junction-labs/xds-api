impl serde::Serialize for RateLimitRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.domain.is_empty() {
            len += 1;
        }
        if !self.descriptors.is_empty() {
            len += 1;
        }
        if self.hits_addend != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ratelimit.v3.RateLimitRequest", len)?;
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if !self.descriptors.is_empty() {
            struct_ser.serialize_field("descriptors", &self.descriptors)?;
        }
        if self.hits_addend != 0 {
            struct_ser.serialize_field("hits_addend", &self.hits_addend)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimitRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "domain",
            "descriptors",
            "hits_addend",
            "hitsAddend",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Domain,
            Descriptors,
            HitsAddend,
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
                            "domain" => Ok(GeneratedField::Domain),
                            "descriptors" => Ok(GeneratedField::Descriptors),
                            "hitsAddend" | "hits_addend" => Ok(GeneratedField::HitsAddend),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimitRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ratelimit.v3.RateLimitRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RateLimitRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut domain__ = None;
                let mut descriptors__ = None;
                let mut hits_addend__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Descriptors => {
                            if descriptors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptors"));
                            }
                            descriptors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HitsAddend => {
                            if hits_addend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hitsAddend"));
                            }
                            hits_addend__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RateLimitRequest {
                    domain: domain__.unwrap_or_default(),
                    descriptors: descriptors__.unwrap_or_default(),
                    hits_addend: hits_addend__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ratelimit.v3.RateLimitRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RateLimitResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.overall_code != 0 {
            len += 1;
        }
        if !self.statuses.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.raw_body.is_empty() {
            len += 1;
        }
        if self.dynamic_metadata.is_some() {
            len += 1;
        }
        if self.quota.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ratelimit.v3.RateLimitResponse", len)?;
        if self.overall_code != 0 {
            let v = rate_limit_response::Code::try_from(self.overall_code)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.overall_code)))?;
            struct_ser.serialize_field("overall_code", &v)?;
        }
        if !self.statuses.is_empty() {
            struct_ser.serialize_field("statuses", &self.statuses)?;
        }
        if !self.response_headers_to_add.is_empty() {
            struct_ser.serialize_field("response_headers_to_add", &self.response_headers_to_add)?;
        }
        if !self.request_headers_to_add.is_empty() {
            struct_ser.serialize_field("request_headers_to_add", &self.request_headers_to_add)?;
        }
        if !self.raw_body.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("raw_body", pbjson::private::base64::encode(&self.raw_body).as_str())?;
        }
        if let Some(v) = self.dynamic_metadata.as_ref() {
            struct_ser.serialize_field("dynamic_metadata", v)?;
        }
        if let Some(v) = self.quota.as_ref() {
            struct_ser.serialize_field("quota", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimitResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "overall_code",
            "overallCode",
            "statuses",
            "response_headers_to_add",
            "responseHeadersToAdd",
            "request_headers_to_add",
            "requestHeadersToAdd",
            "raw_body",
            "rawBody",
            "dynamic_metadata",
            "dynamicMetadata",
            "quota",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OverallCode,
            Statuses,
            ResponseHeadersToAdd,
            RequestHeadersToAdd,
            RawBody,
            DynamicMetadata,
            Quota,
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
                            "overallCode" | "overall_code" => Ok(GeneratedField::OverallCode),
                            "statuses" => Ok(GeneratedField::Statuses),
                            "responseHeadersToAdd" | "response_headers_to_add" => Ok(GeneratedField::ResponseHeadersToAdd),
                            "requestHeadersToAdd" | "request_headers_to_add" => Ok(GeneratedField::RequestHeadersToAdd),
                            "rawBody" | "raw_body" => Ok(GeneratedField::RawBody),
                            "dynamicMetadata" | "dynamic_metadata" => Ok(GeneratedField::DynamicMetadata),
                            "quota" => Ok(GeneratedField::Quota),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimitResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ratelimit.v3.RateLimitResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RateLimitResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut overall_code__ = None;
                let mut statuses__ = None;
                let mut response_headers_to_add__ = None;
                let mut request_headers_to_add__ = None;
                let mut raw_body__ = None;
                let mut dynamic_metadata__ = None;
                let mut quota__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OverallCode => {
                            if overall_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overallCode"));
                            }
                            overall_code__ = Some(map_.next_value::<rate_limit_response::Code>()? as i32);
                        }
                        GeneratedField::Statuses => {
                            if statuses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statuses"));
                            }
                            statuses__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToAdd => {
                            if response_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToAdd"));
                            }
                            response_headers_to_add__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestHeadersToAdd => {
                            if request_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToAdd"));
                            }
                            request_headers_to_add__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RawBody => {
                            if raw_body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rawBody"));
                            }
                            raw_body__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DynamicMetadata => {
                            if dynamic_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicMetadata"));
                            }
                            dynamic_metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Quota => {
                            if quota__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota"));
                            }
                            quota__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RateLimitResponse {
                    overall_code: overall_code__.unwrap_or_default(),
                    statuses: statuses__.unwrap_or_default(),
                    response_headers_to_add: response_headers_to_add__.unwrap_or_default(),
                    request_headers_to_add: request_headers_to_add__.unwrap_or_default(),
                    raw_body: raw_body__.unwrap_or_default(),
                    dynamic_metadata: dynamic_metadata__,
                    quota: quota__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ratelimit.v3.RateLimitResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_response::Code {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::Ok => "OK",
            Self::OverLimit => "OVER_LIMIT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_response::Code {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "OK",
            "OVER_LIMIT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_response::Code;

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
                    "UNKNOWN" => Ok(rate_limit_response::Code::Unknown),
                    "OK" => Ok(rate_limit_response::Code::Ok),
                    "OVER_LIMIT" => Ok(rate_limit_response::Code::OverLimit),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_response::DescriptorStatus {
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
        if self.current_limit.is_some() {
            len += 1;
        }
        if self.limit_remaining != 0 {
            len += 1;
        }
        if self.duration_until_reset.is_some() {
            len += 1;
        }
        if self.quota.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ratelimit.v3.RateLimitResponse.DescriptorStatus", len)?;
        if self.code != 0 {
            let v = rate_limit_response::Code::try_from(self.code)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.code)))?;
            struct_ser.serialize_field("code", &v)?;
        }
        if let Some(v) = self.current_limit.as_ref() {
            struct_ser.serialize_field("current_limit", v)?;
        }
        if self.limit_remaining != 0 {
            struct_ser.serialize_field("limit_remaining", &self.limit_remaining)?;
        }
        if let Some(v) = self.duration_until_reset.as_ref() {
            struct_ser.serialize_field("duration_until_reset", v)?;
        }
        if let Some(v) = self.quota.as_ref() {
            struct_ser.serialize_field("quota", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_response::DescriptorStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "current_limit",
            "currentLimit",
            "limit_remaining",
            "limitRemaining",
            "duration_until_reset",
            "durationUntilReset",
            "quota",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            CurrentLimit,
            LimitRemaining,
            DurationUntilReset,
            Quota,
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
                            "currentLimit" | "current_limit" => Ok(GeneratedField::CurrentLimit),
                            "limitRemaining" | "limit_remaining" => Ok(GeneratedField::LimitRemaining),
                            "durationUntilReset" | "duration_until_reset" => Ok(GeneratedField::DurationUntilReset),
                            "quota" => Ok(GeneratedField::Quota),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_response::DescriptorStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ratelimit.v3.RateLimitResponse.DescriptorStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit_response::DescriptorStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut current_limit__ = None;
                let mut limit_remaining__ = None;
                let mut duration_until_reset__ = None;
                let mut quota__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value::<rate_limit_response::Code>()? as i32);
                        }
                        GeneratedField::CurrentLimit => {
                            if current_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentLimit"));
                            }
                            current_limit__ = map_.next_value()?;
                        }
                        GeneratedField::LimitRemaining => {
                            if limit_remaining__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limitRemaining"));
                            }
                            limit_remaining__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DurationUntilReset => {
                            if duration_until_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("durationUntilReset"));
                            }
                            duration_until_reset__ = map_.next_value()?;
                        }
                        GeneratedField::Quota => {
                            if quota__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota"));
                            }
                            quota__ = map_.next_value()?;
                        }
                    }
                }
                Ok(rate_limit_response::DescriptorStatus {
                    code: code__.unwrap_or_default(),
                    current_limit: current_limit__,
                    limit_remaining: limit_remaining__.unwrap_or_default(),
                    duration_until_reset: duration_until_reset__,
                    quota: quota__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ratelimit.v3.RateLimitResponse.DescriptorStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_response::Quota {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.requests != 0 {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if self.expiration_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ratelimit.v3.RateLimitResponse.Quota", len)?;
        if self.requests != 0 {
            struct_ser.serialize_field("requests", &self.requests)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.expiration_specifier.as_ref() {
            match v {
                rate_limit_response::quota::ExpirationSpecifier::ValidUntil(v) => {
                    struct_ser.serialize_field("valid_until", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_response::Quota {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requests",
            "id",
            "valid_until",
            "validUntil",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requests,
            Id,
            ValidUntil,
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
                            "requests" => Ok(GeneratedField::Requests),
                            "id" => Ok(GeneratedField::Id),
                            "validUntil" | "valid_until" => Ok(GeneratedField::ValidUntil),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_response::Quota;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ratelimit.v3.RateLimitResponse.Quota")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit_response::Quota, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requests__ = None;
                let mut id__ = None;
                let mut expiration_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requests => {
                            if requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requests"));
                            }
                            requests__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValidUntil => {
                            if expiration_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validUntil"));
                            }
                            expiration_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(rate_limit_response::quota::ExpirationSpecifier::ValidUntil)
;
                        }
                    }
                }
                Ok(rate_limit_response::Quota {
                    requests: requests__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    expiration_specifier: expiration_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ratelimit.v3.RateLimitResponse.Quota", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_response::RateLimit {
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
        if self.requests_per_unit != 0 {
            len += 1;
        }
        if self.unit != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ratelimit.v3.RateLimitResponse.RateLimit", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.requests_per_unit != 0 {
            struct_ser.serialize_field("requests_per_unit", &self.requests_per_unit)?;
        }
        if self.unit != 0 {
            let v = rate_limit_response::rate_limit::Unit::try_from(self.unit)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.unit)))?;
            struct_ser.serialize_field("unit", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_response::RateLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "requests_per_unit",
            "requestsPerUnit",
            "unit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            RequestsPerUnit,
            Unit,
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
                            "requestsPerUnit" | "requests_per_unit" => Ok(GeneratedField::RequestsPerUnit),
                            "unit" => Ok(GeneratedField::Unit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_response::RateLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ratelimit.v3.RateLimitResponse.RateLimit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit_response::RateLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut requests_per_unit__ = None;
                let mut unit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestsPerUnit => {
                            if requests_per_unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestsPerUnit"));
                            }
                            requests_per_unit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value::<rate_limit_response::rate_limit::Unit>()? as i32);
                        }
                    }
                }
                Ok(rate_limit_response::RateLimit {
                    name: name__.unwrap_or_default(),
                    requests_per_unit: requests_per_unit__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ratelimit.v3.RateLimitResponse.RateLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_response::rate_limit::Unit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::Second => "SECOND",
            Self::Minute => "MINUTE",
            Self::Hour => "HOUR",
            Self::Day => "DAY",
            Self::Week => "WEEK",
            Self::Month => "MONTH",
            Self::Year => "YEAR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_response::rate_limit::Unit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "SECOND",
            "MINUTE",
            "HOUR",
            "DAY",
            "WEEK",
            "MONTH",
            "YEAR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_response::rate_limit::Unit;

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
                    "UNKNOWN" => Ok(rate_limit_response::rate_limit::Unit::Unknown),
                    "SECOND" => Ok(rate_limit_response::rate_limit::Unit::Second),
                    "MINUTE" => Ok(rate_limit_response::rate_limit::Unit::Minute),
                    "HOUR" => Ok(rate_limit_response::rate_limit::Unit::Hour),
                    "DAY" => Ok(rate_limit_response::rate_limit::Unit::Day),
                    "WEEK" => Ok(rate_limit_response::rate_limit::Unit::Week),
                    "MONTH" => Ok(rate_limit_response::rate_limit::Unit::Month),
                    "YEAR" => Ok(rate_limit_response::rate_limit::Unit::Year),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
