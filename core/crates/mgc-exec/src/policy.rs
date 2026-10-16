//! Policy resolver — typed allow/self-answer/deny/compat-audit decisions.
//! Bộ phân giải policy — quyết định typed cho phép/tự trả lời/từ chối/compat-audit.
//!
//! Implements the runtime rows of the dev-exec decision table (Spec rev4 §7):
//! native mutation-deny wins over profile, exception and local config; only an
//! explicit compat opt-in takes the compat route.
//! (Hiện thực các hàng runtime của decision table dev-exec (Spec rev4 §7):
//! native mutation-deny thắng profile, exception và config local; chỉ opt-in
//! compat tường minh mới đi đường compat.)
//!
//! Verb tables here mirror the enforcement twin in `run.rs`
//! (`reject_external_dependency_resolution`) without touching it — that file
//! has concurrent local edits. Deduplicate the two tables once the tree is
//! clean (follow-up, not this slice).
//! (Bảng động từ ở đây mirror bản enforcement trong `run.rs`
//! (`reject_external_dependency_resolution`) mà không đụng tới nó — file đó
//! đang có sửa local song song. Gộp hai bảng khi cây sạch (việc tiếp theo).)

use crate::allowlist::FORBIDDEN_TOOLS;
use serde::{Deserialize, Serialize};

/// Policy mode for one invocation — chế độ policy cho một lần gọi.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyMode {
    /// Native MagiCore engine — the default, no delegation.
    /// Engine native MagiCore — mặc định, không ủy quyền.
    Native,
    /// Explicit compat runner subtree (e.g. `--compat-runner npm`).
    /// Cây compat tường minh (vd `--compat-runner npm`).
    CompatRunner {
        /// Declared manager name, lowercase basename (e.g. `"npm"`).
        /// Tên manager đã khai báo, basename lowercase (vd `"npm"`).
        manager: String,
    },
}

/// Machine-readable deny reason — mã lý do từ chối đọc được bằng máy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DenyCode {
    /// Mutation verb in native mode (decision table row 4).
    /// Động từ mutation ở native mode (hàng 4 decision table).
    NativeMutationDeny,
    /// Profiled lane, PM invocation outside the active profile (row 6).
    /// Lane đã profile, invocation PM ngoài profile hiệu lực (hàng 6).
    OutsideActiveProfile,
    /// Unprofiled lane spawning a forbidden PM without an exact exception.
    /// Lane chưa profile spawn PM cấm mà không có exception chính xác.
    ForbiddenNameNoException,
    /// No exact read-only exception covers this invocation (row 5).
    /// Không có exception đọc nào khớp invocation này (hàng 5).
    MissingException,
}

/// Typed policy outcome — kết quả policy typed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    /// Ordinary execution; the caller still audit-logs it.
    /// Thực thi thông thường; caller vẫn ghi audit.
    Allow,
    /// MGC answers the probe itself; nothing is spawned.
    /// MGC tự trả lời probe; không spawn gì.
    SelfAnswer(String),
    /// Fail-closed with a reason code.
    /// Chặn cứng kèm mã lý do.
    Deny {
        /// Why this invocation is refused — vì sao invocation bị từ chối.
        code: DenyCode,
    },
    /// Allowed inside the declared compat subtree; caller must warn + audit.
    /// Được phép trong subtree compat đã khai báo; caller phải cảnh báo + audit.
    CompatAudit {
        /// Human-readable note for the audit log (English).
        /// Ghi chú cho audit log (tiếng Anh).
        note: String,
    },
}

/// One process invocation to judge — một lần gọi process cần phán quyết.
#[derive(Debug, Clone)]
pub struct Invocation {
    /// Program as invoked (basename or path) — chương trình được gọi.
    pub program: String,
    /// Arguments as invoked — tham số được gọi.
    pub args: Vec<String>,
    /// Explicit env for flag/env-gated verbs (GOPROXY, `--offline`...).
    /// Env tường minh cho verb phụ thuộc flag/env (GOPROXY, `--offline`...).
    pub env: Vec<(String, String)>,
    /// True when spawned under the declared compat runner.
    /// Đúng khi được spawn dưới runner compat đã khai báo.
    pub in_compat_subtree: bool,
}

