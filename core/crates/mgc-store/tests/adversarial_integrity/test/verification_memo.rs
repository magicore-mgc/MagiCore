//! Verification Memo tests. / Test verification memo.

use crate::adversarial_integrity::support::tmp_store_dir;
use mgc_store::cas::ContentStore;
use std::fs;
use std::path::Path;

pub(crate) fn memo_survives_nothing_same_length_rewrite_with_mtime_restore() {
    // Bypass A (audit): rewrite the blob with DIFFERENT same-length content
    // then RESTORE the original mtime — the memo must still be invalidated
    // (ctime is kernel-maintained and changes on every content write).
    // (Bypass A (audit): ghi đè blob bằng nội dung KHÁC cùng độ dài rồi PHỤC
    // HỒI mtime cũ — memo vẫn phải vô hiệu (ctime do kernel giữ và đổi
    // theo mỗi lần ghi nội dung).)
    let root = tmp_store_dir("memo-mtime-restore");
    let store = ContentStore::new(root.clone()).unwrap();
    let data = b"memo-atomic-identity-A".to_vec();
    let hash = store.import_bytes(&data).unwrap();

    let out1 = root.join("e1.bin");
    store.export_to(&hash, &out1).unwrap(); // verify + memoize

    // Attack: same-length rewrite + restore mtime (ctime cannot be
    // restored from user space — this is the whole point of the identity).
    // (Tấn công: ghi đè cùng độ dài + phục hồi mtime (ctime không thể
    // phục hồi từ user space — đây chính là ý nghĩa của identity).)
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let path = hash.cas_path(&root);
        let meta = fs::metadata(&path).unwrap();
        let saved_secs = meta.mtime();
        let saved_nanos = meta.mtime_nsec() as i64;
        let poison = b"memo-atomic-identity-X".to_vec(); // same length
        assert_eq!(poison.len(), data.len());
        fs::write(&path, &poison).unwrap();

        // Restore mtime exactly via utimensat — the SAME syscall an
        // attacker with file-write access would use (macOS touch has no
        // @epoch; utimensat is the ground-truth attack primitive).
        // (Phục hồi mtime chính xác qua utimensat — cùng syscall mà attacker
        // có quyền ghi file sẽ dùng (touch macOS không có @epoch; utimensat
        // là nguyên thủy tấn công chuẩn thực tế).)
        #[allow(unsafe_code)]
        fn restore_mtime(path: &Path, secs: i64, nanos: i64) {
            use std::ffi::CString;
            let times = [
                libc::timespec {
                    tv_sec: secs,
                    tv_nsec: nanos,
                },
                libc::timespec {
                    tv_sec: secs,
                    tv_nsec: nanos,
                },
            ];
            let c = CString::new(path.as_os_str().to_string_lossy().as_bytes()).unwrap();
            // SAFETY: utimensat with absolute path, plain timespec array by
            // reference, UTIME_OMIT not used — thin syscall, no memory
            // reinterpretation; kernel validates the path.
            // (An toàn: utimensat với path tuyệt đối, mảng timespec thuần
            // theo tham chiếu, không dùng UTIME_OMIT — syscall mỏng, không
            // diễn giải lại bộ nhớ; kernel tự validate path.)
            let ret = unsafe { libc::utimensat(libc::AT_FDCWD, c.as_ptr(), times.as_ptr(), 0) };
            assert_eq!(ret, 0, "attack setup: utimensat must succeed");
        }
        restore_mtime(&path, saved_secs, saved_nanos);

        // mtime really restored (attack is well-formed).
        // (mtime thực sự được phục hồi (tấn công hợp lệ).)
        let after = fs::metadata(&path).unwrap();
        assert_eq!(
            after.mtime(),
            saved_secs,
            "attack precondition: mtime restored"
        );
    }

    let out2 = root.join("e2.bin");
    let res = store.export_to(&hash, &out2);
    assert!(
        res.is_err(),
        "Bypass A: same-length rewrite + mtime restore must NOT pass the memo (ctime changed)"
    );
    assert!(!out2.exists(), "no export may be produced from poison");
}

pub(crate) fn memo_exec_and_nonexec_digests_do_not_share_verification() {
    // Bypass B (audit): the exec and non-exec variants of one digest are
    // DISTINCT CAS files; tampering the non-exec blob must not be masked by
    // a memo entry keyed only on the hex digest. The memo key now includes
    // the executable bit, so each variant verifies independently.
    // (Bypass B (audit): biến thể exec và non-exec của cùng digest là 2 file
    // CAS KHÁC NHAU; sửa blob non-exec không được che bởi memo key chỉ theo
    // hex digest. Memo key giờ gồm bit executable nên mỗi biến thể verify
    // độc lập.)
    let root = tmp_store_dir("memo-exec-split");
    let store = ContentStore::new(root.clone()).unwrap();
    let data = b"shared-bytes-for-exec-split".to_vec();

    // Import the same bytes as BOTH variants (different CAS paths).
    // (Import cùng bytes ở CẢ 2 biến thể (path CAS khác nhau).)
    let exec_hash = store.import_bytes_with_exec(&data, true).unwrap();
    let plain_hash = store.import_bytes_with_exec(&data, false).unwrap();
    assert_ne!(exec_hash.cas_path(&root), plain_hash.cas_path(&root));

    // Verify+memoize BOTH variants through export.
    // (Verify + memo hóa CẢ 2 biến thể qua export.)
    let o1 = root.join("x1").join("a");
    let o2 = root.join("x2").join("a");
    store.export_to(&exec_hash, &o1).unwrap();
    store.export_to(&plain_hash, &o2).unwrap();

    // Tamper the NON-EXEC blob on disk.
    // (Sửa blob NON-EXEC trên đĩa.)
    fs::write(plain_hash.cas_path(&root), b"TAMPERED-NON-EXEC-BYTES").unwrap();

    // Export of the non-exec variant must FAIL (fresh verify catches it)…
    // (Export biến thể non-exec phải FAIL (verify mới bắt được)…)
    let o3 = root.join("x3").join("a");
    let res = store.export_to(&plain_hash, &o3);
    assert!(
        res.is_err(),
        "tampered non-exec blob must fail its own verify"
    );

    // …while the exec variant's memo/file is unrelated and still valid.
    // (…trong khi memo/file của biến thể exec không liên quan và còn hợp lệ.)
    let o4 = root.join("x4").join("a");
    store.export_to(&exec_hash, &o4).unwrap();
    assert_eq!(fs::read(&o4).unwrap(), data);
}

