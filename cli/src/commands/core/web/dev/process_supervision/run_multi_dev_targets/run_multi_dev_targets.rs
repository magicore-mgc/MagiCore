//! Resolves and supervises a group of Web dev targets.
//! Resolve và giám sát một nhóm target Web dev.
use super::super::super::{DevTarget, build_dev_launch};
use super::super::{
    run_dev_hook, run_dev_launch_with_guard, supervise_dev_tasks, wait_for_dev_stop,
};
use anyhow::Result;
use mgc_ui::info;
use std::sync::Arc;

pub(in crate::commands::core::web::implementation) async fn run_multi_dev_targets(
    targets: &[DevTarget],
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
) -> Result<()> {
    if targets.is_empty() {
        return Ok(());
    }

    // Resolve every launch before lifecycle hooks or child processes begin.
    // (Resolve mọi launch trước hook lifecycle hoặc khi process con bắt đầu.)
    let launches = targets
        .iter()
        .map(|target| {
            build_dev_launch(
                &target.dir,
                target.script_name,
                target.host.clone(),
                target.port,
                compat,
                compat_runner,
            )
            .map(|launch| (target.clone(), launch))
        })
        .collect::<Result<Vec<_>>>()?;

    for target in targets {
        run_dev_hook(&target.dir, "predev", compat, compat_runner)?;
    }

    let cancel_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
    mgc_exec::prelude::ensure_signal_handlers()?;
    let mut tasks = tokio::task::JoinSet::<Result<()>>::new();

    for (target, launch) in launches {
        if launch.program.to_string_lossy().ends_with("vite") {
            info(&format!(
                "Starting MgDevServer (Native Rust) for {} in {}",
                target.role,
                target.dir.display()
            ));

            let entry = if target.dir.join("src/main.tsx").exists() {
                target.dir.join("src/main.tsx")
            } else if target.dir.join("src/main.ts").exists() {
                target.dir.join("src/main.ts")
            } else {
                target.dir.join("src/index.tsx")
            };

            let config = crate::bundler::dev_server::DevServerConfig {
                root: target.dir.clone(),
                entry,
                host: target
                    .host
                    .clone()
                    .unwrap_or_else(|| "localhost".to_string()),
                port: target
                    .port
                    .unwrap_or(crate::commands::core::dev_port::PORT_WEB_FE),
            };

            let server = crate::bundler::dev_server::MgDevServer::new(config);
            let cancel_flag = Arc::clone(&cancel_flag);
            tasks.spawn(async move {
                tokio::select! {
                    result = server.serve() => result,
                    stop = wait_for_dev_stop(cancel_flag) => stop,
                }
            });
            continue;
        }

        info(&format!(
            "Starting {} dev server in {}",
            target.role,
            target.dir.display()
        ));
        info(&format!("  {}", launch.describe()));
        tasks.spawn({
            let compat = compat.clone();
            let compat_runner = compat_runner.map(str::to_string);
            let cancel_flag = Arc::clone(&cancel_flag);
            async move {
                tokio::task::spawn_blocking(move || {
                    run_dev_launch_with_guard(
                        &target,
                        &launch,
                        &compat,
                        compat_runner.as_deref(),
                        Some(cancel_flag),
                    )
                })
                .await?
            }
        });
    }

    let result = supervise_dev_tasks(&mut tasks, cancel_flag).await;
    if mgc_exec::prelude::shutdown_requested() {
        mgc_exec::prelude::clear_shutdown();
    }
    for target in targets {
        if let Err(post_err) = run_dev_hook(&target.dir, "postdev", compat, compat_runner) {
            mgc_ui::warning(&format!("post hook 'postdev' failed: {post_err:#}"));
        }
    }
    result
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
