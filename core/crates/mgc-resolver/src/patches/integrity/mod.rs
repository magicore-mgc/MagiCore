//! Patch integrity module exports.
//! Export module xác minh tính toàn vẹn patch.

mod error;
mod integrity;
pub use integrity::verify_patch_integrity;
