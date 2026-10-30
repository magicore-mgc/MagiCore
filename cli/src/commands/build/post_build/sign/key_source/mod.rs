//! Signing key source owner wiring.
//! Wiring owner nguồn khoá ký.

#[path = "key_source.rs"]
mod implementation;

pub(crate) use implementation::{KeyResolutionError, SIGNING_KEYRING_ENV, resolve_signing_key};
#[cfg(test)]
#[allow(unused_imports)] // this owner's tests also read the identifier and trust path
pub(crate) use implementation::{LEGACY_KEY_ID, SigningKey, trust_keyring_path};
