//! Go go.mod native writer round-trip tests.
//! Test round-trip writer native Go go.mod.

use super::support::*;

/// go.mod writer (native add): mgc owns require pins — module/go/replace/
/// exclude lines survive, requires come from the manifest, versions always
/// v-prefixed. Round-trip parse → write → parse is stable.
/// (Writer go.mod: giữ directive, require từ manifest.)
#[test]
fn go_mod_writer_pins_require_and_round_trips() {
    use crate::manifest::write_go_mod_manifest;
    use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};

    let dir = tempfile::tempdir().unwrap();
    write_go_mod(
        dir.path(),
        "module example.com/m\n\ngo 1.21\n\nreplace example.com/old => example.com/new v1.0.0\n",
    );
    let mut manifest = Manifest::new("m", Ecosystem::Lib);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("github.com/google/uuid").unwrap(),
            VersionRange::parse("1.6.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    write_go_mod_manifest(dir.path(), &manifest).unwrap();
    let body = std::fs::read_to_string(dir.path().join("go.mod")).unwrap();
    assert!(
        body.contains("github.com/google/uuid v1.6.0"),
        "require pin must be written v-prefixed: {body}"
    );
    assert!(
        body.contains("module example.com/m") && body.contains("replace example.com/old"),
        "non-require directives must survive: {body}"
    );
    let again = parse_go_mod_manifest(dir.path()).unwrap();
    let dep = again
        .find_dep("github.com/google/uuid")
        .expect("re-parse finds uuid");
    assert_eq!(
        dep.range.satisfying_version().map(|v| v.to_string()),
        Some("1.6.0".to_string())
    );
}
