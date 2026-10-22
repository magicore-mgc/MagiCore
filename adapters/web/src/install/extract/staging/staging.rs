//! Create unique staging roots and publish verified roots under bounded races.
//! Tạo staging root duy nhất và publish root đã xác minh với race có giới hạn.
use crate::cache::ExtractedPackageMarker;
use crate::install::package_marker::{
    extracted_content_matches, extracted_marker_has_content_signature,
    extracted_marker_matches_fast, read_extracted_package_marker,
};
use mgc_types::adapter::ResolvedPackage;
use mgc_types::{MgError, MgResult};
use std::path::{Path, PathBuf};

/// A fresh, unique temp path next to `canonical_root` for extraction. Used
/// both for the real extract+rename and for the warm-cache "import + claim
/// only" throwaway (the temp tree is discarded after import).
/// (Đường dẫn temp mới, duy nhất cạnh `canonical_root` cho extraction. Dùng
/// cho cả extract+rename thật lẫn throwaway "chỉ import + claim" của warm
/// cache — cây temp bị hủy sau khi import.)
pub(in crate::install::extract) fn fresh_extract_temp_root(
    pkg: &ResolvedPackage,
    canonical_root: &Path,
) -> PathBuf {
    let parent = canonical_root.parent().unwrap_or(Path::new("."));
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    // PID + nanos: the staging name must be unique ACROSS processes, not
    // just within one — two processes racing the same digest share the same
    // parent dir, and a same-nanosecond collision made one process
    // remove_dir_all the other's LIVE staging tree.
    // (PID + nano: tên staging phải duy nhất GIỮA CÁC process, không chỉ
    // trong một process — hai process đua cùng digest dùng chung thư mục cha,
    // và trùng nano-giây khiến một process remove_dir_all cây staging ĐANG
    // SỐNG của process kia.)
    parent.join(format!(
        ".mgc-extract-{}-{}-{}",
        pkg.id.name_str(),
        std::process::id(),
        ts
    ))
}

/// Cross-process publish attempts before giving up — bounded, fail-closed.
/// (Số lần thử publish cross-process trước khi bỏ — có chặn trên, fail-closed.)
const PUBLISH_RACE_ATTEMPTS: usize = 4;

/// Backoff between publish attempts — long enough for a racing publisher to
/// finish its remove+rename, short enough not to stall a normal install.
/// (Backoff giữa các lần thử — đủ dài để publisher đua kịp remove+rename,
/// đủ ngắn để không làm chậm install thường.)
const PUBLISH_RACE_BACKOFF: std::time::Duration = std::time::Duration::from_millis(25);

