//! CI and automation diagnostics.
//! Lỗi CI và automation.

use anyhow::Error;
use anyhow::anyhow;

/// verb not applicable to cicd core (07 §4)
pub fn cicd_verb_not_applicable(verb: &str) -> Error {
    anyhow!(
        "'{verb}' does not apply to the cicd core (07 §4) — use `mgc ci-generate`, `mgc verify`, or `mgc deploy`."
    )
}

/// CICD verification test step failed for a specific core.
/// Bước test trong CICD verify thất bại cho một core cụ thể.
pub fn cicd_test_step_failed(core: &str, reason: &dyn std::fmt::Display) -> Error {
    anyhow!("test step for core '{core}' failed: {reason}")
}

/// cloudflare workers build/deploy belongs to cicd core
pub fn cloudflare_build_in_cicd_core() -> Error {
    anyhow!(
        "cloudflare workers build/deploy belongs to the cicd core (Q12) — cloud core does not handle it as P1"
    )
}

pub fn cicd_reason(reason: &str) -> Error {
    anyhow!("'cicd' {reason}")
}

pub fn ci_template_unknown(provider: &str) -> Error {
    anyhow!(
        "'mgc ci-generate' has no template for provider '{provider}' — supported: github-actions | gitlab | circleci | jenkins | azure-pipelines | bitbucket-pipelines | argocd"
    )
}

/// A library language MagiCore supports has no command-line scaffolder.
///
/// Reported instead of writing an empty project: the user needs to know the
/// real next step, not to discover later that `mgc install` has nothing to do.
/// (Ngôn ngữ lib MagiCore hỗ trợ nhưng không có scaffolder CLI. Báo lỗi thay vì
/// ghi project rỗng: người dùng cần biết bước tiếp theo thật, không phải phát
/// hiện sau rằng `mgc install` không có gì để làm.)
pub fn lib_language_without_initializer(language: &str) -> Error {
    anyhow!(
        "'create-lib {language}' has no official initializer: the language ships no library scaffolder and MagiCore does not embed project templates. Create the project with your language's own tool, then run 'mgc install' inside it. No project was created."
    )
}

pub fn cicd_project_not_detected() -> Error {
    anyhow!(
        "Cannot detect a cicd project here (missing mgc.toml [cicd] provider / wrangler.toml / argocd / .github/workflows)."
    )
}

pub fn cicd_verify_unknown_step(step: &str) -> Error {
    anyhow!(
        "mgc.toml [cicd] verify contains unknown step '{step}' — allowed steps: audit, test, build. Fix the config; verify must never skip unknown steps silently."
    )
}

pub fn cicd_verify_empty_chain() -> Error {
    anyhow!(
        "mgc.toml [cicd] verify chain is empty — a verify run with zero steps cannot prove anything. Add at least one of: audit, test, build."
    )
}

pub fn cicd_deploy_target_not_implemented(provider: &str, stack: &str, region: &str) -> Error {
    anyhow!(
        "'mgc deploy' for {provider} target '{stack}' in region '{region}' has no MagiCore-native provider engine yet; no external provider CLI was started"
    )
}
