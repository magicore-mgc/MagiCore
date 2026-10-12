//! Allowlist check — kiểm tra tool trước khi exec (00-index §5.1, §5.2)
//!
//! ## Threat Model (2026-09-02)
//!
//! MagiCore orchestrates package managers, NOT sandboxes them. Two security boundaries:
//!
//! 1. **Install scope (HIGH RISK)**: Package installation, registry fetch, transitive deps
//!    - PM tools (npm/pnpm/yarn/bun) FORBIDDEN → use `mgc install` (resolver + audit)
//!    - Rationale: Prevent arbitrary package fetch bypassing mgc resolver
//!
//! 2. **Test/Build/Dev scopes**: package-manager executables remain forbidden.
//!    Compiler/runtime execution is a separate, explicit allowlist decision.
//!
//! ## ExecutionScope
//! Package managers are forbidden in every scope. Test/build/dev may execute
//! approved compilers and runtimes, but never package resolution/install tools.

use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

/// Execution scope — determines security policy for tool execution.
/// Install scope: HIGH RISK (arbitrary package fetch, transitive deps).
/// Test/build/dev: MEDIUM RISK; DeviceControl: explicit attached-device control.
/// Test/build/dev: rủi ro trung bình; DeviceControl: quyền điều khiển thiết bị riêng.
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

    /// Explicit control of a local development device or emulator.
    /// Điều khiển thiết bị hoặc emulator cục bộ trong luồng phát triển.
    DeviceControl,
}

/// Scope constraints — security policy per execution scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeConstraints {
    /// Must run in project root (not arbitrary cwd).
    pub cwd_locked: bool,

    /// Network access forbidden (not yet enforced, roadmap).
    pub no_network: bool,

    /// Only predefined args (not yet enforced, roadmap).
    pub no_arbitrary_args: bool,

    /// Must log to audit trail.
    pub audit_log_required: bool,

    /// Validate args for shell injection.
    pub shell_injection_check: bool,
}

