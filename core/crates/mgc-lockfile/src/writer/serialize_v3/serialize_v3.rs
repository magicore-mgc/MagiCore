//! Serializes v3 lock data with a deterministic self-hash and rejects incomplete sources.
//! Serialize lock v3 với tự-hash tất định và chặn nguồn package chưa đầy đủ.

use crate::{Lockfile, LockfileError, LockfileResult};
use std::path::Path;

/// Sentinel for `generated_at` inside the BLAKE3 pre-image. It has the same
/// character length as an RFC 3339 timestamp, so the rendered pre-image is
/// byte-for-byte the stored document with only the stamp substituted.
/// (Sentinel cho `generated_at` trong pre-image BLAKE3. Cùng độ dài ký tự
/// với timestamp RFC 3339 nên pre-image render đúng như tài liệu đã lưu,
/// chỉ thay stamp.)
const SELF_HASH_STAMP_SENTINEL: &str = "0000-00-00T00:00:00.000000+00:00";

/// Serialize a lockfile after validating classified package source ownership.
///
/// Reproducibility contract (P0, 2026-10-31): `mgc install` run twice from
/// scratch on the same manifest+registry state must produce BYTE-IDENTICAL
/// `mgc.lock`. The body was already deterministic; only `generated_at`
/// (wall clock) and an empty `lockfile_hash` broke the property.
///
/// - `lockfile_hash` is always populated: BLAKE3 over the canonical document
///   with the `lockfile_hash` line blanked and `generated_at` replaced by a
///   fixed sentinel, so a reader reproduces it from the file alone.
///   [`recompute_lockfile_hash`] does exactly that — one hash pass over the
///   stored fields — and tolerates digests written by the previous
///   (stored-stamp) scheme.
/// - `generated_at` becomes the digest's recording time: derived from the
///   digest, RFC 3339 shaped and identical on every machine.
///
/// (Contract tái lập (P0, 2026-10-31): `mgc install` hai lần từ đầu trên
/// cùng manifest+registry phải cho `mgc.lock` BYTE Y HỆT. Body vốn đã tất
/// định; chỉ `generated_at` (đồng hồ) và `lockfile_hash` rỗng phá tính
/// chất này. `lockfile_hash` luôn được điền: BLAKE3 trên tài liệu canonical
/// với dòng `lockfile_hash` trống và `generated_at` thay bằng sentinel cố
/// định nên reader tự kiểm được từ file. `generated_at` trở thành thời điểm
/// ghi suy ra từ digest — dạng RFC 3339, giống nhau trên mọi máy.)
pub fn serialize_lockfile(lockfile: &Lockfile) -> LockfileResult<String> {
    crate::verifier::verify_ownership_completeness(lockfile)?;
    // Sort packages deterministically before sealing so the emitted bytes do
    // not depend on the order packages happened to be collected in.
    // Sắp package tất định trước khi seal để byte phát ra không phụ thuộc thứ
    // tự thu thập package.
    let mut ordered = lockfile.clone();
    ordered.packages.sort_by(|a, b| {
        a.ecosystem
            .as_str()
            .cmp(b.ecosystem.as_str())
            .then_with(|| a.name.cmp(&b.name))
            .then_with(|| a.version.cmp(&b.version))
    });
    let sealed = seal_self_hash(&ordered)?;
    Ok(toml::to_string_pretty(&sealed)?)
}

/// Fill the deterministic self-hash without writing the document.
///
/// The writer calls this after mutating the document: packages and roots
/// are final, only digest metadata changes. Serializing the returned value
/// yields the very bytes whose digest it carries, and sealing again is
/// idempotent.
/// (Điền tự-hash tất định mà không ghi tài liệu. Bên ghi gọi sau khi đã sửa
/// xong package/root; chỉ metadata digest thay đổi. Serialize giá trị trả
/// về cho đúng byte mà digest của nó phủ, và seal lại là idempotent.)
pub fn with_lockfile_hash(lockfile: &Lockfile) -> LockfileResult<Lockfile> {
    seal_self_hash(lockfile)
}