/// Active spawn-profile of a lane — profile hiệu lực của một lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneProfile {
    /// Framework name (e.g. `"next"`) — tên framework (vd `"next"`).
    pub framework: String,
    /// Locked framework version — version framework đã khóa.
    pub version: String,
    /// Operating system (e.g. `"darwin"`) — hệ điều hành (vd `"darwin"`).
    pub os: String,
    /// Operation (e.g. `"dev"`) — thao tác (vd `"dev"`).
    pub operation: String,
    /// Exact `"program args..."` strings observed as evidence.
    /// Chuỗi `"program args..."` chính xác đã quan sát làm bằng chứng.
    pub allowed_pm_invocations: Vec<String>,
    /// Package managers this lane observably requires at runtime
    /// (e.g. Angular needing `npm` at build). Non-empty means native mode
    /// refuses upfront: the lane needs `--compat-runner` or a native
    /// alternative (Spec rev4 §3.1 row 3).
    /// Package manager lane này chắc chắn cần lúc runtime (vd Angular cần
    /// `npm` lúc build). Không rỗng nghĩa là native mode từ chối upfront:
    /// lane cần `--compat-runner` hoặc đường native khác (Spec rev4 §3.1).
    #[serde(default)]
    pub requires_pm: Vec<String>,
}

/// Lane execution context for exception lookup — ngữ cảnh lane để tra exception.
/// Framework/version come from project metadata; empty version matches only
/// manifests written with an empty version (fail-closed otherwise).
/// (Framework/version từ metadata project; version rỗng chỉ khớp manifest
/// ghi version rỗng (còn lại fail-closed).)
#[derive(Debug, Clone)]
pub struct PolicyContext {
    /// Framework name (e.g. `"next"`, or ecosystem fallback).
    /// Tên framework (vd `"next"`, hoặc fallback ecosystem).
    pub framework: String,
    /// Framework version, empty when unknown — version framework, rỗng khi chưa biết.
    pub version: String,
    /// Operating system (`std::env::consts::OS`) — hệ điều hành.
    pub os: String,
    /// Operation (`"dev"`, `"run"`, `"test"`, `"build"`, `"exec"`).
    /// Thao tác (`"dev"`, `"run"`, `"test"`, `"build"`, `"exec"`).
    pub operation: String,
}

/// Exact read-only exception — ngoại lệ chỉ đọc chính xác.
#[derive(Debug, Clone)]
pub struct ReadOnlyException {
    /// Program basename, lowercase — basename chương trình, lowercase.
    pub program: String,
    /// Exact argument list — danh sách tham số chính xác.
    pub args_exact: Vec<String>,
    /// Manifest verified and cache unexpired — manifest đã xác minh còn hạn.
    pub unexpired: bool,
    /// MGC-synthesized answer; `None` means spawn the real invocation.
    /// Câu trả lời do MGC tổng hợp; `None` nghĩa là chạy invocation thật.
    pub answer: Option<String>,
}

/// Lowercase basename without Windows executable suffix.
/// Basename lowercase, bỏ hậu tố executable Windows.
pub(crate) fn tool_basename(program: &str) -> String {
    let base = program
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(program)
        .to_ascii_lowercase();
    base.strip_suffix(".exe")
        .or_else(|| base.strip_suffix(".cmd"))
        .unwrap_or(&base)
        .to_string()
}

/// Read-only PM probes that may be delegated by a signed, profile-bound
/// exception. Keep this closed: verbs such as `exec`, `run`, `view`, or
/// `list` can have package-manager-specific side effects.
/// Probe PM chỉ đọc có thể ủy quyền bằng exception đã ký và gắn profile.
/// Danh sách đóng: verb như `exec`, `run`, `view`, `list` có thể có tác dụng
/// phụ tùy package manager.
pub fn is_read_only_probe(program: &str, args: &[String]) -> bool {
    if !FORBIDDEN_TOOLS.contains(&tool_basename(program).as_str()) {
        return false;
    }
    matches!(args, [version] if version == "--version" || version == "-v")
}

/// True only when the exact normalized PM invocation was observed in the
/// active lane profile. A profile is mandatory for every spawned exception.
/// Đúng khi profile hiệu lực ghi nhận chính xác invocation PM đã chuẩn hóa.
/// Mọi exception spawn đều bắt buộc có profile.
pub fn profile_allows_invocation(profile: &LaneProfile, program: &str, args: &[String]) -> bool {
    profile
        .allowed_pm_invocations
        .iter()
        .any(|allowed| invocation_key_matches_profile(allowed, program, args))
}

/// First positional subcommand (skips leading flags and `--` terminator).
/// Subcommand vị trí đầu (bỏ qua flag đầu và terminator `--`).
fn first_subcommand(args: &[String]) -> Option<String> {
    let mut ended_flags = false;
    for arg in args {
        if !ended_flags && arg == "--" {
            ended_flags = true;
            continue;
        }
        if !ended_flags && arg.starts_with('-') {
            continue;
        }
        return Some(arg.to_ascii_lowercase());
    }
    None
}

