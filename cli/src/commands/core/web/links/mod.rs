//! Web package link/unlink facade.
//! Facade link/unlink package Web.

#[path = "error.rs"]
mod error;
#[path = "link/mod.rs"]
mod link;
#[path = "package_input/mod.rs"]
mod package_input;
#[path = "package_name/mod.rs"]
mod package_name;
#[path = "package_source/mod.rs"]
mod package_source;
#[path = "unlink/mod.rs"]
mod unlink;

pub use link::link;
pub use unlink::unlink;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
