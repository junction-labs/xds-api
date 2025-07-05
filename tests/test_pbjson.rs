#[cfg(all(test, feature = "pbjson", not(feature = "generate_only")))]
mod pbjson {

    use serde_json::json;
    use xds_api::pb::{
        envoy::{
            config::{
                cluster::v3 as xds_cluster, listener::v3 as xds_listener, route::v3 as xds_route,
            },
            extensions::filters::network::http_connection_manager::v3::{
                http_connection_manager::RouteSpecifier, HttpConnectionManager,
            },
        },
        google::protobuf,
    };

    macro_rules! serialize_test {
        ($name:ident, $value:expr) => {
            #[test]
            fn $name() {
                let value = $value;
                assert!(serde_json::to_vec(&value).is_ok());
            }
        };
    }

    serialize_test!(
        cluster,
        xds_cluster::Cluster {
            name: "some name".to_string(),
            ..Default::default()
        }
    );

    serialize_test!(
        listener,
        xds_listener::Listener {
            name: "some listener".to_string(),
            ..Default::default()
        }
    );

    serialize_test!(
        http_connection_manager,
        HttpConnectionManager {
            route_specifier: Some(RouteSpecifier::RouteConfig(xds_route::RouteConfiguration {
                name: "test_route".to_string(),
                ..Default::default()
            })),
            ..Default::default()
        }
    );

    #[test]
    fn listener_http_conn_manager() {
        let connection_manager = HttpConnectionManager {
            route_specifier: Some(RouteSpecifier::RouteConfig(xds_route::RouteConfiguration {
                name: "test_route".to_string(),
                ..Default::default()
            })),
            ..Default::default()
        };

        let listener = xds_listener::Listener {
            name: "cool_listener".to_string(),
            api_listener: Some(xds_listener::ApiListener {
                api_listener: Some(protobuf::Any::from_msg(&connection_manager).unwrap()),
            }),
            ..Default::default()
        };

        let listener_json = serde_json::to_value(&listener).unwrap();

        assert_json_eq(
            &json!({
                "name": "cool_listener",
                "api_listener": {
                    "api_listener": {
                        "@type": "type.googleapis.com/envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager",
                        "route_config": {
                            "name": "test_route",
                        }
                    }
                }
            }),
            &listener_json,
        );
    }

    #[test]
    fn any_not_well_known() {
        let cors_policy = xds_route::CorsPolicy {
            allow_methods: "HEAD,GET,PUT,POST".to_string(),
            ..Default::default()
        };
        let as_any = protobuf::Any::from_msg(&cors_policy).unwrap();

        assert_json_eq(
            &json!({
                "@type": "type.googleapis.com/envoy.config.route.v3.CorsPolicy",
                "value": &as_any.value,
            }),
            &serde_json::to_value(&as_any).unwrap(),
        )
    }

    #[test]
    fn decode_any() {
        let cors_policy = xds_route::CorsPolicy {
            allow_methods: "HEAD,GET,PUT,POST".to_string(),
            ..Default::default()
        };
        let as_any = protobuf::Any::from_msg(&cors_policy).unwrap();

        assert_eq!(
            serde_json::from_value::<protobuf::Any>(json!({
                "@type": &as_any.type_url,
                "value": &as_any.value,
            }))
            .unwrap(),
            as_any,
        );
    }

    #[inline]
    fn assert_json_eq(expected: &serde_json::Value, actual: &serde_json::Value) {
        assert_eq!(
            expected,
            actual,
            "json values are not equal:\n expected: {}\nactual: {}",
            serde_json::to_string_pretty(expected).unwrap(),
            serde_json::to_string_pretty(actual).unwrap(),
        )
    }
}
