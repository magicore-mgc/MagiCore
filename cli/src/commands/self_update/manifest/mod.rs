//! Declare manifest binding and signature owners.
//! Khai báo owner bind manifest và chữ ký.

mod manifest;
mod signature;
pub(in crate::commands) use manifest::verify_manifest;
