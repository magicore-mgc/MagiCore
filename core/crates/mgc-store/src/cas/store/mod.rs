//! Shared CAS content-store facade.
//! Facade content store CAS dùng chung.

mod error;
mod export;
mod import;
mod store;
mod types;
mod verify;

pub use error::StoreError;
pub use types::{ContentStore, MEMO_CAPACITY, MemoStats, StagingKind};
pub use verify::validate_external_hash;
