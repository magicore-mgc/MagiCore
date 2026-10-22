//! IoT dev lifecycle and controlled process execution.
//! Lifecycle dev IoT và thực thi process có kiểm soát.

use std::path::Path;

use mgc_ui::info;

use super::framework_command::iot_dev_command;

pub(super) fn execution_options(root: &Path) -> mgc_exec::prelude::ExecOptions {
    mgc_exec::prelude::ExecOptions {
        cwd: Some(root.to_path_buf()),
        log_path: Some(root.join(".magicore").join("exec.log")),
        clean_env: true,
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::DeviceControl),
        ..Default::default()
    }
}

pub async fn dev(root: &Path) -> anyhow::Result<()> {
    let (cmd, args) = iot_dev_command(root)?;
    // Pre-flight freshness for Cargo-based lanes (ESP32-Rust).
    // (Freshness pre-flight cho lane Cargo (ESP32-Rust).)
    crate::commands::freshness::check_cargo_freshness(root)?;
    let opts = execution_options(root);
    info(&format!("IoT dev: running `{} {}`...", cmd, args.join(" ")));
    mgc_exec::prelude::run_inherited(&cmd, &args, &opts).map_err(|e| {
        if cmd == "espflash" {
            crate::error::espflash_failed(&e)
        } else {
            crate::error::tool_failed(&cmd, &e)
        }
    })?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