impl ExecutionScope {
    /// Get scope constraints for this execution scope.
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptInvocation {
    pub program: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
}

/// Tools được phép passthrough — allowlist bất biến (00-index §5.1).
/// Mỗi core khai báo subset; thêm tool phải review + ghi lý do.
pub const ALLOWED_TOOLS: &[&str] = &[
    "python3",
    // Android Debug Bridge is restricted to the dedicated DeviceControl scope.
    // ADB chỉ được dùng trong scope DeviceControl riêng, không phải scope dev chung.
    "adb",
    "pytest", // AI test runner
    // TypeScript compiler (lib/ts + web test lanes, P0 finding
    // 2026-09-12): `tsc --noEmit` is the scaffold's own typecheck test
    // script; read-only (no artifact mutation), same family as
    // cargo/pytest. web dev/build lanes already run vite/next via
    // node_modules/.bin; tsc enters the allowlist so the lib/ts
    // scaffold test lane can run honestly.
    // TypeScript compiler (lane test lib/ts + web, P0 finding
    // 2026-09-12): `tsc --noEmit` là script test typecheck của scaffold;
    // chỉ đọc (không đổi artifact), cùng họ với cargo/pytest. Lane
    // web đã chạy vite/next qua node_modules/.bin; tsc vào allowlist
    // để lane test scaffold lib/ts chạy trung thực được.
    "tsc",
    // vue-tsc: the vue scaffold's typecheck test script (same read-only
    // family as tsc — P0 finding 2026-09-12).
    // vue-tsc: script test typecheck của scaffold vue (cùng họ chỉ-đọc
    // với tsc — P0 finding 2026-09-12).
    "vue-tsc",
    // vite: the web scaffold's dev/build/preview entry (P0 finding #6,
    // 2026-09-12). Project scripts run under the user's permission per
    // the threat model (Test/Build/Dev scope: project-local scripts
    // allowed); `run`/`dev` resolve vite from node_modules/.bin FIRST
    // (the run.rs PATH insert), so the spawned program is the
    // project's own dependency, not a PATH implant.
    // vite: entry dev/build/preview của scaffold web (P0 finding #6).
    // Theo threat model, script project chạy dưới quyền user (scope
    // Test/Build/Dev: script local được phép); `run`/`dev` resolve vite
    // từ node_modules/.bin TRƯỚC (run.rs chèn đầu PATH), nên program
    // được spawn là dependency của chính project, không phải file lạ
    // xâm nhập từ PATH.
    "vite",
    "go",
    "dart",
    "gradle",
    "mvn",
    // Language runtimes and compilers — ngôn ngữ chạy và compiler native.
    // These execute project code; dependency-manager verbs remain guarded below.
    // Chúng chạy code của project; verb quản lý dependency vẫn bị policy chặn.
    "bash",
    "clang",
    "clang++",
    "dotnet",
    "gcc",
    "g++",
    "java",
    // Java compilation executes project processors; restrict it to project scopes.
    // Compiler Java có thể chạy processor của project, không dùng cho install hook.
    "javac",
    "kotlinc",
    "php",
    // NOTE: `composer` is deliberately ABSENT here — it is a PHP package
    // manager and lives in FORBIDDEN_TOOLS below. (An older revision
    // listed it in both tables; the forbidden check runs first so it was
    // never actually allowed, but the duplicate lied about the policy.)
    // (`composer` cố ý VẮNG ở đây — nó là package manager PHP, nằm ở
    // FORBIDDEN bên dưới.)
    "node",
    "swift",
    "cargo",
    "zig",
    // `rustc -vV` is a read-only host-target probe; other direct calls are rejected below.
    // `rustc -vV` chỉ đọc host target; các lời gọi trực tiếp khác bị chặn bên dưới.
    "rustc",
    "espflash",
    "west",
    "pio",
    "platformio",
    "terraform",
    "tofu",
    "cdk",
    "pulumi",
    "aws",
    "wrangler",
    "gcloud",
    "gh",
    "git",
    "docker",
    "godot",
    "flutter",
    "kotlinc",
    "python",
    "unity",
    "upm",
    "xcodebuild",
    // xcrun simctl is the iOS counterpart of adb: simulator/device
    // control only (P0 delegation finding — direct Command::new("xcrun")
    // in cli/src/commands/core/dev/app.rs bypassed the executor).
    // Restricted to DeviceControl scope like adb.
    // xcrun simctl là đối tác iOS của adb: chỉ điều khiển
    // simulator/thiết bị (P0 finding — Command::new("xcrun") trực tiếp
    // trong dev/app.rs đã bypass executor). Giới hạn scope
    // DeviceControl như adb.
    "xcrun",
    "echo", // Test tool: prove validator runs before allowlist check
    // Security scanners (audit framework, Tech Lead P1 2026-09-09):
    // official ecosystem scanners — read-only advisory lookups, no
    // package mutation, pinned versions in CI (security.yml).
    // Scanner bảo mật (audit framework): scanner chính thức của từng
    // ecosystem — chỉ tra advisory đọc-đọc, không đổi package, CI ghim
    // version (security.yml).
    "pip-audit",   // Official PyPA Python vulnerability scanner
    "govulncheck", // Official Go vulnerability scanner (golang.org/x/vuln)
];

/// Language runtimes/compilers that execute project code, limited to explicit
/// project execution scopes. Install and device-control calls cannot use them.
/// (Runtime/compiler chạy code project, chỉ dùng trong scope chạy code tường
/// minh; Install và DeviceControl không được gọi.)
const PROJECT_LANGUAGE_EXECUTABLES: &[&str] = &[
    "bash", "clang", "clang++", "dotnet", "gcc", "g++", "godot", "java", "javac", "kotlinc", "php",
    "zig",
];

/// Package-manager executables are forbidden in every execution scope.
pub const FORBIDDEN_TOOLS: &[&str] = &[
    "npm",
    "npx",
    "pnpm",
    "yarn",
    "bun",
    "bunx",
    "composer",
    "pub",
    "deno",
    "pip",
    "pip3",
    "uv",
    "uvx",
    "poetry",
    "pipenv",
    "pdm",
    "conda",
    "mamba",
    "pipx",
    "hatch",
    "rye",
    "pixi",
    "pip-compile",
    "pip-sync",
];

/// Managers that may be named by `--compat-runner` (Spec rev4 §7/§9).
/// Only JS-ecosystem managers have a compat contract today (the reported
/// Next/Angular pain); Python/Gradle/Dart/etc. need separate analysis and
/// stay rejected with a clear error until their contracts exist.
/// (Manager được `--compat-runner` nhận (Spec rev4 §7/§9). Hiện chỉ manager
/// JS có compat contract (đúng chỗ đau Next/Angular đã báo); Python/Gradle/
/// Dart... cần phân tích riêng, vẫn bị từ chối rõ ràng.)
pub const QUALIFIED_COMPAT_RUNNERS: &[&str] = &["npm", "pnpm", "yarn", "bun"];

/// Parse `--compat-runner <manager>`: lowercase basename, must be qualified.
/// `None` (flag absent) means native mode — no delegation.
/// (Parse `--compat-runner <manager>`: basename lowercase, phải qualified.
/// `None` (không truyền flag) nghĩa là native mode — không ủy quyền.)
pub fn parse_compat_runner(flag: Option<&str>) -> Result<Option<String>> {
    let Some(raw) = flag else {
        return Ok(None);
    };
    let name = raw
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(raw)
        .to_ascii_lowercase();
    let name = name
        .strip_suffix(".exe")
        .or_else(|| name.strip_suffix(".cmd"))
        .unwrap_or(&name)
        .to_string();
    if !QUALIFIED_COMPAT_RUNNERS.contains(&name.as_str()) {
        bail!(
            "--compat-runner '{raw}' is not a qualified compat manager (qualified: {}); dependency/lifecycle operations stay owned by MagiCore",
            QUALIFIED_COMPAT_RUNNERS.join(", ")
        );
    }
    Ok(Some(name))
}

/// True when `cmd` is the declared compat runner itself (basename match).
/// The subtree it spawns is governed by the compat audit-only monitor, not
/// by this name check.
/// (Đúng khi `cmd` chính là runner compat đã khai báo (khớp basename).
/// Subtree nó spawn chịu monitor audit-only compat, không chịu name check.)
pub fn compat_runner_allows(cmd: &str, runner: Option<&str>) -> bool {
    let Some(runner) = runner else {
        return false;
    };
    let base = cmd
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(cmd)
        .to_ascii_lowercase();
    let base = base
        .strip_suffix(".exe")
        .or_else(|| base.strip_suffix(".cmd"))
        .unwrap_or(&base);
    base == runner
}

/// Kiểm tool trước khi exec: cấm vĩnh viễn → lỗi rõ lý do; ngoài allowlist → lỗi.
/// DEPRECATED: Use check_tool_with_scope for new code (supports ExecutionScope).
pub fn check_tool(name: &str) -> Result<()> {
    check_tool_with_scope(name, ExecutionScope::Install, None)
}

/// Check tool with execution scope — new primary API.
/// PM tools (npm/pnpm/yarn/bun) are forbidden in every scope.
pub fn check_tool_with_scope(
    name: &str,
    scope: ExecutionScope,
    project_root: Option<&Path>,
) -> Result<()> {
    check_tool_with_scope_compat(name, scope, project_root, None)
}

/// Historical API retained for source compatibility. The compatibility
/// argument never authorizes a process; deny-list policy is enforced here,
/// beneath every CLI/router call site.
/// Giữ API cũ để tương thích source; tham số compat không cấp quyền spawn.
pub fn check_tool_with_scope_compat(
    name: &str,
    scope: ExecutionScope,
    project_root: Option<&Path>,
    _compat_runtime: Option<&str>,
) -> Result<()> {
    let name = name.trim();
    if name.is_empty() {
        bail!("tool name is empty");
    }

    let normalized = normalize_script_token(name).unwrap_or_else(|| name.to_ascii_lowercase());

    // Package managers and rival runtimes are forbidden in every scope.
    let is_pm_tool = FORBIDDEN_TOOLS.contains(&normalized.as_str());
    if is_pm_tool {
        bail!(
            "tool '{name}' is forbidden in {:?} scope; dependency operations must be owned by MagiCore",
            scope
        );
    }

    // ADB can affect attached devices; require the narrow device-control scope.
    // ADB có thể tác động thiết bị thật; bắt buộc dùng scope điều khiển riêng.
    if normalized == "adb" && scope != ExecutionScope::DeviceControl {
        bail!("tool 'adb' is only allowed in DeviceControl scope");
    }

    // xcrun drives simulators/devices; same narrow scope as adb.
    // xcrun điều khiển simulator/thiết bị; scope hẹp như adb.
    if normalized == "xcrun" && scope != ExecutionScope::DeviceControl {
        bail!("tool 'xcrun' is only allowed in DeviceControl scope");
    }

    // These runtimes execute project code; keep them out of install hooks and
    // device-control calls. `mgc run` supplies ScriptRunner explicitly.
    // (Runtime này chạy code project; không cho install hook/device-control
    // gọi. `mgc run` truyền ScriptRunner tường minh.)
    if PROJECT_LANGUAGE_EXECUTABLES.contains(&normalized.as_str())
        && !matches!(
            scope,
            ExecutionScope::TestRunner
                | ExecutionScope::BuildRunner
                | ExecutionScope::DevServer
                | ExecutionScope::ScriptRunner
        )
    {
        bail!(
            "language runtime '{name}' requires a project execution scope; it is not allowed in {scope:?}"
        );
    }

    if matches!(
        scope,
        ExecutionScope::TestRunner
            | ExecutionScope::BuildRunner
            | ExecutionScope::DevServer
            | ExecutionScope::ScriptRunner
    ) && let Some(root) = project_root
        && is_project_local_executable(name, root)
    {
        return Ok(());
    }

    // Non-PM tools: check against general allowlist
    if !ALLOWED_TOOLS.contains(&normalized.as_str()) {
        bail!(
            "tool '{name}' is not on the allowlist (00-index §5.1) — add it there only after review"
        );
    }

    Ok(())
}

/// Allow an explicitly addressed executable only when it resolves inside the project root.
/// Chỉ cho binary được gọi bằng path tường minh khi nó nằm trong project root.
fn is_project_local_executable(name: &str, project_root: &Path) -> bool {
    if !name.contains('/') && !name.contains('\\') {
        return false;
    }
    let requested = PathBuf::from(name);
    let root = match project_root.canonicalize() {
        Ok(root) => root,
        Err(_) => return false,
    };
    let candidate = if requested.is_absolute() {
        requested
    } else {
        project_root.join(requested)
    };
    let executable = match candidate.canonicalize() {
        Ok(executable) => executable,
        Err(_) => return false,
    };
    if !executable.starts_with(&root) {
        return false;
    }
    let metadata = match executable.metadata() {
        Ok(metadata) if metadata.is_file() => metadata,
        _ => return false,
    };

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode() & 0o111 != 0
    }
    #[cfg(windows)]
    {
        executable
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("exe"))
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = metadata;
        false
    }
}

