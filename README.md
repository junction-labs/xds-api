# xds-api

This crate contains generated [Tonic][tonic] bindings for [the xDS gRPC
protocol][xds]. You should depend on this crate if you're interested in
building an xDS client or server or are working with any of Envoy's
configuration types.

[xds]: https://www.envoyproxy.io/docs/envoy/latest/api-docs/xds_protocol
[tonic]: https://crates.io/crates/tonic

## xDS Versions

This crate targets the v3 versions of the xDS APIs. If you are interested
in v2 API support, please reach out or open an issue.

## `WellKnownTypes`

xDS makes heavy use of the protobuf [`Any`][protoany] type. To make working with
`Any` easier, this crate generates its own `Any` implementation and provides
[`WellKnownTypes`] for converting between `Any` and known type URLs.

[protoany]: https://protobuf.dev/reference/protobuf/google.protobuf/#any

## Feature Flags

This crate provides feature flags for:

- `pbjson`: Enables Serde serialization/deserialization for xDS types using the
[pbjson][pbjson] crate, which approximates the [canonical json][protojson]
mapping with Serde APIs. Enabling this flag adds a `Serialize` and `Deserialize`
implementation to all xDS types.

- `descriptor`: Generates and includes an encoded protobuf descriptor for all of
the types in the xDS API. That descriptor can be [registered with
`tonic_reflection`][reflection] to make the xDS APIs visible to a gRPC
Reflection Service.

[pbjson]: https://crates.io/crates/pbjson
[reflection]: https://docs.rs/tonic-reflection/0.12.2/tonic_reflection/server/struct.Builder.html#method.register_encoded_file_descriptor_set
[protojson]: https://protobuf.dev/programming-guides/proto3/#json

## Protoc

Because this crate compiles well-known types on its own, it depends on a pinned
version of `protoc`. The currently pinned version is kept in `.protoc-version`.
