// This file is @generated by prost-build.
/// Default implementation of a local address selector. This implementation is
/// used if :ref:`local_address_selector
/// <envoy_v3_api_field_config.core.v3.BindConfig.local_address_selector>` is not
/// specified.
/// This implementation supports the specification of only one address in
/// :ref:`extra_source_addresses
/// <envoy_v3_api_field_config.core.v3.BindConfig.extra_source_addresses>` which
/// is appended to the address specified in the
/// :ref:`source_address <envoy_v3_api_field_config.core.v3.BindConfig.source_address>`
/// field. The extra address should have a different IP version than the address in the
/// ``source_address`` field. The address which has the same IP
/// version with the target host's address IP version will be used as bind address.
/// If there is no same IP version address found, the address in the ``source_address`` field will
/// be returned.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefaultLocalAddressSelector {}
impl ::prost::Name for DefaultLocalAddressSelector {
    const NAME: &'static str = "DefaultLocalAddressSelector";
    const PACKAGE: &'static str = "envoy.config.upstream.local_address_selector.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.config.upstream.local_address_selector.v3.DefaultLocalAddressSelector"
            .into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.config.upstream.local_address_selector.v3.DefaultLocalAddressSelector"
            .into()
    }
}