/// Scoped check kept for callers that pass cwd, but PM tools stay forbidden everywhere.
/// Kiểm theo cwd vẫn tồn tại để giữ API ổn định; mọi PM ngoài vẫn bị chặn tuyệt đối.
pub fn check_tool_scoped(name: &str, cwd: Option<&Path>) -> Result<()> {
    let _ = cwd;
    let name = name.trim();
    if name.is_empty() {
        bail!("tool name is empty");
    }
    let normalized = normalize_script_token(name).unwrap_or_else(|| name.to_ascii_lowercase());
    if FORBIDDEN_TOOLS.contains(&normalized.as_str()) {
        bail!(
            "tool '{name}' is permanently forbidden (mgc resolver covers its format — use `mgc install` instead)"
        );
    }
    if !ALLOWED_TOOLS.contains(&normalized.as_str()) {
        bail!(
            "tool '{name}' is not on the allowlist (00-index §5.1) — add it there only after review"
        );
    }
    Ok(())
}

/// Historical detector kept for migration diagnostics only; it no longer grants exec bypass.
/// Bộ nhận diện cũ chỉ còn phục vụ chẩn đoán migration, không mở khóa chạy PM.
pub fn is_react_native_subdir(cwd: Option<&Path>) -> bool {
    let Some(cwd) = cwd else { return false };
    let rn_ok = cwd.join("package.json").is_file()
        && std::fs::read_to_string(cwd.join("package.json"))
            .map(|s| s.contains("\"react-native\""))
            .unwrap_or(false);
    if !rn_ok {
        return false;
    }
    let mut current = cwd.parent();
    while let Some(dir) = current {
        let mgc = dir.join("mgc.toml");
        if mgc.is_file() {
            return std::fs::read_to_string(mgc)
                .map(|s| s.contains("language = \"multi\""))
                .unwrap_or(false);
        }
        current = dir.parent();
    }
    false
}

