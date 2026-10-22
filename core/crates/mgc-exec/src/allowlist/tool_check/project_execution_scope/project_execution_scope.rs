use crate::allowlist::ExecutionScope;

/// Whether a scope may authorize an explicitly project-local executable.
/// Xác định scope nào có thể cho phép executable được gọi từ trong project.
pub(in crate::allowlist::tool_check) fn is_project_execution_scope(scope: ExecutionScope) -> bool {
    matches!(
        scope,
        ExecutionScope::TestRunner
            | ExecutionScope::BuildRunner
            | ExecutionScope::DevServer
            | ExecutionScope::ScriptRunner
            | ExecutionScope::FrameworkInitializer
    )
}