/// Publish the extracted staging `package_root` into the canonical root
/// atomically and IDEMPOTENTLY (safe under same-digest concurrent installs).
///
/// The canonical root is a DETERMINISTIC SHARED path (same package+integrity
/// ⇒ same path, `shared_extracted_package_root`) that separate PROCESSES can
/// hit simultaneously — the in-process `extracted_package_root_lock` cannot
/// serialize them. The old publish was a cross-process TOCTOU ("exists? →
/// remove → rename"): a racer's rename landing between our check and our
/// rename made OUR rename fail with ENOTEMPTY ("Directory not empty") — the
/// flake seen in `concurrent_same_digest_does_not_double_store`.
///
/// Race contract (mirrors the CAS publish contract in
/// `mgc_store` compiled-cache `put` — the first COMPLETE writer wins, the
/// loser READS and VERIFIES the winner, never trusted blindly):
///   1. `rename(staging → canonical)` is the publish point (the marker is
///      already inside staging, so a published root is always complete).
///   2. On `AlreadyExists`/`DirectoryNotEmpty` a concurrent publisher won:
///      - its marker matches ours (same digest ⇒ identical content) → keep
///        the winner, discard our own staging (legit dedup, no double-store);
///      - stale/tampered/divergent winner → remove it + bounded retry
///        (last-writer-wins, same as the pre-fix sequential semantics).
///   3. Attempts are bounded: exhaustion returns the last OS error — no
///      faked success.
///
/// (Publish staging `package_root` vào root canonical nguyên tử và
/// IDEMPOTENT (an toàn khi install cùng digest chạy đồng thời).
///
/// Root canonical là path DÙNG CHUNG TẤT ĐỊNH (cùng package+integrity ⇒
/// cùng path, `shared_extracted_package_root`) mà các process RIÊNG BIỆT
/// có thể chạm cùng lúc — lock `extracted_package_root_lock` chỉ
/// trong-process, không tuần tự hóa được chúng. Publish cũ là TOCTOU
/// cross-process ("exists? → remove → rename"): rename của racer rơi vào
/// giữa check và rename của mình khiến rename CỦA MÌNH fail ENOTEMPTY
/// ("Directory not empty") — flake thấy ở
/// `concurrent_same_digest_does_not_double_store`.
///
/// Hợp đồng race (ảnh chiếu hợp đồng publish CAS trong `mgc_store`
/// compiled-cache `put` — writer HOÀN CHỈNH đầu tiên thắng, bên thua ĐỌC
/// và VERIFY winner, không tin mù):
///   1. `rename(staging → canonical)` là điểm publish (marker đã nằm
///      trong staging nên root đã publish luôn trọn vẹn).
///   2. Khi `AlreadyExists`/`DirectoryNotEmpty`, một publisher đồng thời
///      đã thắng:
///      - marker của nó khớp mình (cùng digest ⇒ nội dung giống hệt) →
///        giữ winner, bỏ staging của mình (dedup hợp lệ, không
///        double-store);
///      - winner stale/tamper/phân kỳ → remove + retry có chặn
///        (last-writer-wins, đúng ngữ nghĩa tuần tự trước khi sửa).
///   3. Số lần thử có chặn: cạn lượt trả lỗi OS cuối — không giả thành
///      công.)
pub(in crate::install::extract) fn publish_extracted_root(
    pkg_name: &str,
    package_root: &Path,
    canonical_root: &Path,
    expected_marker: &ExtractedPackageMarker,
) -> MgResult<()> {
    let mut last_err: Option<std::io::Error> = None;
    for _attempt in 0..PUBLISH_RACE_ATTEMPTS {
        if canonical_root.exists() {
            // Same-digest racer already published a GOOD winner: keep it
            // (dedup, no double-store) — NEVER remove-then-replace a root
            // whose marker matches, because a concurrent install may be
            // linking FROM it right now (ENOENT flake in
            // `concurrent_same_digest_does_not_double_store`: the old code
            // removed the winner another process had just published and
            // was about to link).
            // Always validate content: a marker cannot prove that a cached
            // root is complete or untampered. A matching winner is immutable
            // for this publish decision and must never be removed.
            // Luôn xác thực nội dung: marker không chứng minh cây cache đủ
            // file hay chưa bị sửa. Winner khớp không bị xóa trong quyết định này.
            let winner_matches = match read_extracted_package_marker(canonical_root)? {
                Some(marker)
                    if extracted_marker_matches_fast(&marker, expected_marker)
                        && extracted_marker_has_content_signature(&marker) =>
                {
                    extracted_content_matches(canonical_root, expected_marker)?
                }
                _ => false,
            };
            if winner_matches {
                return Ok(());
            }
            match std::fs::remove_dir_all(canonical_root) {
                Ok(()) => {}
                // A racer removed it between our check and our remove —
                // nothing stale left, go straight to the rename.
                // (Racer khác đã xóa giữa lúc mình check và lúc mình xóa —
                // không còn gì stale, đi thẳng vào rename.)
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
                Err(err) => {
                    last_err = Some(err);
                    std::thread::sleep(PUBLISH_RACE_BACKOFF);
                    continue;
                }
            }
        }
        match std::fs::rename(package_root, canonical_root) {
            Ok(()) => return Ok(()),
            Err(err)
                if matches!(
                    err.kind(),
                    std::io::ErrorKind::AlreadyExists | std::io::ErrorKind::DirectoryNotEmpty
                ) =>
            {
                // Lost the publish race to a concurrent installer of the
                // SAME canonical root. Verify the winner before reusing it.
                // (Thua race publish cho installer đồng thời vào CÙNG root
                // canonical. Verify winner trước khi tái sử dụng.)
                let winner_matches = match read_extracted_package_marker(canonical_root)? {
                    Some(marker)
                        if extracted_marker_matches_fast(&marker, expected_marker)
                            && extracted_marker_has_content_signature(&marker) =>
                    {
                        extracted_content_matches(canonical_root, expected_marker)?
                    }
                    _ => false,
                };
                if winner_matches {
                    // Same digest ⇒ identical content: keep the winner, our
                    // staging copy is discarded by the caller's temp cleanup.
                    // (Cùng digest ⇒ nội dung giống hệt: giữ winner, bản
                    // staging của mình do caller dọn cùng temp.)
                    return Ok(());
                }
                // Stale/tampered/divergent winner — replace it (bounded).
                // (Winner stale/tamper/phân kỳ — thay thế (có chặn).)
                last_err = Some(err);
                std::thread::sleep(PUBLISH_RACE_BACKOFF);
                continue;
            }
            Err(err) => {
                return Err(MgError::Other(format!(
                    "failed to rename extracted '{}' to canonical '{}' for '{}': {}",
                    package_root.display(),
                    canonical_root.display(),
                    pkg_name,
                    err
                )));
            }
        }
    }
    Err(MgError::Other(format!(
        "failed to publish extracted '{}' to canonical '{}' for '{}' after {} \
         attempt(s) under concurrent publish race: {}",
        package_root.display(),
        canonical_root.display(),
        pkg_name,
        PUBLISH_RACE_ATTEMPTS,
        last_err
            .map(|err| err.to_string())
            .unwrap_or_else(|| "unknown".to_string())
    )))
}
