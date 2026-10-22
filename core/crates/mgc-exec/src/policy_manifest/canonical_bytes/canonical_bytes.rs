use crate::policy_manifest::PolicyManifest;

/// Stable canonical bytes covered by the signature (field order fixed,
/// entries in document order — the signer and verifier must agree).
/// Bytes canonical ổn định được chữ ký phủ (thứ tự field cố định,
/// entries theo thứ tự tài liệu — bên ký và bên verify phải khớp).
pub fn canonical_bytes(manifest: &PolicyManifest) -> String {
    let mut out = String::from("MagiCore.PolicyException.v2\n");
    push_canonical_field(&mut out, &manifest.schema_version);
    push_canonical_field(&mut out, &manifest.id);
    push_canonical_field(&mut out, &manifest.issued_at.to_string());
    push_canonical_field(&mut out, &manifest.expires_at.to_string());
    push_canonical_field(&mut out, &manifest.key_id);
    let mut revoked = manifest.revoked_keys.clone();
    revoked.sort();
    push_canonical_field(&mut out, &revoked.len().to_string());
    for key in &revoked {
        push_canonical_field(&mut out, key);
    }
    push_canonical_field(&mut out, &manifest.entries.len().to_string());
    for entry in &manifest.entries {
        push_canonical_field(&mut out, &entry.program);
        push_canonical_field(&mut out, &entry.args_exact.len().to_string());
        for arg in &entry.args_exact {
            push_canonical_field(&mut out, arg);
        }
        push_canonical_field(&mut out, &entry.framework);
        push_canonical_field(&mut out, &entry.framework_version);
        push_canonical_field(&mut out, &entry.os);
        push_canonical_field(&mut out, &entry.operation);
        push_canonical_field(&mut out, &entry.reason);
        push_canonical_field(&mut out, &entry.expected);
    }
    out
}

/// Append one UTF-8 byte-length-prefixed field; length prefixes keep fields
/// and argument vectors unambiguous even when values contain delimiters.
/// Nối field có tiền tố độ dài byte UTF-8; không thể nhập nhằng do delimiter.
fn push_canonical_field(out: &mut String, value: &str) {
    out.push_str(&value.len().to_string());
    out.push(':');
    out.push_str(value);
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
