impl serde::Serialize for StreamAccessLogsMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.identifier.is_some() {
            len += 1;
        }
        if self.log_entries.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.accesslog.v3.StreamAccessLogsMessage", len)?;
        if let Some(v) = self.identifier.as_ref() {
            struct_ser.serialize_field("identifier", v)?;
        }
        if let Some(v) = self.log_entries.as_ref() {
            match v {
                stream_access_logs_message::LogEntries::HttpLogs(v) => {
                    struct_ser.serialize_field("http_logs", v)?;
                }
                stream_access_logs_message::LogEntries::TcpLogs(v) => {
                    struct_ser.serialize_field("tcp_logs", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamAccessLogsMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "identifier",
            "http_logs",
            "httpLogs",
            "tcp_logs",
            "tcpLogs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Identifier,
            HttpLogs,
            TcpLogs,
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
                            "identifier" => Ok(GeneratedField::Identifier),
                            "httpLogs" | "http_logs" => Ok(GeneratedField::HttpLogs),
                            "tcpLogs" | "tcp_logs" => Ok(GeneratedField::TcpLogs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamAccessLogsMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.accesslog.v3.StreamAccessLogsMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamAccessLogsMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identifier__ = None;
                let mut log_entries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Identifier => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            identifier__ = map_.next_value()?;
                        }
                        GeneratedField::HttpLogs => {
                            if log_entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpLogs"));
                            }
                            log_entries__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_access_logs_message::LogEntries::HttpLogs)
;
                        }
                        GeneratedField::TcpLogs => {
                            if log_entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tcpLogs"));
                            }
                            log_entries__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_access_logs_message::LogEntries::TcpLogs)
;
                        }
                    }
                }
                Ok(StreamAccessLogsMessage {
                    identifier: identifier__,
                    log_entries: log_entries__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.accesslog.v3.StreamAccessLogsMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for stream_access_logs_message::HttpAccessLogEntries {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.log_entry.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.accesslog.v3.StreamAccessLogsMessage.HTTPAccessLogEntries", len)?;
        if !self.log_entry.is_empty() {
            struct_ser.serialize_field("log_entry", &self.log_entry)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for stream_access_logs_message::HttpAccessLogEntries {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "log_entry",
            "logEntry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LogEntry,
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
                            "logEntry" | "log_entry" => Ok(GeneratedField::LogEntry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = stream_access_logs_message::HttpAccessLogEntries;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.accesslog.v3.StreamAccessLogsMessage.HTTPAccessLogEntries")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<stream_access_logs_message::HttpAccessLogEntries, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut log_entry__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LogEntry => {
                            if log_entry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logEntry"));
                            }
                            log_entry__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(stream_access_logs_message::HttpAccessLogEntries {
                    log_entry: log_entry__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.accesslog.v3.StreamAccessLogsMessage.HTTPAccessLogEntries", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for stream_access_logs_message::Identifier {
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
        if !self.log_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.accesslog.v3.StreamAccessLogsMessage.Identifier", len)?;
        if let Some(v) = self.node.as_ref() {
            struct_ser.serialize_field("node", v)?;
        }
        if !self.log_name.is_empty() {
            struct_ser.serialize_field("log_name", &self.log_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for stream_access_logs_message::Identifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "node",
            "log_name",
            "logName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Node,
            LogName,
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
                            "logName" | "log_name" => Ok(GeneratedField::LogName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = stream_access_logs_message::Identifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.accesslog.v3.StreamAccessLogsMessage.Identifier")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<stream_access_logs_message::Identifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut node__ = None;
                let mut log_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = map_.next_value()?;
                        }
                        GeneratedField::LogName => {
                            if log_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logName"));
                            }
                            log_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(stream_access_logs_message::Identifier {
                    node: node__,
                    log_name: log_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.accesslog.v3.StreamAccessLogsMessage.Identifier", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for stream_access_logs_message::TcpAccessLogEntries {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.log_entry.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.accesslog.v3.StreamAccessLogsMessage.TCPAccessLogEntries", len)?;
        if !self.log_entry.is_empty() {
            struct_ser.serialize_field("log_entry", &self.log_entry)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for stream_access_logs_message::TcpAccessLogEntries {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "log_entry",
            "logEntry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LogEntry,
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
                            "logEntry" | "log_entry" => Ok(GeneratedField::LogEntry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = stream_access_logs_message::TcpAccessLogEntries;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.accesslog.v3.StreamAccessLogsMessage.TCPAccessLogEntries")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<stream_access_logs_message::TcpAccessLogEntries, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut log_entry__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LogEntry => {
                            if log_entry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logEntry"));
                            }
                            log_entry__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(stream_access_logs_message::TcpAccessLogEntries {
                    log_entry: log_entry__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.accesslog.v3.StreamAccessLogsMessage.TCPAccessLogEntries", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamAccessLogsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.service.accesslog.v3.StreamAccessLogsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamAccessLogsResponse {
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
            type Value = StreamAccessLogsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.accesslog.v3.StreamAccessLogsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamAccessLogsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(StreamAccessLogsResponse {
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.accesslog.v3.StreamAccessLogsResponse", FIELDS, GeneratedVisitor)
    }
}
