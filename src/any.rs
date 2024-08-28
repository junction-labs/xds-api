use crate::generated::envoy::{
    config::{
        cluster::v3 as xds_cluster, endpoint::v3 as xds_endpoint, listener::v3 as xds_listener,
        route::v3 as xds_route,
    },
    extensions::filters::network::http_connection_manager::v3 as xds_http,
    extensions::transport_sockets::tls::v3 as xds_tls,
    service::runtime::v3 as xds_runtime,
};
use crate::generated::google::protobuf;

macro_rules! well_known_types {
    ($(#[$id_attr:meta])* pub enum $id_name:ident {  $($(#[$variant_attr:meta])* $variant:ident => $xds_type:ty),* $(,)* }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[derive(enum_map::Enum)]
        $(#[$id_attr])*
        pub enum $id_name {
            $(
                $(#[$variant_attr])*
                $variant,
            )*
        }

        impl $id_name {
            pub fn all() -> &'static [$id_name] {
                &[
                    $(
                        $id_name::$variant,
                    )*
                ]
            }

            pub fn from_type_url(type_url: &str) -> Option<Self> {
                use prost::Name;

                static FROM_TYPE_URL: once_cell::sync::Lazy<Box<[(String, $id_name)]>> = once_cell::sync::Lazy::new(|| {
                    let urls = vec![
                        $(
                            (<$xds_type>::type_url(), $id_name::$variant),
                        )*
                    ];
                    urls.into_boxed_slice()
                });

                FROM_TYPE_URL.iter().find(|(k, _)| k == type_url).map(|(_, v)| *v)
            }

            pub fn type_url(&self) -> &'static str {
                use prost::Name;

                static TO_TYPE_URL: once_cell::sync::Lazy<enum_map::EnumMap<$id_name, String>> = once_cell::sync::Lazy::new(|| {
                    enum_map::enum_map! {
                        $(
                            $id_name::$variant => <$xds_type>::type_url(),
                        )*
                    }
                });

                TO_TYPE_URL[*self].as_str()
            }

            #[cfg(feature = "pbjson")]
            fn decode(&self, bs: &[u8]) -> Result<JsonAny, prost::DecodeError> {
                match self {
                    $(
                        $id_name::$variant => Ok(JsonAny::$variant(prost::Message::decode(bs)?)),
                    )*
                }
            }
        }

        #[cfg(feature = "pbjson")]
        #[derive(Debug)]
        enum JsonAny {
            $(
                $variant($xds_type),
            )*
        }

        #[cfg(feature = "pbjson")]
        impl serde::Serialize for JsonAny {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                #[derive(serde::Serialize)]
                struct AnyWrapper<'a, T> {
                    #[serde(rename = "@type")]
                    type_url: &'static str,

                    #[serde(flatten)]
                    value: &'a T,
                }

                match self {
                    $(
                        JsonAny::$variant(ref inner) => {
                            let type_url = $id_name::$variant.type_url();
                            let wrapped = AnyWrapper {
                                type_url,
                                value: inner,
                            };
                            wrapped.serialize(serializer)
                        }
                    )*
                }
            }
        }
    };
}

