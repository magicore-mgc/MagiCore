//! AI dev runtime preparation and shared-executor invocation.
//! Chuẩn bị runtime dev AI và gọi executor dùng chung.

#[cfg(feature = "ai")]
use anyhow::Result;
#[cfg(feature = "ai")]
use std::path::Path;

/// Run the AI entry script under the native Python runtime and shared executor.
/// Chạy entry script AI bằng Python native và executor dùng chung.
#[cfg(feature = "ai")]
pub async fn ai_dev(_dry_run: bool) -> Result<()> {
    let root = super::super::project_identity::ai_project_root()?;
    run_ai_dev_at(&root).await
}

/// Run the AI dev flow for one validated path; tests use this to assert pre-spawn refusal.
/// Chạy luồng AI dev tại một path; test dùng để kiểm tra từ chối trước spawn.
#[cfg(feature = "ai")]
async fn run_ai_dev_at(root: &Path) -> Result<()> {
    super::super::python_lifecycle_gate::require_native_ai_python(root, "dev")?;
    // Pre-flight freshness (Spec rev4 §8): dependency drift fails before spawn.
    // (Freshness pre-flight (Spec rev4 §8): lệch dependency fail trước spawn.)
    crate::commands::freshness::check_python_freshness(root)?;
    let framework = mgc_ai_adapter::adapter_for(root).ok_or_else(|| {
        anyhow::Error::new(super::super::error::AiCoreError::framework_not_detected(
            root,
        ))
    })?;
    let script = framework.framework.entry_script().to_string();

    let runtime =
        super::super::runtime_detection::detect_ai_runtime(root, framework.framework.as_str());
    let optimizer_envs =
        crate::commands::optimizer::env_loader::load_optimizer_env(root, &runtime)?;
    let mut env: Vec<(String, String)> = optimizer_envs.into_iter().collect();
    crate::commands::python_runtime::extend_native_python_env(&mut env, root, false)?;

    let opts = mgc_exec::prelude::ExecOptions {
        cwd: Some(root.to_path_buf()),
        log_path: Some(root.join(".magicore").join("exec.log")),
        env,
        clean_env: true,
        execution_scope: Some(mgc_exec::prelude::ExecutionScope::DevServer),
        ..Default::default()
    };
    let cmd = crate::commands::build::python_cmd().to_string();
    mgc_ui::info(&format!("AI dev: running `{} {}`...", cmd, script));
    mgc_exec::prelude::run_inherited(&cmd, &[script], &opts).map_err(|error| {
        anyhow::Error::new(super::super::error::AiCoreError::python_runtime_failed(
            &cmd, error,
        ))
    })?;
    Ok(())
}

#[cfg(all(test, feature = "ai"))]
#[path = "test/mod.rs"]
mod tests;
