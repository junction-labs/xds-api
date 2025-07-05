#[cfg(all(test, not(feature = "generate_only")))]
mod any {
    use xds_api::pb::{
        envoy::{
            config::{
                cluster::v3 as xds_cluster, listener::v3 as xds_listener, route::v3 as xds_route,
            },
            extensions::clusters::aggregate::v3::ClusterConfig as AggregateClusterConfig,
            extensions::filters::network::http_connection_manager::v3::{
                http_connection_manager::RouteSpecifier, HttpConnectionManager,
            },
        },
        google::protobuf,
    };

    #[test]
    fn roundtrip_cluster() {
        use xds_cluster::cluster::ring_hash_lb_config::HashFunction;

        assert_roundtrip(xds_cluster::Cluster {
            name: "hello".to_string(),
            lb_config: Some(xds_cluster::cluster::LbConfig::RingHashLbConfig(
                xds_cluster::cluster::RingHashLbConfig {
                    hash_function: HashFunction::XxHash.into(),
                    ..Default::default()
                },
            )),
            ..Default::default()
        });
    }

    #[test]
    fn roundtrip_aggregate_cluster() {
        assert_roundtrip(AggregateClusterConfig {
            clusters: vec!["foo".to_string(), "bar".to_string()],
        })
    }

    #[test]
    fn roundtrip_http_connection_manager() {
        assert_roundtrip(HttpConnectionManager {
            route_specifier: Some(RouteSpecifier::RouteConfig(xds_route::RouteConfiguration {
                name: "test_route".to_string(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }

    #[test]
    fn roundtrip_listener() {
        let connection_manager = HttpConnectionManager {
            route_specifier: Some(RouteSpecifier::RouteConfig(xds_route::RouteConfiguration {
                name: "test_route".to_string(),
                ..Default::default()
            })),
            ..Default::default()
        };

        assert_roundtrip(xds_listener::Listener {
            name: "cool_listener".to_string(),
            api_listener: Some(xds_listener::ApiListener {
                api_listener: Some(protobuf::Any::from_msg(&connection_manager).unwrap()),
            }),
            ..Default::default()
        })
    }

    fn assert_roundtrip<M: prost::Name + Clone + Default + PartialEq>(m: M) {
        let initial_val = m.clone();
        let as_any = protobuf::Any::from_msg(&m).unwrap();
        let round_tripped = as_any.to_msg().unwrap();

        assert_eq!(
            initial_val, round_tripped,
            "message should round-trip to itself"
        );
    }
}
