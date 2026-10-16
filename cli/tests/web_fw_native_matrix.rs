#![allow(clippy::unwrap_used)]

mod common;

static WEB_FRAMEWORK_LIFECYCLE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn native_dev_budget_extends_only_windows_nuxt_and_remains_bounded() {
    assert_eq!(common::native_web_dev_timeouts("nuxt", true), (90, 95));
    for (framework, windows) in [("nuxt", false), ("node", true), ("vanilla", false)] {
        assert_eq!(
            common::native_web_dev_timeouts(framework, windows),
            (25, 30)
        );
    }
    let (startup, command) = common::native_web_dev_timeouts("nuxt", true);
    assert!(startup < command && command <= 95);
}

#[test]
fn lifecycle_evidence_destination_is_not_exposed_to_cli_children() {
    let mut options = mgc_exec::run::ExecOptions::default();
    common::configure_test_env(&mut options);

    assert!(
        options
            .env
            .iter()
            .any(|(key, value)| { key == "MGC_STRESS_LIFECYCLE_EVIDENCE" && value.is_empty() })
    );
}

fn assert_native_web_framework_lifecycle(framework: &str) {
    // Keep registry traffic bounded even when Cargo uses its default parallel
    // test threads. Recover a poisoned lock so one failed lane cannot suppress
    // evidence from the remaining frameworks.
    // Giới hạn tải registry cả khi Cargo chạy test song song. Thu hồi mutex bị
    // poison để lỗi một lane không làm mất bằng chứng của framework còn lại.
    let _guard = WEB_FRAMEWORK_LIFECYCLE_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let workspace = tempfile::tempdir().expect("create isolated framework workspace");
    let project_name = format!("runtime-{framework}");
    let args = ["create-web", framework, project_name.as_str(), "--ts"];
    let (ok, output) = common::mgc_in(workspace.path(), &args);
    assert!(ok, "{framework} scaffold failed:\n{output}");

    let project = workspace.path().join(project_name);
    assert!(
        project.is_dir(),
        "{framework} scaffold did not create project"
    );
    common::assert_native_web_framework_lifecycle(framework, &project, "web-fw-native-matrix");
}

// Generate one Rust test per framework so a failure retains evidence for every
// later lane in the matrix. — Sinh một test riêng cho mỗi framework để lỗi ở
// lane này không làm mất bằng chứng của các lane còn lại.
macro_rules! native_web_framework_test {
    ($name:ident, $framework:literal) => {
        #[test]
        fn $name() {
            assert_native_web_framework_lifecycle($framework);
        }
    };
}

native_web_framework_test!(native_web_nuxt_lifecycle, "nuxt");
native_web_framework_test!(native_web_vanilla_lifecycle, "vanilla");
native_web_framework_test!(native_web_ts_lifecycle, "ts");
native_web_framework_test!(native_web_node_lifecycle, "node");
native_web_framework_test!(native_web_express_lifecycle, "express");
native_web_framework_test!(native_web_fastify_lifecycle, "fastify");
native_web_framework_test!(native_web_hono_lifecycle, "hono");
native_web_framework_test!(native_web_nestjs_lifecycle, "nestjs");
native_web_framework_test!(native_web_qwik_lifecycle, "qwik");
native_web_framework_test!(native_web_solidjs_lifecycle, "solidjs");
native_web_framework_test!(native_web_trpc_lifecycle, "trpc");
