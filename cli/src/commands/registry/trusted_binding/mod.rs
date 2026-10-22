//! Trusted publisher binding behavior and local failures.
//! Hành vi trusted publisher binding và lỗi cục bộ.

mod error;
mod trusted_binding;

pub(super) use trusted_binding::{effective_trusted_binding_package, trust_publisher};
