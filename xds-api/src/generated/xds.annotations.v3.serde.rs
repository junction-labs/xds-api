impl serde::Serialize for FieldStatusAnnotation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.work_in_progress {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.annotations.v3.FieldStatusAnnotation", len)?;
        if self.work_in_progress {
            struct_ser.serialize_field("work_in_progress", &self.work_in_progress)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FieldStatusAnnotation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "work_in_progress",
            "workInProgress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkInProgress,
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
                            "workInProgress" | "work_in_progress" => Ok(GeneratedField::WorkInProgress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldStatusAnnotation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.annotations.v3.FieldStatusAnnotation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FieldStatusAnnotation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut work_in_progress__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkInProgress => {
                            if work_in_progress__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workInProgress"));
                            }
                            work_in_progress__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FieldStatusAnnotation {
                    work_in_progress: work_in_progress__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xds.annotations.v3.FieldStatusAnnotation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileStatusAnnotation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.work_in_progress {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.annotations.v3.FileStatusAnnotation", len)?;
        if self.work_in_progress {
            struct_ser.serialize_field("work_in_progress", &self.work_in_progress)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileStatusAnnotation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "work_in_progress",
            "workInProgress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkInProgress,
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
                            "workInProgress" | "work_in_progress" => Ok(GeneratedField::WorkInProgress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileStatusAnnotation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.annotations.v3.FileStatusAnnotation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FileStatusAnnotation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut work_in_progress__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkInProgress => {
                            if work_in_progress__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workInProgress"));
                            }
                            work_in_progress__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FileStatusAnnotation {
                    work_in_progress: work_in_progress__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xds.annotations.v3.FileStatusAnnotation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MessageStatusAnnotation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.work_in_progress {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.annotations.v3.MessageStatusAnnotation", len)?;
        if self.work_in_progress {
            struct_ser.serialize_field("work_in_progress", &self.work_in_progress)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MessageStatusAnnotation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "work_in_progress",
            "workInProgress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkInProgress,
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
                            "workInProgress" | "work_in_progress" => Ok(GeneratedField::WorkInProgress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MessageStatusAnnotation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.annotations.v3.MessageStatusAnnotation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MessageStatusAnnotation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut work_in_progress__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkInProgress => {
                            if work_in_progress__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workInProgress"));
                            }
                            work_in_progress__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MessageStatusAnnotation {
                    work_in_progress: work_in_progress__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xds.annotations.v3.MessageStatusAnnotation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PackageVersionStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::Frozen => "FROZEN",
            Self::Active => "ACTIVE",
            Self::NextMajorVersionCandidate => "NEXT_MAJOR_VERSION_CANDIDATE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PackageVersionStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "FROZEN",
            "ACTIVE",
            "NEXT_MAJOR_VERSION_CANDIDATE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PackageVersionStatus;

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
                    "UNKNOWN" => Ok(PackageVersionStatus::Unknown),
                    "FROZEN" => Ok(PackageVersionStatus::Frozen),
                    "ACTIVE" => Ok(PackageVersionStatus::Active),
                    "NEXT_MAJOR_VERSION_CANDIDATE" => Ok(PackageVersionStatus::NextMajorVersionCandidate),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StatusAnnotation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.work_in_progress {
            len += 1;
        }
        if self.package_version_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.annotations.v3.StatusAnnotation", len)?;
        if self.work_in_progress {
            struct_ser.serialize_field("work_in_progress", &self.work_in_progress)?;
        }
        if self.package_version_status != 0 {
            let v = PackageVersionStatus::try_from(self.package_version_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.package_version_status)))?;
            struct_ser.serialize_field("package_version_status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatusAnnotation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "work_in_progress",
            "workInProgress",
            "package_version_status",
            "packageVersionStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkInProgress,
            PackageVersionStatus,
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
                            "workInProgress" | "work_in_progress" => Ok(GeneratedField::WorkInProgress),
                            "packageVersionStatus" | "package_version_status" => Ok(GeneratedField::PackageVersionStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatusAnnotation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.annotations.v3.StatusAnnotation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StatusAnnotation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut work_in_progress__ = None;
                let mut package_version_status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkInProgress => {
                            if work_in_progress__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workInProgress"));
                            }
                            work_in_progress__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PackageVersionStatus => {
                            if package_version_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packageVersionStatus"));
                            }
                            package_version_status__ = Some(map_.next_value::<PackageVersionStatus>()? as i32);
                        }
                    }
                }
                Ok(StatusAnnotation {
                    work_in_progress: work_in_progress__.unwrap_or_default(),
                    package_version_status: package_version_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xds.annotations.v3.StatusAnnotation", FIELDS, GeneratedVisitor)
    }
}
