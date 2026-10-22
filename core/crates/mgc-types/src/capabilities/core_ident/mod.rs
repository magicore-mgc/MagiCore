//! Core identity contract module wiring.
//! Wiring module hợp đồng định danh core.

mod core_ident;
pub use self::core_ident::CoreIdent;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
