impl serde::Serialize for FieldMigrateAnnotation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rename.is_empty() {
            len += 1;
        }
        if !self.oneof_promotion.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("udpa.annotations.FieldMigrateAnnotation", len)?;
        if !self.rename.is_empty() {
            struct_ser.serialize_field("rename", &self.rename)?;
        }
        if !self.oneof_promotion.is_empty() {
            struct_ser.serialize_field("oneof_promotion", &self.oneof_promotion)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FieldMigrateAnnotation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rename",
            "oneof_promotion",
            "oneofPromotion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rename,
            OneofPromotion,
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
                            "rename" => Ok(GeneratedField::Rename),
                            "oneofPromotion" | "oneof_promotion" => Ok(GeneratedField::OneofPromotion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldMigrateAnnotation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct udpa.annotations.FieldMigrateAnnotation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FieldMigrateAnnotation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rename__ = None;
                let mut oneof_promotion__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rename => {
                            if rename__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rename"));
                            }
                            rename__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OneofPromotion => {
                            if oneof_promotion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oneofPromotion"));
                            }
                            oneof_promotion__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FieldMigrateAnnotation {
                    rename: rename__.unwrap_or_default(),
                    oneof_promotion: oneof_promotion__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("udpa.annotations.FieldMigrateAnnotation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FieldSecurityAnnotation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.configure_for_untrusted_downstream {
            len += 1;
        }
        if self.configure_for_untrusted_upstream {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("udpa.annotations.FieldSecurityAnnotation", len)?;
        if self.configure_for_untrusted_downstream {
            struct_ser.serialize_field("configure_for_untrusted_downstream", &self.configure_for_untrusted_downstream)?;
        }
        if self.configure_for_untrusted_upstream {
            struct_ser.serialize_field("configure_for_untrusted_upstream", &self.configure_for_untrusted_upstream)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FieldSecurityAnnotation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "configure_for_untrusted_downstream",
            "configureForUntrustedDownstream",
            "configure_for_untrusted_upstream",
            "configureForUntrustedUpstream",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConfigureForUntrustedDownstream,
            ConfigureForUntrustedUpstream,
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
                            "configureForUntrustedDownstream" | "configure_for_untrusted_downstream" => Ok(GeneratedField::ConfigureForUntrustedDownstream),
                            "configureForUntrustedUpstream" | "configure_for_untrusted_upstream" => Ok(GeneratedField::ConfigureForUntrustedUpstream),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldSecurityAnnotation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct udpa.annotations.FieldSecurityAnnotation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FieldSecurityAnnotation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut configure_for_untrusted_downstream__ = None;
                let mut configure_for_untrusted_upstream__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConfigureForUntrustedDownstream => {
                            if configure_for_untrusted_downstream__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configureForUntrustedDownstream"));
                            }
                            configure_for_untrusted_downstream__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConfigureForUntrustedUpstream => {
                            if configure_for_untrusted_upstream__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configureForUntrustedUpstream"));
                            }
                            configure_for_untrusted_upstream__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FieldSecurityAnnotation {
                    configure_for_untrusted_downstream: configure_for_untrusted_downstream__.unwrap_or_default(),
                    configure_for_untrusted_upstream: configure_for_untrusted_upstream__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("udpa.annotations.FieldSecurityAnnotation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileMigrateAnnotation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.move_to_package.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("udpa.annotations.FileMigrateAnnotation", len)?;
        if !self.move_to_package.is_empty() {
            struct_ser.serialize_field("move_to_package", &self.move_to_package)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileMigrateAnnotation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "move_to_package",
            "moveToPackage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MoveToPackage,
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
                            "moveToPackage" | "move_to_package" => Ok(GeneratedField::MoveToPackage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileMigrateAnnotation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct udpa.annotations.FileMigrateAnnotation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FileMigrateAnnotation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut move_to_package__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MoveToPackage => {
                            if move_to_package__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moveToPackage"));
                            }
                            move_to_package__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FileMigrateAnnotation {
                    move_to_package: move_to_package__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("udpa.annotations.FileMigrateAnnotation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MigrateAnnotation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rename.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("udpa.annotations.MigrateAnnotation", len)?;
        if !self.rename.is_empty() {
            struct_ser.serialize_field("rename", &self.rename)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MigrateAnnotation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rename",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rename,
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
                            "rename" => Ok(GeneratedField::Rename),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MigrateAnnotation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct udpa.annotations.MigrateAnnotation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MigrateAnnotation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rename__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rename => {
                            if rename__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rename"));
                            }
                            rename__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MigrateAnnotation {
                    rename: rename__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("udpa.annotations.MigrateAnnotation", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("udpa.annotations.StatusAnnotation", len)?;
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
                formatter.write_str("struct udpa.annotations.StatusAnnotation")
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
        deserializer.deserialize_struct("udpa.annotations.StatusAnnotation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VersioningAnnotation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.previous_message_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("udpa.annotations.VersioningAnnotation", len)?;
        if !self.previous_message_type.is_empty() {
            struct_ser.serialize_field("previous_message_type", &self.previous_message_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VersioningAnnotation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "previous_message_type",
            "previousMessageType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PreviousMessageType,
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
                            "previousMessageType" | "previous_message_type" => Ok(GeneratedField::PreviousMessageType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VersioningAnnotation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct udpa.annotations.VersioningAnnotation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VersioningAnnotation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut previous_message_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PreviousMessageType => {
                            if previous_message_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("previousMessageType"));
                            }
                            previous_message_type__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(VersioningAnnotation {
                    previous_message_type: previous_message_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("udpa.annotations.VersioningAnnotation", FIELDS, GeneratedVisitor)
    }
}
