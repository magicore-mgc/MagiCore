//! Filesystem Staging tests. / Test filesystem staging.

use crate::adversarial_integrity::support::tmp_store_dir;
use mgc_store::cas::ContentStore;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn staging_kind_probe_matches_platform_capability() {
    // Assert the ACTUAL branch used on THIS filesystem — a fallback that
    // silently never runs is a false-green (audit vòng-3 P0-2).
    // On APFS (stock macOS) the probe MUST report CowClone; on a
    // non-reflink filesystem it must report PlainCopy. Either way the
    // answer must be non-accidental and stable.
    // (Assert branch THẬT được dùng trên filesystem NÀY — fallback im lặng
    // không chạy là false-green (P0-2 audit vòng-3). Trên APFS (macOS
    // chuẩn) probe PHẢI báo CowClone; trên filesystem không reflink phải
    // báo PlainCopy. Cách nào thì câu trả lời cũng không được ngẫu nhiên và
    // phải ổn định.)
    let root = tmp_store_dir("staging-probe");
    let store = ContentStore::new(root.clone()).unwrap();
    let hash = store.import_bytes(b"probe-payload").unwrap();

    let probe_dir = root.join("probe-dest");
    fs::create_dir_all(&probe_dir).unwrap();
    let kind = store.staging_kind_probe(&hash, &probe_dir);

    #[cfg(target_os = "macos")]
    {
        // /tmp on stock macOS is APFS → clonefile works.
        // (/tmp trên macOS chuẩn là APFS → clonefile chạy.)
        assert_eq!(
            kind,
            mgc_store::cas::StagingKind::CowClone,
            "APFS must use the clonefile COW branch (fallback silently dead = false-green)"
        );
    }
    #[cfg(target_os = "linux")]
    {
        // CI btrfs/XFS → CowClone; tmpfs/ext4 → PlainCopy. Both are valid;
        // assert consistency instead of a fixed value.
        // (CI btrfs/XFS → CowClone; tmpfs/ext4 → PlainCopy. Cả hai hợp lệ;
        // assert tính nhất quán thay vì giá trị cố định.)
        let again = store.staging_kind_probe(&hash, &probe_dir);
        assert_eq!(kind, again, "probe must be deterministic per filesystem");
    }
    // No probe leftovers.
    // (Không sót probe temp.)
    let leftovers: Vec<_> = fs::read_dir(&probe_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().starts_with("mgc-export-"))
        .collect();
    assert!(leftovers.is_empty(), "probe temp leftovers: {leftovers:?}");
}

pub(crate) fn export_temp_is_created_in_destination_filesystem() {
    // Simulate two "filesystems": the store root dir and a separate export
    // dir. The staging temp (.mgc-export-*) must appear NEXT TO the
    // destination — never under the CAS tmp dir — so the final rename is
    // same-filesystem (EXDEV cannot happen).
    // Mô phỏng 2 "filesystem": store root và thư mục export riêng. Temp
    // staging (.mgc-export-*) phải nằm CẠNH đích — không bao giờ dưới tmp
    // của CAS — để rename cuối cùng filesystem (kh thể EXDEV).
    let root = tmp_store_dir("export-fs");
    let store = ContentStore::new(root.clone()).unwrap();
    let hash = store.import_bytes(b"cross-fs-payload").unwrap();

    let export_dir = root.join("project-fs"); // distinct subtree = distinct "volume"
    fs::create_dir_all(&export_dir).unwrap();
    let dest = export_dir.join("out.txt");

    store.export_to(&hash, &dest).unwrap();

    assert_eq!(fs::read(&dest).unwrap(), b"cross-fs-payload");
    // No staging leftovers anywhere (destination side AND CAS tmp side).
    // Không còn staging sót ở bất kỳ đâu (cả phía đích lẫn tmp CAS).
    let cas_tmp: Vec<_> = fs::read_dir(root.join("tmp"))
        .map(|rd| rd.filter_map(|e| e.ok()).collect())
        .unwrap_or_default();
    assert!(
        cas_tmp.is_empty(),
        "CAS tmp must be clean, found {cas_tmp:?}"
    );
    let dest_dir: Vec<_> = fs::read_dir(&export_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().starts_with("mgc-export-"))
        .collect();
    assert!(
        dest_dir.is_empty(),
        "no .mgc-export-* staging leftovers allowed, found {dest_dir:?}"
    );
}

pub(crate) fn export_failure_leaves_no_staging_temp() {
    let root = tmp_store_dir("export-cleanup");
    let store = ContentStore::new(root.clone()).unwrap();
    let hash = store.import_bytes(b"cleanup-payload").unwrap();

    let export_dir = root.join("dest");
    fs::create_dir_all(&export_dir).unwrap();

    // Destination already exists with DIFFERENT content → hard error, and
    // the pre-existing file must be untouched (no staging leak).
    // Đích đã tồn tại với nội dung KHÁC → lỗi cứng, file cũ phải nguyên vẹn.
    let dest = export_dir.join("exists.bin");
    fs::write(&dest, b"other-content").unwrap();
    let res = store.export_to(&hash, &dest);
    assert!(res.is_err());

    assert_eq!(fs::read(&dest).unwrap(), b"other-content");
    let leftovers: Vec<_> = fs::read_dir(&export_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().starts_with("mgc-export-"))
        .collect();
    assert!(leftovers.is_empty(), "staging leaked: {leftovers:?}");
}

pub(super) fn scan_store_temps(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("compiled-") || name.starts_with("mgc-export-") {
                    found.push(path);
                }
            }
        }
    }
    found
}
