/// Language runtimes/compilers that execute project code, limited to explicit
/// project execution scopes. Install and device-control calls cannot use them.
/// (Runtime/compiler chạy code project, chỉ dùng trong scope chạy code tường
/// minh; Install và DeviceControl không được gọi.)
// Any interpreter that can evaluate an inline program (`-e`/`-c`/`-p`) or
// load a module by name must be in this list; otherwise an install hook gains
// arbitrary code execution while `bash`/`tsx` stay blocked (SEC-02).
// (Mọi interpreter có thể chạy chương trình inline (`-e`/`-c`/`-p`) hoặc nạp
// module theo tên phải có trong danh sách; nếu không, install hook có được
// arbitrary code execution trong khi `bash`/`tsx` vẫn bị chặn (SEC-02).)
pub(crate) const PROJECT_LANGUAGE_EXECUTABLES: &[&str] = &[
    "bash", "clang", "clang++", "dotnet", "gcc", "g++", "godot", "java", "javac", "kotlinc",
    "node", "php", "python", "python3", "tsx", "zig",
];