/// Find forbidden package-manager tools anywhere in a shell-ish script.
/// Tìm tool PM bị cấm trong toàn bộ script, không chỉ token đầu tiên.
pub fn find_forbidden_tool_in_script(script: &str) -> Option<&'static str> {
    // Scan identifier boundaries inside inline code and quoted command strings,
    // not only shell whitespace; ambiguous mentions fail closed intentionally.
    // Quét ranh giới identifier cả trong code/chuỗi; trường hợp mơ hồ chủ động chặn.
    script
        .split(|c: char| {
            c.is_whitespace()
                || matches!(
                    c,
                    ';' | '&' | '|' | '(' | ')' | '<' | '>' | '\'' | '"' | '`' | '='
                )
        })
        .filter_map(normalize_script_token)
        .find_map(|token| {
            FORBIDDEN_TOOLS
                .iter()
                .copied()
                .find(|forbidden| token == *forbidden)
        })
}

/// Reject scripts that attempt to delegate back into external PMs.
/// Chặn script vòng ngược qua npm/pnpm/yarn/bun để giữ MagiCore độc lập.
pub fn reject_forbidden_pm_script(script: &str) -> Result<()> {
    if let Some(tool) = find_forbidden_tool_in_script(script) {
        bail!(
            "script delegates to forbidden package manager '{tool}'; use MagiCore-native install/run commands instead"
        );
    }
    Ok(())
}

