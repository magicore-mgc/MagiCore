//! Wire Web audit pin-source selection and its local error owner.
//! Nối owner chọn pin audit Web và error cục bộ.

#[path = "error.rs"]
mod error;
#[path = "pin_source.rs"]
mod pin_source;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
pub(super) use error::PinSourceError;
pub(super) use pin_source::select_audit_pins;
