//! Tests source lockfile attribution for imported rows.
//! Kiểm tra ghi nhận lockfile nguồn cho row được import.

use crate::{
    EcosystemTag, SOURCE_KIND_REGISTRY_IMPORT, import_file, verify_ownership_completeness,
};

#[test]
fn imported_packages_keep_source_file_and_pass_classified_gate() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let path = directory.path().join("bun.lock");
    std::fs::write(
        &path,
        r#"{"lockfileVersion":1,"packages":{"foo@1.0.0":["foo@1.0.0"]}}"#,
    )
    .expect("test operation must succeed");

    let (mut lockfile, report) = import_file(&path).expect("test operation must succeed");
    assert_eq!(report.source_file, "bun.lock");
    assert_eq!(lockfile.packages.len(), 1);
    let package = &mut lockfile.packages[0];
    assert_eq!(
        package
            .provenance
            .as_ref()
            .map(|provenance| provenance.source_kind.as_str()),
        Some(SOURCE_KIND_REGISTRY_IMPORT)
    );
    assert_eq!(
        package
            .provenance
            .as_ref()
            .and_then(|provenance| provenance.imported_from.as_deref()),
        Some("bun.lock")
    );

    package.ecosystem = EcosystemTag::Web;
    assert!(verify_ownership_completeness(&lockfile).is_ok());
}

// ── Imported root graph ─────────────────────────────────────────────────
// An imported lock that records no roots makes every dependency look transitive:
// `mgc why` reports the declared package as an orphan and `freshness` cannot
// verify it. npm and pnpm state the roots explicitly; yarn classic and bun.lock
// do not, so `package.json` beside the lock is their only authority.
// (Lock import không ghi root khiến mọi dependency trông như bắc cầu. npm/pnpm
// nêu root rõ; yarn classic/bun.lock không, nên `package.json` là nguồn duy nhất.)

use std::fs;

fn write(dir: &std::path::Path, name: &str, body: &str) {
    fs::write(dir.join(name), body).expect("write fixture");
}

/// The npm root entry supplies direct dependencies, resolved to exact versions.
/// (Entry gốc npm cung cấp dependency trực tiếp, resolve về version chính xác.)
#[test]
fn npm_import_records_resolved_root_dependencies() {
    let dir = tempfile::tempdir().expect("temp dir");
    write(
        dir.path(),
        "package-lock.json",
        r#"{"lockfileVersion":3,"packages":{
            "":{"name":"p","dependencies":{"nanoid":"^5"},"devDependencies":{"vitest":"^2"}},
            "node_modules/nanoid":{"version":"5.1.16"},
            "node_modules/vitest":{"version":"2.1.8"}
        }}"#,
    );

    let (lockfile, _report) =
        import_file(&dir.path().join("package-lock.json")).expect("import npm lock");

    assert!(
        lockfile
            .root_dependencies
            .contains(&"nanoid@5.1.16".to_string()),
        "root must record the resolved version, not the range: {:?}",
        lockfile.root_dependencies
    );
    assert!(
        lockfile
            .root_dependencies
            .contains(&"vitest@2.1.8".to_string()),
        "devDependencies are direct too: {:?}",
        lockfile.root_dependencies
    );
}

/// A transitive-only entry never becomes a root.
/// (Entry chỉ bắc cầu không bao giờ thành root.)
#[test]
fn npm_import_does_not_promote_transitive_packages() {
    let dir = tempfile::tempdir().expect("temp dir");
    write(
        dir.path(),
        "package-lock.json",
        r#"{"lockfileVersion":3,"packages":{
            "":{"name":"p","dependencies":{"nanoid":"^5"}},
            "node_modules/nanoid":{"version":"5.1.16"},
            "node_modules/deep-only":{"version":"1.0.0"}
        }}"#,
    );

    let (lockfile, _report) = import_file(&dir.path().join("package-lock.json")).expect("import");

    assert!(
        lockfile
            .root_dependencies
            .contains(&"nanoid@5.1.16".to_string())
    );
    assert!(
        !lockfile
            .root_dependencies
            .iter()
            .any(|root| root.starts_with("deep-only")),
        "a package the manifest does not declare must not be a root: {:?}",
        lockfile.root_dependencies
    );
}

