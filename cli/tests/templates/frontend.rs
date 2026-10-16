// Shared frontend-framework E2E: scaffold, native install, frozen replay,
// `mgc test`, optional build, and a real bounded dev-server lifecycle.
// E2E frontend dùng chung: scaffold, cài native, replay frozen, `mgc test`,
// build nếu scaffold khai báo, và kiểm tra vòng đời dev server thật.

#[test]
fn test_scaffold_succeeds() {
    let name = format!("test-{FRAMEWORK}");
    let dir = common::scaffold(FRAMEWORK, &name);
    assert!(dir.exists(), "project dir {name} created");
    common::assert_file_exists(&dir, "package.json");
    common::assert_file_exists(&dir, ".gitignore");
}

#[test]
fn test_native_framework_lifecycle_succeeds() {
    let name = format!("runtime-{FRAMEWORK}");
    let dir = common::scaffold(FRAMEWORK, &name);
    common::assert_native_web_framework_lifecycle(FRAMEWORK, &dir, "web-fw-frontend-template");
}
