//! Native dependency ownership gate diagnostics.
//! Lỗi rào quyền sở hữu dependency native.

mod error;

pub use error::{
    compat_runner_unsupported_core, dep_gate_compat_disabled, dep_gate_invalid_tool,
    dep_gate_lane_tool_not_owned, dep_gate_requires_compat, dep_gate_tool_mismatch,
    dep_gate_unsupported, dep_gate_wrong_tool, install_packages_toolchain_owned,
    lane_needs_compat_runner, monorepo_compat_tool_denied, native_dependency_engine_unavailable,
    rival_runtime_not_native, tool_manifest_mismatch,
};
