use super::{
    COMPOSER_MUTATION_VERBS, DART_MUTATION_VERBS, DENO_MUTATION_VERBS, JAVASCRIPT_MUTATION_VERBS,
    PYTHON_MUTATION_VERBS, parse_first_subcommand,
};
use crate::allowlist::FORBIDDEN_TOOLS;
use crate::policy::tool_basename;

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
    let tool = tool_basename(program);
    if !FORBIDDEN_TOOLS.contains(&tool.as_str()) {
        return false;
    }
    let sub = match parse_first_subcommand(args) {
        Some(sub) => sub,
        // Bare forbidden manager with no subcommand: probe-shaped, not mutation.
        // (Manager cấm gọi trần không subcommand: dạng probe, không mutation.)
        None => return false,
    };
    match tool.as_str() {
        "npm" | "npx" | "pnpm" | "yarn" | "bun" | "bunx" => {
            JAVASCRIPT_MUTATION_VERBS.contains(&sub.as_str())
        }
        "pip" | "pip3" | "uv" | "uvx" | "poetry" | "pdm" | "conda" | "mamba" | "pipx" | "hatch"
        | "rye" | "pixi" | "pip-compile" | "pip-sync" => {
            PYTHON_MUTATION_VERBS.contains(&sub.as_str())
        }
        "pub" => DART_MUTATION_VERBS.contains(&sub.as_str()),
        "composer" => COMPOSER_MUTATION_VERBS.contains(&sub.as_str()),
        "deno" => DENO_MUTATION_VERBS.contains(&sub.as_str()),
        // Other forbidden names without a verb table: fail closed.
        // (Tên cấm khác không có bảng verb: fail-closed.)
        _ => true,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
