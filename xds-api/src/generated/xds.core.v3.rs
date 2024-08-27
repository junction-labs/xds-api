// This file is @generated by prost-build.
/// Message type for extension configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedExtensionConfig {
    /// The name of an extension. This is not used to select the extension, instead
    /// it serves the role of an opaque identifier.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The typed config for the extension. The type URL will be used to identify
    /// the extension. In the case that the type URL is *xds.type.v3.TypedStruct*
    /// (or, for historical reasons, *udpa.type.v1.TypedStruct*), the inner type
    /// URL of *TypedStruct* will be utilized. See the
    /// :ref:`extension configuration overview
    /// <config_overview_extension_configuration>` for further details.
    #[prost(message, optional, tag = "2")]
    pub typed_config: ::core::option::Option<super::super::super::google::protobuf::Any>,
}
impl ::prost::Name for TypedExtensionConfig {
    const NAME: &'static str = "TypedExtensionConfig";
    const PACKAGE: &'static str = "xds.core.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "xds.core.v3.TypedExtensionConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/xds.core.v3.TypedExtensionConfig".into()
    }
}
/// Additional parameters that can be used to select resource variants. These include any
/// global context parameters, per-resource type client feature capabilities and per-resource
/// type functional attributes. All per-resource type attributes will be `xds.resource.`
/// prefixed and some of these are documented below:
///
/// `xds.resource.listening_address`: The value is "IP:port" (e.g. "10.1.1.3:8080") which is
///    the listening address of a Listener. Used in a Listener resource query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextParams {
    #[prost(map = "string, string", tag = "1")]
    pub params: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
