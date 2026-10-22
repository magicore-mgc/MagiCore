//! OCI Distribution Spec endpoints (/v2/*)
//! (OCI API: blob upload/download, manifest push/pull)
//! NOTE: repository names may contain slashes, so capture a wildcard and split it manually.
//! (Tên repo có thể chứa `/`, nên bắt wildcard rồi tự tách các phần.)

#[path = "blob_read/mod.rs"]
mod blob_read;
#[path = "blob_upload/mod.rs"]
mod blob_upload;
#[path = "catalog/mod.rs"]
mod catalog;
#[path = "manifest_routes/mod.rs"]
mod manifest_routes;
#[path = "manifest_validation/mod.rs"]
mod manifest_validation;
#[path = "prelude/mod.rs"]
mod prelude;

#[path = "router/mod.rs"]
mod router;

pub use router::routes;
