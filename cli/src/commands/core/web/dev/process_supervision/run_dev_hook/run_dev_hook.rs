//! Resolves and executes one optional lifecycle hook.
//! Resolve và chạy một lifecycle hook tùy chọn.
use anyhow::Result;

/// Run a dev lifecycle hook (`predev`/`postdev`) if the project defines it.
/// Missing hooks are a no-op; hook failures propagate to the caller.
/// (Chạy hook lifecycle dev (`predev`/`postdev`) nếu project định nghĩa.
/// Hook vắng là no-op; hook hỏng lan tới caller.)
pub(in crate::commands::core::web::implementation) fn run_dev_hook(
    dir: &std::path::Path,
    hook: &str,
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
) -> Result<()> {
    // Reuse the single task-script implementation (same precedence, gates
    // and compat wiring as `mgc run`) — one behavior, not two.
    // (Dùng chung implementation task-script (cùng precedence, gate và
    // compat như `mgc run`) — một hành vi, không hai.)
    if crate::commands::run::resolve_task_script(dir, hook)?.is_none() {
        return Ok(());
    }
    crate::commands::run::run_task_script(dir, hook, &[], compat, compat_runner, "dev")
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
