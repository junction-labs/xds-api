#[cfg(all(test, not(feature = "generate_only")))]
mod value {
    use std::u64;

    use xds_api::pb::google::protobuf;
    use xds_api::pb::google::protobuf::BoolValue;
    use xds_api::pb::google::protobuf::BytesValue;
    use xds_api::pb::google::protobuf::DoubleValue;
    use xds_api::pb::google::protobuf::FloatValue;
    use xds_api::pb::google::protobuf::Int32Value;
    use xds_api::pb::google::protobuf::Int64Value;
    use xds_api::pb::google::protobuf::StringValue;
    use xds_api::pb::google::protobuf::UInt32Value;
    use xds_api::pb::google::protobuf::UInt64Value;

    macro_rules! test_roundtrip {
        ($name:ident, $proto_ty:ident, $std_ty:ty, $std_value:expr) => {
            #[test]
            fn $name() {
                let proto_value: $proto_ty = $proto_ty { value: $std_value };
                let as_proto: $proto_ty = $std_value.into();
                assert_eq!(as_proto, proto_value);

                let as_std: $std_ty = proto_value.into();
                assert_eq!(as_std, $std_value);
            }
        };
    }

    test_roundtrip!(test_u64, UInt64Value, u64, 123u64);
    test_roundtrip!(test_u32, UInt32Value, u32, 123u32);
    test_roundtrip!(test_i64, Int64Value, i64, 123i64);
    test_roundtrip!(test_i32, Int32Value, i32, 123i32);
    test_roundtrip!(test_f32, FloatValue, f32, 1.2345);
    test_roundtrip!(test_f64, DoubleValue, f64, 1.2345);
    test_roundtrip!(test_bytes, BytesValue, Vec<u8>, vec![0, 1, 2, 3]);
    test_roundtrip!(test_bool, BoolValue, bool, true);
    test_roundtrip!(test_string, StringValue, String, "round trip".to_string());

    #[test]
    fn test_duration() {
        let d = std::time::Duration::new(123, 456);
        let d_as_proto: protobuf::Duration = d.try_into().unwrap();

        assert_eq!(
            d_as_proto,
            protobuf::Duration {
                seconds: 123,
                nanos: 456
            }
        );

        let proto_as_std: std::time::Duration = d_as_proto.try_into().unwrap();
        assert_eq!(proto_as_std, d);
    }

    #[test]
    fn test_duration_overflow() {
        let d = std::time::Duration::new(u64::MAX, 0);
        let as_proto: Result<protobuf::Duration, _> = d.try_into();
        assert!(as_proto.is_err());
    }
}
