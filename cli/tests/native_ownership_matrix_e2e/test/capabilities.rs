//! All-core capability manifest E2E cases. / Test E2E manifest capability cho mọi core.

use crate::native_ownership_matrix_e2e::support::{MatrixSandbox, write};
use tempfile::TempDir;

pub(crate) fn capabilities_accepts_explicit_json_format_flag() {
    let cwd = TempDir::new().unwrap();
    let sandbox = MatrixSandbox::new();
    let (code, output) = sandbox.run(&["capabilities", "--json"], cwd.path());

    assert_eq!(
        code,
        Some(0),
        "mgc capabilities --json must succeed:\n{output}"
    );
    sandbox.assert_no_spawn("mgc capabilities --json");
    let document: serde_json::Value = serde_json::from_str(&output)
        .unwrap_or_else(|error| panic!("capabilities must emit valid JSON: {error}\n{output}"));
    assert!(document["cores"].is_array());
}

pub(crate) fn matrix_capabilities_binary_emits_closed_owner_for_every_framework_operation() {
    let cwd = TempDir::new().unwrap();
    let sandbox = MatrixSandbox::new();
    let (code, output) = sandbox.run(&["capabilities"], cwd.path());
    assert_eq!(code, Some(0), "mgc capabilities must succeed:\n{output}");
    sandbox.assert_no_spawn("mgc capabilities");

    let document: serde_json::Value = serde_json::from_str(&output)
        .unwrap_or_else(|error| panic!("capabilities must emit valid JSON: {error}\n{output}"));
    let cores = document["cores"]
        .as_array()
        .expect("all-core capabilities must be an array");
    let required_cores = [
        "ai", "app", "cicd", "clo", "game", "hardware", "iot", "lib", "web",
    ];
    for core in required_cores {
        let row = cores
            .iter()
            .find(|row| row["core"] == core)
            .unwrap_or_else(|| panic!("mgc capabilities omitted core {core}"));
        let frameworks = row["framework_qualification"]
            .as_array()
            .unwrap_or_else(|| panic!("{core} framework qualification must be an array"));
        for framework in frameworks {
            let framework_name = framework["framework"].as_str().unwrap_or("<missing>");
            let ownership = framework["dependency_ownership"]
                .as_object()
                .unwrap_or_else(|| panic!("{core}/{framework_name} has no operation ownership"));
            for operation in [
                "install",
                "add",
                "remove",
                "update",
                "list",
                "resolve",
                "lock",
                "fetch",
                "verify",
                "store",
                "materialize",
                "frozen-install",
                "offline-reinstall",
                "gc",
            ] {
                let cell = ownership.get(operation).unwrap_or_else(|| {
                    panic!("{core}/{framework_name} missing ownership cell {operation}")
                });
                assert!(
                    matches!(
                        cell["owner"].as_str(),
                        Some("mgc-native" | "scaffold-only" | "unsupported")
                    ),
                    "{core}/{framework_name}/{operation} has invalid owner: {cell}"
                );
            }
        }
    }
    let ai = cores.iter().find(|row| row["core"] == "ai").unwrap();
    let ai_ownership = &ai["dependency_ownership"];
    assert_eq!(
        ai_ownership["operations"]["install"]["owner"], "unsupported",
        "AI must not claim a language-wide native owner"
    );
    assert_eq!(
        ai_ownership["manifest_requirements"]["python"], "mgc-pyproject",
        "AI native ownership must publish its required manifest condition"
    );
    assert_eq!(
        ai_ownership["manifest_variants"]["python/mgc-pyproject"]["install"]["owner"],
        "mgc-native"
    );
    for operation in ["add", "remove", "update", "list"] {
        assert_eq!(
            ai_ownership["manifest_variants"]["python/mgc-pyproject"][operation]["owner"],
            "mgc-native",
            "MGC-owned AI Python manifest should expose native {operation}"
        );
        assert_eq!(
            ai_ownership["manifest_variants"]["python/foreign-lock"][operation]["owner"],
            "unsupported",
            "foreign-lock AI Python must not expose native {operation}"
        );
    }
    let cloud = cores.iter().find(|row| row["core"] == "clo").unwrap();
    let cdk = cloud["framework_qualification"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["framework"] == "cdk")
        .unwrap();
    assert_eq!(
        cdk["dependency_ownership"]["install"]["requires"],
        "package.json; embedded MGC JavaScript engine"
    );
}