well_known_types! {
    /// Well known types that [Any][protobuf::Any] messages may contain while
    /// dealing with XDS `DiscoveryRequest`s and `DiscoveryResponse`s.
    ///
    /// This type currently suppors the top-level types for xDS services and the
    /// `HttpConnectionManager` type that most applications will need to work
    /// with while managing a `Listener`. It doesn't include support for
    /// extension types or typed configs.
    ///
    /// ```no_run
    /// match WellKnownTypes::from_type_url(&any.type_url) {
    ///     Some(WellKnownTypes::Listener) => {
    ///         do_something_with_listener(Listener::decode(&any.value).unwrap())
    ///     }
    ///     _ => todo!(),
    /// }
    /// ```
    ///
    /// # `pbjson`
    ///
    /// With the `pbjson` feature enabled, [Any][protobuf::Any] messages use
    /// `WellKnownTypes` for canonical json conversion and any type listed in
    /// `WellKnownTypes` will be serialized as a struct instead of as an opaque
    /// blob. For example, an `Any` containing an
    /// [HttpConnectionManager][xds_http::HttpConnectionManager] would serialize
    /// to.
    ///
    /// ```no_run,json
    /// {
    ///     "@type": "type.googleapis.com/envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager",
    ///     "route_config": {
    ///         "name": "test_route"
    ///     }
    /// }
    /// ```
    pub enum WellKnownTypes {
        /// An XDS Listener for LDS.
        Listener => xds_listener::Listener,

        /// An HttpConnectionManager, included in XDS Listeners.
        HttpConnectionManager => xds_http::HttpConnectionManager,

        /// A `RouteConfiguration` type, used in RDS.
        RouteConfiguration => xds_route::RouteConfiguration,

        /// A `ScopedRouteConfiguration`, used in SRDS.
        ScopedRouteConfiguration => xds_route::ScopedRouteConfiguration,

        /// A `VirtualHost`, used in VHDS.
        VirtualHost => xds_route::VirtualHost,

        /// A `Cluster`, used in CDS
        Cluster => xds_cluster::Cluster,

        /// A `ClusterLoadAssignment`, used in EDS.
        ClusterLoadAssignment => xds_endpoint::ClusterLoadAssignment,

        /// A `Secret`, used in SDS.
        Secret => xds_tls::Secret,

        /// A `Runtime`, used in RTDS.
        Runtime => xds_runtime::Runtime,
    }
}

impl protobuf::Any {
    pub fn from_msg<M: prost::Name>(m: &M) -> Result<Self, prost::EncodeError> {
        let type_url = M::type_url();
        let mut value = Vec::new();
        prost::Message::encode(m, &mut value)?;

        Ok(Self { type_url, value })
    }

    pub fn to_msg<M: prost::Name + Default + Sized>(&self) -> Result<M, prost::DecodeError> {
        let expected_url = M::type_url();

        if self.type_url != expected_url {
            return Err(prost::DecodeError::new(format!(
                "unexpected type URL: \"{}\": message url: \"{}\"",
                &self.type_url, &expected_url,
            )));
        }

        M::decode(self.value.as_slice())
    }
}

#[cfg(feature = "pbjson")]
impl serde::Serialize for protobuf::Any {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        match WellKnownTypes::from_type_url(&self.type_url) {
            Some(wkt) => {
                let type_url = wkt.type_url();

                let wk_struct = wkt.decode(&self.value).map_err(|_| {
                    serde::ser::Error::custom(format!(
                        "failed to transcode google.protobuf.Any into {type_url}",
                    ))
                })?;

                wk_struct.serialize(serializer)
            }
            None => {
                let mut struct_ser = serializer.serialize_struct("google.protobuf.Any", 2)?;
                struct_ser.serialize_field("@type", &self.type_url)?;
                struct_ser.serialize_field("value", &self.value)?;
                struct_ser.end()
            }
        }
    }
}

#[cfg(feature = "pbjson")]
impl<'de> serde::Deserialize<'de> for protobuf::Any {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["@type", "type_url", "typeUrl", "value"];

        enum Field {
            TypeUrl,
            Value,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(formatter, "expected one of: {FIELDS:?}")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v {
                            "@type" | "type_url" | "typeUrl" => Ok(Field::TypeUrl),
                            "value" => Ok(Field::Value),
                            _ => Err(serde::de::Error::unknown_field(v, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = protobuf::Any;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("google.protobuf.Any")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut type_url = None;
                let mut value = None;

                while let Some(k) = map.next_key()? {
                    match k {
                        Field::TypeUrl => {
                            if type_url.is_some() {
                                return Err(serde::de::Error::duplicate_field("type_url"));
                            }
                            type_url = Some(map.next_value()?);
                        }
                        Field::Value => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                    }
                }

                Ok(protobuf::Any {
                    type_url: type_url.unwrap_or_default(),
                    value: value.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct("google.protobuf.Any", FIELDS, Visitor)
    }
}
