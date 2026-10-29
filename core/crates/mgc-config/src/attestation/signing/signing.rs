//! Attestation signing and storage / Ký và lưu attestation.

use std::path::Path;

use crate::attestation::{Attestation, CoreIdentity, PreviousKey, signed_message};

fn now_rfc3339() -> String {
    time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_default()
}

/// Attest version tag — the mgc build that attested (compile-time).
/// (Version mgc đã chứng thực — compile-time.)
pub fn attester_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
/// Identity block bound into a core signature (P-8.3/P-8.4).
/// Block identity được ràng buộc vào chữ ký core (P-8.3/P-8.4).
///
/// `core_version` is the workspace release and `tool_version` the attesting
/// binary; both come from the same build so a signature cannot travel to a
/// different core or a differently-versioned tool.
/// (`core_version` là release của workspace, `tool_version` là binary chứng
/// thực; cả hai cùng một build nên chữ ký không chuyển được sang core khác
/// hoặc tool khác version.)
pub fn core_identity(core: &str) -> CoreIdentity {
    CoreIdentity {
        core_version: attester_version().to_string(),
        tool_version: attester_version().to_string(),
        msrv: "1.94".to_string(),
        ecosystems: vec![core.to_string()],
    }
}

#[allow(clippy::too_many_arguments)]
pub(in crate::attestation) fn sign_and_store(
    project_root: &Path,
    core: &str,
    digest: &str,
    key_pair: &mgc_crypto::keyring::KeyPair,
    prev_keys: Vec<PreviousKey>,
    slot: &Path,
    path: &Path,
) -> anyhow::Result<Attestation> {
    let signer = key_pair
        .signer()
        .map_err(|error| anyhow::anyhow!("cannot use key: {error}"))?;
    let identity = core_identity(core);
    let signature = signer.sign(signed_message(core, digest, &identity).as_bytes());
    let attestation = Attestation {
        core: core.to_string(),
        config_digest: digest.to_string(),
        project_path: project_root.to_string_lossy().to_string(),
        key_id: key_pair.key_id.clone(),
        public_key: key_pair.public_key.to_base64(),
        prev_keys,
        attested_at: now_rfc3339(),
        mgc_version: attester_version().to_string(),
        core_identity: Some(identity),
        signature: signature.to_base64(),
    };
    std::fs::create_dir_all(slot)
        .map_err(|error| anyhow::anyhow!("cannot create attestation slot: {error}"))?;
    let text = serde_json::to_string_pretty(&attestation)
        .map_err(|error| anyhow::anyhow!("cannot serialize attestation: {error}"))?;
    std::fs::write(path, text)
        .map_err(|error| anyhow::anyhow!("cannot write attestation: {error}"))?;
    // P-8.1/P-8.2: the sibling `<attestation>.sig` binds the document itself.
    // The attestation carries its own signature field, but a rewritten file
    // whose `config_digest` still matches the manifest would pass the message
    // check; the sibling covers the exact bytes on disk and is compared before
    // the document is trusted.
    // (P-8.1/P-8.2: file `<attestation>.sig` cạnh đó ràng buộc chính tài liệu.
    // Attestation có field signature riêng, nhưng file bị ghi lại với
    // `config_digest` vẫn khớp manifest sẽ qua được kiểm message; file cạnh
    // đó phủ đúng bytes trên đĩa và được so trước khi tin tài liệu.)
    let document = std::fs::read(path)
        .map_err(|error| anyhow::anyhow!("cannot read back attestation: {error}"))?;
    let document_signature = signer.sign(&document);
    let mut sidecar = path.as_os_str().to_os_string();
    sidecar.push(".sig");
    std::fs::write(
        std::path::PathBuf::from(sidecar),
        document_signature.to_base64(),
    )
    .map_err(|error| anyhow::anyhow!("cannot write attestation sidecar signature: {error}"))?;
    Ok(attestation)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
