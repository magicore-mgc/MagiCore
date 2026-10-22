//! Web backend fixtures and non-empty install scenario. / Fixture backend Web và kịch bản install có dependency.

use crate::native_ownership_matrix_e2e::support::{MatrixSandbox, write};
use tempfile::TempDir;

pub(crate) fn web_backend_project(
    dir: &std::path::Path,
    language: &str,
    framework: &str,
    manifest_name: &str,
    manifest_body: &str,
    schema: &str,
) {
    write(dir, ".mgc.core", "web\n");
    write(
        dir,
        "mgc.toml",
        &format!(
            "name = \"m\"\nversion = \"0.1.0\"\necosystem = \"web\"\nmode = \"backend\"\nframeworks = [\"{language}\", \"{framework}\"]\n\n[lock]\nschema = \"{schema}\"\n"
        ),
    );
    write(dir, manifest_name, manifest_body);
}

pub(crate) struct WebBackendCase<'a> {
    pub(crate) language: &'a str,
    pub(crate) framework: &'a str,
    pub(crate) manifest_name: &'a str,
    pub(crate) manifest_body: &'a str,
    pub(crate) schema: &'a str,
    pub(crate) package: &'a str,
    pub(crate) manifest_pin: &'a str,
    pub(crate) lock_pin: &'a str,
}

pub(crate) fn web_backend_nonempty_add_install(case: WebBackendCase<'_>, env: &[(&str, &str)]) {
    let WebBackendCase {
        language,
        framework,
        manifest_name,
        manifest_body,
        schema,
        package,
        manifest_pin,
        lock_pin,
    } = case;
    let project = TempDir::new().unwrap();
    web_backend_project(
        project.path(),
        language,
        framework,
        manifest_name,
        manifest_body,
        schema,
    );
    let sandbox = MatrixSandbox::new();
    let (code, out) = sandbox.run_with_env(&["add-web", package], project.path(), env);
    assert_eq!(
        code,
        Some(0),
        "web/{framework} add-web {package} ({schema}) must succeed:\n{out}"
    );
    let manifest = std::fs::read_to_string(project.path().join(manifest_name)).unwrap();
    assert!(
        manifest.contains(manifest_pin),
        "web/{framework} manifest must contain {manifest_pin}:\n{manifest}"
    );
    let lock_path = project.path().join("mgc.lock");
    let lock = std::fs::read_to_string(&lock_path).unwrap_or_default();
    assert!(
        lock.contains(lock_pin),
        "web/{framework} lock must contain {lock_pin}:\n{lock}"
    );
    let expected_version = if schema == "v4" { 4 } else { 3 };
    assert_eq!(
        mgc_lockfile::migrate::detect_lockfile_version(&lock).expect("detect lock schema"),
        expected_version,
        "web/{framework} wrote the wrong lock schema"
    );
    assert_eq!(
        std::fs::read_to_string(project.path().join(".mgc.core")).unwrap(),
        "web\n",
        "web/{framework} add changed the project owner"
    );
    sandbox.assert_no_spawn(&format!("web/{framework} add-web"));

    for args in [&["install-web"][..], &["install-web", "--frozen"][..]] {
        let (code, out) = sandbox.run_with_env(args, project.path(), env);
        assert_eq!(
            code,
            Some(0),
            "web/{framework} {} ({schema}) must succeed:\n{out}",
            args.join(" ")
        );
        sandbox.assert_no_spawn(&format!("web/{framework} {}", args.join(" ")));
        assert_eq!(
            std::fs::read_to_string(project.path().join(".mgc.core")).unwrap(),
            "web\n",
            "web/{framework} install changed the project owner"
        );
    }
}