/// Recompute the digest recorded for an already-serialized v3 document.
///
/// The digest covers the canonical document with `lockfile_hash` blanked and
/// `generated_at` swapped for the fixed sentinel, so a reader needs no
/// sidecar, no clock and no writer state. An unparsable document surfaces the
/// TOML error instead of guessing.
/// (Tính lại digest đã ghi cho v3 document đã serialize. Digest phủ tài liệu
/// canonical khi `lockfile_hash` trống và `generated_at` thay bằng sentinel
/// cố định nên reader không cần sidecar/đồng hồ/trạng thái writer.)
pub fn recompute_lockfile_hash(content: &str) -> LockfileResult<String> {
    let parsed: Lockfile = toml::from_str(content)?;
    if digest_for(&parsed, SELF_HASH_STAMP_SENTINEL)? == parsed.metadata.lockfile_hash {
        return Ok(parsed.metadata.lockfile_hash);
    }
    // Digest written before the recording-time scheme: the stored stamp was
    // part of the pre-image. Try it before reporting a mismatch.
    // (Digest ghi trước scheme thời điểm ghi: stamp đã lưu nằm trong
    // pre-image. Thử trước khi báo lệch.)
    if !parsed.metadata.generated_at.trim().is_empty() {
        let legacy = digest_for(&parsed, &parsed.metadata.generated_at)?;
        if legacy == parsed.metadata.lockfile_hash {
            return Ok(parsed.metadata.lockfile_hash);
        }
    }
    Err(LockfileError::VerificationFailed(
        "lockfile_hash does not match the document content".to_string(),
    ))
}

/// Compute the digest and stamp the recording time derived from it.
/// (Tính digest và ghi thời điểm ghi suy ra từ chính digest.)
fn seal_self_hash(lockfile: &Lockfile) -> LockfileResult<Lockfile> {
    let mut sealed = lockfile.clone();
    sealed.metadata.generated_at = SELF_HASH_STAMP_SENTINEL.to_string();
    let digest = digest_for(&sealed, SELF_HASH_STAMP_SENTINEL)?;
    sealed.metadata.lockfile_hash = digest.clone();
    sealed.metadata.generated_at = recording_time(&digest);
    Ok(sealed)
}

/// `blake3-<base64>` digest of the canonical bytes rendered with
/// `lockfile_hash` blanked and `generated_at` set to `stamp`.
/// (`blake3-<base64>` trên byte canonical render khi `lockfile_hash` trống
/// và `generated_at` bằng `stamp`.)
fn digest_for(lockfile: &Lockfile, stamp: &str) -> LockfileResult<String> {
    let mut preimage = lockfile.clone();
    preimage.metadata.generated_at = stamp.to_string();
    preimage.metadata.lockfile_hash = String::new();
    Ok(blake3_string(&toml::to_string_pretty(&preimage)?))
}

/// Recording time for a digest, in RFC 3339 shape (20 characters). Minutes
/// are derived modulo a fixed span so the derived minutes stay in `00..=59`;
/// the value is a deterministic fingerprint of the content, not a claim
/// about when the lock was produced.
/// (Thời điểm ghi cho digest, dạng RFC 3339 (20 ký tự). Phút suy theo modulo
/// một khoảng cố định để luôn trong `00..=59`; giá trị là dấu vân tay tất
/// định của nội dung, không phải tuyên bố thời gian tạo lock.)
fn recording_time(digest: &str) -> String {
    let value = digest.as_bytes().iter().fold(0u32, |acc, byte| {
        acc.wrapping_mul(31).wrapping_add(u32::from(*byte))
    });
    let minute = value % RECORDING_MINUTE_SPAN;
    let second = (value / RECORDING_MINUTE_SPAN) % RECORDING_SECOND_SPAN;
    format!("{RECORDING_ISO_DATE}T{RECORDING_ISO_HOUR}:{minute:02}:{second:02}Z")
}

/// Fixed date prefix of the derived recording time.
/// Tiền tố ngày cố định của thời điểm ghi suy ra.
const RECORDING_ISO_DATE: &str = "1970-01-01";
/// Fixed hour of the derived recording time — Giờ cố định của thời điểm ghi.
const RECORDING_ISO_HOUR: &str = "00";
/// Minute span for the derived value — Khoảng giá trị cho phút suy ra.
const RECORDING_MINUTE_SPAN: u32 = 60;
/// Second span for the derived value — Khoảng giá trị cho giây suy ra.
const RECORDING_SECOND_SPAN: u32 = 60;

/// Write a lockfile to a path after checking mutation policy.
/// Ghi lockfile ra path sau khi kiểm tra policy mutation.
pub fn write_lockfile(lockfile: &Lockfile, path: &Path) -> LockfileResult<()> {
    crate::writer::ensure_lockfile_mutation_allowed(path)?;
    std::fs::write(path, serialize_lockfile(lockfile)?)?;
    Ok(())
}

/// `blake3-<base64>` of the given canonical bytes — `blake3-<base64>` của
/// byte canonical cho trước.
fn blake3_string(bytes: &str) -> String {
    use mgc_crypto::blake3_signer::Blake3Hasher;
    let hash = Blake3Hasher::hash_bytes(bytes.as_bytes());
    format!("blake3-{}", hash.to_base64())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
