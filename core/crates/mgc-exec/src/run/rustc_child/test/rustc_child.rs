// Rust toolchain child recognition tests — parent trust and compiler argv shape.
// (Đảm bảo Cargo install không bị nhầm với compiler child hợp lệ.)

#[test]
fn rustc_child_requires_a_trusted_cargo_parent_but_cargo_install_is_denied() {
    use std::ffi::OsString;

    let rustc_child = [
        "rustc",
        "--crate-name",
        "probe",
        "--crate-type",
        "bin",
        "probe.rs",
        "--emit=link",
        "--out-dir",
        "/tmp/mgc-rustc-probe",
    ]
    .into_iter()
    .map(OsString::from)
    .collect::<Vec<_>>();
    // Raw classification has no parent evidence, so a compiler-shaped argv remains denied.
    // (Phân loại raw không có bằng chứng parent nên argv giống compiler vẫn bị chặn.)
    assert!(crate::run::implementation::process_policy::forbidden_process_match_argv("cargo", &rustc_child, &[]).is_some());
    assert!(super::super::is_rustc_toolchain_child(
        "cargo",
        &rustc_child,
        Some("cargo")
    ));
    assert!(super::super::is_rustc_toolchain_child(
        "cargo",
        &rustc_child,
        Some("rustdoc")
    ));
    assert!(!super::super::is_rustc_toolchain_child(
        "cargo",
        &rustc_child,
        Some("node")
    ));

    // Cargo and rustdoc can both launch the configured rustc wrapper; other
    // parent processes and incomplete compiler argv stay denied.
    // (Cargo và rustdoc đều có thể gọi rustc wrapper; parent khác và argv
    // compiler thiếu dấu hiệu vẫn bị từ chối.)
    let compiler_path = std::env::temp_dir().join("rustc");
    let cargo_wrapped_rustc = vec![
        OsString::from("cargo"),
        compiler_path.into_os_string(),
        OsString::from("--crate-type"),
        OsString::from("lib"),
        OsString::from("--color"),
        OsString::from("always"),
        OsString::from("--crate-name"),
        OsString::from("probe"),
        OsString::from("--test"),
        OsString::from("--emit=link"),
        OsString::from("--out-dir"),
        std::env::temp_dir().into_os_string(),
    ];
    assert!(
        crate::run::implementation::process_policy::forbidden_process_match_argv("cargo", &cargo_wrapped_rustc, &[]).is_some()
    );
    assert!(super::super::is_rustc_wrapper_argv(&cargo_wrapped_rustc));
    assert!(super::super::is_rustc_toolchain_child(
        "cargo",
        &cargo_wrapped_rustc,
        Some("cargo")
    ));
    assert!(!super::super::is_rustc_toolchain_child(
        "cargo",
        &cargo_wrapped_rustc,
        Some("node")
    ));
    assert!(super::super::is_rustc_toolchain_child(
        "cargo",
        &cargo_wrapped_rustc,
        Some("rustdoc")
    ));
    assert!(!super::super::is_rustc_toolchain_child(
        "cargo",
        &cargo_wrapped_rustc,
        None
    ));

    // PATH lookup may leave only the literal `rustc` operand in argv. Trust it
    // only when Cargo or rustdoc is the observed parent, never in a raw matcher.
    // (argv có thể chỉ ghi operand `rustc` do tra PATH; chỉ tin khi parent là Cargo/rustdoc.)
    let mut relative_cargo_wrapper = cargo_wrapped_rustc.clone();
    relative_cargo_wrapper[1] = OsString::from("rustc");
    assert!(!super::super::is_rustc_wrapper_argv(
        &relative_cargo_wrapper
    ));
    assert!(super::super::is_rustc_toolchain_child(
        "cargo",
        &relative_cargo_wrapper,
        Some("cargo")
    ));
    assert!(super::super::is_rustc_toolchain_child(
        "cargo",
        &relative_cargo_wrapper,
        Some("rustdoc")
    ));
    assert!(!super::super::is_rustc_toolchain_child(
        "cargo",
        &relative_cargo_wrapper,
        Some("node")
    ));
    assert!(
        crate::run::implementation::process_policy::forbidden_process_match_argv("cargo", &relative_cargo_wrapper, &[]).is_some()
    );

    // Rust test/doctest compilation may omit `--crate-type`; require its
    // observed Cargo/rustdoc parent, a test-harness flag, and a Rust source.
    // (Compiler test/doctest có thể thiếu `--crate-type`; cần parent Cargo/
    // rustdoc, cờ test harness và source Rust.)
    let cargo_test_without_crate_type = [
        "--crate-name",
        "test_lib",
        "/tmp/test-lib/src/lib.rs",
        "--test",
        "--emit=dep-info,link",
        "--out-dir",
        "/tmp/test-lib/target/debug/deps",
    ]
    .into_iter()
    .map(OsString::from)
    .collect::<Vec<_>>();
    assert!(super::super::is_rustdoc_test_compilation_argv(
        &cargo_test_without_crate_type
    ));
    assert!(super::super::is_rustc_toolchain_child(
        "cargo",
        &cargo_test_without_crate_type,
        Some("cargo")
    ));
    assert!(!super::super::is_rustc_toolchain_child(
        "cargo",
        &cargo_test_without_crate_type,
        Some("node")
    ));
    assert!(
        crate::run::implementation::process_policy::forbidden_process_match_argv("cargo", &cargo_test_without_crate_type, &[])
            .is_some()
    );
    let cargo_test_missing_source = ["--crate-name", "test_lib", "--test"]
        .into_iter()
        .map(OsString::from)
        .collect::<Vec<_>>();
    assert!(!super::super::is_rustc_toolchain_child(
        "cargo",
        &cargo_test_missing_source,
        Some("cargo")
    ));
    let cargo_rustc_test_subcommand = [
        "cargo",
        "rustc",
        "--",
        "--crate-name",
        "test_lib",
        "--test",
        "/tmp/test-lib/src/lib.rs",
    ]
    .into_iter()
    .map(OsString::from)
    .collect::<Vec<_>>();
    assert!(!super::super::is_rustc_toolchain_child(
        "cargo",
        &cargo_rustc_test_subcommand,
        Some("cargo")
    ));

    // Rustdoc doctest compilation also omits emit/output arguments; it still
    // requires a trusted parent and a compiler operand plus Rust test source.
    // (Rustdoc doctest cũng có thể thiếu emit/output; vẫn cần parent đáng tin,
    // operand compiler và source Rust dạng test.)
    let rustdoc_doctest_wrapper = vec![
        OsString::from("cargo"),
        std::env::temp_dir().join("rustc").into_os_string(),
        OsString::from("--crate-type"),
        OsString::from("bin"),
        OsString::from("--color"),
        OsString::from("always"),
        OsString::from("--crate-name"),
        OsString::from("rust_out"),
        OsString::from("--test"),
        OsString::from("/tmp/rustdoc-doctest.rs"),
    ];
    assert!(super::super::is_rustdoc_test_compilation_argv(
        &rustdoc_doctest_wrapper
    ));
    assert!(super::super::is_rustc_toolchain_child(
        "cargo",
        &rustdoc_doctest_wrapper,
        Some("rustdoc")
    ));
    assert!(super::super::is_rustc_toolchain_child(
        "cargo",
        &rustdoc_doctest_wrapper,
        Some("cargo")
    ));
    assert!(!super::super::is_rustc_toolchain_child(
        "cargo",
        &rustdoc_doctest_wrapper,
        Some("node")
    ));

    // Some OS process snapshots omit argv[0] entirely for Cargo/rustdoc children.
    // (Một số OS snapshot bỏ hẳn argv[0] của tiến trình con Cargo/rustdoc.)
    let rustdoc_doctest_without_program = [
        "--crate-type",
        "bin",
        "--crate-name",
        "rust_out",
        "--test",
        "/tmp/rustdoc-doctest.rs",
    ]
    .into_iter()
    .map(OsString::from)
    .collect::<Vec<_>>();
    assert!(super::super::is_rustdoc_test_compilation_argv(
        &rustdoc_doctest_without_program
    ));
    assert!(super::super::is_rustc_toolchain_child(
        "cargo",
        &rustdoc_doctest_without_program,
        Some("rustdoc")
    ));
    assert!(super::super::is_rustc_toolchain_child(
        "cargo",
        &rustdoc_doctest_without_program,
        Some("cargo")
    ));

    // A runner may expose a doctest compiler wrapper under a name other than rustc;
    // compiler flags directly after argv[0] are trusted only under Cargo/rustdoc.
    // (Runner có thể hiện wrapper compiler doctest dưới tên khác rustc;
    // chỉ tin flags compiler ngay sau argv[0] khi parent là Cargo/rustdoc.)
    let rustdoc_wrapped_compiler = [
        "compiler-wrapper",
        "--crate-type",
        "bin",
        "--color",
        "always",
        "--crate-name",
        "rust_out",
        "--test",
        "/tmp/rustdoc-doctest.rs",
    ]
    .into_iter()
    .map(OsString::from)
    .collect::<Vec<_>>();
    assert!(super::super::is_rustdoc_test_compilation_argv(
        &rustdoc_wrapped_compiler
    ));
    assert!(super::super::is_rustc_toolchain_child(
        "cargo",
        &rustdoc_wrapped_compiler,
        Some("rustdoc")
    ));
    assert!(super::super::is_rustc_toolchain_child(
        "cargo",
        &rustdoc_wrapped_compiler,
        Some("cargo")
    ));

    let cargo_manager_with_compiler_flags = [
        "cargo",
        "install",
        "--crate-type",
        "bin",
        "--crate-name",
        "rust_out",
        "--test",
        "/tmp/rustdoc-doctest.rs",
    ]
    .into_iter()
    .map(OsString::from)
    .collect::<Vec<_>>();
    assert!(!super::super::is_rustdoc_test_compilation_argv(
        &cargo_manager_with_compiler_flags
    ));
    assert!(!super::super::is_rustc_toolchain_child(
        "cargo",
        &cargo_manager_with_compiler_flags,
        Some("cargo")
    ));

    // OS snapshots may omit argv[0] for Cargo/rustdoc compiler children;
    // require a trusted toolchain parent plus the full compiler-only signature.
    // (Snapshot có thể bỏ argv[0]; bắt buộc parent thuộc toolchain và đủ chữ ký compiler.)
    let rustc_args_without_program = [
        "--crate-name",
        "probe",
        "--crate-type",
        "lib",
        "--emit",
        "metadata",
        "--out-dir",
        "/tmp/mgc-rustc-probe",
    ]
    .into_iter()
    .map(OsString::from)
    .collect::<Vec<_>>();
    assert!(
        crate::run::implementation::process_policy::forbidden_process_match_argv("cargo", &rustc_args_without_program, &[])
            .is_some()
    );
    assert!(super::super::is_rustc_toolchain_child(
        "cargo",
        &rustc_args_without_program,
        Some("cargo")
    ));
    assert!(super::super::is_rustc_toolchain_child(
        "cargo",
        &rustc_args_without_program,
        Some("rustdoc")
    ));
    assert!(!super::super::is_rustc_toolchain_child(
        "cargo",
        &rustc_args_without_program,
        Some("node")
    ));
    assert!(!super::super::is_rustc_toolchain_child(
        "cargo",
        &rustc_args_without_program,
        None
    ));
    let incomplete_args_without_program = ["--crate-name", "probe", "--crate-type", "lib"]
        .into_iter()
        .map(OsString::from)
        .collect::<Vec<_>>();
    assert!(!super::super::is_rustc_toolchain_child(
        "cargo",
        &incomplete_args_without_program,
        Some("cargo")
    ));

    let cargo_install = ["cargo", "install", "some-tool"]
        .into_iter()
        .map(OsString::from)
        .collect::<Vec<_>>();
    assert!(crate::run::implementation::process_policy::forbidden_process_match_argv("cargo", &cargo_install, &[]).is_some());
    assert!(!super::super::is_rustc_toolchain_child(
        "cargo",
        &cargo_install,
        Some("rustdoc")
    ));

    let incomplete_rustc_shape = ["rustc", "--crate-name", "probe"]
        .into_iter()
        .map(OsString::from)
        .collect::<Vec<_>>();
    assert!(
        crate::run::implementation::process_policy::forbidden_process_match_argv("cargo", &incomplete_rustc_shape, &[]).is_some()
    );

    let incomplete_wrapped_shape = [
        OsString::from("cargo"),
        std::env::temp_dir().join("rustc").into_os_string(),
        OsString::from("--crate-type"),
        OsString::from("lib"),
        OsString::from("--crate-name"),
        OsString::from("probe"),
    ];
    assert!(
        crate::run::implementation::process_policy::forbidden_process_match_argv("cargo", &incomplete_wrapped_shape, &[])
            .is_some()
    );
    assert!(!super::super::is_rustc_wrapper_argv(
        &incomplete_wrapped_shape
    ));

    let cargo_rustc_subcommand = [
        "cargo",
        "rustc",
        "--",
        "--crate-type",
        "lib",
        "--crate-name",
        "probe",
    ]
    .into_iter()
    .map(OsString::from)
    .collect::<Vec<_>>();
    assert!(
        crate::run::implementation::process_policy::forbidden_process_match_argv("cargo", &cargo_rustc_subcommand, &[]).is_some(),
        "Cargo's rustc subcommand must remain subject to dependency policy"
    );

    let cargo_rustc_with_compiler_flags = [
        "cargo",
        "rustc",
        "--locked",
        "--offline",
        "--",
        "--crate-type",
        "lib",
        "--crate-name",
        "probe",
        "--emit=link",
        "--out-dir",
        "/tmp/mgc-rustc-probe",
    ]
    .into_iter()
    .map(OsString::from)
    .collect::<Vec<_>>();
    assert!(
        crate::run::implementation::process_policy::forbidden_process_match_argv("cargo", &cargo_rustc_with_compiler_flags, &[])
            .is_some()
    );
    assert!(!super::super::is_rustc_toolchain_child(
        "cargo",
        &cargo_rustc_with_compiler_flags,
        Some("cargo")
    ));
}
