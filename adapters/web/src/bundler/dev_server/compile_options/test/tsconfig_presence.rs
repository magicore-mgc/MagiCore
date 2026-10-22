#![allow(clippy::unwrap_used)]
//! Nearest-owner compile-options regression tests.
//! Test regression gần owner compile-options.

use super::super::canonical_compile_options;
use super::support::canonical_compile_options_ok;
use tempfile::tempdir;

/// Absent tsconfig is its own namespace: a project WITHOUT tsconfig must
/// not collide with one whose tsconfig exists (empty ≠ absent).
/// (Tsconfig vắng là namespace riêng: project KHÔNG có tsconfig không
/// được đụng project có tsconfig (rỗng ≠ vắng).)
#[test]
fn absent_tsconfig_is_distinct_from_any_present_one() {
    let tmp = tempdir().unwrap();
    let with_config = tmp.path().join("with");
    let without_config = tmp.path().join("without");
    std::fs::create_dir_all(with_config.join("src")).unwrap();
    std::fs::create_dir_all(without_config.join("src")).unwrap();
    std::fs::write(
        with_config.join("tsconfig.json"),
        r#"{"compilerOptions":{"target":"ES2020"}}"#,
    )
    .unwrap();

    let opts_with = canonical_compile_options_ok(&with_config, &with_config.join("src/a.ts"));
    let opts_without =
        canonical_compile_options_ok(&without_config, &without_config.join("src/a.ts"));
    assert_ne!(
        opts_with, opts_without,
        "a project with tsconfig must never share a key with one without (P0-2)"
    );
}

/// Fail-closed tsconfig (Gate 11-C, vòng-11): a tsconfig that EXISTS but
/// cannot be read (chmod 000) must FAIL canonical options — never fold
/// into "absent" and never poison-and-compile. Skipped where chmod is not
/// enforceable (Windows/root).
/// (Tsconfig fail-closed: tsconfig TỒN TẠI nhưng không đọc được (chmod
/// 000) phải FAIL options — không gộp vào "absent" và không
/// poison-rồi-biên-dịch. Bỏ qua khi chmod không hiệu lực (Windows/root).)
#[cfg(unix)]
#[test]
fn unreadable_tsconfig_fails_options_fail_closed() {
    let tmp = tempdir().unwrap();
    let project = tmp.path();
    std::fs::create_dir_all(project.join("src")).unwrap();
    std::fs::write(project.join("tsconfig.json"), r#"{"compilerOptions":{}}"#).unwrap();
    let mode_before = {
        use std::os::unix::fs::PermissionsExt;
        std::fs::metadata(project.join("tsconfig.json"))
            .unwrap()
            .permissions()
            .mode()
    };
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(project.join("tsconfig.json"))
            .unwrap()
            .permissions();
        perms.set_mode(0o000);
        std::fs::set_permissions(project.join("tsconfig.json"), perms).unwrap();
    }
    let result = canonical_compile_options(project, &project.join("src/a.ts"));
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(project.join("tsconfig.json"))
            .unwrap()
            .permissions();
        perms.set_mode(mode_before);
        std::fs::set_permissions(project.join("tsconfig.json"), perms).unwrap();
    }
    assert!(
        result.is_err(),
        "an unreadable tsconfig must fail canonical options (fail-closed), got {:?}",
        result
    );
}
