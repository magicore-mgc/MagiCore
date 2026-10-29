//! Signed-message and digest functions / Hàm thông điệp ký và digest.

/// Domain-separated message prefix — Tiền tố tách miền ký.
pub const ATTESTATION_MESSAGE_DOMAIN: &str = "mgc-core-attestation/v1";
/// Canonical signed message — Thông điệp ký canonical.
pub fn canonical_message(core: &str, config_digest: &str) -> String {
    format!("{ATTESTATION_MESSAGE_DOMAIN}\n{core}\n{config_digest}")
}

/// Identity fields a core signature binds beyond the config digest.
/// Các trường identity mà chữ ký core ràng buộc thêm ngoài digest config.
///
/// Two cores may ship the same config but must never accept each other's
/// signature: the tool version and ecosystem list make the signed statement
/// specific to one core identity (P-8.3/P-8.4).
/// (Hai core có thể có cùng config nhưng không bao giờ nhận chữ ký của nhau:
/// version tool và danh sách ecosystem làm câu được ký đặc thù cho một core.)
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CoreIdentity {
    pub core_version: String,
    pub tool_version: String,
    pub msrv: String,
    pub ecosystems: Vec<String>,
}

impl CoreIdentity {
    fn canonical_suffix(&self) -> String {
        let mut ecosystems = self.ecosystems.clone();
        ecosystems.sort();
        ecosystems.dedup();
        format!(
            "{}\n{}\n{}\n{}",
            self.core_version,
            self.tool_version,
            self.msrv,
            ecosystems.join(",")
        )
    }
}

/// Canonical signed message including the core identity block.
/// Thông điệp ký canonical bao gồm block identity của core.
pub fn signed_message(core: &str, config_digest: &str, identity: &CoreIdentity) -> String {
    format!(
        "{}\n{core}\n{config_digest}\n{}",
        ATTESTATION_MESSAGE_DOMAIN,
        identity.canonical_suffix()
    )
}

/// BLAKE3 hex of bytes — BLAKE3 hex của bytes.
pub fn blake3_hex(data: &[u8]) -> String {
    let hash = mgc_crypto::blake3_signer::Blake3Hasher::hash_bytes(data);
    hex::encode(hash.0)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
