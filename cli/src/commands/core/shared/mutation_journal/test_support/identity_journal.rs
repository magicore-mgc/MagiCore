//! Construct explicit project-identity journals for tests.
//! Tạo journal identity tường minh cho test.

/// Craft a v4 journal with an explicit owner identity (no staging
/// needed — identity is verified before any backup is touched). A
/// matching project.id is written so the check under test is the
/// IDENTITY, not the lineage.
/// (Dựng journal v4 + project.id khớp — test identity thuần.)
pub(crate) fn craft_identity_journal(
    root: &std::path::Path,
    core: &str,
    language: &str,
    format: &str,
    relpath: &str,
) {
    const FIXED_ID: &str = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
    std::fs::write(root.join(".magicore/project.id"), FIXED_ID).unwrap();
    let dir = root.join(".magicore/journal/dependency-mutation");
    std::fs::create_dir_all(&dir).unwrap();
    let canonical = root.canonicalize().unwrap().display().to_string();
    let journal = serde_json::json!({
        "v": 4,
        "op": "remove",
        "pid": 4242,
        "packages": ["x"],
        "lock_existed": false,
        "identity": {
            "core": core,
            "language": language,
            "format": format,
            "relpath": relpath,
        },
        "project_root": canonical,
        "project_id": FIXED_ID,
        "pre_digest": [],
        "post_digest": null,
        "state": "in_progress",
    });
    std::fs::write(
        dir.join("journal.json"),
        serde_json::to_string_pretty(&journal).unwrap(),
    )
    .unwrap();
}
