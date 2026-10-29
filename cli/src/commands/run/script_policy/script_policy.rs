//! Enforce native-runtime and package-manager script gates.
//! Áp dụng cổng runtime native và package manager cho script.

use anyhow::Result;
use std::path::Path;

pub(in crate::commands::run) fn reject_external_package_manager_script(
    cmd: &str,
    manifest_path: &Path,
    compat_runner: Option<&str>,
) -> Result<()> {
    // Spec rev4 §7/§9: the declared `--compat-runner` manager is exempt from
    // the script-token check; its subtree runs compat-audit downstream.
    // (Spec rev4 §7/§9: manager `--compat-runner` đã khai báo được miễn
    // check token script; subtree của nó chịu compat-audit phía sau.)
    if let Some(pm) = mgc_exec::allowlist::find_forbidden_tool_in_script(cmd)
        && !mgc_exec::prelude::compat_runner_allows(pm, compat_runner)
    {
        return Err(crate::error::forbidden_pm_script(cmd, manifest_path, pm));
    }

    Ok(())
}

/// Gate the script's PROGRAM against the native-engine contract: rival
/// JS runtimes need an explicit compat opt-in; external PMs are never
/// spawnable. The program token is inspected WITHOUT spawning anything.
/// Chặn PROGRAM của script theo hợp đồng engine native: runtime đối thủ
/// cần compat tường minh; PM ngoài không bao giờ spawn. Token program
/// được soi KHÔNG spawn gì cả.
pub(in crate::commands::run) fn gate_script_program(
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
    cmd: &str,
) -> Result<()> {
    let invocation = mgc_exec::allowlist::parse_script_invocation(cmd)
        .map_err(|e| crate::error::unsupported_script("run", &e))?;
    // The exact, qualified runner selected by the user enters the scoped
    // compat path in mgc-exec. Every other program keeps the native gate.
    // Runner đã chọn chính xác đi vào compat scope của mgc-exec. Mọi
    // chương trình khác vẫn chịu native gate.
    if mgc_exec::prelude::compat_runner_allows(&invocation.program, compat_runner) {
        return Ok(());
    }
    crate::commands::compat::gate_runtime_spawn(compat, &invocation.program)?;
    validate_runtime_flags(&invocation.program, &invocation.args)
}

/// Refuse inline-code flags for a JavaScript runtime.
///
/// `mgc dev` has always consulted `LauncherPolicy`; `mgc run` did not, so the
/// same `mgc.toml [scripts]` entry that `mgc dev` refused ran unchanged through
/// `mgc run <script>` — `node -e "…"` executed arbitrary code. Both commands
/// start a user-authored script, so they must share one flag policy.
/// (`mgc dev` luôn hỏi `LauncherPolicy`; `mgc run` thì không, nên cùng một entry
/// `mgc.toml [scripts]` bị `mgc dev` từ chối lại chạy được qua `mgc run
/// <script>`. Cả hai đều khởi chạy script do người dùng viết nên phải dùng
/// chung một policy flag.)
pub(in crate::commands::run) fn validate_runtime_flags(
    program: &str,
    args: &[String],
) -> Result<()> {
    use crate::commands::launcher_policy::{LauncherPolicy, Runtime};
    let runtime = match program {
        "node" => Runtime::Node,
        "tsx" => Runtime::Tsx,
        "bun" => Runtime::Bun,
        "deno" => Runtime::Deno,
        // Not a JavaScript runtime: its flag surface belongs to the toolchain,
        // not to this policy. `mgc-exec` still enforces the program allowlist and
        // path-traversal rules for it.
        // (Không phải runtime JS: bề mặt flag thuộc toolchain, không thuộc policy
        // này. `mgc-exec` vẫn áp allowlist chương trình và chống traversal.)
        _ => return Ok(()),
    };
    let owned: Vec<&str> = args.iter().map(String::as_str).collect();
    LauncherPolicy::dev_server(runtime)
        .validate_args(&owned)
        .map_err(|error| crate::error::unsupported_script("run", &error))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
