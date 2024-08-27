impl serde::Serialize for BucketId {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bucket.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.rate_limit_quota.v3.BucketId", len)?;
        if !self.bucket.is_empty() {
            struct_ser.serialize_field("bucket", &self.bucket)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BucketId {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bucket",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bucket,
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
                            "bucket" => Ok(GeneratedField::Bucket),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BucketId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.rate_limit_quota.v3.BucketId")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BucketId, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bucket__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bucket => {
                            if bucket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucket"));
                            }
                            bucket__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(BucketId {
                    bucket: bucket__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.rate_limit_quota.v3.BucketId", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RateLimitQuotaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bucket_action.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse", len)?;
        if !self.bucket_action.is_empty() {
            struct_ser.serialize_field("bucket_action", &self.bucket_action)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimitQuotaResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bucket_action",
            "bucketAction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BucketAction,
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
                            "bucketAction" | "bucket_action" => Ok(GeneratedField::BucketAction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimitQuotaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RateLimitQuotaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bucket_action__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BucketAction => {
                            if bucket_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucketAction"));
                            }
                            bucket_action__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RateLimitQuotaResponse {
                    bucket_action: bucket_action__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_quota_response::BucketAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bucket_id.is_some() {
            len += 1;
        }
        if self.bucket_action.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction", len)?;
        if let Some(v) = self.bucket_id.as_ref() {
            struct_ser.serialize_field("bucket_id", v)?;
        }
        if let Some(v) = self.bucket_action.as_ref() {
            match v {
                rate_limit_quota_response::bucket_action::BucketAction::QuotaAssignmentAction(v) => {
                    struct_ser.serialize_field("quota_assignment_action", v)?;
                }
                rate_limit_quota_response::bucket_action::BucketAction::AbandonAction(v) => {
                    struct_ser.serialize_field("abandon_action", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_quota_response::BucketAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bucket_id",
            "bucketId",
            "quota_assignment_action",
            "quotaAssignmentAction",
            "abandon_action",
            "abandonAction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BucketId,
            QuotaAssignmentAction,
            AbandonAction,
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
                            "bucketId" | "bucket_id" => Ok(GeneratedField::BucketId),
                            "quotaAssignmentAction" | "quota_assignment_action" => Ok(GeneratedField::QuotaAssignmentAction),
                            "abandonAction" | "abandon_action" => Ok(GeneratedField::AbandonAction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_quota_response::BucketAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit_quota_response::BucketAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bucket_id__ = None;
                let mut bucket_action__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BucketId => {
                            if bucket_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucketId"));
                            }
                            bucket_id__ = map_.next_value()?;
                        }
                        GeneratedField::QuotaAssignmentAction => {
                            if bucket_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotaAssignmentAction"));
                            }
                            bucket_action__ = map_.next_value::<::std::option::Option<_>>()?.map(rate_limit_quota_response::bucket_action::BucketAction::QuotaAssignmentAction)
;
                        }
                        GeneratedField::AbandonAction => {
                            if bucket_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("abandonAction"));
                            }
                            bucket_action__ = map_.next_value::<::std::option::Option<_>>()?.map(rate_limit_quota_response::bucket_action::BucketAction::AbandonAction)
;
                        }
                    }
                }
                Ok(rate_limit_quota_response::BucketAction {
                    bucket_id: bucket_id__,
                    bucket_action: bucket_action__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_quota_response::bucket_action::AbandonAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.AbandonAction", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_quota_response::bucket_action::AbandonAction {
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
            type Value = rate_limit_quota_response::bucket_action::AbandonAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.AbandonAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit_quota_response::bucket_action::AbandonAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(rate_limit_quota_response::bucket_action::AbandonAction {
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.AbandonAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_quota_response::bucket_action::QuotaAssignmentAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.assignment_time_to_live.is_some() {
            len += 1;
        }
        if self.rate_limit_strategy.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.QuotaAssignmentAction", len)?;
        if let Some(v) = self.assignment_time_to_live.as_ref() {
            struct_ser.serialize_field("assignment_time_to_live", v)?;
        }
        if let Some(v) = self.rate_limit_strategy.as_ref() {
            struct_ser.serialize_field("rate_limit_strategy", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_quota_response::bucket_action::QuotaAssignmentAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "assignment_time_to_live",
            "assignmentTimeToLive",
            "rate_limit_strategy",
            "rateLimitStrategy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssignmentTimeToLive,
            RateLimitStrategy,
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
                            "assignmentTimeToLive" | "assignment_time_to_live" => Ok(GeneratedField::AssignmentTimeToLive),
                            "rateLimitStrategy" | "rate_limit_strategy" => Ok(GeneratedField::RateLimitStrategy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_quota_response::bucket_action::QuotaAssignmentAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.QuotaAssignmentAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit_quota_response::bucket_action::QuotaAssignmentAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut assignment_time_to_live__ = None;
                let mut rate_limit_strategy__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssignmentTimeToLive => {
                            if assignment_time_to_live__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assignmentTimeToLive"));
                            }
                            assignment_time_to_live__ = map_.next_value()?;
                        }
                        GeneratedField::RateLimitStrategy => {
                            if rate_limit_strategy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimitStrategy"));
                            }
                            rate_limit_strategy__ = map_.next_value()?;
                        }
                    }
                }
                Ok(rate_limit_quota_response::bucket_action::QuotaAssignmentAction {
                    assignment_time_to_live: assignment_time_to_live__,
                    rate_limit_strategy: rate_limit_strategy__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.rate_limit_quota.v3.RateLimitQuotaResponse.BucketAction.QuotaAssignmentAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RateLimitQuotaUsageReports {
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
        if !self.bucket_quota_usages.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.rate_limit_quota.v3.RateLimitQuotaUsageReports", len)?;
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if !self.bucket_quota_usages.is_empty() {
            struct_ser.serialize_field("bucket_quota_usages", &self.bucket_quota_usages)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimitQuotaUsageReports {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "domain",
            "bucket_quota_usages",
            "bucketQuotaUsages",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Domain,
            BucketQuotaUsages,
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
                            "bucketQuotaUsages" | "bucket_quota_usages" => Ok(GeneratedField::BucketQuotaUsages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimitQuotaUsageReports;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.rate_limit_quota.v3.RateLimitQuotaUsageReports")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RateLimitQuotaUsageReports, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut domain__ = None;
                let mut bucket_quota_usages__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BucketQuotaUsages => {
                            if bucket_quota_usages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucketQuotaUsages"));
                            }
                            bucket_quota_usages__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RateLimitQuotaUsageReports {
                    domain: domain__.unwrap_or_default(),
                    bucket_quota_usages: bucket_quota_usages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.rate_limit_quota.v3.RateLimitQuotaUsageReports", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_quota_usage_reports::BucketQuotaUsage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bucket_id.is_some() {
            len += 1;
        }
        if self.time_elapsed.is_some() {
            len += 1;
        }
        if self.num_requests_allowed != 0 {
            len += 1;
        }
        if self.num_requests_denied != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.rate_limit_quota.v3.RateLimitQuotaUsageReports.BucketQuotaUsage", len)?;
        if let Some(v) = self.bucket_id.as_ref() {
            struct_ser.serialize_field("bucket_id", v)?;
        }
        if let Some(v) = self.time_elapsed.as_ref() {
            struct_ser.serialize_field("time_elapsed", v)?;
        }
        if self.num_requests_allowed != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("num_requests_allowed", ToString::to_string(&self.num_requests_allowed).as_str())?;
        }
        if self.num_requests_denied != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("num_requests_denied", ToString::to_string(&self.num_requests_denied).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_quota_usage_reports::BucketQuotaUsage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bucket_id",
            "bucketId",
            "time_elapsed",
            "timeElapsed",
            "num_requests_allowed",
            "numRequestsAllowed",
            "num_requests_denied",
            "numRequestsDenied",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BucketId,
            TimeElapsed,
            NumRequestsAllowed,
            NumRequestsDenied,
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
                            "bucketId" | "bucket_id" => Ok(GeneratedField::BucketId),
                            "timeElapsed" | "time_elapsed" => Ok(GeneratedField::TimeElapsed),
                            "numRequestsAllowed" | "num_requests_allowed" => Ok(GeneratedField::NumRequestsAllowed),
                            "numRequestsDenied" | "num_requests_denied" => Ok(GeneratedField::NumRequestsDenied),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_quota_usage_reports::BucketQuotaUsage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.rate_limit_quota.v3.RateLimitQuotaUsageReports.BucketQuotaUsage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rate_limit_quota_usage_reports::BucketQuotaUsage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bucket_id__ = None;
                let mut time_elapsed__ = None;
                let mut num_requests_allowed__ = None;
                let mut num_requests_denied__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BucketId => {
                            if bucket_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucketId"));
                            }
                            bucket_id__ = map_.next_value()?;
                        }
                        GeneratedField::TimeElapsed => {
                            if time_elapsed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeElapsed"));
                            }
                            time_elapsed__ = map_.next_value()?;
                        }
                        GeneratedField::NumRequestsAllowed => {
                            if num_requests_allowed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numRequestsAllowed"));
                            }
                            num_requests_allowed__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NumRequestsDenied => {
                            if num_requests_denied__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numRequestsDenied"));
                            }
                            num_requests_denied__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(rate_limit_quota_usage_reports::BucketQuotaUsage {
                    bucket_id: bucket_id__,
                    time_elapsed: time_elapsed__,
                    num_requests_allowed: num_requests_allowed__.unwrap_or_default(),
                    num_requests_denied: num_requests_denied__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.rate_limit_quota.v3.RateLimitQuotaUsageReports.BucketQuotaUsage", FIELDS, GeneratedVisitor)
    }
}