/// Parse a simple single-command script without invoking a shell.
/// Parse script đơn lệnh để tránh shell injection trong lifecycle/task runner.
pub fn parse_simple_script(script: &str) -> Result<(String, Vec<String>)> {
    let invocation = parse_script_invocation(script)?;
    Ok((invocation.program, invocation.args))
}

/// Parse one command with optional leading KEY=value env assignments.
/// Parse lệnh đơn kèm env đầu dòng để hỗ trợ script thật mà vẫn không cần shell.
pub fn parse_script_invocation(script: &str) -> Result<ScriptInvocation> {
    reject_shell_control(script)?;
    let tokens = split_simple_words(script)?;
    let program_index = tokens
        .iter()
        .position(|token| !is_env_assignment(token))
        .ok_or_else(|| anyhow::anyhow!("script is missing a program"))?;

    let mut env = Vec::with_capacity(program_index);
    for token in &tokens[..program_index] {
        let (key, value) = parse_env_assignment(token)?;
        env.push((key.to_string(), value.to_string()));
    }

    let Some((program, args)) = tokens[program_index..].split_first() else {
        bail!("script is empty");
    };
    Ok(ScriptInvocation {
        program: program.clone(),
        args: args.to_vec(),
        env,
    })
}

/// Parse top-level AND chains without interpreting quoted or escaped operators.
/// Parse chuỗi AND ngoài quote; giữ nguyên toán tử được quote hoặc escape.
pub fn parse_script_chain(script: &str) -> Result<Vec<ScriptInvocation>> {
    let mut commands = Vec::new();
    let mut quote = None;
    let mut escaped = false;
    let mut start = 0;
    let mut chars = script.char_indices().peekable();
    while let Some((index, ch)) = chars.next() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if let Some(active) = quote {
            if ch == active {
                quote = None;
            }
            continue;
        }
        match ch {
            '\'' | '"' => quote = Some(ch),
            '&' if chars.peek().is_some_and(|(_, next)| *next == '&') => {
                commands.push(parse_script_invocation(&script[start..index])?);
                chars.next();
                start = index + 2;
            }
            _ => {}
        }
    }
    commands.push(parse_script_invocation(&script[start..])?);
    Ok(commands)
}

