use crate::allowlist::{ExecutionScope, PROJECT_LANGUAGE_EXECUTABLES};
use anyhow::{Result, bail};

/// Restrict project-language executables to explicit project execution scopes.
/// Chỉ cho runtime/ngôn ngữ chạy trong execution scope của project.
pub(in crate::allowlist::tool_check) fn restrict_language_runtime_scope(
    name: &str,
    normalized: &str,
    scope: ExecutionScope,
) -> Result<()> {
    if PROJECT_LANGUAGE_EXECUTABLES.contains(&normalized)
        && !matches!(
            scope,
            ExecutionScope::TestRunner
                | ExecutionScope::BuildRunner
                | ExecutionScope::DevServer
                | ExecutionScope::ScriptRunner
                | ExecutionScope::FrameworkInitializer
        )
    {
        bail!(
            "language runtime '{name}' requires a project execution scope; it is not allowed in {scope:?}"
        );
    }
    Ok(())
}