impl ::prost::Name for ContextParams {
    const NAME: &'static str = "ContextParams";
    const PACKAGE: &'static str = "xds.core.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "xds.core.v3.ContextParams".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/xds.core.v3.ContextParams".into()
    }
}
/// xDS authority information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authority {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
impl ::prost::Name for Authority {
    const NAME: &'static str = "Authority";
    const PACKAGE: &'static str = "xds.core.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "xds.core.v3.Authority".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/xds.core.v3.Authority".into()
    }
}
/// xDS resource locators identify a xDS resource name and instruct the
/// data-plane load balancer on how the resource may be located.
///
/// Resource locators have a canonical xdstp:// URI representation:
///
///    xdstp://{authority}/{type_url}/{id}?{context_params}{#directive,*}
///
/// where context_params take the form of URI query parameters.
///
/// Resource locators have a similar canonical http:// URI representation:
///
///    <http://{authority}/{type_url}/{id}?{context_params}{#directive,*}>
///
/// Resource locators also have a simplified file:// URI representation:
///
///    file:///{id}{#directive,*}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceLocator {
    /// URI scheme.
    #[prost(enumeration = "resource_locator::Scheme", tag = "1")]
    pub scheme: i32,
    /// Opaque identifier for the resource. Any '/' will not be escaped during URI
    /// encoding and will form part of the URI path. This may end
    /// with ‘*’ for glob collection references.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// Logical authority for resource (not necessarily transport network address).
    /// Authorities are opaque in the xDS API, data-plane load balancers will map
    /// them to concrete network transports such as an xDS management server, e.g.
    /// via envoy.config.core.v3.ConfigSource.
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
    /// Fully qualified resource type (as in type URL without types.googleapis.com/
    /// prefix).
    #[prost(string, tag = "4")]
    pub resource_type: ::prost::alloc::string::String,
    /// A list of directives that appear in the xDS resource locator #fragment.
    ///
    /// When encoding to URI form, directives are percent encoded with comma
    /// separation.
    #[prost(message, repeated, tag = "6")]
    pub directives: ::prost::alloc::vec::Vec<resource_locator::Directive>,
    #[prost(oneof = "resource_locator::ContextParamSpecifier", tags = "5")]
    pub context_param_specifier: ::core::option::Option<
        resource_locator::ContextParamSpecifier,
    >,
}
/// Nested message and enum types in `ResourceLocator`.
pub mod resource_locator {
    /// Directives provide information to data-plane load balancers on how xDS
    /// resource names are to be interpreted and potentially further resolved. For
    /// example, they may provide alternative resource locators for when primary
    /// resolution fails. Directives are not part of resource names and do not
    /// appear in a xDS transport discovery request.
    ///
    /// When encoding to URIs, directives take the form:
    ///
    /// <directive name>=<string representation of directive value>
    ///
    /// For example, we can have alt=xdstp://foo/bar or entry=some%20thing. Each
    /// directive value type may have its own string encoding, in the case of
    /// ResourceLocator there is a recursive URI encoding.
    ///
    /// Percent encoding applies to the URI encoding of the directive value.
    /// Multiple directives are comma-separated, so the reserved characters that
    /// require percent encoding in a directive value are \[',', '#', '[', '\]',
    /// '%']. These are the RFC3986 fragment reserved characters with the addition
    /// of the xDS scheme specific ','. See
    /// <https://tools.ietf.org/html/rfc3986#page-49> for further details on URI ABNF
    /// and reserved characters.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Directive {
        #[prost(oneof = "directive::Directive", tags = "1, 2")]
        pub directive: ::core::option::Option<directive::Directive>,
    }
    /// Nested message and enum types in `Directive`.
    pub mod directive {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Directive {
            /// An alternative resource locator for fallback if the resource is
            /// unavailable. For example, take the resource locator:
            ///
            ///    xdstp://foo/some-type/some-route-table#alt=xdstp://bar/some-type/another-route-table
            ///
            /// If the data-plane load balancer is unable to reach `foo` to fetch the
            /// resource, it will fallback to `bar`. Alternative resources do not need
            /// to have equivalent content, but they should be functional substitutes.
            #[prost(message, tag = "1")]
            Alt(super::super::ResourceLocator),
            /// List collections support inlining of resources via the entry field in
            /// Resource. These inlined Resource objects may have an optional name
            /// field specified. When specified, the entry directive allows
            /// ResourceLocator to directly reference these inlined resources, e.g.
            /// xdstp://.../foo#entry=bar.
            #[prost(string, tag = "2")]
            Entry(::prost::alloc::string::String),
        }
    }
    impl ::prost::Name for Directive {
        const NAME: &'static str = "Directive";
        const PACKAGE: &'static str = "xds.core.v3";
        fn full_name() -> ::prost::alloc::string::String {
            "xds.core.v3.ResourceLocator.Directive".into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "/xds.core.v3.ResourceLocator.Directive".into()
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Scheme {
        Xdstp = 0,
        Http = 1,
        File = 2,
    }
    impl Scheme {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Scheme::Xdstp => "XDSTP",
                Scheme::Http => "HTTP",
                Scheme::File => "FILE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "XDSTP" => Some(Self::Xdstp),
                "HTTP" => Some(Self::Http),
                "FILE" => Some(Self::File),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ContextParamSpecifier {
        /// Additional parameters that can be used to select resource variants.
        /// Matches must be exact, i.e. all context parameters must match exactly and
        /// there must be no additional context parameters set on the matched
        /// resource.
        #[prost(message, tag = "5")]
        ExactContext(super::ContextParams),
    }
}
impl ::prost::Name for ResourceLocator {
    const NAME: &'static str = "ResourceLocator";
    const PACKAGE: &'static str = "xds.core.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "xds.core.v3.ResourceLocator".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/xds.core.v3.ResourceLocator".into()
    }
}
/// xDS collection resource wrapper. This encapsulates a xDS resource when
/// appearing inside a list collection resource. List collection resources are
/// regular Resource messages of type:
///
/// .. code-block:: proto
///
///    message <T>Collection {
///      repeated CollectionEntry resources = 1;
///    }
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionEntry {
    #[prost(oneof = "collection_entry::ResourceSpecifier", tags = "1, 2")]
    pub resource_specifier: ::core::option::Option<collection_entry::ResourceSpecifier>,
}
/// Nested message and enum types in `CollectionEntry`.
pub mod collection_entry {
    /// Inlined resource entry.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InlineEntry {
        /// Optional name to describe the inlined resource. Resource names must match
        /// ``\[a-zA-Z0-9_-\./\]+`` (TODO(htuch): turn this into a PGV constraint once
        /// finalized, probably should be a RFC3986 pchar). This name allows
        /// reference via the #entry directive in ResourceLocator.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The resource's logical version. It is illegal to have the same named xDS
        /// resource name at a given version with different resource payloads.
        #[prost(string, tag = "2")]
        pub version: ::prost::alloc::string::String,
        /// The resource payload, including type URL.
        #[prost(message, optional, tag = "3")]
        pub resource: ::core::option::Option<
            super::super::super::super::google::protobuf::Any,
        >,
    }
    impl ::prost::Name for InlineEntry {
        const NAME: &'static str = "InlineEntry";
        const PACKAGE: &'static str = "xds.core.v3";
        fn full_name() -> ::prost::alloc::string::String {
            "xds.core.v3.CollectionEntry.InlineEntry".into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "/xds.core.v3.CollectionEntry.InlineEntry".into()
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResourceSpecifier {
        /// A resource locator describing how the member resource is to be located.
        #[prost(message, tag = "1")]
        Locator(super::ResourceLocator),
        /// The resource is inlined in the list collection.
        #[prost(message, tag = "2")]
        InlineEntry(InlineEntry),
    }
}
impl ::prost::Name for CollectionEntry {
    const NAME: &'static str = "CollectionEntry";
    const PACKAGE: &'static str = "xds.core.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "xds.core.v3.CollectionEntry".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/xds.core.v3.CollectionEntry".into()
    }
}