/// Reject shell control characters that would require `sh -c` semantics.
/// Chặn chaining/redirection/subshell để beta chạy fail-closed.
pub fn reject_shell_control(script: &str) -> Result<()> {
    let mut quote: Option<char> = None;
    let mut escaped = false;

    for ch in script.chars() {
        if escaped {
            escaped = false;
            continue;
        }

        if ch == '\\' {
            escaped = true;
            continue;
        }

        if let Some(active_quote) = quote {
            if ch == active_quote {
                quote = None;
            }
            continue;
        }

        match ch {
            '\'' | '"' => quote = Some(ch),
            ch if ch.is_control()
                || matches!(ch, ';' | '&' | '|' | '<' | '>' | '(' | ')' | '$' | '`') =>
            {
                let printable = match ch {
                    '\n' => "\\n".to_string(),
                    '\r' => "\\r".to_string(),
                    '\t' => "\\t".to_string(),
                    other => other.to_string(),
                };
                bail!(
                    "script uses unsupported shell control token '{printable}'; MagiCore runs scripts without a shell in beta"
                );
            }
            _ => {}
        }
    }
    Ok(())
}

pub(crate) fn normalize_script_token(token: &str) -> Option<String> {
    let trimmed = token
        .trim()
        .trim_matches(|c| matches!(c, '"' | '\'' | '`'))
        .trim();
    if trimmed.is_empty() {
        return None;
    }
    let base = trimmed
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(trimmed)
        .to_ascii_lowercase();
    let base = base
        .strip_suffix(".cmd")
        .or_else(|| base.strip_suffix(".exe"))
        .or_else(|| base.strip_suffix(".bat"))
        .or_else(|| base.strip_suffix(".ps1"))
        .or_else(|| base.strip_suffix(".cjs"))
        .or_else(|| base.strip_suffix(".mjs"))
        .or_else(|| base.strip_suffix(".js"))
        .or_else(|| base.strip_suffix(".phar"))
        .unwrap_or(&base)
        .to_string();
    // Recognize common executable entrypoint filenames passed through a
    // language runtime, not just direct package-manager commands.
    // Nhận diện entrypoint phổ biến chạy qua runtime, không chỉ lệnh gọi trực tiếp.
    Some(match base.as_str() {
        "npm-cli" => "npm".to_string(),
        "pnpm-cli" => "pnpm".to_string(),
        "yarn-cli" => "yarn".to_string(),
        "bun-cli" => "bun".to_string(),
        other => other.to_string(),
    })
}

fn is_env_assignment(token: &str) -> bool {
    token
        .split_once('=')
        .is_some_and(|(key, _)| is_valid_env_key(key))
}

fn parse_env_assignment(token: &str) -> Result<(&str, &str)> {
    let Some((key, value)) = token.split_once('=') else {
        bail!("invalid env assignment");
    };
    if !is_valid_env_key(key) {
        bail!("invalid env assignment key '{key}'");
    }
    Ok((key, value))
}

fn is_valid_env_key(key: &str) -> bool {
    let mut chars = key.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first.is_ascii_alphabetic()) {
        return false;
    }
    chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

fn split_simple_words(script: &str) -> Result<Vec<String>> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut escaped = false;

    for ch in script.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }

        if ch == '\\' {
            escaped = true;
            continue;
        }

        if let Some(active_quote) = quote {
            if ch == active_quote {
                quote = None;
            } else {
                current.push(ch);
            }
            continue;
        }

        match ch {
            '\'' | '"' => quote = Some(ch),
            ch if ch.is_whitespace() => {
                if !current.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
            }
            other => current.push(other),
        }
    }

    if escaped {
        bail!("script ends with an unfinished escape");
    }
    if quote.is_some() {
        bail!("script contains an unterminated quote");
    }
    if !current.is_empty() {
        words.push(current);
    }
    Ok(words)
}
