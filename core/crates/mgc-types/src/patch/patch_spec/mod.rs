//! Configuration-time patch specification exports and tests.
//! Export và test cho spec patch ở thời điểm cấu hình.

mod patch_spec;

pub use self::patch_spec::{PatchKind, PatchSpec};

#[cfg(test)]
#[path = "test/patch_spec.rs"]
mod tests;
