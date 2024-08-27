impl serde::Serialize for DogStatsdSink {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.prefix.is_empty() {
            len += 1;
        }
        if self.max_bytes_per_datagram.is_some() {
            len += 1;
        }
        if self.dog_statsd_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.metrics.v3.DogStatsdSink", len)?;
        if !self.prefix.is_empty() {
            struct_ser.serialize_field("prefix", &self.prefix)?;
        }
        if let Some(v) = self.max_bytes_per_datagram.as_ref() {
            struct_ser.serialize_field("max_bytes_per_datagram", v)?;
        }
        if let Some(v) = self.dog_statsd_specifier.as_ref() {
            match v {
                dog_statsd_sink::DogStatsdSpecifier::Address(v) => {
                    struct_ser.serialize_field("address", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DogStatsdSink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "prefix",
            "max_bytes_per_datagram",
            "maxBytesPerDatagram",
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Prefix,
            MaxBytesPerDatagram,
            Address,
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
                            "prefix" => Ok(GeneratedField::Prefix),
                            "maxBytesPerDatagram" | "max_bytes_per_datagram" => Ok(GeneratedField::MaxBytesPerDatagram),
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DogStatsdSink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.metrics.v3.DogStatsdSink")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DogStatsdSink, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut prefix__ = None;
                let mut max_bytes_per_datagram__ = None;
                let mut dog_statsd_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxBytesPerDatagram => {
                            if max_bytes_per_datagram__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxBytesPerDatagram"));
                            }
                            max_bytes_per_datagram__ = map_.next_value()?;
                        }
                        GeneratedField::Address => {
                            if dog_statsd_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            dog_statsd_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(dog_statsd_sink::DogStatsdSpecifier::Address)
;
                        }
                    }
                }
                Ok(DogStatsdSink {
                    prefix: prefix__.unwrap_or_default(),
                    max_bytes_per_datagram: max_bytes_per_datagram__,
                    dog_statsd_specifier: dog_statsd_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.metrics.v3.DogStatsdSink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HistogramBucketSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#match.is_some() {
            len += 1;
        }
        if !self.buckets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.metrics.v3.HistogramBucketSettings", len)?;
        if let Some(v) = self.r#match.as_ref() {
            struct_ser.serialize_field("match", v)?;
        }
        if !self.buckets.is_empty() {
            struct_ser.serialize_field("buckets", &self.buckets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HistogramBucketSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "match",
            "buckets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Match,
            Buckets,
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
                            "match" => Ok(GeneratedField::Match),
                            "buckets" => Ok(GeneratedField::Buckets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HistogramBucketSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.metrics.v3.HistogramBucketSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HistogramBucketSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#match__ = None;
                let mut buckets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Match => {
                            if r#match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            r#match__ = map_.next_value()?;
                        }
                        GeneratedField::Buckets => {
                            if buckets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            buckets__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(HistogramBucketSettings {
                    r#match: r#match__,
                    buckets: buckets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.metrics.v3.HistogramBucketSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HistogramEmitMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::SummaryAndHistogram => "SUMMARY_AND_HISTOGRAM",
            Self::Summary => "SUMMARY",
            Self::Histogram => "HISTOGRAM",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for HistogramEmitMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SUMMARY_AND_HISTOGRAM",
            "SUMMARY",
            "HISTOGRAM",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HistogramEmitMode;

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
                    "SUMMARY_AND_HISTOGRAM" => Ok(HistogramEmitMode::SummaryAndHistogram),
                    "SUMMARY" => Ok(HistogramEmitMode::Summary),
                    "HISTOGRAM" => Ok(HistogramEmitMode::Histogram),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for HystrixSink {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.num_buckets != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.metrics.v3.HystrixSink", len)?;
        if self.num_buckets != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("num_buckets", ToString::to_string(&self.num_buckets).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HystrixSink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "num_buckets",
            "numBuckets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NumBuckets,
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
                            "numBuckets" | "num_buckets" => Ok(GeneratedField::NumBuckets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HystrixSink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.metrics.v3.HystrixSink")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HystrixSink, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut num_buckets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NumBuckets => {
                            if num_buckets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numBuckets"));
                            }
                            num_buckets__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(HystrixSink {
                    num_buckets: num_buckets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.metrics.v3.HystrixSink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MetricsServiceConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.grpc_service.is_some() {
            len += 1;
        }
        if self.transport_api_version != 0 {
            len += 1;
        }
        if self.report_counters_as_deltas.is_some() {
            len += 1;
        }
        if self.emit_tags_as_labels {
            len += 1;
        }
        if self.histogram_emit_mode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.metrics.v3.MetricsServiceConfig", len)?;
        if let Some(v) = self.grpc_service.as_ref() {
            struct_ser.serialize_field("grpc_service", v)?;
        }
        if self.transport_api_version != 0 {
            let v = super::super::core::v3::ApiVersion::try_from(self.transport_api_version)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.transport_api_version)))?;
            struct_ser.serialize_field("transport_api_version", &v)?;
        }
        if let Some(v) = self.report_counters_as_deltas.as_ref() {
            struct_ser.serialize_field("report_counters_as_deltas", v)?;
        }
        if self.emit_tags_as_labels {
            struct_ser.serialize_field("emit_tags_as_labels", &self.emit_tags_as_labels)?;
        }
        if self.histogram_emit_mode != 0 {
            let v = HistogramEmitMode::try_from(self.histogram_emit_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.histogram_emit_mode)))?;
            struct_ser.serialize_field("histogram_emit_mode", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MetricsServiceConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "grpc_service",
            "grpcService",
            "transport_api_version",
            "transportApiVersion",
            "report_counters_as_deltas",
            "reportCountersAsDeltas",
            "emit_tags_as_labels",
            "emitTagsAsLabels",
            "histogram_emit_mode",
            "histogramEmitMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GrpcService,
            TransportApiVersion,
            ReportCountersAsDeltas,
            EmitTagsAsLabels,
            HistogramEmitMode,
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
                            "grpcService" | "grpc_service" => Ok(GeneratedField::GrpcService),
                            "transportApiVersion" | "transport_api_version" => Ok(GeneratedField::TransportApiVersion),
                            "reportCountersAsDeltas" | "report_counters_as_deltas" => Ok(GeneratedField::ReportCountersAsDeltas),
                            "emitTagsAsLabels" | "emit_tags_as_labels" => Ok(GeneratedField::EmitTagsAsLabels),
                            "histogramEmitMode" | "histogram_emit_mode" => Ok(GeneratedField::HistogramEmitMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MetricsServiceConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.metrics.v3.MetricsServiceConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MetricsServiceConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut grpc_service__ = None;
                let mut transport_api_version__ = None;
                let mut report_counters_as_deltas__ = None;
                let mut emit_tags_as_labels__ = None;
                let mut histogram_emit_mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GrpcService => {
                            if grpc_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcService"));
                            }
                            grpc_service__ = map_.next_value()?;
                        }
                        GeneratedField::TransportApiVersion => {
                            if transport_api_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportApiVersion"));
                            }
                            transport_api_version__ = Some(map_.next_value::<super::super::core::v3::ApiVersion>()? as i32);
                        }
                        GeneratedField::ReportCountersAsDeltas => {
                            if report_counters_as_deltas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportCountersAsDeltas"));
                            }
                            report_counters_as_deltas__ = map_.next_value()?;
                        }
                        GeneratedField::EmitTagsAsLabels => {
                            if emit_tags_as_labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emitTagsAsLabels"));
                            }
                            emit_tags_as_labels__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HistogramEmitMode => {
                            if histogram_emit_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("histogramEmitMode"));
                            }
                            histogram_emit_mode__ = Some(map_.next_value::<HistogramEmitMode>()? as i32);
                        }
                    }
                }
                Ok(MetricsServiceConfig {
                    grpc_service: grpc_service__,
                    transport_api_version: transport_api_version__.unwrap_or_default(),
                    report_counters_as_deltas: report_counters_as_deltas__,
                    emit_tags_as_labels: emit_tags_as_labels__.unwrap_or_default(),
                    histogram_emit_mode: histogram_emit_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.metrics.v3.MetricsServiceConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StatsConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stats_tags.is_empty() {
            len += 1;
        }
        if self.use_all_default_tags.is_some() {
            len += 1;
        }
        if self.stats_matcher.is_some() {
            len += 1;
        }
        if !self.histogram_bucket_settings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.metrics.v3.StatsConfig", len)?;
        if !self.stats_tags.is_empty() {
            struct_ser.serialize_field("stats_tags", &self.stats_tags)?;
        }
        if let Some(v) = self.use_all_default_tags.as_ref() {
            struct_ser.serialize_field("use_all_default_tags", v)?;
        }
        if let Some(v) = self.stats_matcher.as_ref() {
            struct_ser.serialize_field("stats_matcher", v)?;
        }
        if !self.histogram_bucket_settings.is_empty() {
            struct_ser.serialize_field("histogram_bucket_settings", &self.histogram_bucket_settings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatsConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stats_tags",
            "statsTags",
            "use_all_default_tags",
            "useAllDefaultTags",
            "stats_matcher",
            "statsMatcher",
            "histogram_bucket_settings",
            "histogramBucketSettings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatsTags,
            UseAllDefaultTags,
            StatsMatcher,
            HistogramBucketSettings,
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
                            "statsTags" | "stats_tags" => Ok(GeneratedField::StatsTags),
                            "useAllDefaultTags" | "use_all_default_tags" => Ok(GeneratedField::UseAllDefaultTags),
                            "statsMatcher" | "stats_matcher" => Ok(GeneratedField::StatsMatcher),
                            "histogramBucketSettings" | "histogram_bucket_settings" => Ok(GeneratedField::HistogramBucketSettings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatsConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.metrics.v3.StatsConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StatsConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stats_tags__ = None;
                let mut use_all_default_tags__ = None;
                let mut stats_matcher__ = None;
                let mut histogram_bucket_settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StatsTags => {
                            if stats_tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statsTags"));
                            }
                            stats_tags__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UseAllDefaultTags => {
                            if use_all_default_tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useAllDefaultTags"));
                            }
                            use_all_default_tags__ = map_.next_value()?;
                        }
                        GeneratedField::StatsMatcher => {
                            if stats_matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statsMatcher"));
                            }
                            stats_matcher__ = map_.next_value()?;
                        }
                        GeneratedField::HistogramBucketSettings => {
                            if histogram_bucket_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("histogramBucketSettings"));
                            }
                            histogram_bucket_settings__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StatsConfig {
                    stats_tags: stats_tags__.unwrap_or_default(),
                    use_all_default_tags: use_all_default_tags__,
                    stats_matcher: stats_matcher__,
                    histogram_bucket_settings: histogram_bucket_settings__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.metrics.v3.StatsConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StatsMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stats_matcher.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.metrics.v3.StatsMatcher", len)?;
        if let Some(v) = self.stats_matcher.as_ref() {
            match v {
                stats_matcher::StatsMatcher::RejectAll(v) => {
                    struct_ser.serialize_field("reject_all", v)?;
                }
                stats_matcher::StatsMatcher::ExclusionList(v) => {
                    struct_ser.serialize_field("exclusion_list", v)?;
                }
                stats_matcher::StatsMatcher::InclusionList(v) => {
                    struct_ser.serialize_field("inclusion_list", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatsMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reject_all",
            "rejectAll",
            "exclusion_list",
            "exclusionList",
            "inclusion_list",
            "inclusionList",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RejectAll,
            ExclusionList,
            InclusionList,
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
                            "rejectAll" | "reject_all" => Ok(GeneratedField::RejectAll),
                            "exclusionList" | "exclusion_list" => Ok(GeneratedField::ExclusionList),
                            "inclusionList" | "inclusion_list" => Ok(GeneratedField::InclusionList),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatsMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.metrics.v3.StatsMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StatsMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stats_matcher__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RejectAll => {
                            if stats_matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rejectAll"));
                            }
                            stats_matcher__ = map_.next_value::<::std::option::Option<_>>()?.map(stats_matcher::StatsMatcher::RejectAll);
                        }
                        GeneratedField::ExclusionList => {
                            if stats_matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exclusionList"));
                            }
                            stats_matcher__ = map_.next_value::<::std::option::Option<_>>()?.map(stats_matcher::StatsMatcher::ExclusionList)
;
                        }
                        GeneratedField::InclusionList => {
                            if stats_matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inclusionList"));
                            }
                            stats_matcher__ = map_.next_value::<::std::option::Option<_>>()?.map(stats_matcher::StatsMatcher::InclusionList)
;
                        }
                    }
                }
                Ok(StatsMatcher {
                    stats_matcher: stats_matcher__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.metrics.v3.StatsMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StatsSink {
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.metrics.v3.StatsSink", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                stats_sink::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typed_config", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatsSink {
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
            type Value = StatsSink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.metrics.v3.StatsSink")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StatsSink, V::Error>
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
                            config_type__ = map_.next_value::<::std::option::Option<_>>()?.map(stats_sink::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(StatsSink {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.metrics.v3.StatsSink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StatsdSink {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.prefix.is_empty() {
            len += 1;
        }
        if self.statsd_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.metrics.v3.StatsdSink", len)?;
        if !self.prefix.is_empty() {
            struct_ser.serialize_field("prefix", &self.prefix)?;
        }
        if let Some(v) = self.statsd_specifier.as_ref() {
            match v {
                statsd_sink::StatsdSpecifier::Address(v) => {
                    struct_ser.serialize_field("address", v)?;
                }
                statsd_sink::StatsdSpecifier::TcpClusterName(v) => {
                    struct_ser.serialize_field("tcp_cluster_name", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatsdSink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "prefix",
            "address",
            "tcp_cluster_name",
            "tcpClusterName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Prefix,
            Address,
            TcpClusterName,
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
                            "prefix" => Ok(GeneratedField::Prefix),
                            "address" => Ok(GeneratedField::Address),
                            "tcpClusterName" | "tcp_cluster_name" => Ok(GeneratedField::TcpClusterName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatsdSink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.metrics.v3.StatsdSink")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StatsdSink, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut prefix__ = None;
                let mut statsd_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Address => {
                            if statsd_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            statsd_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(statsd_sink::StatsdSpecifier::Address)
;
                        }
                        GeneratedField::TcpClusterName => {
                            if statsd_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tcpClusterName"));
                            }
                            statsd_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(statsd_sink::StatsdSpecifier::TcpClusterName);
                        }
                    }
                }
                Ok(StatsdSink {
                    prefix: prefix__.unwrap_or_default(),
                    statsd_specifier: statsd_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.metrics.v3.StatsdSink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TagSpecifier {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tag_name.is_empty() {
            len += 1;
        }
        if self.tag_value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.metrics.v3.TagSpecifier", len)?;
        if !self.tag_name.is_empty() {
            struct_ser.serialize_field("tag_name", &self.tag_name)?;
        }
        if let Some(v) = self.tag_value.as_ref() {
            match v {
                tag_specifier::TagValue::Regex(v) => {
                    struct_ser.serialize_field("regex", v)?;
                }
                tag_specifier::TagValue::FixedValue(v) => {
                    struct_ser.serialize_field("fixed_value", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TagSpecifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tag_name",
            "tagName",
            "regex",
            "fixed_value",
            "fixedValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TagName,
            Regex,
            FixedValue,
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
                            "tagName" | "tag_name" => Ok(GeneratedField::TagName),
                            "regex" => Ok(GeneratedField::Regex),
                            "fixedValue" | "fixed_value" => Ok(GeneratedField::FixedValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TagSpecifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.metrics.v3.TagSpecifier")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TagSpecifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tag_name__ = None;
                let mut tag_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TagName => {
                            if tag_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagName"));
                            }
                            tag_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Regex => {
                            if tag_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regex"));
                            }
                            tag_value__ = map_.next_value::<::std::option::Option<_>>()?.map(tag_specifier::TagValue::Regex);
                        }
                        GeneratedField::FixedValue => {
                            if tag_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedValue"));
                            }
                            tag_value__ = map_.next_value::<::std::option::Option<_>>()?.map(tag_specifier::TagValue::FixedValue);
                        }
                    }
                }
                Ok(TagSpecifier {
                    tag_name: tag_name__.unwrap_or_default(),
                    tag_value: tag_value__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.metrics.v3.TagSpecifier", FIELDS, GeneratedVisitor)
    }
}
