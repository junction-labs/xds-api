impl serde::Serialize for Authority {
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
        let mut struct_ser = serializer.serialize_struct("xds.core.v3.Authority", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Authority {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Authority;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.core.v3.Authority")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Authority, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Authority {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xds.core.v3.Authority", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CidrRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address_prefix.is_empty() {
            len += 1;
        }
        if self.prefix_len.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.core.v3.CidrRange", len)?;
        if !self.address_prefix.is_empty() {
            struct_ser.serialize_field("address_prefix", &self.address_prefix)?;
        }
        if let Some(v) = self.prefix_len.as_ref() {
            struct_ser.serialize_field("prefix_len", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CidrRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address_prefix",
            "addressPrefix",
            "prefix_len",
            "prefixLen",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AddressPrefix,
            PrefixLen,
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
                            "addressPrefix" | "address_prefix" => Ok(GeneratedField::AddressPrefix),
                            "prefixLen" | "prefix_len" => Ok(GeneratedField::PrefixLen),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CidrRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.core.v3.CidrRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CidrRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address_prefix__ = None;
                let mut prefix_len__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AddressPrefix => {
                            if address_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressPrefix"));
                            }
                            address_prefix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrefixLen => {
                            if prefix_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefixLen"));
                            }
                            prefix_len__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CidrRange {
                    address_prefix: address_prefix__.unwrap_or_default(),
                    prefix_len: prefix_len__,
                })
            }
        }
        deserializer.deserialize_struct("xds.core.v3.CidrRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CollectionEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.core.v3.CollectionEntry", len)?;
        if let Some(v) = self.resource_specifier.as_ref() {
            match v {
                collection_entry::ResourceSpecifier::Locator(v) => {
                    struct_ser.serialize_field("locator", v)?;
                }
                collection_entry::ResourceSpecifier::InlineEntry(v) => {
                    struct_ser.serialize_field("inline_entry", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CollectionEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "locator",
            "inline_entry",
            "inlineEntry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Locator,
            InlineEntry,
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
                            "locator" => Ok(GeneratedField::Locator),
                            "inlineEntry" | "inline_entry" => Ok(GeneratedField::InlineEntry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CollectionEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.core.v3.CollectionEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CollectionEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Locator => {
                            if resource_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locator"));
                            }
                            resource_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(collection_entry::ResourceSpecifier::Locator)
;
                        }
                        GeneratedField::InlineEntry => {
                            if resource_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inlineEntry"));
                            }
                            resource_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(collection_entry::ResourceSpecifier::InlineEntry)
;
                        }
                    }
                }
                Ok(CollectionEntry {
                    resource_specifier: resource_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("xds.core.v3.CollectionEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for collection_entry::InlineEntry {
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
        if !self.version.is_empty() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.core.v3.CollectionEntry.InlineEntry", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for collection_entry::InlineEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "version",
            "resource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Version,
            Resource,
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
                            "version" => Ok(GeneratedField::Version),
                            "resource" => Ok(GeneratedField::Resource),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = collection_entry::InlineEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.core.v3.CollectionEntry.InlineEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<collection_entry::InlineEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut version__ = None;
                let mut resource__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(collection_entry::InlineEntry {
                    name: name__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    resource: resource__,
                })
            }
        }
        deserializer.deserialize_struct("xds.core.v3.CollectionEntry.InlineEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContextParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.params.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.core.v3.ContextParams", len)?;
        if !self.params.is_empty() {
            struct_ser.serialize_field("params", &self.params)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContextParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "params",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
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
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContextParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.core.v3.ContextParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ContextParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(ContextParams {
                    params: params__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xds.core.v3.ContextParams", FIELDS, GeneratedVisitor)
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
        if self.scheme != 0 {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.resource_type.is_empty() {
            len += 1;
        }
        if !self.directives.is_empty() {
            len += 1;
        }
        if self.context_param_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.core.v3.ResourceLocator", len)?;
        if self.scheme != 0 {
            let v = resource_locator::Scheme::try_from(self.scheme)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.scheme)))?;
            struct_ser.serialize_field("scheme", &v)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.resource_type.is_empty() {
            struct_ser.serialize_field("resource_type", &self.resource_type)?;
        }
        if !self.directives.is_empty() {
            struct_ser.serialize_field("directives", &self.directives)?;
        }
        if let Some(v) = self.context_param_specifier.as_ref() {
            match v {
                resource_locator::ContextParamSpecifier::ExactContext(v) => {
                    struct_ser.serialize_field("exact_context", v)?;
                }
            }
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
            "scheme",
            "id",
            "authority",
            "resource_type",
            "resourceType",
            "directives",
            "exact_context",
            "exactContext",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Scheme,
            Id,
            Authority,
            ResourceType,
            Directives,
            ExactContext,
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
                            "scheme" => Ok(GeneratedField::Scheme),
                            "id" => Ok(GeneratedField::Id),
                            "authority" => Ok(GeneratedField::Authority),
                            "resourceType" | "resource_type" => Ok(GeneratedField::ResourceType),
                            "directives" => Ok(GeneratedField::Directives),
                            "exactContext" | "exact_context" => Ok(GeneratedField::ExactContext),
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
                formatter.write_str("struct xds.core.v3.ResourceLocator")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceLocator, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut scheme__ = None;
                let mut id__ = None;
                let mut authority__ = None;
                let mut resource_type__ = None;
                let mut directives__ = None;
                let mut context_param_specifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Scheme => {
                            if scheme__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scheme"));
                            }
                            scheme__ = Some(map_.next_value::<resource_locator::Scheme>()? as i32);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceType"));
                            }
                            resource_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Directives => {
                            if directives__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directives"));
                            }
                            directives__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExactContext => {
                            if context_param_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exactContext"));
                            }
                            context_param_specifier__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_locator::ContextParamSpecifier::ExactContext)
;
                        }
                    }
                }
                Ok(ResourceLocator {
                    scheme: scheme__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    authority: authority__.unwrap_or_default(),
                    resource_type: resource_type__.unwrap_or_default(),
                    directives: directives__.unwrap_or_default(),
                    context_param_specifier: context_param_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("xds.core.v3.ResourceLocator", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for resource_locator::Directive {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.directive.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.core.v3.ResourceLocator.Directive", len)?;
        if let Some(v) = self.directive.as_ref() {
            match v {
                resource_locator::directive::Directive::Alt(v) => {
                    struct_ser.serialize_field("alt", v)?;
                }
                resource_locator::directive::Directive::Entry(v) => {
                    struct_ser.serialize_field("entry", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for resource_locator::Directive {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "alt",
            "entry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Alt,
            Entry,
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
                            "alt" => Ok(GeneratedField::Alt),
                            "entry" => Ok(GeneratedField::Entry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = resource_locator::Directive;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.core.v3.ResourceLocator.Directive")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<resource_locator::Directive, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut directive__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Alt => {
                            if directive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alt"));
                            }
                            directive__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_locator::directive::Directive::Alt)
;
                        }
                        GeneratedField::Entry => {
                            if directive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entry"));
                            }
                            directive__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_locator::directive::Directive::Entry);
                        }
                    }
                }
                Ok(resource_locator::Directive {
                    directive: directive__,
                })
            }
        }
        deserializer.deserialize_struct("xds.core.v3.ResourceLocator.Directive", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for resource_locator::Scheme {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Xdstp => "XDSTP",
            Self::Http => "HTTP",
            Self::File => "FILE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for resource_locator::Scheme {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "XDSTP",
            "HTTP",
            "FILE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = resource_locator::Scheme;

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
                    "XDSTP" => Ok(resource_locator::Scheme::Xdstp),
                    "HTTP" => Ok(resource_locator::Scheme::Http),
                    "FILE" => Ok(resource_locator::Scheme::File),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TypedExtensionConfig {
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
        if self.typed_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xds.core.v3.TypedExtensionConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.typed_config.as_ref() {
            struct_ser.serialize_field("typed_config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TypedExtensionConfig {
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
            type Value = TypedExtensionConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xds.core.v3.TypedExtensionConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TypedExtensionConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut typed_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypedConfig => {
                            if typed_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            typed_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TypedExtensionConfig {
                    name: name__.unwrap_or_default(),
                    typed_config: typed_config__,
                })
            }
        }
        deserializer.deserialize_struct("xds.core.v3.TypedExtensionConfig", FIELDS, GeneratedVisitor)
    }
}
