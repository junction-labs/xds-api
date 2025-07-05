impl serde::Serialize for CertificateProviderPluginInstance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.instance_name.is_empty() {
            len += 1;
        }
        if !self.certificate_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.CertificateProviderPluginInstance", len)?;
        if !self.instance_name.is_empty() {
            struct_ser.serialize_field("instance_name", &self.instance_name)?;
        }
        if !self.certificate_name.is_empty() {
            struct_ser.serialize_field("certificate_name", &self.certificate_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CertificateProviderPluginInstance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_name",
            "instanceName",
            "certificate_name",
            "certificateName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceName,
            CertificateName,
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
                            "instanceName" | "instance_name" => Ok(GeneratedField::InstanceName),
                            "certificateName" | "certificate_name" => Ok(GeneratedField::CertificateName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CertificateProviderPluginInstance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.CertificateProviderPluginInstance")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CertificateProviderPluginInstance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_name__ = None;
                let mut certificate_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceName => {
                            if instance_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceName"));
                            }
                            instance_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CertificateName => {
                            if certificate_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("certificateName"));
                            }
                            certificate_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CertificateProviderPluginInstance {
                    instance_name: instance_name__.unwrap_or_default(),
                    certificate_name: certificate_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.CertificateProviderPluginInstance", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CertificateValidationContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.trusted_ca.is_some() {
            len += 1;
        }
        if self.ca_certificate_provider_instance.is_some() {
            len += 1;
        }
        if self.system_root_certs.is_some() {
            len += 1;
        }
        if self.watched_directory.is_some() {
            len += 1;
        }
        if !self.verify_certificate_spki.is_empty() {
            len += 1;
        }
        if !self.verify_certificate_hash.is_empty() {
            len += 1;
        }
        if !self.match_typed_subject_alt_names.is_empty() {
            len += 1;
        }
        if !self.match_subject_alt_names.is_empty() {
            len += 1;
        }
        if self.require_signed_certificate_timestamp.is_some() {
            len += 1;
        }
        if self.crl.is_some() {
            len += 1;
        }
        if self.allow_expired_certificate {
            len += 1;
        }
        if self.trust_chain_verification != 0 {
            len += 1;
        }
        if self.custom_validator_config.is_some() {
            len += 1;
        }
        if self.only_verify_leaf_cert_crl {
            len += 1;
        }
        if self.max_verify_depth.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext", len)?;
        if let Some(v) = self.trusted_ca.as_ref() {
            struct_ser.serialize_field("trusted_ca", v)?;
        }
        if let Some(v) = self.ca_certificate_provider_instance.as_ref() {
            struct_ser.serialize_field("ca_certificate_provider_instance", v)?;
        }
        if let Some(v) = self.system_root_certs.as_ref() {
            struct_ser.serialize_field("system_root_certs", v)?;
        }
        if let Some(v) = self.watched_directory.as_ref() {
            struct_ser.serialize_field("watched_directory", v)?;
        }
        if !self.verify_certificate_spki.is_empty() {
            struct_ser.serialize_field("verify_certificate_spki", &self.verify_certificate_spki)?;
        }
        if !self.verify_certificate_hash.is_empty() {
            struct_ser.serialize_field("verify_certificate_hash", &self.verify_certificate_hash)?;
        }
        if !self.match_typed_subject_alt_names.is_empty() {
            struct_ser.serialize_field("match_typed_subject_alt_names", &self.match_typed_subject_alt_names)?;
        }
        if !self.match_subject_alt_names.is_empty() {
            struct_ser.serialize_field("match_subject_alt_names", &self.match_subject_alt_names)?;
        }
        if let Some(v) = self.require_signed_certificate_timestamp.as_ref() {
            struct_ser.serialize_field("require_signed_certificate_timestamp", v)?;
        }
        if let Some(v) = self.crl.as_ref() {
            struct_ser.serialize_field("crl", v)?;
        }
        if self.allow_expired_certificate {
            struct_ser.serialize_field("allow_expired_certificate", &self.allow_expired_certificate)?;
        }
        if self.trust_chain_verification != 0 {
            let v = certificate_validation_context::TrustChainVerification::try_from(self.trust_chain_verification)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.trust_chain_verification)))?;
            struct_ser.serialize_field("trust_chain_verification", &v)?;
        }
        if let Some(v) = self.custom_validator_config.as_ref() {
            struct_ser.serialize_field("custom_validator_config", v)?;
        }
        if self.only_verify_leaf_cert_crl {
            struct_ser.serialize_field("only_verify_leaf_cert_crl", &self.only_verify_leaf_cert_crl)?;
        }
        if let Some(v) = self.max_verify_depth.as_ref() {
            struct_ser.serialize_field("max_verify_depth", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CertificateValidationContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trusted_ca",
            "trustedCa",
            "ca_certificate_provider_instance",
            "caCertificateProviderInstance",
            "system_root_certs",
            "systemRootCerts",
            "watched_directory",
            "watchedDirectory",
            "verify_certificate_spki",
            "verifyCertificateSpki",
            "verify_certificate_hash",
            "verifyCertificateHash",
            "match_typed_subject_alt_names",
            "matchTypedSubjectAltNames",
            "match_subject_alt_names",
            "matchSubjectAltNames",
            "require_signed_certificate_timestamp",
            "requireSignedCertificateTimestamp",
            "crl",
            "allow_expired_certificate",
            "allowExpiredCertificate",
            "trust_chain_verification",
            "trustChainVerification",
            "custom_validator_config",
            "customValidatorConfig",
            "only_verify_leaf_cert_crl",
            "onlyVerifyLeafCertCrl",
            "max_verify_depth",
            "maxVerifyDepth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TrustedCa,
            CaCertificateProviderInstance,
            SystemRootCerts,
            WatchedDirectory,
            VerifyCertificateSpki,
            VerifyCertificateHash,
            MatchTypedSubjectAltNames,
            MatchSubjectAltNames,
            RequireSignedCertificateTimestamp,
            Crl,
            AllowExpiredCertificate,
            TrustChainVerification,
            CustomValidatorConfig,
            OnlyVerifyLeafCertCrl,
            MaxVerifyDepth,
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
                            "trustedCa" | "trusted_ca" => Ok(GeneratedField::TrustedCa),
                            "caCertificateProviderInstance" | "ca_certificate_provider_instance" => Ok(GeneratedField::CaCertificateProviderInstance),
                            "systemRootCerts" | "system_root_certs" => Ok(GeneratedField::SystemRootCerts),
                            "watchedDirectory" | "watched_directory" => Ok(GeneratedField::WatchedDirectory),
                            "verifyCertificateSpki" | "verify_certificate_spki" => Ok(GeneratedField::VerifyCertificateSpki),
                            "verifyCertificateHash" | "verify_certificate_hash" => Ok(GeneratedField::VerifyCertificateHash),
                            "matchTypedSubjectAltNames" | "match_typed_subject_alt_names" => Ok(GeneratedField::MatchTypedSubjectAltNames),
                            "matchSubjectAltNames" | "match_subject_alt_names" => Ok(GeneratedField::MatchSubjectAltNames),
                            "requireSignedCertificateTimestamp" | "require_signed_certificate_timestamp" => Ok(GeneratedField::RequireSignedCertificateTimestamp),
                            "crl" => Ok(GeneratedField::Crl),
                            "allowExpiredCertificate" | "allow_expired_certificate" => Ok(GeneratedField::AllowExpiredCertificate),
                            "trustChainVerification" | "trust_chain_verification" => Ok(GeneratedField::TrustChainVerification),
                            "customValidatorConfig" | "custom_validator_config" => Ok(GeneratedField::CustomValidatorConfig),
                            "onlyVerifyLeafCertCrl" | "only_verify_leaf_cert_crl" => Ok(GeneratedField::OnlyVerifyLeafCertCrl),
                            "maxVerifyDepth" | "max_verify_depth" => Ok(GeneratedField::MaxVerifyDepth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CertificateValidationContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CertificateValidationContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trusted_ca__ = None;
                let mut ca_certificate_provider_instance__ = None;
                let mut system_root_certs__ = None;
                let mut watched_directory__ = None;
                let mut verify_certificate_spki__ = None;
                let mut verify_certificate_hash__ = None;
                let mut match_typed_subject_alt_names__ = None;
                let mut match_subject_alt_names__ = None;
                let mut require_signed_certificate_timestamp__ = None;
                let mut crl__ = None;
                let mut allow_expired_certificate__ = None;
                let mut trust_chain_verification__ = None;
                let mut custom_validator_config__ = None;
                let mut only_verify_leaf_cert_crl__ = None;
                let mut max_verify_depth__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TrustedCa => {
                            if trusted_ca__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trustedCa"));
                            }
                            trusted_ca__ = map_.next_value()?;
                        }
                        GeneratedField::CaCertificateProviderInstance => {
                            if ca_certificate_provider_instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caCertificateProviderInstance"));
                            }
                            ca_certificate_provider_instance__ = map_.next_value()?;
                        }
                        GeneratedField::SystemRootCerts => {
                            if system_root_certs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("systemRootCerts"));
                            }
                            system_root_certs__ = map_.next_value()?;
                        }
                        GeneratedField::WatchedDirectory => {
                            if watched_directory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watchedDirectory"));
                            }
                            watched_directory__ = map_.next_value()?;
                        }
                        GeneratedField::VerifyCertificateSpki => {
                            if verify_certificate_spki__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifyCertificateSpki"));
                            }
                            verify_certificate_spki__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VerifyCertificateHash => {
                            if verify_certificate_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifyCertificateHash"));
                            }
                            verify_certificate_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MatchTypedSubjectAltNames => {
                            if match_typed_subject_alt_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchTypedSubjectAltNames"));
                            }
                            match_typed_subject_alt_names__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MatchSubjectAltNames => {
                            if match_subject_alt_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchSubjectAltNames"));
                            }
                            match_subject_alt_names__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequireSignedCertificateTimestamp => {
                            if require_signed_certificate_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requireSignedCertificateTimestamp"));
                            }
                            require_signed_certificate_timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::Crl => {
                            if crl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("crl"));
                            }
                            crl__ = map_.next_value()?;
                        }
                        GeneratedField::AllowExpiredCertificate => {
                            if allow_expired_certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowExpiredCertificate"));
                            }
                            allow_expired_certificate__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TrustChainVerification => {
                            if trust_chain_verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trustChainVerification"));
                            }
                            trust_chain_verification__ = Some(map_.next_value::<certificate_validation_context::TrustChainVerification>()? as i32);
                        }
                        GeneratedField::CustomValidatorConfig => {
                            if custom_validator_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customValidatorConfig"));
                            }
                            custom_validator_config__ = map_.next_value()?;
                        }
                        GeneratedField::OnlyVerifyLeafCertCrl => {
                            if only_verify_leaf_cert_crl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onlyVerifyLeafCertCrl"));
                            }
                            only_verify_leaf_cert_crl__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxVerifyDepth => {
                            if max_verify_depth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxVerifyDepth"));
                            }
                            max_verify_depth__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CertificateValidationContext {
                    trusted_ca: trusted_ca__,
                    ca_certificate_provider_instance: ca_certificate_provider_instance__,
                    system_root_certs: system_root_certs__,
                    watched_directory: watched_directory__,
                    verify_certificate_spki: verify_certificate_spki__.unwrap_or_default(),
                    verify_certificate_hash: verify_certificate_hash__.unwrap_or_default(),
                    match_typed_subject_alt_names: match_typed_subject_alt_names__.unwrap_or_default(),
                    match_subject_alt_names: match_subject_alt_names__.unwrap_or_default(),
                    require_signed_certificate_timestamp: require_signed_certificate_timestamp__,
                    crl: crl__,
                    allow_expired_certificate: allow_expired_certificate__.unwrap_or_default(),
                    trust_chain_verification: trust_chain_verification__.unwrap_or_default(),
                    custom_validator_config: custom_validator_config__,
                    only_verify_leaf_cert_crl: only_verify_leaf_cert_crl__.unwrap_or_default(),
                    max_verify_depth: max_verify_depth__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for certificate_validation_context::SystemRootCerts {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext.SystemRootCerts", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for certificate_validation_context::SystemRootCerts {
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
            type Value = certificate_validation_context::SystemRootCerts;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext.SystemRootCerts")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<certificate_validation_context::SystemRootCerts, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(certificate_validation_context::SystemRootCerts {
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext.SystemRootCerts", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for certificate_validation_context::TrustChainVerification {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::VerifyTrustChain => "VERIFY_TRUST_CHAIN",
            Self::AcceptUntrusted => "ACCEPT_UNTRUSTED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for certificate_validation_context::TrustChainVerification {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "VERIFY_TRUST_CHAIN",
            "ACCEPT_UNTRUSTED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = certificate_validation_context::TrustChainVerification;

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
                    "VERIFY_TRUST_CHAIN" => Ok(certificate_validation_context::TrustChainVerification::VerifyTrustChain),
                    "ACCEPT_UNTRUSTED" => Ok(certificate_validation_context::TrustChainVerification::AcceptUntrusted),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GenericSecret {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.secret.is_some() {
            len += 1;
        }
        if !self.secrets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.GenericSecret", len)?;
        if let Some(v) = self.secret.as_ref() {
            struct_ser.serialize_field("secret", v)?;
        }
        if !self.secrets.is_empty() {
            struct_ser.serialize_field("secrets", &self.secrets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenericSecret {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "secret",
            "secrets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Secret,
            Secrets,
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
                            "secret" => Ok(GeneratedField::Secret),
                            "secrets" => Ok(GeneratedField::Secrets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenericSecret;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.GenericSecret")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenericSecret, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut secret__ = None;
                let mut secrets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Secret => {
                            if secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secret"));
                            }
                            secret__ = map_.next_value()?;
                        }
                        GeneratedField::Secrets => {
                            if secrets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secrets"));
                            }
                            secrets__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(GenericSecret {
                    secret: secret__,
                    secrets: secrets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.GenericSecret", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PrivateKeyProvider {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider_name.is_empty() {
            len += 1;
        }
        if self.fallback {
            len += 1;
        }
        if self.config_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.PrivateKeyProvider", len)?;
        if !self.provider_name.is_empty() {
            struct_ser.serialize_field("provider_name", &self.provider_name)?;
        }
        if self.fallback {
            struct_ser.serialize_field("fallback", &self.fallback)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                private_key_provider::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typed_config", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PrivateKeyProvider {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider_name",
            "providerName",
            "fallback",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProviderName,
            Fallback,
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
                            "providerName" | "provider_name" => Ok(GeneratedField::ProviderName),
                            "fallback" => Ok(GeneratedField::Fallback),
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
            type Value = PrivateKeyProvider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.PrivateKeyProvider")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PrivateKeyProvider, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider_name__ = None;
                let mut fallback__ = None;
                let mut config_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProviderName => {
                            if provider_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerName"));
                            }
                            provider_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fallback => {
                            if fallback__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fallback"));
                            }
                            fallback__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map_.next_value::<::std::option::Option<_>>()?.map(private_key_provider::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(PrivateKeyProvider {
                    provider_name: provider_name__.unwrap_or_default(),
                    fallback: fallback__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.PrivateKeyProvider", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SdsSecretConfig {
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
        if self.sds_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.sds_config.as_ref() {
            struct_ser.serialize_field("sds_config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SdsSecretConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "sds_config",
            "sdsConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            SdsConfig,
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
                            "sdsConfig" | "sds_config" => Ok(GeneratedField::SdsConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SdsSecretConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SdsSecretConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut sds_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SdsConfig => {
                            if sds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sdsConfig"));
                            }
                            sds_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SdsSecretConfig {
                    name: name__.unwrap_or_default(),
                    sds_config: sds_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Secret {
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
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.Secret", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            match v {
                secret::Type::TlsCertificate(v) => {
                    struct_ser.serialize_field("tls_certificate", v)?;
                }
                secret::Type::SessionTicketKeys(v) => {
                    struct_ser.serialize_field("session_ticket_keys", v)?;
                }
                secret::Type::ValidationContext(v) => {
                    struct_ser.serialize_field("validation_context", v)?;
                }
                secret::Type::GenericSecret(v) => {
                    struct_ser.serialize_field("generic_secret", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Secret {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "tls_certificate",
            "tlsCertificate",
            "session_ticket_keys",
            "sessionTicketKeys",
            "validation_context",
            "validationContext",
            "generic_secret",
            "genericSecret",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            TlsCertificate,
            SessionTicketKeys,
            ValidationContext,
            GenericSecret,
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
                            "tlsCertificate" | "tls_certificate" => Ok(GeneratedField::TlsCertificate),
                            "sessionTicketKeys" | "session_ticket_keys" => Ok(GeneratedField::SessionTicketKeys),
                            "validationContext" | "validation_context" => Ok(GeneratedField::ValidationContext),
                            "genericSecret" | "generic_secret" => Ok(GeneratedField::GenericSecret),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Secret;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.Secret")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Secret, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TlsCertificate => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsCertificate"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(secret::Type::TlsCertificate)
;
                        }
                        GeneratedField::SessionTicketKeys => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionTicketKeys"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(secret::Type::SessionTicketKeys)
;
                        }
                        GeneratedField::ValidationContext => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationContext"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(secret::Type::ValidationContext)
;
                        }
                        GeneratedField::GenericSecret => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genericSecret"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(secret::Type::GenericSecret)
;
                        }
                    }
                }
                Ok(Secret {
                    name: name__.unwrap_or_default(),
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.Secret", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubjectAltNameMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.san_type != 0 {
            len += 1;
        }
        if self.matcher.is_some() {
            len += 1;
        }
        if !self.oid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.SubjectAltNameMatcher", len)?;
        if self.san_type != 0 {
            let v = subject_alt_name_matcher::SanType::try_from(self.san_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.san_type)))?;
            struct_ser.serialize_field("san_type", &v)?;
        }
        if let Some(v) = self.matcher.as_ref() {
            struct_ser.serialize_field("matcher", v)?;
        }
        if !self.oid.is_empty() {
            struct_ser.serialize_field("oid", &self.oid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubjectAltNameMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "san_type",
            "sanType",
            "matcher",
            "oid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SanType,
            Matcher,
            Oid,
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
                            "sanType" | "san_type" => Ok(GeneratedField::SanType),
                            "matcher" => Ok(GeneratedField::Matcher),
                            "oid" => Ok(GeneratedField::Oid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubjectAltNameMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.SubjectAltNameMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubjectAltNameMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut san_type__ = None;
                let mut matcher__ = None;
                let mut oid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SanType => {
                            if san_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sanType"));
                            }
                            san_type__ = Some(map_.next_value::<subject_alt_name_matcher::SanType>()? as i32);
                        }
                        GeneratedField::Matcher => {
                            if matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matcher"));
                            }
                            matcher__ = map_.next_value()?;
                        }
                        GeneratedField::Oid => {
                            if oid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oid"));
                            }
                            oid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SubjectAltNameMatcher {
                    san_type: san_type__.unwrap_or_default(),
                    matcher: matcher__,
                    oid: oid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.SubjectAltNameMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for subject_alt_name_matcher::SanType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SAN_TYPE_UNSPECIFIED",
            Self::Email => "EMAIL",
            Self::Dns => "DNS",
            Self::Uri => "URI",
            Self::IpAddress => "IP_ADDRESS",
            Self::OtherName => "OTHER_NAME",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for subject_alt_name_matcher::SanType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SAN_TYPE_UNSPECIFIED",
            "EMAIL",
            "DNS",
            "URI",
            "IP_ADDRESS",
            "OTHER_NAME",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = subject_alt_name_matcher::SanType;

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
                    "SAN_TYPE_UNSPECIFIED" => Ok(subject_alt_name_matcher::SanType::Unspecified),
                    "EMAIL" => Ok(subject_alt_name_matcher::SanType::Email),
                    "DNS" => Ok(subject_alt_name_matcher::SanType::Dns),
                    "URI" => Ok(subject_alt_name_matcher::SanType::Uri),
                    "IP_ADDRESS" => Ok(subject_alt_name_matcher::SanType::IpAddress),
                    "OTHER_NAME" => Ok(subject_alt_name_matcher::SanType::OtherName),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TlsCertificate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.certificate_chain.is_some() {
            len += 1;
        }
        if self.private_key.is_some() {
            len += 1;
        }
        if self.pkcs12.is_some() {
            len += 1;
        }
        if self.watched_directory.is_some() {
            len += 1;
        }
        if self.private_key_provider.is_some() {
            len += 1;
        }
        if self.password.is_some() {
            len += 1;
        }
        if self.ocsp_staple.is_some() {
            len += 1;
        }
        if !self.signed_certificate_timestamp.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.TlsCertificate", len)?;
        if let Some(v) = self.certificate_chain.as_ref() {
            struct_ser.serialize_field("certificate_chain", v)?;
        }
        if let Some(v) = self.private_key.as_ref() {
            struct_ser.serialize_field("private_key", v)?;
        }
        if let Some(v) = self.pkcs12.as_ref() {
            struct_ser.serialize_field("pkcs12", v)?;
        }
        if let Some(v) = self.watched_directory.as_ref() {
            struct_ser.serialize_field("watched_directory", v)?;
        }
        if let Some(v) = self.private_key_provider.as_ref() {
            struct_ser.serialize_field("private_key_provider", v)?;
        }
        if let Some(v) = self.password.as_ref() {
            struct_ser.serialize_field("password", v)?;
        }
        if let Some(v) = self.ocsp_staple.as_ref() {
            struct_ser.serialize_field("ocsp_staple", v)?;
        }
        if !self.signed_certificate_timestamp.is_empty() {
            struct_ser.serialize_field("signed_certificate_timestamp", &self.signed_certificate_timestamp)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TlsCertificate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "certificate_chain",
            "certificateChain",
            "private_key",
            "privateKey",
            "pkcs12",
            "watched_directory",
            "watchedDirectory",
            "private_key_provider",
            "privateKeyProvider",
            "password",
            "ocsp_staple",
            "ocspStaple",
            "signed_certificate_timestamp",
            "signedCertificateTimestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CertificateChain,
            PrivateKey,
            Pkcs12,
            WatchedDirectory,
            PrivateKeyProvider,
            Password,
            OcspStaple,
            SignedCertificateTimestamp,
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
                            "certificateChain" | "certificate_chain" => Ok(GeneratedField::CertificateChain),
                            "privateKey" | "private_key" => Ok(GeneratedField::PrivateKey),
                            "pkcs12" => Ok(GeneratedField::Pkcs12),
                            "watchedDirectory" | "watched_directory" => Ok(GeneratedField::WatchedDirectory),
                            "privateKeyProvider" | "private_key_provider" => Ok(GeneratedField::PrivateKeyProvider),
                            "password" => Ok(GeneratedField::Password),
                            "ocspStaple" | "ocsp_staple" => Ok(GeneratedField::OcspStaple),
                            "signedCertificateTimestamp" | "signed_certificate_timestamp" => Ok(GeneratedField::SignedCertificateTimestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TlsCertificate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.TlsCertificate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TlsCertificate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut certificate_chain__ = None;
                let mut private_key__ = None;
                let mut pkcs12__ = None;
                let mut watched_directory__ = None;
                let mut private_key_provider__ = None;
                let mut password__ = None;
                let mut ocsp_staple__ = None;
                let mut signed_certificate_timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CertificateChain => {
                            if certificate_chain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("certificateChain"));
                            }
                            certificate_chain__ = map_.next_value()?;
                        }
                        GeneratedField::PrivateKey => {
                            if private_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privateKey"));
                            }
                            private_key__ = map_.next_value()?;
                        }
                        GeneratedField::Pkcs12 => {
                            if pkcs12__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pkcs12"));
                            }
                            pkcs12__ = map_.next_value()?;
                        }
                        GeneratedField::WatchedDirectory => {
                            if watched_directory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watchedDirectory"));
                            }
                            watched_directory__ = map_.next_value()?;
                        }
                        GeneratedField::PrivateKeyProvider => {
                            if private_key_provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privateKeyProvider"));
                            }
                            private_key_provider__ = map_.next_value()?;
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = map_.next_value()?;
                        }
                        GeneratedField::OcspStaple => {
                            if ocsp_staple__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ocspStaple"));
                            }
                            ocsp_staple__ = map_.next_value()?;
                        }
                        GeneratedField::SignedCertificateTimestamp => {
                            if signed_certificate_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signedCertificateTimestamp"));
                            }
                            signed_certificate_timestamp__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TlsCertificate {
                    certificate_chain: certificate_chain__,
                    private_key: private_key__,
                    pkcs12: pkcs12__,
                    watched_directory: watched_directory__,
                    private_key_provider: private_key_provider__,
                    password: password__,
                    ocsp_staple: ocsp_staple__,
                    signed_certificate_timestamp: signed_certificate_timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.TlsCertificate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TlsParameters {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tls_minimum_protocol_version != 0 {
            len += 1;
        }
        if self.tls_maximum_protocol_version != 0 {
            len += 1;
        }
        if !self.cipher_suites.is_empty() {
            len += 1;
        }
        if !self.ecdh_curves.is_empty() {
            len += 1;
        }
        if !self.signature_algorithms.is_empty() {
            len += 1;
        }
        if !self.compliance_policies.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.TlsParameters", len)?;
        if self.tls_minimum_protocol_version != 0 {
            let v = tls_parameters::TlsProtocol::try_from(self.tls_minimum_protocol_version)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.tls_minimum_protocol_version)))?;
            struct_ser.serialize_field("tls_minimum_protocol_version", &v)?;
        }
        if self.tls_maximum_protocol_version != 0 {
            let v = tls_parameters::TlsProtocol::try_from(self.tls_maximum_protocol_version)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.tls_maximum_protocol_version)))?;
            struct_ser.serialize_field("tls_maximum_protocol_version", &v)?;
        }
        if !self.cipher_suites.is_empty() {
            struct_ser.serialize_field("cipher_suites", &self.cipher_suites)?;
        }
        if !self.ecdh_curves.is_empty() {
            struct_ser.serialize_field("ecdh_curves", &self.ecdh_curves)?;
        }
        if !self.signature_algorithms.is_empty() {
            struct_ser.serialize_field("signature_algorithms", &self.signature_algorithms)?;
        }
        if !self.compliance_policies.is_empty() {
            let v = self.compliance_policies.iter().cloned().map(|v| {
                tls_parameters::CompliancePolicy::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("compliance_policies", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TlsParameters {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tls_minimum_protocol_version",
            "tlsMinimumProtocolVersion",
            "tls_maximum_protocol_version",
            "tlsMaximumProtocolVersion",
            "cipher_suites",
            "cipherSuites",
            "ecdh_curves",
            "ecdhCurves",
            "signature_algorithms",
            "signatureAlgorithms",
            "compliance_policies",
            "compliancePolicies",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TlsMinimumProtocolVersion,
            TlsMaximumProtocolVersion,
            CipherSuites,
            EcdhCurves,
            SignatureAlgorithms,
            CompliancePolicies,
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
                            "tlsMinimumProtocolVersion" | "tls_minimum_protocol_version" => Ok(GeneratedField::TlsMinimumProtocolVersion),
                            "tlsMaximumProtocolVersion" | "tls_maximum_protocol_version" => Ok(GeneratedField::TlsMaximumProtocolVersion),
                            "cipherSuites" | "cipher_suites" => Ok(GeneratedField::CipherSuites),
                            "ecdhCurves" | "ecdh_curves" => Ok(GeneratedField::EcdhCurves),
                            "signatureAlgorithms" | "signature_algorithms" => Ok(GeneratedField::SignatureAlgorithms),
                            "compliancePolicies" | "compliance_policies" => Ok(GeneratedField::CompliancePolicies),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TlsParameters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.TlsParameters")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TlsParameters, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tls_minimum_protocol_version__ = None;
                let mut tls_maximum_protocol_version__ = None;
                let mut cipher_suites__ = None;
                let mut ecdh_curves__ = None;
                let mut signature_algorithms__ = None;
                let mut compliance_policies__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TlsMinimumProtocolVersion => {
                            if tls_minimum_protocol_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsMinimumProtocolVersion"));
                            }
                            tls_minimum_protocol_version__ = Some(map_.next_value::<tls_parameters::TlsProtocol>()? as i32);
                        }
                        GeneratedField::TlsMaximumProtocolVersion => {
                            if tls_maximum_protocol_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsMaximumProtocolVersion"));
                            }
                            tls_maximum_protocol_version__ = Some(map_.next_value::<tls_parameters::TlsProtocol>()? as i32);
                        }
                        GeneratedField::CipherSuites => {
                            if cipher_suites__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cipherSuites"));
                            }
                            cipher_suites__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EcdhCurves => {
                            if ecdh_curves__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ecdhCurves"));
                            }
                            ecdh_curves__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SignatureAlgorithms => {
                            if signature_algorithms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatureAlgorithms"));
                            }
                            signature_algorithms__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CompliancePolicies => {
                            if compliance_policies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("compliancePolicies"));
                            }
                            compliance_policies__ = Some(map_.next_value::<Vec<tls_parameters::CompliancePolicy>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(TlsParameters {
                    tls_minimum_protocol_version: tls_minimum_protocol_version__.unwrap_or_default(),
                    tls_maximum_protocol_version: tls_maximum_protocol_version__.unwrap_or_default(),
                    cipher_suites: cipher_suites__.unwrap_or_default(),
                    ecdh_curves: ecdh_curves__.unwrap_or_default(),
                    signature_algorithms: signature_algorithms__.unwrap_or_default(),
                    compliance_policies: compliance_policies__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.TlsParameters", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for tls_parameters::CompliancePolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Fips202205 => "FIPS_202205",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for tls_parameters::CompliancePolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FIPS_202205",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = tls_parameters::CompliancePolicy;

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
                    "FIPS_202205" => Ok(tls_parameters::CompliancePolicy::Fips202205),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for tls_parameters::TlsProtocol {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::TlsAuto => "TLS_AUTO",
            Self::TlSv10 => "TLSv1_0",
            Self::TlSv11 => "TLSv1_1",
            Self::TlSv12 => "TLSv1_2",
            Self::TlSv13 => "TLSv1_3",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for tls_parameters::TlsProtocol {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TLS_AUTO",
            "TLSv1_0",
            "TLSv1_1",
            "TLSv1_2",
            "TLSv1_3",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = tls_parameters::TlsProtocol;

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
                    "TLS_AUTO" => Ok(tls_parameters::TlsProtocol::TlsAuto),
                    "TLSv1_0" => Ok(tls_parameters::TlsProtocol::TlSv10),
                    "TLSv1_1" => Ok(tls_parameters::TlsProtocol::TlSv11),
                    "TLSv1_2" => Ok(tls_parameters::TlsProtocol::TlSv12),
                    "TLSv1_3" => Ok(tls_parameters::TlsProtocol::TlSv13),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TlsSessionTicketKeys {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.TlsSessionTicketKeys", len)?;
        if !self.keys.is_empty() {
            struct_ser.serialize_field("keys", &self.keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TlsSessionTicketKeys {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "keys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Keys,
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
                            "keys" => Ok(GeneratedField::Keys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TlsSessionTicketKeys;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.TlsSessionTicketKeys")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TlsSessionTicketKeys, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Keys => {
                            if keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keys"));
                            }
                            keys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TlsSessionTicketKeys {
                    keys: keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.TlsSessionTicketKeys", FIELDS, GeneratedVisitor)
    }
}
