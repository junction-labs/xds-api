#![doc = include_str!("../README.md")]

mod any;
mod generated;

pub mod pb {
    pub use crate::generated::*;
}

pub use any::WellKnownTypes;

/// A serialized file descriptor set containing the entirety of the XDS API.
///
/// See `prost_types` and the GRPC documentation for information on how to use
/// a descriptor set.
#[cfg(feature = "descriptor")]
pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("xds-descriptors.bin");