/// True when this invocation resolves/fetches/publishes dependencies.
///
/// Two layers matching enforcement order (Spec rev4 §7 row 4): first the
/// shared classifier (`run.rs`) for allowlisted tools with verb-gated
/// resolution, then the forbidden-name verb tables so pure probes
/// (`npm --version`) can reach the exception path instead of being
/// misclassified as mutation.
/// (Đúng khi invocation resolve/tải/publish dependency. Hai lớp khớp thứ tự
/// enforcement (Spec rev4 §7 hàng 4): trước là bộ phân loại dùng chung
/// (`run.rs`), sau là bảng verb cho tên cấm để probe thuần tới được đường
/// exception thay vì bị coi nhầm mutation.)
pub fn is_mutation_invocation(program: &str, args: &[String], env: &[(String, String)]) -> bool {
    if crate::run::dependency_resolution_deny_reason(program, args, env).is_some() {
        return true;
    }
    // Forbidden managers: verb precision for the exception path.
    // (Manager bị cấm: chính xác verb cho đường exception.)
    const JS_MUTATION: &[&str] = &[
        "install",
        "ci",
        "add",
        "remove",
        "uninstall",
        "update",
        "upgrade",
        "publish",
        "unpublish",
        "link",
        "unlink",
        "dedupe",
        "prune",
        "rebuild",
    ];
    const PY_MUTATION: &[&str] = &[
        "install",
        "add",
        "remove",
        "uninstall",
        "update",
        "upgrade",
        "sync",
        "lock",
    ];
    let tool = tool_basename(program);
    if !FORBIDDEN_TOOLS.contains(&tool.as_str()) {
        return false;
    }
    let sub = match first_subcommand(args) {
        Some(sub) => sub,
        // Bare forbidden manager with no subcommand: probe-shaped, not mutation.
        // (Manager cấm gọi trần không subcommand: dạng probe, không mutation.)
        None => return false,
    };
    match tool.as_str() {
        "npm" | "npx" | "pnpm" | "yarn" | "bun" | "bunx" => JS_MUTATION.contains(&sub.as_str()),
        "pip" | "pip3" | "uv" | "uvx" | "poetry" | "pdm" | "conda" | "mamba" | "pipx" | "hatch"
        | "rye" | "pixi" | "pip-compile" | "pip-sync" => PY_MUTATION.contains(&sub.as_str()),
        "pub" => matches!(
            sub.as_str(),
            "get" | "add" | "remove" | "upgrade" | "downgrade"
        ),
        "composer" => matches!(sub.as_str(), "install" | "update" | "require" | "remove"),
        "deno" => matches!(
            sub.as_str(),
            "add" | "remove" | "install" | "publish" | "upgrade" | "update"
        ),
        // Other forbidden names without a verb table: fail closed.
        // (Tên cấm khác không có bảng verb: fail-closed.)
        _ => true,
    }
}

/// Canonical `"program args..."` pairing for profile comparison.
/// Cặp `"program args..."` chuẩn để so với profile.
fn invocation_key(program: &str, args: &[String]) -> String {
    if args.is_empty() {
        return tool_basename(program);
    }
    format!("{} {}", tool_basename(program), args.join(" "))
}

