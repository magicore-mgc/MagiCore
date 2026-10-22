//! `error.rs` — local error ownership for `mgc-js-service` (S07; R08).
//!
//! Decision (S07, updated by S440): manifest-specific policy and atomic-write
//! failures now have typed variants in `manifest::error`; public boundaries
//! retain the same `MgResult` / `anyhow::Error` signatures and display text.
//! Other owners retain their prior error contracts:
//!   - `registry` surfaces `anyhow::Error` (typed fail-closed rejections,
//!     P0-4 — same as before the move);
//!   - `audit` primitives surface `MgError` for contract violations and
//!     `String` for per-entry schema reasons collected by the caller (same as
//!     before the move).
//!
//! Quyết định (S07, cập nhật tại S440): lỗi policy manifest và atomic-write
//! hiện có variant typed tại `manifest::error`; boundary công khai giữ nguyên
//! chữ ký `MgResult` / `anyhow::Error` và text hiển thị. Các owner còn lại
//! giữ contract lỗi trước đó.
//!
//! Future-work trigger / Điều kiện mở rộng: add typed variants beside the
//! functional owner when it gains a new failure class. The crate-level module
//! remains an ownership index, not a central error enum. Wrapping at core
//! boundaries stays the callers' job; the root diagnostic renderer remains
//! the only formatter (R08).
