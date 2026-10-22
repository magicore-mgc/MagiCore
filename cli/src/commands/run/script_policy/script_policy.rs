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
    crate::commands::compat::gate_runtime_spawn(compat, &invocation.program)
}
