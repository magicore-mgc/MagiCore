//! CocoaPods request, verification, and local error wiring.
//! Nối request, xác minh và lỗi cục bộ CocoaPods.

mod error;
#[path = "client.rs"]
mod implementation;

pub(super) use error::CocoaPodsClientError;
pub(super) use implementation::fetch_verified_spec;
