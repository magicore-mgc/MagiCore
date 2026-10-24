// ponytail: single template reused across all backend frameworks.
// Including file MUST define `const FRAMEWORK: &str` before include.

#[test]
fn test_scaffold_succeeds() {
    if !common::initializer_available(FRAMEWORK) {
        eprintln!(
            "skipping {FRAMEWORK}: its official initializer needs a toolchain this machine lacks"
        );
        return;
    }
    let name = format!("test-{FRAMEWORK}");
    let dir = common::scaffold(FRAMEWORK, &name);
    assert!(dir.exists(), "project dir {name} created");
    // Each backend framework generates different files; just verify scaffold itself passes.
    common::assert_file_exists(&dir, ".gitignore");
    common::assert_file_exists(&dir, "README.md");
}

#[test]
fn test_scaffold_speed() {
    if !common::initializer_available(FRAMEWORK) {
        eprintln!(
            "skipping {FRAMEWORK} speed: its official initializer needs a toolchain this machine lacks"
        );
        return;
    }
    let name = format!("speed-{FRAMEWORK}");
    let ms = common::bench_scaffold(FRAMEWORK, &name);
    // ponytail: 90s ceiling for parallel test runs on CI/dev machines
    assert!(ms < 90000, "scaffold {FRAMEWORK} took {ms}ms (limit 90000)");
}
