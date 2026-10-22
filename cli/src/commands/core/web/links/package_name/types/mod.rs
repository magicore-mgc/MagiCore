//! Web package-link name type facade.
//! Facade type tên target link package Web.

#[path = "types.rs"]
mod implementation;

pub(in crate::commands::core::web::links) use implementation::PackageLinkName;
