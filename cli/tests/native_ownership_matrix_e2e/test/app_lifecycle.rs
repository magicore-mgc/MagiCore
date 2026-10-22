//! App Lifecycle lifecycle E2E cases. / Test E2E lifecycle cho app lifecycle.

use crate::native_ownership_matrix_e2e::support::{HermeticPub, MatrixSandbox, flutter_project};
use tempfile::TempDir;

pub(crate) fn matrix_app_flutter_install() {
    let project = TempDir::new().unwrap();
    flutter_project(project.path());
    let sandbox = MatrixSandbox::new();
    let pub_registry = HermeticPub::new("meta", "1.13.0");
    let env = [("MGC_PUB_INDEX_URL", pub_registry.url.as_str())];
    let (code, out) = sandbox.run_with_env(&["install-app"], project.path(), &env);
    assert_eq!(code, Some(0), "install-app must succeed:\n{out}");
    sandbox.assert_no_spawn("native Flutter install-app");
    pub_registry.assert_all_routes_used();
    sandbox.assert_no_spawn("install-app");
    let lock = std::fs::read_to_string(project.path().join("mgc.lock")).unwrap_or_default();
    assert!(lock.contains("meta"), "mgc.lock must record meta:\n{lock}");
}
