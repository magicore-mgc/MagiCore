//! Wires the Web flag-policy implementation and its nearest tests.
//! Nối implementation policy flag Web và test gần nhất.

#[path = "flag_policy.rs"]
mod implementation;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

pub(in crate::commands::core::web::implementation) use self::implementation::{
    apply_preset, validate_flags,
};
