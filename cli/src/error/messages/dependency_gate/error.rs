//! Native dependency ownership gate diagnostics.
//! Lỗi rào quyền sở hữu dependency native.

use anyhow::Error;
use anyhow::anyhow;

/// Legacy delegated operation is blocked because dependency compatibility is disabled.
pub fn dep_gate_requires_compat(
    core: &str,
    op: &str,
    ecosystem: Option<&str>,
    tools: &[&str],
) -> Error {
    let lane = match ecosystem {
        Some(eco) => format!("`{core}` {op} (ecosystem '{eco}')"),
        None => format!("`{core}` {op}"),
    };
    anyhow!(
        "{lane} is unsupported by MagiCore's native dependency engine (known external tools: {}) — external package-manager execution is disabled for dependency operations",
        tools.join(", ")
    )
}

/// An operation with no dependency lifecycle at all (no engine, no lane).
/// cicd/hardware lanes are scaffold-only by design — the message says so
/// instead of letting anyone read them as package installs.
pub fn dep_gate_unsupported(core: &str, op: &str, ecosystem: Option<&str>) -> Error {
    let lane = match ecosystem {
        Some(eco) => format!("`{core}` {op} (ecosystem '{eco}')"),
        None => format!("`{core}` {op}"),
    };
    if matches!(core, "cicd" | "hardware") {
        anyhow!(
            "{lane} has no package lifecycle — this is a scaffold-only lane (templates/pipelines/generators), never a native package install"
        )
    } else {
        anyhow!(
            "{lane} is unsupported for dependency lifecycle — the manifest is toolchain-owned, but no approved compatibility runner is configured for this operation"
        )
    }
}

/// Refuse dependency operations until MagiCore owns their complete native
/// lifecycle; this deliberately offers no toolchain/compatibility escape.
/// Từ chối lifecycle dependency khi MagiCore chưa sở hữu engine native;
/// không mở đường vòng qua package manager ngoài.
pub fn native_dependency_engine_unavailable(core: &str, ecosystem: &str, op: &str) -> Error {
    anyhow!(
        "`{core}` {op} for `{ecosystem}` is unavailable: MagiCore does not yet own the complete native dependency lifecycle for this lane. No external package manager was invoked; use a supported native lane or wait for native resolver, lock, fetch, verify, store, and materializer support."
    )
}

/// Legacy compat flag named a tool for a lane without native ownership.
/// Compatibility execution is disabled; tool identity does not grant an exception.
pub fn dep_gate_wrong_tool(
    core: &str,
    op: &str,
    ecosystem: Option<&str>,
    got: &str,
    tools: &[&str],
) -> Error {
    let lane = match ecosystem {
        Some(eco) => format!("`{core}` {op} (ecosystem '{eco}')"),
        None => format!("`{core}` {op}"),
    };
    anyhow!(
        "`--compat-runtime '{got}'` cannot authorize {lane} (known external tools: {}) — external package-manager execution is disabled for dependency operations",
        tools.join(", ")
    )
}

/// The lane's exact tool is not in the owner set at all — a lane/table
/// mismatch (lane bug), failed closed, never silently run.
pub fn dep_gate_lane_tool_not_owned(
    core: &str,
    op: &str,
    ecosystem: Option<&str>,
    actual: &str,
    tools: &[&str],
) -> Error {
    let lane = match ecosystem {
        Some(eco) => format!("`{core}` {op} (ecosystem '{eco}')"),
        None => format!("`{core}` {op}"),
    };
    anyhow!(
        "lane tool '{actual}' does not own {lane} (owner toolchain: {}) — refusing to run (lane/table mismatch, fail-closed)",
        tools.join(", ")
    )
}

/// The opt-in names a DIFFERENT tool than the lane will actually spawn
/// (e.g. `--compat-runtime uv` on a lane that spawns pip): the flag must
/// equal the process (flag==process contract) — failed closed with both
/// names so the user sees the mismatch.
pub fn dep_gate_tool_mismatch(
    core: &str,
    op: &str,
    ecosystem: Option<&str>,
    wanted: &str,
    actual: &str,
    tools: &[&str],
) -> Error {
    let lane = match ecosystem {
        Some(eco) => format!("`{core}` {op} (ecosystem '{eco}')"),
        None => format!("`{core}` {op}"),
    };
    anyhow!(
        "`--compat-runtime '{wanted}'` does not own {lane}: this lane spawns '{actual}' (owner toolchain: {}) — pass `--compat-runtime {actual}` (the flag must name the process that will run)",
        tools.join(", ")
    )
}

