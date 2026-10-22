// Web scaffold smoke-test generation tests.
// Kiểm thử tạo smoke test cho scaffold Web.

#[test]
fn web_scaffold_adds_a_native_smoke_test_when_template_has_no_test_script() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("index.html"),
        "<!doctype html><html><body></body></html>",
    )
    .unwrap();

    Scaffolder::ensure_web_test_runner(root.path(), "sample").unwrap();

    let package: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.path().join("package.json")).unwrap()).unwrap();
    assert_eq!(
        package["scripts"]["test"],
        "node --test test/scaffold.test.cjs"
    );
    assert!(root.path().join("test/scaffold.test.cjs").is_file());
}
#[test]
fn web_scaffold_smoke_test_checks_local_scripts_and_ignores_external_scripts() {
    if std::process::Command::new("node")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("skipped: Node.js is required to execute the generated web smoke test");
        return;
    }

    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("index.html"),
        "<html><body><script src=\"/app.js\"></script><script src=\"https://cdn.example/app.js\"></script></body></html>",
    )
    .unwrap();
    std::fs::write(root.path().join("app.js"), "console.log('ok');\n").unwrap();
    Scaffolder::ensure_web_test_runner(root.path(), "sample").unwrap();

    let run_smoke_test = || {
        std::process::Command::new("node")
            .args(["--test", "test/scaffold.test.cjs"])
            .current_dir(root.path())
            .output()
            .unwrap()
    };
    let valid = run_smoke_test();
    assert!(
        valid.status.success(),
        "external script URLs should not be treated as local files: {}",
        String::from_utf8_lossy(&valid.stderr)
    );

    std::fs::write(
        root.path().join("index.html"),
        "<html><body><script src=\"/missing.js\"></script><script src=\"https://cdn.example/app.js\"></script></body></html>",
    )
    .unwrap();
    let invalid = run_smoke_test();
    assert!(
        !invalid.status.success(),
        "missing local script entries must fail the generated smoke test"
    );
}
#[test]
fn web_scaffold_smoke_test_accepts_typescript_node_entrypoints() {
    if std::process::Command::new("node")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("skipped: Node.js is required to execute the generated web smoke test");
        return;
    }

    let root = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(root.path().join("src")).unwrap();
    std::fs::write(
        root.path().join("src/server.ts"),
        "export const ready = true;\n",
    )
    .unwrap();
    Scaffolder::ensure_web_test_runner(root.path(), "sample").unwrap();

    let output = std::process::Command::new("node")
        .args(["--test", "test/scaffold.test.cjs"])
        .current_dir(root.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "TypeScript Node entrypoints should satisfy the scaffold smoke test: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}
#[test]
fn web_scaffold_smoke_test_accepts_framework_entrypoints() {
    if std::process::Command::new("node")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("skipped: Node.js is required to execute the generated web smoke test");
        return;
    }

    for entry in [
        "app.vue",
        "app/app.vue",
        "pages/index.vue",
        "src/pages/index.astro",
        "src/routes/+page.svelte",
        "src/root.tsx",
        "app/page.tsx",
        "app/page.jsx",
        "src/app/page.jsx",
        "app/routes/_index.tsx",
    ] {
        let root = tempfile::tempdir().unwrap();
        let entry_path = root.path().join(entry);
        std::fs::create_dir_all(entry_path.parent().unwrap()).unwrap();
        std::fs::write(&entry_path, "// generated framework entrypoint\n").unwrap();
        Scaffolder::ensure_web_test_runner(root.path(), "sample").unwrap();

        let output = std::process::Command::new("node")
            .args(["--test", "test/scaffold.test.cjs"])
            .current_dir(root.path())
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "framework entrypoint {entry} should pass the generated smoke test: {}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}
