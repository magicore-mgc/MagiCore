//! Exact exceptions, compat-runner scope, and pre-spawn invocation gates.
//! Exception chính xác, phạm vi compat-runner và guard invocation trước spawn.

use super::super::{ExecOptions, ExecReport};
use anyhow::Result;

/// Exact-manifest consultation (Spec rev4 §5/§7): true when the verified
/// manifest holds this precise invocation for the caller-supplied lane
/// context. Lifts only the name-based deny — verb and script guards still
/// run, and profile binding for profiled lanes is enforced by the caller
/// through `policy::resolve`.
/// (Tra manifest chính xác (Spec rev4 §5/§7): đúng khi manifest đã xác minh
/// chứa đúng invocation này cho ngữ cảnh lane caller đưa. Chỉ gỡ deny theo
/// tên — guard verb và script vẫn chạy, ràng buộc profile cho lane đã profile
/// do caller thực thi qua `policy::resolve`.)
pub(in crate::run::implementation) fn manifest_exception_allows(
    cmd: &str,
    args: &[String],
    opts: &ExecOptions,
) -> bool {
    let (Some(manifest), Some(ctx)) = (opts.policy_manifest.as_ref(), opts.policy_context.as_ref())
    else {
        return false;
    };
    let Some(profile) = opts.policy_profile.as_ref() else {
        return false;
    };
    let Some(exception) = crate::policy_manifest::find_exception(
        manifest,
        cmd,
        args,
        &ctx.framework,
        &ctx.version,
        &ctx.os,
        &ctx.operation,
    ) else {
        return false;
    };
    matches!(
        crate::policy::resolve(
            &crate::policy::PolicyMode::Native,
            &crate::policy::Invocation {
                program: cmd.to_string(),
                args: args.to_vec(),
                env: opts.env.clone(),
                in_compat_subtree: false,
            },
            Some(profile),
            Some(&exception),
        ),
        crate::policy::PolicyDecision::Allow
    )
}

/// Return an MGC-answered probe without spawning, and audit it like a spawn.
/// Trả probe do MGC tự trả lời mà không spawn, đồng thời ghi audit như spawn.
pub(in crate::run::implementation) fn self_answered_report(
    cmd: &str,
    args: &[String],
    opts: &ExecOptions,
    answer: &str,
) -> Result<ExecReport> {
    let cwd = opts
        .cwd
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let report = ExecReport {
        cmd: cmd.to_string(),
        args: args.to_vec(),
        exit_code: 0,
        duration_ms: 0,
        stdout_tail: answer.to_string(),
        stderr_tail: String::new(),
        stdout_full: String::new(),
        dry_run: false,
    };
    if let Some(path) = &opts.log_path {
        super::super::append(path, &super::super::entry_from(&report, &cwd))?;
    }
    Ok(report)
}

/// True only for the declared compat runner itself.
/// Chỉ trả true với chính compat runner đã khai báo.
pub(crate) fn compat_runner_exempt(cmd: &str, opts: &ExecOptions) -> bool {
    compat_runner_declared(cmd, opts)
}

pub(crate) fn compat_runner_declared(cmd: &str, opts: &ExecOptions) -> bool {
    crate::allowlist::compat_runner_allows(cmd, opts.compat_runner.as_deref())
}

/// Apply dependency, self-answer, allowlist, manifest and script gates before spawn.
/// Áp guard dependency, self-answer, allowlist, manifest và script trước spawn.
pub(crate) fn check_invocation_policy(
    cmd: &str,
    args: &[String],
    opts: &ExecOptions,
) -> Result<Option<String>> {
    // Dependency mutation policy has precedence over runtime allowlist and
    // read-only exceptions, regardless of the requested execution scope.
    // (Mutation dependency ưu tiên cao hơn allowlist runtime và exception đọc,
    // bất kể scope được yêu cầu.)
    super::super::reject_external_dependency_resolution(cmd, args, &opts.env)?;
    if let Some(answer) = crate::policy::self_answer_for(cmd, args) {
        return Ok(Some(answer));
    }
    let scope = opts
        .execution_scope
        .unwrap_or(crate::allowlist::ExecutionScope::Install);
    if !manifest_exception_allows(cmd, args, opts) {
        crate::allowlist::check_tool_with_scope_compat(
            cmd,
            scope,
            opts.cwd.as_deref(),
            opts.compat_runtime.as_deref(),
        )?;
    }
    super::super::reject_forbidden_script_file(cmd)?;
    Ok(None)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
