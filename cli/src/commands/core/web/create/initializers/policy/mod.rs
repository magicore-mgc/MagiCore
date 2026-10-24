//! Initializer policy owner.
//! Owner policy initializer.

#[path = "policy.rs"]
mod implementation;

#[cfg(test)]
pub(in crate::commands::core::web::implementation) use implementation::Refusal;
pub(in crate::commands::core::web::implementation) use implementation::check;
