use anyhow::Result;
use std::path::Path;
use std::time::Duration;

pub(in crate::commands) fn publish_lifecycle_decision(
    pkg_json: &serde_json::Value,
    ignore_scripts: bool,
    allow_scripts: bool,
) -> Result<bool> {
    if ignore_scripts {
        return Ok(false);
    }
    let Some(scripts_value) = pkg_json.get("scripts") else {
        return Ok(false);
    };
    let Some(scripts) = scripts_value.as_object() else {
        return Err(crate::error::publish_lifecycle_policy_invalid());
    };

    let mut has_publish_hook = false;
    for hook in PUBLISH_LIFECYCLE_HOOKS {
        if let Some(value) = scripts.get(*hook) {
            if !value.is_string() {
                return Err(crate::error::publish_lifecycle_policy_invalid());
            }
            has_publish_hook = true;
        }
    }
    if !has_publish_hook {
        return Ok(false);
    }
    if !allow_scripts {
        return Err(crate::error::publish_lifecycle_opt_in_required());
    }
    Ok(true)
}

pub(in crate::commands) fn run_lifecycle(
    pkg_json: &serde_json::Value,
    script: &str,
    project_root: &Path,
) -> Result<()> {
    if let Some(cmd) = pkg_json
        .get("scripts")
        .and_then(|s| s.get(script))
        .and_then(|v| v.as_str())
    {
        println!("Running approved publish lifecycle hook: {script}");
        mgc_exec::allowlist::reject_forbidden_pm_script(cmd)?;
        let invocation = mgc_exec::allowlist::parse_script_invocation(cmd)?;
        let mut env = invocation.env;
        env.push(("INIT_CWD".to_string(), project_root.display().to_string()));
        if let Some(path) = std::env::var_os("PATH") {
            env.push(("PATH".to_string(), path.to_string_lossy().to_string()));
        }
        let opts = mgc_exec::prelude::ExecOptions {
            cwd: Some(project_root.to_path_buf()),
            timeout: Some(publish_lifecycle_timeout()),
            env,
            clean_env: true,
            execution_scope: Some(mgc_exec::allowlist::ExecutionScope::Install),
            ..Default::default()
        };
        mgc_exec::prelude::run_inherited(&invocation.program, &invocation.args, &opts)?;
    }
    Ok(())
}

pub(in crate::commands) fn publish_lifecycle_timeout() -> Duration {
    std::env::var(PUBLISH_LIFECYCLE_TIMEOUT_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|secs| *secs > 0)
        .map(Duration::from_secs)
        .unwrap_or(Duration::from_secs(DEFAULT_PUBLISH_LIFECYCLE_TIMEOUT_SECS))
}

const DEFAULT_PUBLISH_LIFECYCLE_TIMEOUT_SECS: u64 = 300;

const PUBLISH_LIFECYCLE_TIMEOUT_ENV: &str = "MGC_PUBLISH_LIFECYCLE_TIMEOUT_SECS";

const PUBLISH_LIFECYCLE_HOOKS: &[&str] = &["prepublishOnly", "prepublish", "prepare"];
