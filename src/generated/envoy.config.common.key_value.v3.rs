// This file is @generated by prost-build.
/// This shared configuration for Envoy key value stores.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValueStoreConfig {
    /// \[#extension-category: envoy.common.key_value\]
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<
        super::super::super::core::v3::TypedExtensionConfig,
    >,
}
impl ::prost::Name for KeyValueStoreConfig {
    const NAME: &'static str = "KeyValueStoreConfig";
    const PACKAGE: &'static str = "envoy.config.common.key_value.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.config.common.key_value.v3.KeyValueStoreConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.config.common.key_value.v3.KeyValueStoreConfig".into()
    }
}