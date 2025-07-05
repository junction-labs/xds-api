impl serde::Serialize for AdsDummy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.service.discovery.v3.AdsDummy", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AdsDummy {
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
            type Value = AdsDummy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v3.AdsDummy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AdsDummy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AdsDummy {
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v3.AdsDummy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeltaDiscoveryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.node.is_some() {
            len += 1;
        }
        if !self.type_url.is_empty() {
            len += 1;
        }
        if !self.resource_names_subscribe.is_empty() {
            len += 1;
        }
        if !self.resource_names_unsubscribe.is_empty() {
            len += 1;
        }
        if !self.resource_locators_subscribe.is_empty() {
            len += 1;
        }
        if !self.resource_locators_unsubscribe.is_empty() {
            len += 1;
        }
        if !self.initial_resource_versions.is_empty() {
            len += 1;
        }
        if !self.response_nonce.is_empty() {
            len += 1;
        }
        if self.error_detail.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v3.DeltaDiscoveryRequest", len)?;
        if let Some(v) = self.node.as_ref() {
            struct_ser.serialize_field("node", v)?;
        }
        if !self.type_url.is_empty() {
            struct_ser.serialize_field("type_url", &self.type_url)?;
        }
        if !self.resource_names_subscribe.is_empty() {
            struct_ser.serialize_field("resource_names_subscribe", &self.resource_names_subscribe)?;
        }
        if !self.resource_names_unsubscribe.is_empty() {
            struct_ser.serialize_field("resource_names_unsubscribe", &self.resource_names_unsubscribe)?;
        }
        if !self.resource_locators_subscribe.is_empty() {
            struct_ser.serialize_field("resource_locators_subscribe", &self.resource_locators_subscribe)?;
        }
        if !self.resource_locators_unsubscribe.is_empty() {
            struct_ser.serialize_field("resource_locators_unsubscribe", &self.resource_locators_unsubscribe)?;
        }
        if !self.initial_resource_versions.is_empty() {
            struct_ser.serialize_field("initial_resource_versions", &self.initial_resource_versions)?;
        }
        if !self.response_nonce.is_empty() {
            struct_ser.serialize_field("response_nonce", &self.response_nonce)?;
        }
        if let Some(v) = self.error_detail.as_ref() {
            struct_ser.serialize_field("error_detail", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeltaDiscoveryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "node",
            "type_url",
            "typeUrl",
            "resource_names_subscribe",
            "resourceNamesSubscribe",
            "resource_names_unsubscribe",
            "resourceNamesUnsubscribe",
            "resource_locators_subscribe",
            "resourceLocatorsSubscribe",
            "resource_locators_unsubscribe",
            "resourceLocatorsUnsubscribe",
            "initial_resource_versions",
            "initialResourceVersions",
            "response_nonce",
            "responseNonce",
            "error_detail",
            "errorDetail",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Node,
            TypeUrl,
            ResourceNamesSubscribe,
            ResourceNamesUnsubscribe,
            ResourceLocatorsSubscribe,
            ResourceLocatorsUnsubscribe,
            InitialResourceVersions,
            ResponseNonce,
            ErrorDetail,
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
                            "node" => Ok(GeneratedField::Node),
                            "typeUrl" | "type_url" => Ok(GeneratedField::TypeUrl),
                            "resourceNamesSubscribe" | "resource_names_subscribe" => Ok(GeneratedField::ResourceNamesSubscribe),
                            "resourceNamesUnsubscribe" | "resource_names_unsubscribe" => Ok(GeneratedField::ResourceNamesUnsubscribe),
                            "resourceLocatorsSubscribe" | "resource_locators_subscribe" => Ok(GeneratedField::ResourceLocatorsSubscribe),
                            "resourceLocatorsUnsubscribe" | "resource_locators_unsubscribe" => Ok(GeneratedField::ResourceLocatorsUnsubscribe),
                            "initialResourceVersions" | "initial_resource_versions" => Ok(GeneratedField::InitialResourceVersions),
                            "responseNonce" | "response_nonce" => Ok(GeneratedField::ResponseNonce),
                            "errorDetail" | "error_detail" => Ok(GeneratedField::ErrorDetail),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeltaDiscoveryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v3.DeltaDiscoveryRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeltaDiscoveryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut node__ = None;
                let mut type_url__ = None;
                let mut resource_names_subscribe__ = None;
                let mut resource_names_unsubscribe__ = None;
                let mut resource_locators_subscribe__ = None;
                let mut resource_locators_unsubscribe__ = None;
                let mut initial_resource_versions__ = None;
                let mut response_nonce__ = None;
                let mut error_detail__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = map_.next_value()?;
                        }
                        GeneratedField::TypeUrl => {
                            if type_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeUrl"));
                            }
                            type_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceNamesSubscribe => {
                            if resource_names_subscribe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceNamesSubscribe"));
                            }
                            resource_names_subscribe__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceNamesUnsubscribe => {
                            if resource_names_unsubscribe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceNamesUnsubscribe"));
                            }
                            resource_names_unsubscribe__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceLocatorsSubscribe => {
                            if resource_locators_subscribe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceLocatorsSubscribe"));
                            }
                            resource_locators_subscribe__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceLocatorsUnsubscribe => {
                            if resource_locators_unsubscribe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceLocatorsUnsubscribe"));
                            }
                            resource_locators_unsubscribe__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InitialResourceVersions => {
                            if initial_resource_versions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialResourceVersions"));
                            }
                            initial_resource_versions__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::ResponseNonce => {
                            if response_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseNonce"));
                            }
                            response_nonce__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ErrorDetail => {
                            if error_detail__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorDetail"));
                            }
                            error_detail__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DeltaDiscoveryRequest {
                    node: node__,
                    type_url: type_url__.unwrap_or_default(),
                    resource_names_subscribe: resource_names_subscribe__.unwrap_or_default(),
                    resource_names_unsubscribe: resource_names_unsubscribe__.unwrap_or_default(),
                    resource_locators_subscribe: resource_locators_subscribe__.unwrap_or_default(),
                    resource_locators_unsubscribe: resource_locators_unsubscribe__.unwrap_or_default(),
                    initial_resource_versions: initial_resource_versions__.unwrap_or_default(),
                    response_nonce: response_nonce__.unwrap_or_default(),
                    error_detail: error_detail__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v3.DeltaDiscoveryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeltaDiscoveryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.system_version_info.is_empty() {
            len += 1;
        }
        if !self.resources.is_empty() {
            len += 1;
        }
        if !self.type_url.is_empty() {
            len += 1;
        }
        if !self.removed_resources.is_empty() {
            len += 1;
        }
        if !self.removed_resource_names.is_empty() {
            len += 1;
        }
        if !self.nonce.is_empty() {
            len += 1;
        }
        if self.control_plane.is_some() {
            len += 1;
        }
        if !self.resource_errors.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v3.DeltaDiscoveryResponse", len)?;
        if !self.system_version_info.is_empty() {
            struct_ser.serialize_field("system_version_info", &self.system_version_info)?;
        }
        if !self.resources.is_empty() {
            struct_ser.serialize_field("resources", &self.resources)?;
        }
        if !self.type_url.is_empty() {
            struct_ser.serialize_field("type_url", &self.type_url)?;
        }
        if !self.removed_resources.is_empty() {
            struct_ser.serialize_field("removed_resources", &self.removed_resources)?;
        }
        if !self.removed_resource_names.is_empty() {
            struct_ser.serialize_field("removed_resource_names", &self.removed_resource_names)?;
        }
        if !self.nonce.is_empty() {
            struct_ser.serialize_field("nonce", &self.nonce)?;
        }
        if let Some(v) = self.control_plane.as_ref() {
            struct_ser.serialize_field("control_plane", v)?;
        }
        if !self.resource_errors.is_empty() {
            struct_ser.serialize_field("resource_errors", &self.resource_errors)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeltaDiscoveryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "system_version_info",
            "systemVersionInfo",
            "resources",
            "type_url",
            "typeUrl",
            "removed_resources",
            "removedResources",
            "removed_resource_names",
            "removedResourceNames",
            "nonce",
            "control_plane",
            "controlPlane",
            "resource_errors",
            "resourceErrors",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SystemVersionInfo,
            Resources,
            TypeUrl,
            RemovedResources,
            RemovedResourceNames,
            Nonce,
            ControlPlane,
            ResourceErrors,
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
                            "systemVersionInfo" | "system_version_info" => Ok(GeneratedField::SystemVersionInfo),
                            "resources" => Ok(GeneratedField::Resources),
                            "typeUrl" | "type_url" => Ok(GeneratedField::TypeUrl),
                            "removedResources" | "removed_resources" => Ok(GeneratedField::RemovedResources),
                            "removedResourceNames" | "removed_resource_names" => Ok(GeneratedField::RemovedResourceNames),
                            "nonce" => Ok(GeneratedField::Nonce),
                            "controlPlane" | "control_plane" => Ok(GeneratedField::ControlPlane),
                            "resourceErrors" | "resource_errors" => Ok(GeneratedField::ResourceErrors),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeltaDiscoveryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v3.DeltaDiscoveryResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeltaDiscoveryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut system_version_info__ = None;
                let mut resources__ = None;
                let mut type_url__ = None;
                let mut removed_resources__ = None;
                let mut removed_resource_names__ = None;
                let mut nonce__ = None;
                let mut control_plane__ = None;
                let mut resource_errors__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SystemVersionInfo => {
                            if system_version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("systemVersionInfo"));
                            }
                            system_version_info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Resources => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypeUrl => {
                            if type_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeUrl"));
                            }
                            type_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RemovedResources => {
                            if removed_resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("removedResources"));
                            }
                            removed_resources__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RemovedResourceNames => {
                            if removed_resource_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("removedResourceNames"));
                            }
                            removed_resource_names__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ControlPlane => {
                            if control_plane__.is_some() {
                                return Err(serde::de::Error::duplicate_field("controlPlane"));
                            }
                            control_plane__ = map_.next_value()?;
                        }
                        GeneratedField::ResourceErrors => {
                            if resource_errors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceErrors"));
                            }
                            resource_errors__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeltaDiscoveryResponse {
                    system_version_info: system_version_info__.unwrap_or_default(),
                    resources: resources__.unwrap_or_default(),
                    type_url: type_url__.unwrap_or_default(),
                    removed_resources: removed_resources__.unwrap_or_default(),
                    removed_resource_names: removed_resource_names__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    control_plane: control_plane__,
                    resource_errors: resource_errors__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v3.DeltaDiscoveryResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DiscoveryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if self.node.is_some() {
            len += 1;
        }
        if !self.resource_names.is_empty() {
            len += 1;
        }
        if !self.resource_locators.is_empty() {
            len += 1;
        }
        if !self.type_url.is_empty() {
            len += 1;
        }
        if !self.response_nonce.is_empty() {
            len += 1;
        }
        if self.error_detail.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v3.DiscoveryRequest", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("version_info", &self.version_info)?;
        }
        if let Some(v) = self.node.as_ref() {
            struct_ser.serialize_field("node", v)?;
        }
        if !self.resource_names.is_empty() {
            struct_ser.serialize_field("resource_names", &self.resource_names)?;
        }
        if !self.resource_locators.is_empty() {
            struct_ser.serialize_field("resource_locators", &self.resource_locators)?;
        }
        if !self.type_url.is_empty() {
            struct_ser.serialize_field("type_url", &self.type_url)?;
        }
        if !self.response_nonce.is_empty() {
            struct_ser.serialize_field("response_nonce", &self.response_nonce)?;
        }
        if let Some(v) = self.error_detail.as_ref() {
            struct_ser.serialize_field("error_detail", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DiscoveryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "node",
            "resource_names",
            "resourceNames",
            "resource_locators",
            "resourceLocators",
            "type_url",
            "typeUrl",
            "response_nonce",
            "responseNonce",
            "error_detail",
            "errorDetail",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            Node,
            ResourceNames,
            ResourceLocators,
            TypeUrl,
            ResponseNonce,
            ErrorDetail,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "node" => Ok(GeneratedField::Node),
                            "resourceNames" | "resource_names" => Ok(GeneratedField::ResourceNames),
                            "resourceLocators" | "resource_locators" => Ok(GeneratedField::ResourceLocators),
                            "typeUrl" | "type_url" => Ok(GeneratedField::TypeUrl),
                            "responseNonce" | "response_nonce" => Ok(GeneratedField::ResponseNonce),
                            "errorDetail" | "error_detail" => Ok(GeneratedField::ErrorDetail),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DiscoveryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v3.DiscoveryRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DiscoveryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut node__ = None;
                let mut resource_names__ = None;
                let mut resource_locators__ = None;
                let mut type_url__ = None;
                let mut response_nonce__ = None;
                let mut error_detail__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = map_.next_value()?;
                        }
                        GeneratedField::ResourceNames => {
                            if resource_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceNames"));
                            }
                            resource_names__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceLocators => {
                            if resource_locators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceLocators"));
                            }
                            resource_locators__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypeUrl => {
                            if type_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeUrl"));
                            }
                            type_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResponseNonce => {
                            if response_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseNonce"));
                            }
                            response_nonce__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ErrorDetail => {
                            if error_detail__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorDetail"));
                            }
                            error_detail__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DiscoveryRequest {
                    version_info: version_info__.unwrap_or_default(),
                    node: node__,
                    resource_names: resource_names__.unwrap_or_default(),
                    resource_locators: resource_locators__.unwrap_or_default(),
                    type_url: type_url__.unwrap_or_default(),
                    response_nonce: response_nonce__.unwrap_or_default(),
                    error_detail: error_detail__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v3.DiscoveryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DiscoveryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if !self.resources.is_empty() {
            len += 1;
        }
        if self.canary {
            len += 1;
        }
        if !self.type_url.is_empty() {
            len += 1;
        }
        if !self.nonce.is_empty() {
            len += 1;
        }
        if self.control_plane.is_some() {
            len += 1;
        }
        if !self.resource_errors.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v3.DiscoveryResponse", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("version_info", &self.version_info)?;
        }
        if !self.resources.is_empty() {
            struct_ser.serialize_field("resources", &self.resources)?;
        }
        if self.canary {
            struct_ser.serialize_field("canary", &self.canary)?;
        }
        if !self.type_url.is_empty() {
            struct_ser.serialize_field("type_url", &self.type_url)?;
        }
        if !self.nonce.is_empty() {
            struct_ser.serialize_field("nonce", &self.nonce)?;
        }
        if let Some(v) = self.control_plane.as_ref() {
            struct_ser.serialize_field("control_plane", v)?;
        }
        if !self.resource_errors.is_empty() {
            struct_ser.serialize_field("resource_errors", &self.resource_errors)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DiscoveryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "resources",
            "canary",
            "type_url",
            "typeUrl",
            "nonce",
            "control_plane",
            "controlPlane",
            "resource_errors",
            "resourceErrors",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            Resources,
            Canary,
            TypeUrl,
            Nonce,
            ControlPlane,
            ResourceErrors,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "resources" => Ok(GeneratedField::Resources),
                            "canary" => Ok(GeneratedField::Canary),
                            "typeUrl" | "type_url" => Ok(GeneratedField::TypeUrl),
                            "nonce" => Ok(GeneratedField::Nonce),
                            "controlPlane" | "control_plane" => Ok(GeneratedField::ControlPlane),
                            "resourceErrors" | "resource_errors" => Ok(GeneratedField::ResourceErrors),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DiscoveryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v3.DiscoveryResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DiscoveryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut resources__ = None;
                let mut canary__ = None;
                let mut type_url__ = None;
                let mut nonce__ = None;
                let mut control_plane__ = None;
                let mut resource_errors__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Resources => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Canary => {
                            if canary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canary"));
                            }
                            canary__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypeUrl => {
                            if type_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeUrl"));
                            }
                            type_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ControlPlane => {
                            if control_plane__.is_some() {
                                return Err(serde::de::Error::duplicate_field("controlPlane"));
                            }
                            control_plane__ = map_.next_value()?;
                        }
                        GeneratedField::ResourceErrors => {
                            if resource_errors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceErrors"));
                            }
                            resource_errors__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DiscoveryResponse {
                    version_info: version_info__.unwrap_or_default(),
                    resources: resources__.unwrap_or_default(),
                    canary: canary__.unwrap_or_default(),
                    type_url: type_url__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    control_plane: control_plane__,
                    resource_errors: resource_errors__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v3.DiscoveryResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DynamicParameterConstraints {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v3.DynamicParameterConstraints", len)?;
        if let Some(v) = self.r#type.as_ref() {
            match v {
                dynamic_parameter_constraints::Type::Constraint(v) => {
                    struct_ser.serialize_field("constraint", v)?;
                }
                dynamic_parameter_constraints::Type::OrConstraints(v) => {
                    struct_ser.serialize_field("or_constraints", v)?;
                }
                dynamic_parameter_constraints::Type::AndConstraints(v) => {
                    struct_ser.serialize_field("and_constraints", v)?;
                }
                dynamic_parameter_constraints::Type::NotConstraints(v) => {
                    struct_ser.serialize_field("not_constraints", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DynamicParameterConstraints {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "constraint",
            "or_constraints",
            "orConstraints",
            "and_constraints",
            "andConstraints",
            "not_constraints",
            "notConstraints",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Constraint,
            OrConstraints,
            AndConstraints,
            NotConstraints,
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
                            "constraint" => Ok(GeneratedField::Constraint),
                            "orConstraints" | "or_constraints" => Ok(GeneratedField::OrConstraints),
                            "andConstraints" | "and_constraints" => Ok(GeneratedField::AndConstraints),
                            "notConstraints" | "not_constraints" => Ok(GeneratedField::NotConstraints),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DynamicParameterConstraints;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v3.DynamicParameterConstraints")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DynamicParameterConstraints, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Constraint => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("constraint"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(dynamic_parameter_constraints::Type::Constraint)
;
                        }
                        GeneratedField::OrConstraints => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orConstraints"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(dynamic_parameter_constraints::Type::OrConstraints)
;
                        }
                        GeneratedField::AndConstraints => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("andConstraints"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(dynamic_parameter_constraints::Type::AndConstraints)
;
                        }
                        GeneratedField::NotConstraints => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notConstraints"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(dynamic_parameter_constraints::Type::NotConstraints)
;
                        }
                    }
                }
                Ok(DynamicParameterConstraints {
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v3.DynamicParameterConstraints", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for dynamic_parameter_constraints::ConstraintList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.constraints.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v3.DynamicParameterConstraints.ConstraintList", len)?;
        if !self.constraints.is_empty() {
            struct_ser.serialize_field("constraints", &self.constraints)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for dynamic_parameter_constraints::ConstraintList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "constraints",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Constraints,
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
                            "constraints" => Ok(GeneratedField::Constraints),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = dynamic_parameter_constraints::ConstraintList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v3.DynamicParameterConstraints.ConstraintList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<dynamic_parameter_constraints::ConstraintList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut constraints__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Constraints => {
                            if constraints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("constraints"));
                            }
                            constraints__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(dynamic_parameter_constraints::ConstraintList {
                    constraints: constraints__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v3.DynamicParameterConstraints.ConstraintList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for dynamic_parameter_constraints::SingleConstraint {
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
        if self.constraint_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v3.DynamicParameterConstraints.SingleConstraint", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if let Some(v) = self.constraint_type.as_ref() {
            match v {
                dynamic_parameter_constraints::single_constraint::ConstraintType::Value(v) => {
                    struct_ser.serialize_field("value", v)?;
                }
                dynamic_parameter_constraints::single_constraint::ConstraintType::Exists(v) => {
                    struct_ser.serialize_field("exists", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for dynamic_parameter_constraints::SingleConstraint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
            "exists",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
            Exists,
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
                            "exists" => Ok(GeneratedField::Exists),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = dynamic_parameter_constraints::SingleConstraint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v3.DynamicParameterConstraints.SingleConstraint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<dynamic_parameter_constraints::SingleConstraint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut constraint_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if constraint_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            constraint_type__ = map_.next_value::<::std::option::Option<_>>()?.map(dynamic_parameter_constraints::single_constraint::ConstraintType::Value);
                        }
                        GeneratedField::Exists => {
                            if constraint_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exists"));
                            }
                            constraint_type__ = map_.next_value::<::std::option::Option<_>>()?.map(dynamic_parameter_constraints::single_constraint::ConstraintType::Exists)
;
                        }
                    }
                }
                Ok(dynamic_parameter_constraints::SingleConstraint {
                    key: key__.unwrap_or_default(),
                    constraint_type: constraint_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v3.DynamicParameterConstraints.SingleConstraint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for dynamic_parameter_constraints::single_constraint::Exists {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.service.discovery.v3.DynamicParameterConstraints.SingleConstraint.Exists", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for dynamic_parameter_constraints::single_constraint::Exists {
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
            type Value = dynamic_parameter_constraints::single_constraint::Exists;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v3.DynamicParameterConstraints.SingleConstraint.Exists")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<dynamic_parameter_constraints::single_constraint::Exists, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(dynamic_parameter_constraints::single_constraint::Exists {
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v3.DynamicParameterConstraints.SingleConstraint.Exists", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Resource {
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
        if self.resource_name.is_some() {
            len += 1;
        }
        if !self.aliases.is_empty() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        if self.ttl.is_some() {
            len += 1;
        }
        if self.cache_control.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v3.Resource", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.resource_name.as_ref() {
            struct_ser.serialize_field("resource_name", v)?;
        }
        if !self.aliases.is_empty() {
            struct_ser.serialize_field("aliases", &self.aliases)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if let Some(v) = self.ttl.as_ref() {
            struct_ser.serialize_field("ttl", v)?;
        }
        if let Some(v) = self.cache_control.as_ref() {
            struct_ser.serialize_field("cache_control", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Resource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "resource_name",
            "resourceName",
            "aliases",
            "version",
            "resource",
            "ttl",
            "cache_control",
            "cacheControl",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ResourceName,
            Aliases,
            Version,
            Resource,
            Ttl,
            CacheControl,
            Metadata,
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
                            "resourceName" | "resource_name" => Ok(GeneratedField::ResourceName),
                            "aliases" => Ok(GeneratedField::Aliases),
                            "version" => Ok(GeneratedField::Version),
                            "resource" => Ok(GeneratedField::Resource),
                            "ttl" => Ok(GeneratedField::Ttl),
                            "cacheControl" | "cache_control" => Ok(GeneratedField::CacheControl),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Resource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v3.Resource")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Resource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut resource_name__ = None;
                let mut aliases__ = None;
                let mut version__ = None;
                let mut resource__ = None;
                let mut ttl__ = None;
                let mut cache_control__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceName => {
                            if resource_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceName"));
                            }
                            resource_name__ = map_.next_value()?;
                        }
                        GeneratedField::Aliases => {
                            if aliases__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aliases"));
                            }
                            aliases__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map_.next_value()?;
                        }
                        GeneratedField::Ttl => {
                            if ttl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ttl"));
                            }
                            ttl__ = map_.next_value()?;
                        }
                        GeneratedField::CacheControl => {
                            if cache_control__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheControl"));
                            }
                            cache_control__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Resource {
                    name: name__.unwrap_or_default(),
                    resource_name: resource_name__,
                    aliases: aliases__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    resource: resource__,
                    ttl: ttl__,
                    cache_control: cache_control__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v3.Resource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for resource::CacheControl {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.do_not_cache {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v3.Resource.CacheControl", len)?;
        if self.do_not_cache {
            struct_ser.serialize_field("do_not_cache", &self.do_not_cache)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for resource::CacheControl {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "do_not_cache",
            "doNotCache",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DoNotCache,
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
                            "doNotCache" | "do_not_cache" => Ok(GeneratedField::DoNotCache),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = resource::CacheControl;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v3.Resource.CacheControl")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<resource::CacheControl, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut do_not_cache__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DoNotCache => {
                            if do_not_cache__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doNotCache"));
                            }
                            do_not_cache__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(resource::CacheControl {
                    do_not_cache: do_not_cache__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v3.Resource.CacheControl", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_name.is_some() {
            len += 1;
        }
        if self.error_detail.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v3.ResourceError", len)?;
        if let Some(v) = self.resource_name.as_ref() {
            struct_ser.serialize_field("resource_name", v)?;
        }
        if let Some(v) = self.error_detail.as_ref() {
            struct_ser.serialize_field("error_detail", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_name",
            "resourceName",
            "error_detail",
            "errorDetail",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceName,
            ErrorDetail,
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
                            "resourceName" | "resource_name" => Ok(GeneratedField::ResourceName),
                            "errorDetail" | "error_detail" => Ok(GeneratedField::ErrorDetail),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v3.ResourceError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_name__ = None;
                let mut error_detail__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceName => {
                            if resource_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceName"));
                            }
                            resource_name__ = map_.next_value()?;
                        }
                        GeneratedField::ErrorDetail => {
                            if error_detail__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorDetail"));
                            }
                            error_detail__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ResourceError {
                    resource_name: resource_name__,
                    error_detail: error_detail__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v3.ResourceError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceLocator {
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
        if !self.dynamic_parameters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v3.ResourceLocator", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.dynamic_parameters.is_empty() {
            struct_ser.serialize_field("dynamic_parameters", &self.dynamic_parameters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceLocator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "dynamic_parameters",
            "dynamicParameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            DynamicParameters,
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
                            "dynamicParameters" | "dynamic_parameters" => Ok(GeneratedField::DynamicParameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceLocator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v3.ResourceLocator")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceLocator, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut dynamic_parameters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DynamicParameters => {
                            if dynamic_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicParameters"));
                            }
                            dynamic_parameters__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(ResourceLocator {
                    name: name__.unwrap_or_default(),
                    dynamic_parameters: dynamic_parameters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v3.ResourceLocator", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceName {
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
        if self.dynamic_parameter_constraints.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v3.ResourceName", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.dynamic_parameter_constraints.as_ref() {
            struct_ser.serialize_field("dynamic_parameter_constraints", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "dynamic_parameter_constraints",
            "dynamicParameterConstraints",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            DynamicParameterConstraints,
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
                            "dynamicParameterConstraints" | "dynamic_parameter_constraints" => Ok(GeneratedField::DynamicParameterConstraints),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceName;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v3.ResourceName")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceName, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut dynamic_parameter_constraints__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DynamicParameterConstraints => {
                            if dynamic_parameter_constraints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicParameterConstraints"));
                            }
                            dynamic_parameter_constraints__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ResourceName {
                    name: name__.unwrap_or_default(),
                    dynamic_parameter_constraints: dynamic_parameter_constraints__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v3.ResourceName", FIELDS, GeneratedVisitor)
    }
}
