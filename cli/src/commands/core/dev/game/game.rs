//! Game dev lifecycle and controlled process execution.
//! Lifecycle dev Game và thực thi process có kiểm soát.

use std::path::Path;

use mgc_ui::info;

use super::framework_command::game_dev_command;

pub(super) fn execution_options(root: &Path) -> mgc_exec::prelude::ExecOptions {
    // The optimizer's settings belong to a long-running dev process most of all:
    // a hot-reload loop holds memory the whole time a developer works.
    // Cấu hình optimizer thuộc về process dev chạy lâu nhất: vòng hot-reload giữ
    // bộ nhớ suốt thời gian lập trình viên làm việc.
    let env = crate::commands::optimizer::env_loader::load_optimizer_env(
        root,
        &crate::commands::optimizer::runtime_detect::DetectedRuntime::RustLib,
    )
    .map(|envs| envs.into_iter().collect::<Vec<_>>())
    .unwrap_or_default();
    mgc_exec::prelude::ExecOptions {
        cwd: Some(root.to_path_buf()),
        log_path: Some(root.join(".magicore").join("exec.log")),
        env,
        clean_env: true,
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::DevServer),
        ..Default::default()
    }
}

pub async fn dev(root: &Path) -> anyhow::Result<()> {
    let (cmd, args) = game_dev_command(root)?;
    // Pre-flight freshness for Cargo-based lanes (Bevy).
    // (Freshness pre-flight cho lane Cargo (Bevy).)
    crate::commands::freshness::check_cargo_freshness(root)?;
    let opts = execution_options(root);
    info(&format!(
        "Game dev: running `{} {}`...",
        cmd,
        args.join(" ")
    ));
    mgc_exec::prelude::run_inherited(&cmd, &args, &opts).map_err(|e| {
        if cmd == "godot" {
            crate::error::godot_failed(&e)
        } else {
            crate::error::tool_failed(&cmd, &e)
        }
    })?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
