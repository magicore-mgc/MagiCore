//! Shared package-file index exports and responsibility owners.
//! Xuất API chỉ mục file package dùng chung và khai báo các owner.

mod error;
mod index;
mod persistence;
mod prune;
mod rebuild;
mod types;
mod upsert;

pub use index::StoreIndex;
pub use prune::file_has_no_external_hardlinks;
pub use types::FileEntry;
