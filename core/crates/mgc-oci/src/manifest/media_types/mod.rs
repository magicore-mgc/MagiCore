//! OCI Distribution media type constants.
//! Hằng media type của OCI Distribution.

mod media_types;
pub use media_types::{
    OCI_CONFIG, OCI_EMPTY_JSON, OCI_LAYER_GZIP, OCI_LAYER_ZSTD, OCI_MANIFEST_V1,
};

#[cfg(test)]
mod test;
