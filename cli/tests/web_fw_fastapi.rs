#![allow(clippy::unwrap_used)]

mod common;
const FRAMEWORK: &str = "fastapi";
include!("templates/backend.rs");

#[test]
fn test_native_dev_server_lifecycle() {
    let project = common::scaffold(FRAMEWORK, "runtime-fastapi");
    let (ok, output) = common::mgc_in(&project, &["install"]);
    assert!(ok, "FastAPI native dependency install failed:\n{output}");
    common::assert_native_web_dev_server_lifecycle(
        FRAMEWORK,
        &project,
        "web-fw-fastapi-dev-native",
    );
}
