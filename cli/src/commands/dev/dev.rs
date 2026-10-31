//! Shared `mgc dev` argument validation and project coordination.
//! Kiểm tra tham số và điều phối project dùng chung cho `mgc dev`.

use mgc_ui::info;

use crate::context::ProjectContext;

/// Host every dev server binds to when `--host` is absent.
/// Host mà mọi dev server bind khi không truyền `--host`.
///
/// Centralized so the shared coordinator, the frontend bundler and both halves of
/// a split project state the same default; a per-owner literal let the printed
/// URL drift from the bound address.
/// Tập trung để coordinator dùng chung, bundler frontend và cả hai nửa của
/// project tách rời cùng một mặc định; literal riêng từng owner làm URL in ra
/// lệch địa chỉ đã bind.
pub(crate) const DEFAULT_DEV_HOST: &str = "localhost";

#[path = "dispatch/mod.rs"]
mod dispatch;

pub async fn run(
    core: Option<&str>,
    host: Option<String>,
    port: Option<u16>,
    clear: bool,
    compat_runtime: Option<&str>,
    compat_runner: Option<&str>,
) -> anyhow::Result<()> {
    // Native-engine gate (2026-09-10): --compat-runtime chỉ chọn lane
    // compat tường minh; validate giá trị sớm để fail trước khi load ctx.
    // P0-1 fix (2026-09-10 audit): CompatMode phải TRUYỀN XUống call chain —
    // không còn parse rồi bỏ đi. Native mode = không runtime đối thủ nào
    // được spawn; web.rs dùng mode này để gate từng DevLaunch.
    let compat = crate::commands::compat::CompatMode::from_flag(compat_runtime)?;
    // Delegated runner lane (Spec rev4 §7/§9): qualified only, warned loudly.
    // (Lane runner ủy quyền (Spec rev4 §7/§9): chỉ qualified, cảnh báo lớn.)
    let compat_runner = crate::commands::compat::parse_compat_runner(compat_runner)?;
    let ctx = ProjectContext::load_with_core(core)?;
    // Spec rev4 §9/§11: compat contracts exist for web only; other cores
    // fail loudly instead of silently ignoring the flag.
    // (Spec rev4 §9/§11: mới web có compat contract; core khác fail rõ ràng
    // thay vì lặng lẽ bỏ qua flag.)
    if compat_runner.is_some() && ctx.adapter().name() != "web" {
        return Err(crate::error::compat_runner_unsupported_core(
            ctx.adapter().name(),
        ));
    }
    let host = default_host(host);
    let root = ctx.root().to_path_buf();

    info("Starting MagiCore Native Dev Server...");
    info(&format!("Project root: {}", root.display()));
    info(&format!("Execution profile: {}", ctx.execution_summary()));

    dispatch::dispatch(
        ctx.adapter().name(),
        &root,
        host,
        port,
        clear,
        &compat,
        compat_runner.as_deref(),
    )
    .await
}

fn default_host(host: Option<String>) -> String {
    host.unwrap_or_else(|| DEFAULT_DEV_HOST.to_string())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
