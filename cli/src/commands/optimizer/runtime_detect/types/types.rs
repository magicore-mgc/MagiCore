//! Shared runtime result types for core detection owners.

/// Detected runtime environment for a project — môi trường runtime đã phát hiện cho project
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectedRuntime {
    // Web runtimes — runtime web
    NodeJs {
        package_manager: PackageManager,
    },
    Deno,
    Bun,

    // AI runtimes — runtime AI
    PythonPyTorch,
    RustCandle,
    GoTensorFlow,

    // Lib runtimes — runtime thư viện
    RustLib,
    GoLib,
    PythonLib,
    TypeScriptLib,
    /// JVM toolchain (Maven or Gradle). JVM tuning is expressed as MAVEN_OPTS /
    /// GRADLE_OPTS, so the two build programs share one runtime.
    /// (Toolchain JVM (Maven hoặc Gradle) — tuning JVM qua MAVEN_OPTS/GRADLE_OPTS
    /// nên hai chương trình build dùng chung runtime.)
    JvmLib,
    /// .NET SDK (`csproj`/`sln`). Tuning is expressed as MSBuild/DOTNET
    /// environment settings.
    /// (.NET SDK — tuning qua MSBuild/DOTNET env.)
    DotNetLib,
    /// SwiftPM (`Package.swift`).
    /// (SwiftPM.)
    SwiftLib,

    // App runtimes — runtime ứng dụng
    Flutter,
    ReactNative,
    RustNative,

    /// A project whose toolchain has no runtime we tune by name (Godot, hardware
    /// marker projects, CICD pipeline-only repos).
    ///
    /// The optimizer can still do real work for these: a hardware profile and a
    /// GPU snapshot are properties of the MACHINE, not of the language, and every
    /// build benefits from them. `Unknown` is different — it means "nothing to
    /// optimize" and makes `mgc optimizer` refuse, which is why these cores used
    /// to fail with "No runtime detected" while `web`/`app` succeeded.
    /// (Project mà toolchain không có runtime đánh tên. Optimizer vẫn làm được
    /// việc thật: profile phần cứng và snapshot GPU là thuộc tính của MÁY, không
    /// của ngôn ngữ. `Unknown` khác — nó nghĩa "không có gì để tối ưu" và khiến
    /// `mgc optimizer` từ chối.)
    GenericToolchain,

    // Fallback — dự phòng
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
    Bun,
    Deno,
}
