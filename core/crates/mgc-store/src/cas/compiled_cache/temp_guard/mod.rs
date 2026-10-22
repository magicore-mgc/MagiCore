//! Temporary-file cleanup wiring.
//! Wiring dọn file tạm.

mod temp_guard;
pub(in crate::cas::compiled_cache) use temp_guard::TmpGuard;
