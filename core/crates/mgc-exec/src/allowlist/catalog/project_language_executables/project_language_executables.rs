/// Language runtimes/compilers that execute project code, limited to explicit
/// project execution scopes. Install and device-control calls cannot use them.
/// (Runtime/compiler chạy code project, chỉ dùng trong scope chạy code tường
/// minh; Install và DeviceControl không được gọi.)
pub(crate) const PROJECT_LANGUAGE_EXECUTABLES: &[&str] = &[
    "bash", "clang", "clang++", "dotnet", "gcc", "g++", "godot", "java", "javac", "kotlinc", "php",
    "tsx", "zig",
];
