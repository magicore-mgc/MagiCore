/// Selects the current executor tool-policy lane.
/// This scope is not a process sandbox; it does not activate every field in
/// [`ScopeConstraints`].
/// Chọn policy lane hiện tại của executor.
/// Scope này không sandbox process và không tự bật mọi trường trong
/// [`ScopeConstraints`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionScope {
    /// HIGH RISK: Package installation, fetch from registry, install scripts.
    /// npm/pnpm/yarn/bun FORBIDDEN in this scope.
    Install,

    /// Test runner execution; package managers remain forbidden.
    TestRunner,

    /// Build runner execution; package managers remain forbidden.
    BuildRunner,

    /// Dev server execution; package managers remain forbidden.
    DevServer,

    /// Explicit `mgc run` project script execution.
    /// Chạy script project tường minh qua `mgc run`.
    ScriptRunner,

    /// Verified framework initializer execution. Only exact read-only package-manager version probes are tolerated by the process monitor.
    /// Chạy initializer framework đã xác minh. Process monitor chỉ dung nạp probe version package-manager chỉ đọc chính xác.
    FrameworkInitializer,

    /// Explicit control of a local development device or emulator.
    /// Điều khiển thiết bị hoặc emulator cục bộ trong luồng phát triển.
    DeviceControl,
}

/// Descriptive policy targets associated with an execution scope.
///
/// The executor does not read this struct. In particular, these booleans do
/// not enforce cwd, network, audit or general argument restrictions. Runtime
/// enforcement remains in explicit invocation/tool checks and the caller's
/// `ExecOptions`; do not use this struct as proof that a process is sandboxed.
///
/// Các mục tiêu policy mô tả tương ứng với execution scope.
///
/// Executor không đọc struct này. Các boolean không tự ép cwd, network, audit
/// hay giới hạn đối số tổng quát. Enforcement hiện nằm ở các guard invocation/
/// tool riêng và `ExecOptions` của caller; không dùng struct này làm bằng
/// chứng rằng process đã được sandbox.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeConstraints {
    /// Desired project-root cwd restriction; not centrally enforced.
    /// Mong muốn khóa cwd ở project root; executor chưa ép buộc tập trung.
    pub cwd_locked: bool,

    /// Desired network restriction; not enforced by the current executor.
    /// Mong muốn chặn mạng; executor hiện chưa thực thi.
    pub no_network: bool,

    /// Desired fixed-argument policy; not enforced by the current executor.
    /// Mong muốn chỉ dùng argv định trước; executor hiện chưa thực thi.
    pub no_arbitrary_args: bool,

    /// Desired audit requirement; logging remains optional via `ExecOptions::log_path`.
    /// Mong muốn bắt buộc audit; hiện log là tùy chọn qua `ExecOptions::log_path`.
    pub audit_log_required: bool,

    /// Documents shell-validation intent; this flag is not read by execution code.
    /// Mô tả mục tiêu kiểm tra shell; execution code hiện không đọc cờ này.
    pub shell_injection_check: bool,
}

impl ExecutionScope {
    /// Return descriptive policy targets; this method does not enforce them.
    /// Trả về mục tiêu policy mô tả; method này không thực thi các mục tiêu đó.
    pub fn constraints(self) -> ScopeConstraints {
        match self {
            ExecutionScope::Install => ScopeConstraints {
                cwd_locked: false, // Install can run anywhere
                no_network: false, // Install needs network
                no_arbitrary_args: false,
                audit_log_required: true,
                shell_injection_check: true,
            },
            ExecutionScope::TestRunner
            | ExecutionScope::BuildRunner
            | ExecutionScope::DevServer
            | ExecutionScope::ScriptRunner
            | ExecutionScope::FrameworkInitializer
            | ExecutionScope::DeviceControl => ScopeConstraints {
                cwd_locked: true,        // Must run in project root
                no_network: false,       // Tests may need network (integration tests)
                no_arbitrary_args: true, // Only predefined commands
                audit_log_required: true,
                shell_injection_check: true,
            },
        }
    }

    /// Check if PM tools (npm/pnpm/yarn/bun) are allowed in this scope.
    pub fn allows_pm_tools(self) -> bool {
        false
    }
}
