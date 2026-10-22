//! CAS root symlink and permission boundaries.
//! Biên symlink và quyền truy cập của root CAS.

use crate::compiled_cache_integrity::support::tmp_store_dir;
use mgc_store::cas::ContentStore;
use std::fs;

#[cfg(unix)]
pub(crate) fn cas_root_that_is_a_symlink_is_rejected() {
    // P0-A: the OLD validate_cas_root used fs::metadata (follows the
    // link), so a symlinked store root sailed through as a real dir. The
    // fixed check uses symlink_metadata and MUST reject the link itself.
    // (P0-A: validate_cas_root CŨ dùng fs::metadata (follow link) nên
    // store root là symlink vẫn lọt như thư mục thật. Check sửa dùng
    // symlink_metadata và PHẢI từ chối chính link.)
    let base = tmp_store_dir("root-symlink-base");
    let real = base.join("real-store");
    fs::create_dir_all(&real).unwrap();
    let link = base.join("link-store");
    std::os::unix::fs::symlink(&real, &link).unwrap();
    assert!(
        ContentStore::new(link).is_err(),
        "a symlinked CAS root must be rejected (P0-A)"
    );
}

#[cfg(unix)]
pub(crate) fn cas_root_under_symlinked_ancestor_is_rejected() {
    // The root itself is a real dir, but an ANCESTOR is a symlink — the
    // whole store is redirected. Must be rejected (P0-A ancestor sweep).
    // (Root là thư mục thật nhưng MỘT ANCESTOR là symlink — cả store bị
    // chuyển hướng. Phải từ chối (quét ancestor P0-A).)
    let base = tmp_store_dir("root-ancestor-symlink");
    let real_parent = base.join("real-parent");
    fs::create_dir_all(&real_parent).unwrap();
    let link_parent = base.join("link-parent");
    std::os::unix::fs::symlink(&real_parent, &link_parent).unwrap();
    let root = link_parent.join("cas");
    assert!(
        ContentStore::new(root).is_err(),
        "a CAS root under a symlinked ancestor must be rejected (P0-A)"
    );
}

#[cfg(unix)]
pub(crate) fn cas_root_dangling_symlink_is_rejected() {
    // A DANGLING symlink root (target never exists): fs::metadata errors
    // NotFound while fs::symlink_metadata succeeds and shows the link.
    // The old code treated NotFound as "root does not exist yet" and
    // created the store THROUGH the dangling link's parent path; the new
    // check must reject the link itself.
    // (Root là symlink TREO (đích không tồn tại): fs::metadata lỗi NotFound
    // trong khi fs::symlink_metadata thành công và cho thấy link. Code cũ
    // coi NotFound là "root chưa tồn tại" rồi tạo store xuyên qua path mẹ
    // của link treo; check mới phải từ chối chính link.)
    let base = tmp_store_dir("root-dangling");
    let dangling = base.join("dangling-store");
    std::os::unix::fs::symlink(base.join("never-created"), &dangling).unwrap();
    assert!(
        ContentStore::new(dangling).is_err(),
        "a dangling-symlink CAS root must be rejected, not created through (P0-A)"
    );
}

pub(crate) fn cas_root_regular_dir_is_accepted() {
    // Negative control: an ordinary fresh root still initializes fine.
    // (Đối chứng âm: root mới hoàn toàn bình thường vẫn khởi tạo tốt.)
    let root = tmp_store_dir("root-plain");
    assert!(ContentStore::new(root).is_ok());
}

#[cfg(unix)]
pub(crate) fn cas_root_and_blobs_are_owner_only() {
    // P0-D: store dirs must be 0700 and blobs 0600/0700 (owner-only) —
    // compiled JS + inline sourcemaps must not be world/group-readable.
    // (P0-D: thư mục store phải 0700 và blob 0600/0700 (chỉ-owner) — JS
    // biên dịch + sourcemap inline không được cho group/others đọc.)
    use std::os::unix::fs::PermissionsExt;
    let root = tmp_store_dir("perm-tight");
    let store = ContentStore::new(root.clone()).unwrap();

    let root_mode = fs::metadata(&root).unwrap().permissions().mode() & 0o777;
    assert_eq!(root_mode, 0o700, "CAS root must be 0700 (P0-D)");

    let hash = store.import_bytes(b"perm-blob").unwrap();
    let blob_mode = fs::metadata(hash.cas_path(&root))
        .unwrap()
        .permissions()
        .mode()
        & 0o777;
    assert_eq!(blob_mode, 0o600, "non-exec CAS blob must be 0600 (P0-D)");

    let exec_hash = store.import_bytes_with_exec(b"#!/bin/sh\n", true).unwrap();
    let exec_mode = fs::metadata(exec_hash.cas_path(&root))
        .unwrap()
        .permissions()
        .mode()
        & 0o777;
    assert_eq!(exec_mode, 0o700, "exec CAS blob must be 0700 (P0-D)");

    // Export to a project still yields ORDINARY project modes (0644).
    // (Export ra project vẫn ra mode project BÌNH THƯỜNG (0644).)
    let dest = root.join("exported.sh");
    store.export_to(&exec_hash, &dest).unwrap();
    let export_mode = fs::metadata(&dest).unwrap().permissions().mode() & 0o777;
    assert_eq!(export_mode, 0o755, "exported executable keeps 0755 (P0-D)");
}

#[cfg(unix)]
pub(crate) fn system_root_owned_symlink_ancestor_is_tolerated() {
    // REVIEW vòng-2 finding: macOS ships `/var → private/var` (root-owned
    // system symlink) and tempfile::tempdir() on macOS/CI lives under it
    // — a hard ancestor-symlink ban bricks every standard temp dir. The
    // contract: root-OWNED symlink ancestors are tolerated (a same-user
    // attacker cannot write a root-owned link), every OTHER symlink
    // ancestor stays a hard reject (see
    // cas_root_under_symlinked_ancestor_is_rejected).
    // (Phát hiện REVIEW vòng-2: macOS ship `/var → private/var` (symlink
    // hệ thống do root sở hữu) và tempfile::tempdir() trên macOS/CI nằm
    // dưới nó — cứng cấm ancestor-symlink brick mọi temp dir chuẩn. Hợp
    // đồng: ancestor symlink do ROOT sở hữu được dung thứ (attacker
    // cùng-user không ghi được link root-owned), mọi ancestor symlink
    // KHÁC vẫn bị chặn tuyệt đối (xem
    // cas_root_under_symlinked_ancestor_is_rejected).
    let dir = tempfile::tempdir().unwrap();
    // tempdir() itself proves the exemption: on macOS it resolves under
    // /var (root-owned symlink) and MUST still open a store.
    // (tempdir() tự chứng minh exemption: trên macOS nó resolve dưới
    // /var (symlink root-owned) và VẪN phải mở được store.)
    let store_root = dir.path().join("store");
    let store = ContentStore::new(store_root).unwrap();
    let hash = store.import_bytes(b"under-system-symlink").unwrap();
    assert!(store.contains(&hash));
}
