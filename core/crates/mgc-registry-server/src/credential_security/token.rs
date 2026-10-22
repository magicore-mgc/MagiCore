//! Domain-separated bearer-token digests; raw client tokens stay unchanged.
//! Digest token tách domain; token client không đổi.

use sha2::{Digest, Sha256};

const PREFIX: &str = "mgc-token-sha256-v1:";

pub(crate) struct TokenDigest(String);

impl TokenDigest {
    pub(crate) fn from_secret(token: &str) -> Self {
        let mut hash = Sha256::new();
        hash.update(b"magicore.registry.bearer.v1\0");
        hash.update(token.as_bytes());
        Self(format!("{PREFIX}{}", hex::encode(hash.finalize())))
    }

    pub(crate) fn valid_stored(value: &str) -> bool {
        value.strip_prefix(PREFIX).is_some_and(|digest| {
            digest.len() == 64
                && digest
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        })
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn into_string(self) -> String {
        self.0
    }
}
