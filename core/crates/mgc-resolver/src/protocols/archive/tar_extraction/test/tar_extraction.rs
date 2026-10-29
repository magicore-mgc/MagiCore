//! Tests safe extraction success and parent-traversal rejection.
//! Kiểm thử giải nén an toàn và từ chối đường dẫn traversal.

use super::super::super::test_support::{tar_gz_file, tar_gz_file_with_raw_path};
use super::super::super::tar_extraction::extract_tar_gz;
use crate::protocols::archive::InstallHookPolicy;
use std::path::Path;

#[test]
fn extracts_regular_files_under_the_destination() {
    let root = tempfile::tempdir().expect("test operation must succeed");
    let archive = tar_gz_file("nested/file.txt", b"content");

    extract_tar_gz(&archive, root.path(), &InstallHookPolicy::deny()).expect("test operation must succeed");

    assert_eq!(
        std::fs::read(root.path().join("nested/file.txt")).expect("test operation must succeed"),
        b"content"
    );
}

#[test]
fn rejects_parent_traversal_without_writing_outside_destination() {
    let parent = tempfile::tempdir().expect("test operation must succeed");
    let destination = parent.path().join("extract");
    let escaped = parent.path().join("escaped.txt");
    let archive = tar_gz_file_with_raw_path("../escaped.txt", b"outside");

    assert!(extract_tar_gz(&archive, &destination, &InstallHookPolicy::deny()).is_err());
    assert!(!escaped.exists());
}

#[test]
fn refuses_more_entries_than_the_shared_entry_cap() {
    // SEC-15: the resolver path had per-entry and total byte caps but no entry
    // COUNT cap, so millions of zero-length headers were pure CPU/disk work.
    // (SEC-15: đường resolver có chặn byte mỗi entry và tổng nhưng không chặn
    // SỐ entry, nên hàng triệu header rỗng chỉ tốn CPU/đĩa.)
    use super::super::super::archive_limits::MAX_TAR_ENTRIES;

    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut builder = tar::Builder::new(encoder);
    for index in 0..=MAX_TAR_ENTRIES {
        let mut header = tar::Header::new_gnu();
        header.set_size(0);
        header.set_mode(0o644);
        header.set_cksum();
        builder
            .append_data(&mut header, format!("f{index}"), std::io::empty())
            .expect("test operation must succeed");
    }
    let archive = builder
        .into_inner()
        .expect("test operation must succeed")
        .finish()
        .expect("test operation must succeed");

    let root = tempfile::tempdir().expect("test operation must succeed");
    let error = extract_tar_gz(&archive, root.path(), &InstallHookPolicy::deny())
        .err()
        .map(|error| error.to_string())
        .expect("entry cap must reject the archive");
    assert!(error.contains("entry limit"), "{error}");
}

#[test]
fn refuses_a_pth_entry_without_writing_earlier_entries() {
    use super::super::super::test_support::tar_gz_entries;

    let archive = tar_gz_entries(&[
        ("package/index.js", b"module.exports = 1;"),
        ("package/evil.pth", b"import os\n"),
    ]);
    let root = tempfile::tempdir().expect("test operation must succeed");
    let dest = root.path().join("extract");
    assert!(
        extract_tar_gz(&archive, &dest, &InstallHookPolicy::deny()).is_err(),
        "pth entry must be refused"
    );
    // Validation completes before the first write, so the safe entry from the
    // same archive must not be left behind as a partial tree.
    // (Xác minh xong trước lần ghi đầu nên entry an toàn cùng archive không
    // được để lại thành cây dở.)
    assert!(!dest.join("package/index.js").exists());
}

/// Build a raw tar header exactly as an attacker would, bypassing tar-rs's own
/// path/type validation so the extractor's checks are what is under test.
/// (Tạo header tar thô đúng như kẻ tấn công, bỏ qua kiểm tra path/type của
/// tar-rs để chính kiểm tra của extractor là thứ được test.)
fn raw_tar_gz(path: &str, link: Option<&str>, kind: tar::EntryType) -> Vec<u8> {
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut builder = tar::Builder::new(encoder);
    let path_bytes = path.as_bytes();
    assert!(path_bytes.len() <= 100, "fixture path must fit the name field");
    let mut header = tar::Header::new_gnu();
    header.as_mut_bytes()[..100].fill(0);
    header.as_mut_bytes()[..path_bytes.len()].copy_from_slice(path_bytes);
    if let Some(link) = link {
        let link_bytes = link.as_bytes();
        assert!(link_bytes.len() <= 100, "fixture link must fit the link field");
        header.as_mut_bytes()[157..257].fill(0);
        header.as_mut_bytes()[157..157 + link_bytes.len()].copy_from_slice(link_bytes);
    }
    header.set_entry_type(kind);
    header.set_size(0);
    header.set_mode(0o644);
    header.set_cksum();
    builder
        .append(&header, std::io::empty())
        .expect("test operation must succeed");
    builder
        .into_inner()
        .expect("test operation must succeed")
        .finish()
        .expect("test operation must succeed")
}

#[test]
fn rejects_absolute_and_parent_paths_written_as_raw_headers() {
    let root = tempfile::tempdir().expect("test operation must succeed");
    for path in ["/etc/pwned", "../../../etc/pwned", "a/../../b"] {
        let archive = raw_tar_gz(path, None, tar::EntryType::Regular);
        let verdict = extract_tar_gz(&archive, root.path(), &InstallHookPolicy::deny());
        assert!(verdict.is_err(), "raw path '{path}' must be rejected");
    }
    assert!(!Path::new("/etc/pwned").exists());
}

#[test]
fn rejects_symlink_and_hardlink_entries_with_outside_targets() {
    let root = tempfile::tempdir().expect("test operation must succeed");
    let outside = tempfile::tempdir().expect("test operation must succeed");
    let outside_target = outside.path().join("target");
    std::fs::write(&outside_target, b"do-not-touch").expect("test operation must succeed");
    let outside_target = outside_target.to_string_lossy().to_string();

    for kind in [tar::EntryType::Symlink, tar::EntryType::Link] {
        for link in [outside_target.as_str(), "/etc/passwd", "../../escape"] {
            let archive = raw_tar_gz("pkg/link", Some(link), kind);
            let verdict = extract_tar_gz(&archive, root.path(), &InstallHookPolicy::deny());
            assert!(verdict.is_err(), "{kind:?} -> {link} must be rejected");
        }
    }
    assert_eq!(
        std::fs::read(outside.path().join("target")).expect("test operation must succeed"),
        b"do-not-touch"
    );
    assert!(!root.path().join("pkg/link").exists());
}
