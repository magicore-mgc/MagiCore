// Marker resolution test when the ours side is empty.
// Test resolve marker khi phía ours rỗng.
use super::super::resolve_git_conflict_markers;

#[test]
fn empty_ours_side_keeps_theirs_package() {
    let content = r#"version = "2"

[metadata]
generated_at = "2026-01-01T00:00:00Z"
generator = "mgc/test"
lockfile_hash = ""

<<<<<<< ours
=======
[[package]]
name = "theirs-only"
version = "2.0.0"
resolved = "https://registry.example/theirs-only"
integrity = ""
dependencies = []
>>>>>>> theirs
"#;

    let resolved = resolve_git_conflict_markers(content).expect("valid sides should merge");

    assert_eq!(resolved.packages.len(), 1);
    assert_eq!(resolved.packages[0].name, "theirs-only");
}
