//! Manifest variant wiring — nối owner biến thể manifest.
//! Separate core-specific manifest condition tables from orchestration.
//! Tách bảng điều kiện manifest theo core khỏi phần điều phối.

mod manifest_variants;

pub(super) use manifest_variants::{ai_manifest_variants, lib_manifest_variants};
