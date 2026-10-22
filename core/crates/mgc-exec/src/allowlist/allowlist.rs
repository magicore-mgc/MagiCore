//! Allowlist check — kiểm tra tool trước khi exec (00-index §5.1, §5.2)
//!
//! ## Threat Model (2026-09-02)
//!
//! MagiCore orchestrates package managers, NOT sandboxes them. Two security boundaries:
//!
//! 1. **Install scope (HIGH RISK)**: Package installation, registry fetch, transitive deps
//!    - PM tools (npm/pnpm/yarn/bun) FORBIDDEN → use `mgc install` (resolver + audit)
//!    - Rationale: Prevent arbitrary package fetch bypassing mgc resolver
//!
//! 2. **Test/Build/Dev scopes**: package-manager executables remain forbidden.
//!    Compiler/runtime execution is a separate, explicit allowlist decision.
//!
//! ## ExecutionScope
//! Package managers are forbidden in every scope. Test/build/dev may execute
//! approved compilers and runtimes, but never package resolution/install tools.

use anyhow::Result;
use std::path::Path;

/// Kiểm tool trước khi exec: cấm vĩnh viễn → lỗi rõ lý do; ngoài allowlist → lỗi.
/// DEPRECATED: Use check_tool_with_scope for new code (supports ExecutionScope).
pub fn check_tool(name: &str) -> Result<()> {
    super::tool_check::check_tool_with_scope(name, super::ExecutionScope::Install, None)
}

/// Historical React Native detector retained only for migration diagnostics.
/// Bộ nhận diện React Native cũ chỉ phục vụ chẩn đoán migration.
pub fn is_react_native_subdir(cwd: Option<&Path>) -> bool {
    super::react_native_subdir::is_react_native_subdir(cwd)
}