/// Resolve one invocation to a typed decision (pure function, no I/O).
/// Phân giải một invocation thành quyết định typed (hàm thuần, không I/O).
///
/// Precedence (Spec rev4 §7): mutation-deny first — profile, exception and
/// local config never turn mutation into allow in native mode.
/// (Ưu tiên (Spec rev4 §7): mutation-deny trước — profile, exception và
/// config local không bao giờ biến mutation thành cho phép ở native mode.)
pub fn resolve(
    mode: &PolicyMode,
    invocation: &Invocation,
    profile: Option<&LaneProfile>,
    exception: Option<&ReadOnlyException>,
) -> PolicyDecision {
    let mutation = is_mutation_invocation(&invocation.program, &invocation.args, &invocation.env);

    // Row 4: mutation-deny wins over everything in native mode.
    // (Hàng 4: mutation-deny thắng mọi thứ ở native mode.)
    if mutation {
        match mode {
            PolicyMode::Native => {
                return PolicyDecision::Deny {
                    code: DenyCode::NativeMutationDeny,
                };
            }
            PolicyMode::CompatRunner { manager } => {
                if invocation.in_compat_subtree {
                    return PolicyDecision::CompatAudit {
                        note: format!(
                            "mutation inside declared '{manager}' compat subtree: {}",
                            invocation_key(&invocation.program, &invocation.args)
                        ),
                    };
                }
                return PolicyDecision::Deny {
                    code: DenyCode::NativeMutationDeny,
                };
            }
        }
    }

    // Compat subtree: non-mutation work runs with warning + audit.
    // (Subtree compat: việc không mutation chạy kèm cảnh báo + audit.)
    if let PolicyMode::CompatRunner { manager } = mode
        && invocation.in_compat_subtree
    {
        return PolicyDecision::CompatAudit {
            note: format!(
                "execution inside declared '{manager}' compat subtree: {}",
                invocation_key(&invocation.program, &invocation.args)
            ),
        };
    }

    // Row 5: exact read-only exception (unexpired; profile-bound on profiled lanes).
    // (Hàng 5: exception đọc chính xác (còn hạn; gắn profile ở lane đã profile).)
    if let Some(exc) = exception
        && tool_basename(&invocation.program) == exc.program
        && invocation.args == exc.args_exact
        && exc.unexpired
        && is_read_only_probe(&invocation.program, &invocation.args)
        && profile.is_some_and(|lane| {
            profile_allows_invocation(lane, &invocation.program, &invocation.args)
        })
    {
        if let Some(answer) = &exc.answer {
            return PolicyDecision::SelfAnswer(answer.clone());
        }
        return PolicyDecision::Allow;
    }

    let tool = tool_basename(&invocation.program);
    if FORBIDDEN_TOOLS.contains(&tool.as_str()) {
        // Row 6: profiled lane outside the active profile → deny.
        // (Hàng 6: lane đã profile mà ngoài profile hiệu lực → deny.)
        if let Some(lane) = profile {
            let in_profile = lane.allowed_pm_invocations.iter().any(|allowed| {
                invocation_key_matches_profile(allowed, &invocation.program, &invocation.args)
            });
            // In-profile but without a valid exception: the exception path
            // failed (missing or expired) — deny with the row-5 code.
            // (Trong profile nhưng không có exception hợp lệ: đường exception
            // hỏng (thiếu/hết hạn) — deny bằng mã hàng 5.)
            if in_profile {
                return PolicyDecision::Deny {
                    code: DenyCode::MissingException,
                };
            }
            return PolicyDecision::Deny {
                code: DenyCode::OutsideActiveProfile,
            };
        }
        // Unprofiled lane: forbidden name without an exact exception → deny.
        // (Lane chưa profile: tên cấm mà không có exception chính xác → deny.)
        return PolicyDecision::Deny {
            code: DenyCode::ForbiddenNameNoException,
        };
    }

    // Ordinary descendant: runs with observability audit (not a safety proof).
    // (Descendant thường: chạy kèm audit quan sát (không phải chứng minh an toàn).)
    PolicyDecision::Allow
}

/// Exact invocation match against one profile entry (program + full args).
/// Khớp invocation chính xác với một entry profile (program + đủ args).
fn invocation_key_matches_profile(allowed: &str, program: &str, args: &[String]) -> bool {
    *allowed == invocation_key(program, args)
}

/// Test-only key helper (stable `"program args..."` format for assertions).
/// Helper key cho test (định dạng `"program args..."` ổn định để assert).
#[doc(hidden)]
pub fn invocation_key_for_tests(program: &str, args: &[String]) -> String {
    invocation_key(program, args)
}

/// Public re-export for callers that only need the missing-exception code.
/// Re-export cho caller chỉ cần mã thiếu exception.
pub fn missing_exception() -> PolicyDecision {
    PolicyDecision::Deny {
        code: DenyCode::MissingException,
    }
}

/// MGC-synthesized probe answers (Spec rev4 §5): only probes whose semantics
/// are exactly defined may be answered without spawning anything. Today that
/// is exactly one — `npm --version` → `"0.0.0"`, mirroring the PATH deny shim
/// (a compat behavior under review, not a general pattern).
/// (Probe do MGC tự trả lời (Spec rev4 §5): chỉ probe có semantics xác định
/// chính xác mới được trả lời mà không spawn. Hiện đúng một — `npm --version`
/// → `"0.0.0"`, mirror PATH deny shim (compat behavior đang review, không
/// phải mẫu chung).)
pub fn self_answer_for(program: &str, args: &[String]) -> Option<String> {
    if tool_basename(program) == "npm" && args.len() == 1 && args[0] == "--version" {
        return Some("0.0.0".to_string());
    }
    None
}
