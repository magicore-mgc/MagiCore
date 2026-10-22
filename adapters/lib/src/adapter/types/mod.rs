//! LibAdapter state and internal language/manifest enums.
//! Trạng thái LibAdapter và enum nội bộ cho ngôn ngữ/manifest.

mod types;
pub use types::LibAdapter;
pub(in crate::adapter) use types::{JavaManifestKind, ResolveFirst};
