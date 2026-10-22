//! Model tree conversion tests.
//! Test chuyển đổi model tree.

use mgc_types::MgError;
use serde_json::json;

#[test]
fn keeps_only_sorted_files_and_lfs_integrity() {
    let entries = vec![
        json!({"type":"file","path":"z.bin","oid":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb","size":8}),
        json!({"type":"directory","path":"folder","oid":"cccccccccccccccccccccccccccccccccccccccc"}),
        json!({
            "type": "file",
            "path": "a.bin",
            "oid": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "size": 7,
            "lfs": {
                "oid": "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
                "size": 6
            }
        }),
    ];
    let files = super::super::parse_model_tree(
        &entries,
        "org/model",
        "0123456789abcdef0123456789abcdef01234567",
        "https://huggingface.co",
    )
    .expect("test operation must succeed");
    assert_eq!(files.len(), 2);
    assert_eq!(files[0].path, "a.bin");
    assert_eq!(
        files[0].sha256.as_deref(),
        Some("dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd")
    );
    assert_eq!(files[1].path, "z.bin");
}

#[test]
fn rejects_unsafe_path_and_malformed_lfs_oid_with_legacy_category() {
    let unsafe_entry = vec![
        json!({"type":"file","path":"../escape","oid":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}),
    ];
    let error = super::super::parse_model_tree(
        &unsafe_entry,
        "org/model",
        "0123456789abcdef0123456789abcdef01234567",
        "https://huggingface.co",
    )
    .expect_err("test operation must fail");
    assert!(
        matches!(error, MgError::Other(ref message) if message.contains("model tree path escapes the repo"))
    );

    let bad_lfs = vec![json!({
        "type": "file",
        "path": "weights.bin",
        "oid": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "lfs": {"oid": "not-a-sha256"}
    })];
    let error = super::super::parse_model_tree(
        &bad_lfs,
        "org/model",
        "0123456789abcdef0123456789abcdef01234567",
        "https://huggingface.co",
    )
    .expect_err("test operation must fail");
    assert!(matches!(
        error,
        MgError::Other(ref message)
            if message == "LFS oid for 'weights.bin' is not a sha256 (fail-closed)"
    ));
}
