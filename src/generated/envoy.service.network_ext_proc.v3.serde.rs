impl serde::Serialize for Data {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        if self.end_of_stream {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.network_ext_proc.v3.Data", len)?;
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        if self.end_of_stream {
            struct_ser.serialize_field("end_of_stream", &self.end_of_stream)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Data {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
            "end_of_stream",
            "endOfStream",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
            EndOfStream,
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
                            "endOfStream" | "end_of_stream" => Ok(GeneratedField::EndOfStream),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Data;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.network_ext_proc.v3.Data")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Data, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                let mut end_of_stream__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EndOfStream => {
                            if end_of_stream__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endOfStream"));
                            }
                            end_of_stream__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Data {
                    data: data__.unwrap_or_default(),
                    end_of_stream: end_of_stream__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.network_ext_proc.v3.Data", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProcessingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.read_data.is_some() {
            len += 1;
        }
        if self.write_data.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.network_ext_proc.v3.ProcessingRequest", len)?;
        if let Some(v) = self.read_data.as_ref() {
            struct_ser.serialize_field("read_data", v)?;
        }
        if let Some(v) = self.write_data.as_ref() {
            struct_ser.serialize_field("write_data", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProcessingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "read_data",
            "readData",
            "write_data",
            "writeData",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReadData,
            WriteData,
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
                            "readData" | "read_data" => Ok(GeneratedField::ReadData),
                            "writeData" | "write_data" => Ok(GeneratedField::WriteData),
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
            type Value = ProcessingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.network_ext_proc.v3.ProcessingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProcessingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut read_data__ = None;
                let mut write_data__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReadData => {
                            if read_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readData"));
                            }
                            read_data__ = map_.next_value()?;
                        }
                        GeneratedField::WriteData => {
                            if write_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("writeData"));
                            }
                            write_data__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ProcessingRequest {
                    read_data: read_data__,
                    write_data: write_data__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.network_ext_proc.v3.ProcessingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProcessingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.read_data.is_some() {
            len += 1;
        }
        if self.write_data.is_some() {
            len += 1;
        }
        if self.data_processing_status != 0 {
            len += 1;
        }
        if self.connection_status != 0 {
            len += 1;
        }
        if self.dynamic_metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.network_ext_proc.v3.ProcessingResponse", len)?;
        if let Some(v) = self.read_data.as_ref() {
            struct_ser.serialize_field("read_data", v)?;
        }
        if let Some(v) = self.write_data.as_ref() {
            struct_ser.serialize_field("write_data", v)?;
        }
        if self.data_processing_status != 0 {
            let v = processing_response::DataProcessedStatus::try_from(self.data_processing_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.data_processing_status)))?;
            struct_ser.serialize_field("data_processing_status", &v)?;
        }
        if self.connection_status != 0 {
            let v = processing_response::ConnectionStatus::try_from(self.connection_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.connection_status)))?;
            struct_ser.serialize_field("connection_status", &v)?;
        }
        if let Some(v) = self.dynamic_metadata.as_ref() {
            struct_ser.serialize_field("dynamic_metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProcessingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "read_data",
            "readData",
            "write_data",
            "writeData",
            "data_processing_status",
            "dataProcessingStatus",
            "connection_status",
            "connectionStatus",
            "dynamic_metadata",
            "dynamicMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReadData,
            WriteData,
            DataProcessingStatus,
            ConnectionStatus,
            DynamicMetadata,
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
                            "readData" | "read_data" => Ok(GeneratedField::ReadData),
                            "writeData" | "write_data" => Ok(GeneratedField::WriteData),
                            "dataProcessingStatus" | "data_processing_status" => Ok(GeneratedField::DataProcessingStatus),
                            "connectionStatus" | "connection_status" => Ok(GeneratedField::ConnectionStatus),
                            "dynamicMetadata" | "dynamic_metadata" => Ok(GeneratedField::DynamicMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProcessingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.network_ext_proc.v3.ProcessingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProcessingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut read_data__ = None;
                let mut write_data__ = None;
                let mut data_processing_status__ = None;
                let mut connection_status__ = None;
                let mut dynamic_metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReadData => {
                            if read_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readData"));
                            }
                            read_data__ = map_.next_value()?;
                        }
                        GeneratedField::WriteData => {
                            if write_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("writeData"));
                            }
                            write_data__ = map_.next_value()?;
                        }
                        GeneratedField::DataProcessingStatus => {
                            if data_processing_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataProcessingStatus"));
                            }
                            data_processing_status__ = Some(map_.next_value::<processing_response::DataProcessedStatus>()? as i32);
                        }
                        GeneratedField::ConnectionStatus => {
                            if connection_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionStatus"));
                            }
                            connection_status__ = Some(map_.next_value::<processing_response::ConnectionStatus>()? as i32);
                        }
                        GeneratedField::DynamicMetadata => {
                            if dynamic_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicMetadata"));
                            }
                            dynamic_metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ProcessingResponse {
                    read_data: read_data__,
                    write_data: write_data__,
                    data_processing_status: data_processing_status__.unwrap_or_default(),
                    connection_status: connection_status__.unwrap_or_default(),
                    dynamic_metadata: dynamic_metadata__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.network_ext_proc.v3.ProcessingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for processing_response::ConnectionStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Continue => "CONTINUE",
            Self::Close => "CLOSE",
            Self::CloseRst => "CLOSE_RST",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for processing_response::ConnectionStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CONTINUE",
            "CLOSE",
            "CLOSE_RST",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = processing_response::ConnectionStatus;

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
                    "CONTINUE" => Ok(processing_response::ConnectionStatus::Continue),
                    "CLOSE" => Ok(processing_response::ConnectionStatus::Close),
                    "CLOSE_RST" => Ok(processing_response::ConnectionStatus::CloseRst),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for processing_response::DataProcessedStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::Unmodified => "UNMODIFIED",
            Self::Modified => "MODIFIED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for processing_response::DataProcessedStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "UNMODIFIED",
            "MODIFIED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = processing_response::DataProcessedStatus;

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
                    "UNKNOWN" => Ok(processing_response::DataProcessedStatus::Unknown),
                    "UNMODIFIED" => Ok(processing_response::DataProcessedStatus::Unmodified),
                    "MODIFIED" => Ok(processing_response::DataProcessedStatus::Modified),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
