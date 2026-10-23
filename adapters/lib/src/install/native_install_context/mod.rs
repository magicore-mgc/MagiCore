//! Wires the native install context owner.
//! Wiring owner native install context.

mod native_install_context;

pub(crate) use native_install_context::content_store;
pub use native_install_context::entry_from_package;
