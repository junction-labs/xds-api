use crate::generated::google::protobuf;

macro_rules! wrapper_type {
    ($proto_ty:ty, $primitive_ty:ty) => {
        impl From<$primitive_ty> for $proto_ty {
            fn from(value: $primitive_ty) -> Self {
                Self { value }
            }
        }

        impl From<$proto_ty> for $primitive_ty {
            fn from(value: $proto_ty) -> Self {
                value.value
            }
        }
    };
}

wrapper_type!(protobuf::UInt64Value, u64);
wrapper_type!(protobuf::UInt32Value, u32);
wrapper_type!(protobuf::Int32Value, i32);
wrapper_type!(protobuf::Int64Value, i64);
wrapper_type!(protobuf::FloatValue, f32);
wrapper_type!(protobuf::DoubleValue, f64);
wrapper_type!(protobuf::BytesValue, Vec<u8>);
wrapper_type!(protobuf::BoolValue, bool);
wrapper_type!(protobuf::StringValue, String);

impl TryFrom<std::time::Duration> for protobuf::Duration {
    type Error = std::num::TryFromIntError;

    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        let seconds: i64 = value.as_secs().try_into()?;
        let nanos: i32 = value.subsec_nanos().try_into()?;
        Ok(protobuf::Duration { seconds, nanos })
    }
}

impl TryFrom<protobuf::Duration> for std::time::Duration {
    type Error = std::num::TryFromIntError;

    fn try_from(value: protobuf::Duration) -> Result<Self, Self::Error> {
        let seconds: u64 = value.seconds.try_into()?;
        let nanos: u32 = value.nanos.try_into()?;
        Ok(std::time::Duration::new(seconds, nanos))
    }
}
