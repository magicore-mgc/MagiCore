//! Ownership policy wiring — wiring policy ownership.
//! Module này chỉ khai báo owner con và xuất API; không chứa logic nghiệp vụ.
mod core_ownership;
mod dep_owner;
mod ownership;

pub(super) use core_ownership::owner_for_core;
pub use dep_owner::DepOwner;
pub(super) use ownership::{install_projection, preflight_allows};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
