impl serde::Serialize for AttributeContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source.is_some() {
            len += 1;
        }
        if self.destination.is_some() {
            len += 1;
        }
        if self.request.is_some() {
            len += 1;
        }
        if !self.context_extensions.is_empty() {
            len += 1;
        }
        if self.metadata_context.is_some() {
            len += 1;
        }
        if self.route_metadata_context.is_some() {
            len += 1;
        }
        if self.tls_session.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.auth.v3.AttributeContext", len)?;
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if let Some(v) = self.destination.as_ref() {
            struct_ser.serialize_field("destination", v)?;
        }
        if let Some(v) = self.request.as_ref() {
            struct_ser.serialize_field("request", v)?;
        }
        if !self.context_extensions.is_empty() {
            struct_ser.serialize_field("context_extensions", &self.context_extensions)?;
        }
        if let Some(v) = self.metadata_context.as_ref() {
            struct_ser.serialize_field("metadata_context", v)?;
        }
        if let Some(v) = self.route_metadata_context.as_ref() {
            struct_ser.serialize_field("route_metadata_context", v)?;
        }
        if let Some(v) = self.tls_session.as_ref() {
            struct_ser.serialize_field("tls_session", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttributeContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source",
            "destination",
            "request",
            "context_extensions",
            "contextExtensions",
            "metadata_context",
            "metadataContext",
            "route_metadata_context",
            "routeMetadataContext",
            "tls_session",
            "tlsSession",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Source,
            Destination,
            Request,
            ContextExtensions,
            MetadataContext,
            RouteMetadataContext,
            TlsSession,
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
                            "source" => Ok(GeneratedField::Source),
                            "destination" => Ok(GeneratedField::Destination),
                            "request" => Ok(GeneratedField::Request),
                            "contextExtensions" | "context_extensions" => Ok(GeneratedField::ContextExtensions),
                            "metadataContext" | "metadata_context" => Ok(GeneratedField::MetadataContext),
                            "routeMetadataContext" | "route_metadata_context" => Ok(GeneratedField::RouteMetadataContext),
                            "tlsSession" | "tls_session" => Ok(GeneratedField::TlsSession),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttributeContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.auth.v3.AttributeContext")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AttributeContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source__ = None;
                let mut destination__ = None;
                let mut request__ = None;
                let mut context_extensions__ = None;
                let mut metadata_context__ = None;
                let mut route_metadata_context__ = None;
                let mut tls_session__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = map_.next_value()?;
                        }
                        GeneratedField::Destination => {
                            if destination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destination"));
                            }
                            destination__ = map_.next_value()?;
                        }
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ = map_.next_value()?;
                        }
                        GeneratedField::ContextExtensions => {
                            if context_extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextExtensions"));
                            }
                            context_extensions__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::MetadataContext => {
                            if metadata_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataContext"));
                            }
                            metadata_context__ = map_.next_value()?;
                        }
                        GeneratedField::RouteMetadataContext => {
                            if route_metadata_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeMetadataContext"));
                            }
                            route_metadata_context__ = map_.next_value()?;
                        }
                        GeneratedField::TlsSession => {
                            if tls_session__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsSession"));
                            }
                            tls_session__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AttributeContext {
                    source: source__,
                    destination: destination__,
                    request: request__,
                    context_extensions: context_extensions__.unwrap_or_default(),
                    metadata_context: metadata_context__,
                    route_metadata_context: route_metadata_context__,
                    tls_session: tls_session__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.auth.v3.AttributeContext", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for attribute_context::HttpRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.method.is_empty() {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        if self.header_map.is_some() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.host.is_empty() {
            len += 1;
        }
        if !self.scheme.is_empty() {
            len += 1;
        }
        if !self.query.is_empty() {
            len += 1;
        }
        if !self.fragment.is_empty() {
            len += 1;
        }
        if self.size != 0 {
            len += 1;
        }
        if !self.protocol.is_empty() {
            len += 1;
        }
        if !self.body.is_empty() {
            len += 1;
        }
        if !self.raw_body.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.auth.v3.AttributeContext.HttpRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.method.is_empty() {
            struct_ser.serialize_field("method", &self.method)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if let Some(v) = self.header_map.as_ref() {
            struct_ser.serialize_field("header_map", v)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.host.is_empty() {
            struct_ser.serialize_field("host", &self.host)?;
        }
        if !self.scheme.is_empty() {
            struct_ser.serialize_field("scheme", &self.scheme)?;
        }
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        if !self.fragment.is_empty() {
            struct_ser.serialize_field("fragment", &self.fragment)?;
        }
        if self.size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("size", ToString::to_string(&self.size).as_str())?;
        }
        if !self.protocol.is_empty() {
            struct_ser.serialize_field("protocol", &self.protocol)?;
        }
        if !self.body.is_empty() {
            struct_ser.serialize_field("body", &self.body)?;
        }
        if !self.raw_body.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("raw_body", pbjson::private::base64::encode(&self.raw_body).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for attribute_context::HttpRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "method",
            "headers",
            "header_map",
            "headerMap",
            "path",
            "host",
            "scheme",
            "query",
            "fragment",
            "size",
            "protocol",
            "body",
            "raw_body",
            "rawBody",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Method,
            Headers,
            HeaderMap,
            Path,
            Host,
            Scheme,
            Query,
            Fragment,
            Size,
            Protocol,
            Body,
            RawBody,
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
                            "id" => Ok(GeneratedField::Id),
                            "method" => Ok(GeneratedField::Method),
                            "headers" => Ok(GeneratedField::Headers),
                            "headerMap" | "header_map" => Ok(GeneratedField::HeaderMap),
                            "path" => Ok(GeneratedField::Path),
                            "host" => Ok(GeneratedField::Host),
                            "scheme" => Ok(GeneratedField::Scheme),
                            "query" => Ok(GeneratedField::Query),
                            "fragment" => Ok(GeneratedField::Fragment),
                            "size" => Ok(GeneratedField::Size),
                            "protocol" => Ok(GeneratedField::Protocol),
                            "body" => Ok(GeneratedField::Body),
                            "rawBody" | "raw_body" => Ok(GeneratedField::RawBody),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = attribute_context::HttpRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.auth.v3.AttributeContext.HttpRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<attribute_context::HttpRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut method__ = None;
                let mut headers__ = None;
                let mut header_map__ = None;
                let mut path__ = None;
                let mut host__ = None;
                let mut scheme__ = None;
                let mut query__ = None;
                let mut fragment__ = None;
                let mut size__ = None;
                let mut protocol__ = None;
                let mut body__ = None;
                let mut raw_body__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::HeaderMap => {
                            if header_map__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerMap"));
                            }
                            header_map__ = map_.next_value()?;
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Host => {
                            if host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            host__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scheme => {
                            if scheme__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scheme"));
                            }
                            scheme__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fragment => {
                            if fragment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fragment"));
                            }
                            fragment__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Body => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("body"));
                            }
                            body__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RawBody => {
                            if raw_body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rawBody"));
                            }
                            raw_body__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(attribute_context::HttpRequest {
                    id: id__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                    headers: headers__.unwrap_or_default(),
                    header_map: header_map__,
                    path: path__.unwrap_or_default(),
                    host: host__.unwrap_or_default(),
                    scheme: scheme__.unwrap_or_default(),
                    query: query__.unwrap_or_default(),
                    fragment: fragment__.unwrap_or_default(),
                    size: size__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                    body: body__.unwrap_or_default(),
                    raw_body: raw_body__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.auth.v3.AttributeContext.HttpRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for attribute_context::Peer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.address.is_some() {
            len += 1;
        }
        if !self.service.is_empty() {
            len += 1;
        }
        if !self.labels.is_empty() {
            len += 1;
        }
        if !self.principal.is_empty() {
            len += 1;
        }
        if !self.certificate.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.auth.v3.AttributeContext.Peer", len)?;
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if !self.service.is_empty() {
            struct_ser.serialize_field("service", &self.service)?;
        }
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        if !self.principal.is_empty() {
            struct_ser.serialize_field("principal", &self.principal)?;
        }
        if !self.certificate.is_empty() {
            struct_ser.serialize_field("certificate", &self.certificate)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for attribute_context::Peer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "service",
            "labels",
            "principal",
            "certificate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Service,
            Labels,
            Principal,
            Certificate,
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
                            "address" => Ok(GeneratedField::Address),
                            "service" => Ok(GeneratedField::Service),
                            "labels" => Ok(GeneratedField::Labels),
                            "principal" => Ok(GeneratedField::Principal),
                            "certificate" => Ok(GeneratedField::Certificate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = attribute_context::Peer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.auth.v3.AttributeContext.Peer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<attribute_context::Peer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut service__ = None;
                let mut labels__ = None;
                let mut principal__ = None;
                let mut certificate__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                        GeneratedField::Service => {
                            if service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            service__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Principal => {
                            if principal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principal"));
                            }
                            principal__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Certificate => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("certificate"));
                            }
                            certificate__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(attribute_context::Peer {
                    address: address__,
                    service: service__.unwrap_or_default(),
                    labels: labels__.unwrap_or_default(),
                    principal: principal__.unwrap_or_default(),
                    certificate: certificate__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.auth.v3.AttributeContext.Peer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for attribute_context::Request {
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
        if self.http.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.auth.v3.AttributeContext.Request", len)?;
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if let Some(v) = self.http.as_ref() {
            struct_ser.serialize_field("http", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for attribute_context::Request {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "time",
            "http",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Time,
            Http,
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
                            "http" => Ok(GeneratedField::Http),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = attribute_context::Request;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.auth.v3.AttributeContext.Request")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<attribute_context::Request, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut time__ = None;
                let mut http__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                        GeneratedField::Http => {
                            if http__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http"));
                            }
                            http__ = map_.next_value()?;
                        }
                    }
                }
                Ok(attribute_context::Request {
                    time: time__,
                    http: http__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.auth.v3.AttributeContext.Request", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for attribute_context::TlsSession {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sni.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.auth.v3.AttributeContext.TLSSession", len)?;
        if !self.sni.is_empty() {
            struct_ser.serialize_field("sni", &self.sni)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for attribute_context::TlsSession {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sni",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sni,
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
                            "sni" => Ok(GeneratedField::Sni),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = attribute_context::TlsSession;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.auth.v3.AttributeContext.TLSSession")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<attribute_context::TlsSession, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sni__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sni => {
                            if sni__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sni"));
                            }
                            sni__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(attribute_context::TlsSession {
                    sni: sni__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.auth.v3.AttributeContext.TLSSession", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attributes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.auth.v3.CheckRequest", len)?;
        if let Some(v) = self.attributes.as_ref() {
            struct_ser.serialize_field("attributes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = CheckRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.auth.v3.CheckRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attributes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CheckRequest {
                    attributes: attributes__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.auth.v3.CheckRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status.is_some() {
            len += 1;
        }
        if self.dynamic_metadata.is_some() {
            len += 1;
        }
        if self.http_response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.auth.v3.CheckResponse", len)?;
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if let Some(v) = self.dynamic_metadata.as_ref() {
            struct_ser.serialize_field("dynamic_metadata", v)?;
        }
        if let Some(v) = self.http_response.as_ref() {
            match v {
                check_response::HttpResponse::DeniedResponse(v) => {
                    struct_ser.serialize_field("denied_response", v)?;
                }
                check_response::HttpResponse::OkResponse(v) => {
                    struct_ser.serialize_field("ok_response", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "dynamic_metadata",
            "dynamicMetadata",
            "denied_response",
            "deniedResponse",
            "ok_response",
            "okResponse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            DynamicMetadata,
            DeniedResponse,
            OkResponse,
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
                            "status" => Ok(GeneratedField::Status),
                            "dynamicMetadata" | "dynamic_metadata" => Ok(GeneratedField::DynamicMetadata),
                            "deniedResponse" | "denied_response" => Ok(GeneratedField::DeniedResponse),
                            "okResponse" | "ok_response" => Ok(GeneratedField::OkResponse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.auth.v3.CheckResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut dynamic_metadata__ = None;
                let mut http_response__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                        GeneratedField::DynamicMetadata => {
                            if dynamic_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicMetadata"));
                            }
                            dynamic_metadata__ = map_.next_value()?;
                        }
                        GeneratedField::DeniedResponse => {
                            if http_response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deniedResponse"));
                            }
                            http_response__ = map_.next_value::<::std::option::Option<_>>()?.map(check_response::HttpResponse::DeniedResponse)
;
                        }
                        GeneratedField::OkResponse => {
                            if http_response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("okResponse"));
                            }
                            http_response__ = map_.next_value::<::std::option::Option<_>>()?.map(check_response::HttpResponse::OkResponse)
;
                        }
                    }
                }
                Ok(CheckResponse {
                    status: status__,
                    dynamic_metadata: dynamic_metadata__,
                    http_response: http_response__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.auth.v3.CheckResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeniedHttpResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status.is_some() {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        if !self.body.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.auth.v3.DeniedHttpResponse", len)?;
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if !self.body.is_empty() {
            struct_ser.serialize_field("body", &self.body)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeniedHttpResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "headers",
            "body",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            Headers,
            Body,
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
                            "status" => Ok(GeneratedField::Status),
                            "headers" => Ok(GeneratedField::Headers),
                            "body" => Ok(GeneratedField::Body),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeniedHttpResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.auth.v3.DeniedHttpResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeniedHttpResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut headers__ = None;
                let mut body__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Body => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("body"));
                            }
                            body__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeniedHttpResponse {
                    status: status__,
                    headers: headers__.unwrap_or_default(),
                    body: body__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.auth.v3.DeniedHttpResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OkHttpResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.headers.is_empty() {
            len += 1;
        }
        if !self.headers_to_remove.is_empty() {
            len += 1;
        }
        if self.dynamic_metadata.is_some() {
            len += 1;
        }
        if !self.response_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.query_parameters_to_set.is_empty() {
            len += 1;
        }
        if !self.query_parameters_to_remove.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.auth.v3.OkHttpResponse", len)?;
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if !self.headers_to_remove.is_empty() {
            struct_ser.serialize_field("headers_to_remove", &self.headers_to_remove)?;
        }
        if let Some(v) = self.dynamic_metadata.as_ref() {
            struct_ser.serialize_field("dynamic_metadata", v)?;
        }
        if !self.response_headers_to_add.is_empty() {
            struct_ser.serialize_field("response_headers_to_add", &self.response_headers_to_add)?;
        }
        if !self.query_parameters_to_set.is_empty() {
            struct_ser.serialize_field("query_parameters_to_set", &self.query_parameters_to_set)?;
        }
        if !self.query_parameters_to_remove.is_empty() {
            struct_ser.serialize_field("query_parameters_to_remove", &self.query_parameters_to_remove)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OkHttpResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "headers",
            "headers_to_remove",
            "headersToRemove",
            "dynamic_metadata",
            "dynamicMetadata",
            "response_headers_to_add",
            "responseHeadersToAdd",
            "query_parameters_to_set",
            "queryParametersToSet",
            "query_parameters_to_remove",
            "queryParametersToRemove",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Headers,
            HeadersToRemove,
            DynamicMetadata,
            ResponseHeadersToAdd,
            QueryParametersToSet,
            QueryParametersToRemove,
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
                            "headers" => Ok(GeneratedField::Headers),
                            "headersToRemove" | "headers_to_remove" => Ok(GeneratedField::HeadersToRemove),
                            "dynamicMetadata" | "dynamic_metadata" => Ok(GeneratedField::DynamicMetadata),
                            "responseHeadersToAdd" | "response_headers_to_add" => Ok(GeneratedField::ResponseHeadersToAdd),
                            "queryParametersToSet" | "query_parameters_to_set" => Ok(GeneratedField::QueryParametersToSet),
                            "queryParametersToRemove" | "query_parameters_to_remove" => Ok(GeneratedField::QueryParametersToRemove),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OkHttpResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.auth.v3.OkHttpResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OkHttpResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut headers__ = None;
                let mut headers_to_remove__ = None;
                let mut dynamic_metadata__ = None;
                let mut response_headers_to_add__ = None;
                let mut query_parameters_to_set__ = None;
                let mut query_parameters_to_remove__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HeadersToRemove => {
                            if headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headersToRemove"));
                            }
                            headers_to_remove__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DynamicMetadata => {
                            if dynamic_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicMetadata"));
                            }
                            dynamic_metadata__ = map_.next_value()?;
                        }
                        GeneratedField::ResponseHeadersToAdd => {
                            if response_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToAdd"));
                            }
                            response_headers_to_add__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QueryParametersToSet => {
                            if query_parameters_to_set__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryParametersToSet"));
                            }
                            query_parameters_to_set__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QueryParametersToRemove => {
                            if query_parameters_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryParametersToRemove"));
                            }
                            query_parameters_to_remove__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OkHttpResponse {
                    headers: headers__.unwrap_or_default(),
                    headers_to_remove: headers_to_remove__.unwrap_or_default(),
                    dynamic_metadata: dynamic_metadata__,
                    response_headers_to_add: response_headers_to_add__.unwrap_or_default(),
                    query_parameters_to_set: query_parameters_to_set__.unwrap_or_default(),
                    query_parameters_to_remove: query_parameters_to_remove__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.auth.v3.OkHttpResponse", FIELDS, GeneratedVisitor)
    }
}
