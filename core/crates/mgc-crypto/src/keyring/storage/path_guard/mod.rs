//! Keyring path-policy wiring — Wiring policy path keyring.
//! The implementation owns home containment — Implementation sở hữu containment trong home.

mod path_guard;
pub(in crate::keyring) use path_guard::validate_keyring_path;

#[cfg(test)]
#[path = "test/path_guard.rs"]
mod tests;
