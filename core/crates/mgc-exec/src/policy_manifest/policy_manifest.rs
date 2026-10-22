use crate::policy_manifest::{PolicyManifest, key_id_hex, verify_manifest};

/// Select the active manifest from rotation candidates (Spec rev4 §5/§16.7).
/// Newest valid `issued_at` wins; revoked keys are unioned across all
/// verified snapshots, so a newer revocation snapshot invalidates older
/// manifests signed by the revoked key. Unverifiable candidates are
/// skipped (the base native-deny layers stay enforced without them).
/// (Chọn manifest hiệu lực từ các bản rotation (Spec rev4 §16.7). Bản hợp lệ
/// `issued_at` mới nhất thắng; khóa thu hồi được hợp nhất qua mọi snapshot
/// đã xác minh, nên snapshot thu hồi mới hơn vô hiệu manifest cũ do khóa đó
/// ký. Bản không xác minh được thì bỏ qua (các lớp native-deny gốc vẫn thực
/// thi khi không có chúng).)
pub fn select_active_manifest(
    manifests: Vec<PolicyManifest>,
    trusted_hex_ids: &[String],
    extra_revoked_hex_ids: &[String],
    now_secs: i64,
) -> Option<PolicyManifest> {
    // Verify signed snapshots at the last second of their declared validity.
    // This keeps authentic revocations effective after the exception payload
    // expires, while expired payloads themselves cannot supply exceptions.
    // Xác minh snapshot ký tại giây cuối của hạn đã khai báo. Revocation xác
    // thực vẫn hiệu lực sau expiry, nhưng payload hết hạn không cấp exception.
    let mut verified: Vec<PolicyManifest> = Vec::new();
    let mut revoked: Vec<String> = extra_revoked_hex_ids.to_vec();
    for manifest in manifests {
        let hex = match key_id_hex(&manifest.key_id) {
            Some(hex) => hex,
            None => continue,
        };
        if !trusted_hex_ids.contains(&hex) {
            continue;
        }
        let signature_time = manifest.expires_at.saturating_sub(1);
        if verify_manifest(
            &manifest,
            std::slice::from_ref(&manifest.key_id),
            &[],
            signature_time,
        )
        .is_err()
        {
            continue;
        }
        if manifest.issued_at <= now_secs {
            revoked.extend(manifest.revoked_keys.iter().cloned());
        }
        if manifest.issued_at <= now_secs && now_secs < manifest.expires_at {
            verified.push(manifest);
        }
    }
    // Drop revoked signers and pick the newest active payload.
    // Loại signer bị thu hồi rồi chọn payload còn hạn mới nhất.
    verified
        .into_iter()
        .filter(|manifest| {
            key_id_hex(&manifest.key_id)
                .map(|hex| !revoked.contains(&hex))
                .unwrap_or(false)
        })
        .max_by_key(|manifest| manifest.issued_at)
}