pub(crate) fn memo_catches_file_replacement_by_rename() {
    // Replace the blob file with a DIFFERENT file (new inode) carrying
    // attacker content — inode is part of the identity, the memo dies.
    // (Thay file blob bằng file KHÁC (inode mới) mang nội dung tấn công —
    // inode nằm trong identity nên memo chết.)
    let root = tmp_store_dir("memo-rename-replace");
    let store = ContentStore::new(root.clone()).unwrap();
    let data = b"identity-rename-original".to_vec();
    let hash = store.import_bytes(&data).unwrap();

    let out1 = root.join("r1.bin");
    store.export_to(&hash, &out1).unwrap(); // verify + memoize

    // Swap the blob by rename (same length is not even needed).
    // (Đổi blob bằng rename (không cần cùng độ dài).)
    let path = hash.cas_path(&root);
    let fake = root.join(".fake-replacement");
    fs::write(&fake, b"identity-rename-attacker!").unwrap();
    fs::rename(&fake, &path).unwrap();

    let out2 = root.join("r2.bin");
    let res = store.export_to(&hash, &out2);
    assert!(
        res.is_err(),
        "file replacement by rename must invalidate the memo (inode changed)"
    );
}

pub(crate) fn verified_memo_invalidates_on_disk_mutation() {
    // Same-process scenario: export #1 verifies the blob (memo recorded);
    // an attacker then rewrites the blob on disk; export #2 must FAIL —
    // the (len, mtime) fingerprint changed, the memo is dead, the fresh
    // verify catches the poison and quarantines it.
    // Kịch bản cùng-process: export #1 verify blob (ghi memo); kẻ tấn công
    // ghi đè blob trên đĩa; export #2 phải FAIL — fingerprint (len, mtime)
    // đã đổi, memo chết, verify mới bắt được độc và cách ly.
    let root = tmp_store_dir("memo-invalidate");
    let store = ContentStore::new(root.clone()).unwrap();
    let data = b"memo-protected-payload".to_vec();
    let hash = store.import_bytes(&data).unwrap();

    let out1 = root.join("e1").join("f.bin");
    store.export_to(&hash, &out1).unwrap(); // verifies + memoizes
    assert_eq!(fs::read(&out1).unwrap(), data);

    // Tamper directly on disk (mtime + content change).
    // Sửa trực tiếp trên đĩa (mtime + nội dung đổi).
    fs::write(hash.cas_path(&root), b"POISONED-AFTER-MEMO").unwrap();

    let out2 = root.join("e2").join("f.bin");
    let res = store.export_to(&hash, &out2);
    assert!(
        res.is_err(),
        "memo must NOT survive an on-disk mutation — poisoned blob must fail export"
    );
    assert!(!out2.exists(), "no export may be produced from poison");

    // The blob was quarantined + a re-import repairs it (self-healing).
    // Blob bị cách ly + import lại sẽ tự sửa.
    let repaired = store.import_bytes(&data).unwrap();
    assert_eq!(repaired.as_hex(), hash.as_hex());
    let out3 = root.join("e3").join("f.bin");
    store.export_to(&hash, &out3).unwrap();
    assert_eq!(fs::read(&out3).unwrap(), data);
}

pub(crate) fn memo_catches_rewrite_with_identical_length() {
    // Same-length rewrite still changes mtime (write updates it on every
    // mainstream filesystem), so the memo must invalidate and the fresh
    // verify must fail-closed.
    // Ghi đè cùng độ dài vẫn đổi mtime (ghi luôn cập nhật trên mọi filesystem
    // phổ biến), nên memo phải vô hiệu và verify mới phải fail-cứng.
    let root = tmp_store_dir("memo-samelen");
    let store = ContentStore::new(root.clone()).unwrap();
    let data = b"same-length-aaaa".to_vec();
    let hash = store.import_bytes(&data).unwrap();

    let out1 = root.join("a1.bin");
    store.export_to(&hash, &out1).unwrap();

    // Overwrite with SAME length, different bytes.
    // Ghi đè CÙNG độ dài, bytes khác.
    fs::write(hash.cas_path(&root), b"same-length-bbbb").unwrap();

    let out2 = root.join("a2.bin");
    let res = store.export_to(&hash, &out2);
    assert!(
        res.is_err(),
        "same-length rewrite must be caught by the mtime half of the fingerprint"
    );
}
