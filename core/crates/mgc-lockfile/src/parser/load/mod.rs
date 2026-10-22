//! Wires lockfile loading and legacy/v3 compatibility gates.
//! Nối thao tác nạp lockfile và các cổng tương thích legacy/v3.

mod error;
mod load;

pub(in crate::parser) use self::load::require_legacy_document;
pub use self::load::{ensure_v3_readable_lock, load_lock_document, load_lockfile};
