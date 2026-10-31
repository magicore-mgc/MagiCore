//! Runs one Web dev target through the established lifecycle.
//! Chạy một target Web dev theo lifecycle hiện hành.
use super::super::super::{DevTarget, build_dev_launch};
use super::super::{
    BACKEND_ROLE, backend_starting_line, build_backend_launch, readiness_line, run_dev_hook,
    run_dev_launch_with_guard, wait_for_dev_stop,
};
use anyhow::Result;
use mgc_ui::info;
use std::sync::Arc;

pub(in crate::commands::core::web::implementation) async fn run_single_dev_target(
    target: &DevTarget,
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
) -> Result<()> {
    // A lone backend target still resolves through the backend owner: the server
    // half is described by its own manifest, not by a Node script name.
    // (Target backend đơn lẻ vẫn resolve qua owner backend: nửa server được mô tả
    // bằng manifest riêng, không bằng tên script Node.)
    let launch = if target.role == BACKEND_ROLE {
        build_backend_launch(&target.dir, compat, compat_runner)?
    } else {
        build_dev_launch(
            &target.dir,
            target.script_name,
            target.host.clone(),
            target.port,
            compat,
            compat_runner,
        )?
    };

    if let Some(port) = target.port {
        let host = target
            .host
            .as_deref()
            .unwrap_or(crate::commands::dev::DEFAULT_DEV_HOST);
        info(&readiness_line(target.role, host, port));
    }
    if target.role == BACKEND_ROLE && !launch.program.to_string_lossy().ends_with("vite") {
        info(&backend_starting_line(
            target.role,
            &launch.program.to_string_lossy(),
        ));
    }

    if launch.program.to_string_lossy().ends_with("vite") {
        info(&format!(
            "Starting MgDevServer (Native Rust) in {}",
            target.dir.display()
        ));
    } else {
        info(&format!(
            "Starting web dev server in {}",
            target.dir.display()
        ));
        info(&format!("  {}", launch.describe()));
    }

    // Lifecycle hooks (Spec rev4 §16.1 — decided): predev aborts on failure,
    // postdev always runs once after the server stops.
    // (Hook lifecycle (Spec rev4 §16.1 — đã chốt): predev hỏng thì dừng,
    // postdev luôn chạy một lần sau khi server dừng.)
    run_dev_hook(&target.dir, "predev", compat, compat_runner)?;
    let result = if launch.program.to_string_lossy().ends_with("vite") {
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
                .unwrap_or_else(|| crate::commands::dev::DEFAULT_DEV_HOST.to_string()),
            port: target
                .port
                .unwrap_or(crate::commands::core::dev_port::PORT_WEB_FE),
        };
        // Show the compiler settings the dev server will actually use. A dev
        // server that ran on defaults while `mgc build` used the optimizer
        // profile would preview a different program than it ships.
        // Hiện cấu hình compiler mà dev server thật sự dùng. Dev server chạy
        // mặc định trong khi `mgc build` dùng profile optimizer sẽ xem trước
        // một chương trình khác với bản sẽ phát hành.
        match crate::commands::core::web::implementation::dev::dev_optimizer::load(&target.dir) {
            Ok(profile) => {
                if let Some(note) = &profile.profile_note {
                    info(&format!("Dev compiler: {} ({note})", profile.target));
                }
            }
            Err(error) => {
                mgc_ui::warning(&format!(
                    "optimizer profile unreadable, dev uses compiler defaults: {error}"
                ));
            }
        }
        // Surface sources a build would refuse while they are still being
        // edited: the dev server never produces `dist/`, so the build scan
        // would not see this file until release.
        // Nêu source mà build sẽ từ chối ngay khi còn đang sửa: dev server không
        // tạo `dist/`, nên scan build sẽ không thấy file này cho tới lúc phát
        // hành.
        match crate::commands::core::web::implementation::dev::scan_dev_sources(&target.dir) {
            Ok(scan) => {
                if let Some(summary) = scan.summary() {
                    mgc_ui::warning(&summary);
                }
            }
            Err(error) => {
                mgc_ui::warning(&format!("dev source scan failed: {error}"));
            }
        }
        mgc_exec::prelude::ensure_signal_handlers()?;
        let server = crate::bundler::dev_server::MgDevServer::new(config);
        let cancel_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
        tokio::select! {
            result = server.serve() => result,
            stop = wait_for_dev_stop(cancel_flag) => stop,
        }
    } else {
        run_dev_launch_with_guard(target, &launch, compat, compat_runner, None)
    };
    if mgc_exec::prelude::shutdown_requested() {
        mgc_exec::prelude::clear_shutdown();
    }
    let post = run_dev_hook(&target.dir, "postdev", compat, compat_runner);
    if let Err(post_err) = post {
        mgc_ui::warning(&format!("post hook 'postdev' failed: {post_err:#}"));
    }
    result
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
