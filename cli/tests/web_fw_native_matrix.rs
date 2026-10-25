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

    // A fixture project carries no dependencies, so it cannot run the
    // framework's own build or test command. This lane therefore needs the
    // official initializer, which needs the registry.
    // Project fixture không có dependency nên không chạy được lệnh build/test
    // của framework. Lane này cần initializer chính thức, tức cần registry.
    if !common::initializer_available(framework) {
        eprintln!("skipping {framework}: needs the official initializer and a reachable registry");
        return;
    }
    let workspace = tempfile::tempdir().expect("create isolated framework workspace");
    let project_name = format!("runtime-{framework}");
    // With an official initializer, scaffold through it so the project has the
    // framework's real dependencies; the fixture path cannot run the framework's
    // own build or test command.
    // Có initializer chính thức thì scaffold qua nó để project có dependency
    // thật; đường fixture không chạy được lệnh build/test của framework.
    let source = if common::has_official_initializer("web", framework) {
        None
    } else {
        Some(common::local_source(workspace.path(), "web", framework))
    };
    // Backend frameworks are scaffolded as a backend project; `--ts` alone
    // selects the frontend initializer table.
    // Framework backend phải scaffold như project backend; chỉ `--ts` sẽ chọn
    // bảng initializer frontend.
    const BACKEND: &[&str] = &["express", "nestjs", "nest", "fastify", "hono", "trpc"];
    let mut args = vec![
        "create-web".to_string(),
        framework.to_string(),
        project_name.clone(),
    ];
    if BACKEND.contains(&framework) {
        args.push("--project-type".to_string());
        args.push("backend".to_string());
    } else {
        args.push("--ts".to_string());
    }
    if let Some(source) = &source {
        args.push(source.flag());
    }
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let (ok, output) = common::mgc_in(workspace.path(), &arg_refs);
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

// Only frameworks whose official initializer produces a project that can build
// and test itself are listed. A framework that publishes no initializer, or
// whose starter omits the toolchain its own build script needs (`vanilla` ships
// no `tsc`), would fail here for reasons unrelated to MagiCore.
// Chỉ liệt kê framework mà initializer chính thức tạo ra project tự build và
// tự test được. Framework không có initializer, hoặc starter thiếu toolchain mà
// script build của nó cần (`vanilla` không có `tsc`), sẽ fail ở đây vì lý do
// không liên quan MagiCore.
native_web_framework_test!(native_web_qwik_lifecycle, "qwik");
native_web_framework_test!(native_web_solidjs_lifecycle, "solidjs");