/// pnpm importers."." supplies direct dependencies with resolved versions.
/// (importers."." của pnpm cung cấp dependency trực tiếp kèm version đã resolve.)
#[test]
fn pnpm_import_records_root_dependencies() {
    let dir = tempfile::tempdir().expect("temp dir");
    write(
        dir.path(),
        "pnpm-lock.yaml",
        "lockfileVersion: '9.0'\nimporters:\n  .:\n    dependencies:\n      nanoid:\n        specifier: ^5\n        version: 5.1.16\npackages:\n  nanoid@5.1.16: {}\n",
    );

    let (lockfile, _report) =
        import_file(&dir.path().join("pnpm-lock.yaml")).expect("import pnpm lock");

    assert!(
        lockfile
            .root_dependencies
            .contains(&"nanoid@5.1.16".to_string()),
        "pnpm root must record the resolved version: {:?}",
        lockfile.root_dependencies
    );
}

/// yarn classic has no root information, so `package.json` provides it.
/// (yarn classic không có thông tin root nên `package.json` cung cấp.)
#[test]
fn yarn_import_reads_roots_from_the_manifest() {
    let dir = tempfile::tempdir().expect("temp dir");
    write(
        dir.path(),
        "package.json",
        r#"{"name":"y","version":"1.0.0","dependencies":{"nanoid":"^5"}}"#,
    );
    write(
        dir.path(),
        "yarn.lock",
        "nanoid@^5:\n  version \"5.1.16\"\n",
    );

    let (lockfile, _report) = import_file(&dir.path().join("yarn.lock")).expect("import yarn lock");

    // A qualified pin, not a bare name: consumers (`freshness`, `merge`) reject a
    // root without `@version`, so a bare name made every direct dependency fail
    // freshness right after a successful import.
    // (Pin có định danh, không phải tên trần: consumer từ chối root thiếu
    // `@version`.)
    assert!(
        lockfile
            .root_dependencies
            .iter()
            .any(|root| root == "mgc-root-v1:web:nanoid@5.1.16"),
        "yarn root must be a qualified pin with the locked version: {:?}",
        lockfile.root_dependencies
    );
}

/// A `package.json` symlinked from outside the project is refused, loudly.
///
/// The manifest is read with the same bounded, no-follow reader as the lockfile.
/// Reading it directly followed the link, so a file outside the project supplied
/// dependency names that went straight into `mgc.lock`.
/// (Manifest đọc bằng reader bounded/no-follow như lockfile. Đọc trực tiếp sẽ theo
/// symlink, nên file ngoài project cấp tên dependency đi thẳng vào `mgc.lock`.)
#[cfg(unix)]
#[test]
fn a_symlinked_manifest_is_refused_with_a_warning() {
    let outside = tempfile::tempdir().expect("outside");
    fs::write(
        outside.path().join("package.json"),
        r#"{"name":"evil","dependencies":{"evil-pkg":"1"}}"#,
    )
    .expect("write outside manifest");

    let project = tempfile::tempdir().expect("project");
    std::os::unix::fs::symlink(
        outside.path().join("package.json"),
        project.path().join("package.json"),
    )
    .expect("symlink");
    fs::write(
        project.path().join("yarn.lock"),
        "evil-pkg@1:\n  version \"1.0.0\"\n",
    )
    .expect("write lock");

    let (lockfile, report) =
        import_file(&project.path().join("yarn.lock")).expect("import must still succeed");

    assert!(
        lockfile.root_dependencies.is_empty(),
        "a symlinked manifest must not contribute roots: {:?}",
        lockfile.root_dependencies
    );
    assert!(
        report
            .warnings
            .iter()
            .any(|warning| warning.contains("root graph")),
        "the refusal must be reported, not silent: {:?}",
        report.warnings
    );
}

/// A root declared in the manifest but absent from the lock is reported.
/// (Root khai trong manifest nhưng vắng trong lock được báo.)
#[test]
fn a_declared_package_missing_from_the_lock_is_reported() {
    let dir = tempfile::tempdir().expect("temp dir");
    write(
        dir.path(),
        "package.json",
        r#"{"name":"y","version":"1.0.0","dependencies":{"ghost":"^1"}}"#,
    );
    write(
        dir.path(),
        "yarn.lock",
        "nanoid@^5:\n  version \"5.1.16\"\n",
    );

    let (_lockfile, report) = import_file(&dir.path().join("yarn.lock")).expect("import");

    assert!(
        report
            .warnings
            .iter()
            .any(|warning| warning.contains("ghost")),
        "an unresolved declared dependency must be named: {:?}",
        report.warnings
    );
}
