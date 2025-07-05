impl serde::Serialize for Body {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.truncated {
            len += 1;
        }
        if self.body_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.tap.v3.Body", len)?;
        if self.truncated {
            struct_ser.serialize_field("truncated", &self.truncated)?;
        }
        if let Some(v) = self.body_type.as_ref() {
            match v {
                body::BodyType::AsBytes(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("as_bytes", pbjson::private::base64::encode(&v).as_str())?;
                }
                body::BodyType::AsString(v) => {
                    struct_ser.serialize_field("as_string", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Body {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "truncated",
            "as_bytes",
            "asBytes",
            "as_string",
            "asString",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Truncated,
            AsBytes,
            AsString,
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
                            "truncated" => Ok(GeneratedField::Truncated),
                            "asBytes" | "as_bytes" => Ok(GeneratedField::AsBytes),
                            "asString" | "as_string" => Ok(GeneratedField::AsString),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Body;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.tap.v3.Body")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Body, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut truncated__ = None;
                let mut body_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Truncated => {
                            if truncated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("truncated"));
                            }
                            truncated__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AsBytes => {
                            if body_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("asBytes"));
                            }
                            body_type__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| body::BodyType::AsBytes(x.0));
                        }
                        GeneratedField::AsString => {
                            if body_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("asString"));
                            }
                            body_type__ = map_.next_value::<::std::option::Option<_>>()?.map(body::BodyType::AsString);
                        }
                    }
                }
                Ok(Body {
                    truncated: truncated__.unwrap_or_default(),
                    body_type: body_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.tap.v3.Body", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Connection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.local_address.is_some() {
            len += 1;
        }
        if self.remote_address.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.tap.v3.Connection", len)?;
        if let Some(v) = self.local_address.as_ref() {
            struct_ser.serialize_field("local_address", v)?;
        }
        if let Some(v) = self.remote_address.as_ref() {
            struct_ser.serialize_field("remote_address", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Connection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "local_address",
            "localAddress",
            "remote_address",
            "remoteAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LocalAddress,
            RemoteAddress,
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
                            "localAddress" | "local_address" => Ok(GeneratedField::LocalAddress),
                            "remoteAddress" | "remote_address" => Ok(GeneratedField::RemoteAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Connection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.tap.v3.Connection")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Connection, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut local_address__ = None;
                let mut remote_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LocalAddress => {
                            if local_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localAddress"));
                            }
                            local_address__ = map_.next_value()?;
                        }
                        GeneratedField::RemoteAddress => {
                            if remote_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteAddress"));
                            }
                            remote_address__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Connection {
                    local_address: local_address__,
                    remote_address: remote_address__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.tap.v3.Connection", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpBufferedTrace {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request.is_some() {
            len += 1;
        }
        if self.response.is_some() {
            len += 1;
        }
        if self.downstream_connection.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.tap.v3.HttpBufferedTrace", len)?;
        if let Some(v) = self.request.as_ref() {
            struct_ser.serialize_field("request", v)?;
        }
        if let Some(v) = self.response.as_ref() {
            struct_ser.serialize_field("response", v)?;
        }
        if let Some(v) = self.downstream_connection.as_ref() {
            struct_ser.serialize_field("downstream_connection", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpBufferedTrace {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request",
            "response",
            "downstream_connection",
            "downstreamConnection",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Request,
            Response,
            DownstreamConnection,
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
                            "request" => Ok(GeneratedField::Request),
                            "response" => Ok(GeneratedField::Response),
                            "downstreamConnection" | "downstream_connection" => Ok(GeneratedField::DownstreamConnection),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpBufferedTrace;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.tap.v3.HttpBufferedTrace")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpBufferedTrace, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ = None;
                let mut response__ = None;
                let mut downstream_connection__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ = map_.next_value()?;
                        }
                        GeneratedField::Response => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            response__ = map_.next_value()?;
                        }
                        GeneratedField::DownstreamConnection => {
                            if downstream_connection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downstreamConnection"));
                            }
                            downstream_connection__ = map_.next_value()?;
                        }
                    }
                }
                Ok(HttpBufferedTrace {
                    request: request__,
                    response: response__,
                    downstream_connection: downstream_connection__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.tap.v3.HttpBufferedTrace", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http_buffered_trace::Message {
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
        if self.body.is_some() {
            len += 1;
        }
        if !self.trailers.is_empty() {
            len += 1;
        }
        if self.headers_received_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.tap.v3.HttpBufferedTrace.Message", len)?;
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if let Some(v) = self.body.as_ref() {
            struct_ser.serialize_field("body", v)?;
        }
        if !self.trailers.is_empty() {
            struct_ser.serialize_field("trailers", &self.trailers)?;
        }
        if let Some(v) = self.headers_received_time.as_ref() {
            struct_ser.serialize_field("headers_received_time", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http_buffered_trace::Message {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "headers",
            "body",
            "trailers",
            "headers_received_time",
            "headersReceivedTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Headers,
            Body,
            Trailers,
            HeadersReceivedTime,
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
                            "body" => Ok(GeneratedField::Body),
                            "trailers" => Ok(GeneratedField::Trailers),
                            "headersReceivedTime" | "headers_received_time" => Ok(GeneratedField::HeadersReceivedTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_buffered_trace::Message;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.tap.v3.HttpBufferedTrace.Message")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<http_buffered_trace::Message, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut headers__ = None;
                let mut body__ = None;
                let mut trailers__ = None;
                let mut headers_received_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                            body__ = map_.next_value()?;
                        }
                        GeneratedField::Trailers => {
                            if trailers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trailers"));
                            }
                            trailers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HeadersReceivedTime => {
                            if headers_received_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headersReceivedTime"));
                            }
                            headers_received_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(http_buffered_trace::Message {
                    headers: headers__.unwrap_or_default(),
                    body: body__,
                    trailers: trailers__.unwrap_or_default(),
                    headers_received_time: headers_received_time__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.tap.v3.HttpBufferedTrace.Message", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpStreamedTraceSegment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.trace_id != 0 {
            len += 1;
        }
        if self.message_piece.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.tap.v3.HttpStreamedTraceSegment", len)?;
        if self.trace_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("trace_id", ToString::to_string(&self.trace_id).as_str())?;
        }
        if let Some(v) = self.message_piece.as_ref() {
            match v {
                http_streamed_trace_segment::MessagePiece::RequestHeaders(v) => {
                    struct_ser.serialize_field("request_headers", v)?;
                }
                http_streamed_trace_segment::MessagePiece::RequestBodyChunk(v) => {
                    struct_ser.serialize_field("request_body_chunk", v)?;
                }
                http_streamed_trace_segment::MessagePiece::RequestTrailers(v) => {
                    struct_ser.serialize_field("request_trailers", v)?;
                }
                http_streamed_trace_segment::MessagePiece::ResponseHeaders(v) => {
                    struct_ser.serialize_field("response_headers", v)?;
                }
                http_streamed_trace_segment::MessagePiece::ResponseBodyChunk(v) => {
                    struct_ser.serialize_field("response_body_chunk", v)?;
                }
                http_streamed_trace_segment::MessagePiece::ResponseTrailers(v) => {
                    struct_ser.serialize_field("response_trailers", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpStreamedTraceSegment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trace_id",
            "traceId",
            "request_headers",
            "requestHeaders",
            "request_body_chunk",
            "requestBodyChunk",
            "request_trailers",
            "requestTrailers",
            "response_headers",
            "responseHeaders",
            "response_body_chunk",
            "responseBodyChunk",
            "response_trailers",
            "responseTrailers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TraceId,
            RequestHeaders,
            RequestBodyChunk,
            RequestTrailers,
            ResponseHeaders,
            ResponseBodyChunk,
            ResponseTrailers,
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
                            "traceId" | "trace_id" => Ok(GeneratedField::TraceId),
                            "requestHeaders" | "request_headers" => Ok(GeneratedField::RequestHeaders),
                            "requestBodyChunk" | "request_body_chunk" => Ok(GeneratedField::RequestBodyChunk),
                            "requestTrailers" | "request_trailers" => Ok(GeneratedField::RequestTrailers),
                            "responseHeaders" | "response_headers" => Ok(GeneratedField::ResponseHeaders),
                            "responseBodyChunk" | "response_body_chunk" => Ok(GeneratedField::ResponseBodyChunk),
                            "responseTrailers" | "response_trailers" => Ok(GeneratedField::ResponseTrailers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpStreamedTraceSegment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.tap.v3.HttpStreamedTraceSegment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpStreamedTraceSegment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trace_id__ = None;
                let mut message_piece__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TraceId => {
                            if trace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceId"));
                            }
                            trace_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RequestHeaders => {
                            if message_piece__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeaders"));
                            }
                            message_piece__ = map_.next_value::<::std::option::Option<_>>()?.map(http_streamed_trace_segment::MessagePiece::RequestHeaders)
;
                        }
                        GeneratedField::RequestBodyChunk => {
                            if message_piece__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestBodyChunk"));
                            }
                            message_piece__ = map_.next_value::<::std::option::Option<_>>()?.map(http_streamed_trace_segment::MessagePiece::RequestBodyChunk)
;
                        }
                        GeneratedField::RequestTrailers => {
                            if message_piece__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestTrailers"));
                            }
                            message_piece__ = map_.next_value::<::std::option::Option<_>>()?.map(http_streamed_trace_segment::MessagePiece::RequestTrailers)
;
                        }
                        GeneratedField::ResponseHeaders => {
                            if message_piece__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeaders"));
                            }
                            message_piece__ = map_.next_value::<::std::option::Option<_>>()?.map(http_streamed_trace_segment::MessagePiece::ResponseHeaders)
;
                        }
                        GeneratedField::ResponseBodyChunk => {
                            if message_piece__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseBodyChunk"));
                            }
                            message_piece__ = map_.next_value::<::std::option::Option<_>>()?.map(http_streamed_trace_segment::MessagePiece::ResponseBodyChunk)
;
                        }
                        GeneratedField::ResponseTrailers => {
                            if message_piece__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseTrailers"));
                            }
                            message_piece__ = map_.next_value::<::std::option::Option<_>>()?.map(http_streamed_trace_segment::MessagePiece::ResponseTrailers)
;
                        }
                    }
                }
                Ok(HttpStreamedTraceSegment {
                    trace_id: trace_id__.unwrap_or_default(),
                    message_piece: message_piece__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.tap.v3.HttpStreamedTraceSegment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SocketBufferedTrace {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.trace_id != 0 {
            len += 1;
        }
        if self.connection.is_some() {
            len += 1;
        }
        if !self.events.is_empty() {
            len += 1;
        }
        if self.read_truncated {
            len += 1;
        }
        if self.write_truncated {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.tap.v3.SocketBufferedTrace", len)?;
        if self.trace_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("trace_id", ToString::to_string(&self.trace_id).as_str())?;
        }
        if let Some(v) = self.connection.as_ref() {
            struct_ser.serialize_field("connection", v)?;
        }
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        if self.read_truncated {
            struct_ser.serialize_field("read_truncated", &self.read_truncated)?;
        }
        if self.write_truncated {
            struct_ser.serialize_field("write_truncated", &self.write_truncated)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SocketBufferedTrace {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trace_id",
            "traceId",
            "connection",
            "events",
            "read_truncated",
            "readTruncated",
            "write_truncated",
            "writeTruncated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TraceId,
            Connection,
            Events,
            ReadTruncated,
            WriteTruncated,
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
                            "traceId" | "trace_id" => Ok(GeneratedField::TraceId),
                            "connection" => Ok(GeneratedField::Connection),
                            "events" => Ok(GeneratedField::Events),
                            "readTruncated" | "read_truncated" => Ok(GeneratedField::ReadTruncated),
                            "writeTruncated" | "write_truncated" => Ok(GeneratedField::WriteTruncated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SocketBufferedTrace;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.tap.v3.SocketBufferedTrace")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SocketBufferedTrace, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trace_id__ = None;
                let mut connection__ = None;
                let mut events__ = None;
                let mut read_truncated__ = None;
                let mut write_truncated__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TraceId => {
                            if trace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceId"));
                            }
                            trace_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Connection => {
                            if connection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connection"));
                            }
                            connection__ = map_.next_value()?;
                        }
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReadTruncated => {
                            if read_truncated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readTruncated"));
                            }
                            read_truncated__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WriteTruncated => {
                            if write_truncated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("writeTruncated"));
                            }
                            write_truncated__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SocketBufferedTrace {
                    trace_id: trace_id__.unwrap_or_default(),
                    connection: connection__,
                    events: events__.unwrap_or_default(),
                    read_truncated: read_truncated__.unwrap_or_default(),
                    write_truncated: write_truncated__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.tap.v3.SocketBufferedTrace", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SocketEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timestamp.is_some() {
            len += 1;
        }
        if self.connection.is_some() {
            len += 1;
        }
        if self.event_selector.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.tap.v3.SocketEvent", len)?;
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if let Some(v) = self.connection.as_ref() {
            struct_ser.serialize_field("connection", v)?;
        }
        if let Some(v) = self.event_selector.as_ref() {
            match v {
                socket_event::EventSelector::Read(v) => {
                    struct_ser.serialize_field("read", v)?;
                }
                socket_event::EventSelector::Write(v) => {
                    struct_ser.serialize_field("write", v)?;
                }
                socket_event::EventSelector::Closed(v) => {
                    struct_ser.serialize_field("closed", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SocketEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamp",
            "connection",
            "read",
            "write",
            "closed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamp,
            Connection,
            Read,
            Write,
            Closed,
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
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "connection" => Ok(GeneratedField::Connection),
                            "read" => Ok(GeneratedField::Read),
                            "write" => Ok(GeneratedField::Write),
                            "closed" => Ok(GeneratedField::Closed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SocketEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.tap.v3.SocketEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SocketEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamp__ = None;
                let mut connection__ = None;
                let mut event_selector__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::Connection => {
                            if connection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connection"));
                            }
                            connection__ = map_.next_value()?;
                        }
                        GeneratedField::Read => {
                            if event_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("read"));
                            }
                            event_selector__ = map_.next_value::<::std::option::Option<_>>()?.map(socket_event::EventSelector::Read)
;
                        }
                        GeneratedField::Write => {
                            if event_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("write"));
                            }
                            event_selector__ = map_.next_value::<::std::option::Option<_>>()?.map(socket_event::EventSelector::Write)
;
                        }
                        GeneratedField::Closed => {
                            if event_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closed"));
                            }
                            event_selector__ = map_.next_value::<::std::option::Option<_>>()?.map(socket_event::EventSelector::Closed)
;
                        }
                    }
                }
                Ok(SocketEvent {
                    timestamp: timestamp__,
                    connection: connection__,
                    event_selector: event_selector__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.tap.v3.SocketEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for socket_event::Closed {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.data.tap.v3.SocketEvent.Closed", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for socket_event::Closed {
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
            type Value = socket_event::Closed;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.tap.v3.SocketEvent.Closed")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<socket_event::Closed, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(socket_event::Closed {
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.tap.v3.SocketEvent.Closed", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for socket_event::Read {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.tap.v3.SocketEvent.Read", len)?;
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for socket_event::Read {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
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
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = socket_event::Read;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.tap.v3.SocketEvent.Read")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<socket_event::Read, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map_.next_value()?;
                        }
                    }
                }
                Ok(socket_event::Read {
                    data: data__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.tap.v3.SocketEvent.Read", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for socket_event::Write {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.data.is_some() {
            len += 1;
        }
        if self.end_stream {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.tap.v3.SocketEvent.Write", len)?;
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        if self.end_stream {
            struct_ser.serialize_field("end_stream", &self.end_stream)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for socket_event::Write {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
            "end_stream",
            "endStream",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
            EndStream,
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
                            "data" => Ok(GeneratedField::Data),
                            "endStream" | "end_stream" => Ok(GeneratedField::EndStream),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = socket_event::Write;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.tap.v3.SocketEvent.Write")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<socket_event::Write, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                let mut end_stream__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map_.next_value()?;
                        }
                        GeneratedField::EndStream => {
                            if end_stream__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endStream"));
                            }
                            end_stream__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(socket_event::Write {
                    data: data__,
                    end_stream: end_stream__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.tap.v3.SocketEvent.Write", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SocketEvents {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.events.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.tap.v3.SocketEvents", len)?;
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SocketEvents {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "events",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Events,
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
                            "events" => Ok(GeneratedField::Events),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SocketEvents;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.tap.v3.SocketEvents")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SocketEvents, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut events__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SocketEvents {
                    events: events__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.tap.v3.SocketEvents", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SocketStreamedTraceSegment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.trace_id != 0 {
            len += 1;
        }
        if self.message_piece.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.tap.v3.SocketStreamedTraceSegment", len)?;
        if self.trace_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("trace_id", ToString::to_string(&self.trace_id).as_str())?;
        }
        if let Some(v) = self.message_piece.as_ref() {
            match v {
                socket_streamed_trace_segment::MessagePiece::Connection(v) => {
                    struct_ser.serialize_field("connection", v)?;
                }
                socket_streamed_trace_segment::MessagePiece::Event(v) => {
                    struct_ser.serialize_field("event", v)?;
                }
                socket_streamed_trace_segment::MessagePiece::Events(v) => {
                    struct_ser.serialize_field("events", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SocketStreamedTraceSegment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trace_id",
            "traceId",
            "connection",
            "event",
            "events",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TraceId,
            Connection,
            Event,
            Events,
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
                            "traceId" | "trace_id" => Ok(GeneratedField::TraceId),
                            "connection" => Ok(GeneratedField::Connection),
                            "event" => Ok(GeneratedField::Event),
                            "events" => Ok(GeneratedField::Events),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SocketStreamedTraceSegment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.tap.v3.SocketStreamedTraceSegment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SocketStreamedTraceSegment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trace_id__ = None;
                let mut message_piece__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TraceId => {
                            if trace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceId"));
                            }
                            trace_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Connection => {
                            if message_piece__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connection"));
                            }
                            message_piece__ = map_.next_value::<::std::option::Option<_>>()?.map(socket_streamed_trace_segment::MessagePiece::Connection)
;
                        }
                        GeneratedField::Event => {
                            if message_piece__.is_some() {
                                return Err(serde::de::Error::duplicate_field("event"));
                            }
                            message_piece__ = map_.next_value::<::std::option::Option<_>>()?.map(socket_streamed_trace_segment::MessagePiece::Event)
;
                        }
                        GeneratedField::Events => {
                            if message_piece__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            message_piece__ = map_.next_value::<::std::option::Option<_>>()?.map(socket_streamed_trace_segment::MessagePiece::Events)
;
                        }
                    }
                }
                Ok(SocketStreamedTraceSegment {
                    trace_id: trace_id__.unwrap_or_default(),
                    message_piece: message_piece__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.tap.v3.SocketStreamedTraceSegment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TraceWrapper {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.trace.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.tap.v3.TraceWrapper", len)?;
        if let Some(v) = self.trace.as_ref() {
            match v {
                trace_wrapper::Trace::HttpBufferedTrace(v) => {
                    struct_ser.serialize_field("http_buffered_trace", v)?;
                }
                trace_wrapper::Trace::HttpStreamedTraceSegment(v) => {
                    struct_ser.serialize_field("http_streamed_trace_segment", v)?;
                }
                trace_wrapper::Trace::SocketBufferedTrace(v) => {
                    struct_ser.serialize_field("socket_buffered_trace", v)?;
                }
                trace_wrapper::Trace::SocketStreamedTraceSegment(v) => {
                    struct_ser.serialize_field("socket_streamed_trace_segment", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TraceWrapper {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "http_buffered_trace",
            "httpBufferedTrace",
            "http_streamed_trace_segment",
            "httpStreamedTraceSegment",
            "socket_buffered_trace",
            "socketBufferedTrace",
            "socket_streamed_trace_segment",
            "socketStreamedTraceSegment",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HttpBufferedTrace,
            HttpStreamedTraceSegment,
            SocketBufferedTrace,
            SocketStreamedTraceSegment,
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
                            "httpBufferedTrace" | "http_buffered_trace" => Ok(GeneratedField::HttpBufferedTrace),
                            "httpStreamedTraceSegment" | "http_streamed_trace_segment" => Ok(GeneratedField::HttpStreamedTraceSegment),
                            "socketBufferedTrace" | "socket_buffered_trace" => Ok(GeneratedField::SocketBufferedTrace),
                            "socketStreamedTraceSegment" | "socket_streamed_trace_segment" => Ok(GeneratedField::SocketStreamedTraceSegment),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TraceWrapper;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.tap.v3.TraceWrapper")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TraceWrapper, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trace__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HttpBufferedTrace => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpBufferedTrace"));
                            }
                            trace__ = map_.next_value::<::std::option::Option<_>>()?.map(trace_wrapper::Trace::HttpBufferedTrace)
;
                        }
                        GeneratedField::HttpStreamedTraceSegment => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpStreamedTraceSegment"));
                            }
                            trace__ = map_.next_value::<::std::option::Option<_>>()?.map(trace_wrapper::Trace::HttpStreamedTraceSegment)
;
                        }
                        GeneratedField::SocketBufferedTrace => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("socketBufferedTrace"));
                            }
                            trace__ = map_.next_value::<::std::option::Option<_>>()?.map(trace_wrapper::Trace::SocketBufferedTrace)
;
                        }
                        GeneratedField::SocketStreamedTraceSegment => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("socketStreamedTraceSegment"));
                            }
                            trace__ = map_.next_value::<::std::option::Option<_>>()?.map(trace_wrapper::Trace::SocketStreamedTraceSegment)
;
                        }
                    }
                }
                Ok(TraceWrapper {
                    trace: trace__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.tap.v3.TraceWrapper", FIELDS, GeneratedVisitor)
    }
}
