//! Output signing owner.
//! Owner ký output.

#[path = "sign.rs"]
pub(crate) mod implementation;
mod key_source;

pub(crate) use implementation::{Signed, sign_artifact};
#[cfg(test)]
#[allow(unused_imports)] // the signing tests read the keyring surface through here
pub(crate) use key_source::{
    LEGACY_KEY_ID, SIGNING_KEYRING_ENV, resolve_signing_key, trust_keyring_path,
};