/// --compat-runtime value outside the known tool universe for dependency ops.
pub fn dep_gate_invalid_tool(tool: &str, valid: &[&str]) -> Error {
    anyhow!(
        "invalid --compat-runtime '{tool}' for dependency operations — accepted tool names: {}; compatibility execution is disabled for dependency operations",
        valid.join(", ")
    )
}

/// Dependency commands do not permit compatibility package-manager execution.
/// Dependency-op không cho phép chạy package manager qua compatibility.
pub fn dep_gate_compat_disabled(value: &str) -> Error {
    anyhow!(
        "--compat-runtime '{value}' is disabled for dependency operations: MagiCore will not invoke an external package manager; use a fully supported native lane"
    )
}

/// Generic install with package arguments on a manifest MagiCore cannot mutate.
/// Install tổng quát không được sửa manifest mà MagiCore chưa sở hữu.
pub fn install_packages_toolchain_owned(adapter_name: &str) -> Error {
    anyhow!(
        "generic install cannot add packages to the '{adapter_name}' manifest because MagiCore does not own its native dependency lifecycle; no external package manager was invoked"
    )
}

/// A rival JS runtime cannot be used as a hidden or explicit MGC engine.
/// Runtime JS đối thủ không được dùng làm engine ẩn hay opt-in của MGC.
pub fn rival_runtime_not_native(runtime: &str) -> Error {
    anyhow!(
        "'{runtime}' cannot run through MagiCore: the native MagiCore runtime for this project is not implemented; no external runtime was invoked"
    )
}

/// Monorepo member has no package.json and its toolchain was NOT opted
/// into: pass exactly `--compat-runtime=<tool>` for the member kind
/// (go/pip/cargo/mvn/composer) — one flag never opens every toolchain.
/// NOTE: takes the pre-formatted opt-in description (not CompatMode) so
/// this module stays includable from integration tests without the full
/// command tree.
/// (Thành viên monorepo không có package.json — phải opt-in ĐÚNG tool;
/// một flag không mở mọi toolchain.)
pub fn monorepo_compat_tool_denied(
    project_root: &std::path::Path,
    tool: Option<&str>,
    have_opt_in: &str,
) -> Error {
    match tool {
        Some(tool) => anyhow!(
            "monorepo member '{}' needs its '{tool}' toolchain, but the invocation opted into '{have_opt_in}' — pass --compat-runtime={tool} (explicit only, never auto)",
            project_root.display(),
        ),
        None => anyhow!(
            "monorepo member '{}' has no package.json and no recognized toolchain manifest (go.mod/requirements.txt/Cargo.toml/pom.xml/composer.json) — nothing to delegate to",
            project_root.display()
        ),
    }
}

/// A mutation adapter returned success but rereading the manifest shows no change.
/// Adapter báo thành công nhưng đọc lại manifest không thấy thay đổi.
pub fn tool_manifest_mismatch(package: &str, adapter: &str, detail: &str) -> Error {
    anyhow!(
        "'{package}' was not {detail} by {adapter}; MagiCore re-read the manifest and found no requested change"
    )
}

/// `--compat-runner` on a core without a qualified compat contract.
/// (`--compat-runner` trên core chưa có compat contract.)
pub fn compat_runner_unsupported_core(core: &str) -> Error {
    anyhow!(
        "--compat-runner is only supported for the web core right now; refusing to claim compat parity for '{core}'"
    )
}

/// Pre-flight profile gate: this lane observably requires package managers
/// at runtime, so native mode refuses before spawning anything.
/// (Cổng profile pre-flight: lane này chắc chắn cần package manager lúc
/// runtime, nên native mode từ chối trước khi spawn.)
pub fn lane_needs_compat_runner(framework: &str, managers: &[String]) -> Error {
    anyhow!(
        "lane '{framework}' requires package managers at runtime ({}) — native mode refuses before spawning; rerun with --compat-runner <manager> or use a native alternative",
        managers.join(", ")
    )
}
