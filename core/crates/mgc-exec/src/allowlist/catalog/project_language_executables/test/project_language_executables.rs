use crate::allowlist::PROJECT_LANGUAGE_EXECUTABLES;

#[test]
fn runtime_catalog_contains_only_the_scoped_language_set() {
    for runtime in ["bash", "clang", "dotnet", "java", "tsx", "zig"] {
        assert!(PROJECT_LANGUAGE_EXECUTABLES.contains(&runtime));
    }
    // SEC-02: interpreters that can evaluate an inline program are scoped
    // exactly like `bash`, otherwise an install hook regains code execution.
    // (SEC-02: interpreter chạy được chương trình inline bị scope như `bash`,
    // nếu không install hook lấy lại quyền chạy code.)
    for interpreter in ["node", "python", "python3", "dart", "swift"] {
        assert!(
            PROJECT_LANGUAGE_EXECUTABLES.contains(&interpreter),
            "{interpreter} evaluates inline programs and must be scoped"
        );
    }
    for tool in ["npm", "adb", "xcrun", "cargo", "uv"] {
        assert!(!PROJECT_LANGUAGE_EXECUTABLES.contains(&tool));
    }
}
