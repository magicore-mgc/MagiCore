//! Maps a detected project core to its required toolchain binaries.
//! Ánh xạ core của project sang các binary toolchain bắt buộc.

use mgc_types::Ecosystem;

use super::super::toolchain::TOOLCHAIN_PROBES;

/// Canonical tool name used when a required tool has no general probe row.
/// Tên tool chuẩn dùng khi tool bắt buộc chưa có hàng probe chung.
///
/// A core tool that is not in `TOOLCHAIN_PROBES` is still probed through the
/// same executor path; its version argument defaults to `--version`.
/// Tool core không nằm trong `TOOLCHAIN_PROBES` vẫn được probe qua cùng
/// đường executor; đối số version mặc định là `--version`.
pub(in crate::commands::doctor) const DEFAULT_TOOL_PROBE_ARG: &str = "--version";

/// One core's required toolchain row (a data label, never a command route).
/// Một hàng dữ liệu: core và các toolchain nó yêu cầu (nhãn dữ liệu, không
/// phải route lệnh).
pub(in crate::commands::doctor) struct CoreRow {
    pub(in crate::commands::doctor) core: &'static str,
    pub(in crate::commands::doctor) tools: &'static [&'static str],
}

/// Core → required toolchain binary names, in display order.
/// Bảng core → binary toolchain bắt buộc, theo thứ tự hiển thị.
///
/// Names are the canonical core id from `mgc_types::Ecosystem` (`clo`, not
/// `cloud`). `lib` intentionally lists a superset because its language is not
/// known until the manifests are read. This is a data table of labels, not a
/// command descriptor list.
/// Tên là core id chuẩn của `mgc_types::Ecosystem` (`clo`, không phải `cloud`).
/// `lib` cố tình liệt kê superset vì chưa biết ngôn ngữ đến khi đọc manifest.
/// Đây là bảng dữ liệu nhãn, không phải danh sách descriptor lệnh.
pub(in crate::commands::doctor) const CORE_REQUIRED_TOOLCHAINS: &[CoreRow] = &[
    CoreRow {
        core: "web",
        tools: &["node"],
    },
    CoreRow {
        core: "lib",
        tools: &["rustc", "cargo", "python3", "node", "go", "java"],
    },
    CoreRow {
        core: "ai",
        tools: &["python3"],
    },
    CoreRow {
        core: "app",
        tools: &["flutter", "gradle", "swift"],
    },
    CoreRow {
        core: "game",
        tools: &["cargo", "godot"],
    },
    CoreRow {
        core: "iot",
        tools: &["cargo", "pio", "west"],
    },
    CoreRow {
        core: "clo",
        tools: &["terraform", "node"],
    },
    CoreRow {
        core: "cicd",
        tools: &["git", "node"],
    },
    CoreRow {
        core: "hardware",
        tools: &[],
    },
];

/// A toolchain binary required by a project core.
/// Một binary toolchain mà core của project yêu cầu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::commands::doctor) struct CoreToolRequirement {
    /// Canonical core id that requires this tool.
    /// Core id chuẩn yêu cầu tool này.
    pub core: &'static str,
    /// Executable basename searched on PATH.
    /// Basename executable tra trên PATH.
    pub binary: &'static str,
    /// Fixed, read-only version probe arguments.
    /// Đối số probe version chỉ-đọc cố định.
    pub probe_args: &'static [&'static str],
}

/// Resolve the required toolchain entries for an already-detected core name.
/// Trả các entry toolchain bắt buộc cho một tên core đã nhận diện.
///
/// The input may be a `cloud` alias; it is normalized through the workspace
/// alias table so the lookup stays single-tabled.
/// Đầu vào có thể là alias `cloud`; chuẩn hoá qua bảng alias của workspace để
/// việc tra cứu chỉ có MỘT bảng.
pub(in crate::commands::doctor) fn requirements_for_core(core: &str) -> Vec<CoreToolRequirement> {
    let canonical = Ecosystem::canonical_core_name(core);
    let Some(row) = CORE_REQUIRED_TOOLCHAINS
        .iter()
        .find(|row| row.core == canonical)
    else {
        return Vec::new();
    };
    requirements_from_binaries(core_name(canonical.as_str()), row.tools)
}

/// Build requirements for an explicit binary list under one canonical core.
/// Dựng requirement cho danh sách binary tường minh dưới một core chuẩn.
///
/// Shared by the core table and the project-aware variant so a tool's probe
/// arguments never come from two places.
/// Dùng chung cho bảng core và biến thể theo project để đối số probe của một tool
/// không đến từ hai nơi.
fn requirements_from_binaries(
    core: &'static str,
    binaries: &[&'static str],
) -> Vec<CoreToolRequirement> {
    binaries
        .iter()
        .map(|binary| CoreToolRequirement {
            core,
            binary,
            probe_args: probe_args_for(binary),
        })
        .collect()
}

/// Required toolchain for a core plus the project's own split server half.
/// Toolchain bắt buộc cho core cộng nửa server tách rời của chính project.
///
/// A `web` project whose `server/` half runs on Python/Rust/Go must probe that
/// runner too; every other core keeps exactly its table row.
/// Project `web` có nửa `server/` chạy bằng Python/Rust/Go phải probe cả runner đó;
/// core khác giữ đúng hàng của bảng.
pub(in crate::commands::doctor) fn requirements_for_project(
    core: &str,
    project_root: &std::path::Path,
) -> Vec<CoreToolRequirement> {
    let mut requirements = requirements_for_core(core);
    if Ecosystem::canonical_core_name(core) != "web" {
        return requirements;
    }
    let Some(core_name) = requirements
        .first()
        .map(|requirement| requirement.core)
        .or_else(|| {
            CORE_REQUIRED_TOOLCHAINS
                .iter()
                .map(|row| row.core)
                .find(|name| *name == "web")
        })
    else {
        return requirements;
    };
    for binary in super::split_server_half_tools(project_root) {
        if requirements
            .iter()
            .any(|requirement| requirement.binary == *binary)
        {
            continue;
        }
        requirements.push(CoreToolRequirement {
            core: core_name,
            binary,
            probe_args: probe_args_for(binary),
        });
    }
    requirements
}

/// Pin a canonical core string to the static table lifetime for display.
/// Ghim chuỗi core chuẩn vào lifetime của bảng tĩnh để hiển thị.
fn core_name(canonical: &str) -> &'static str {
    CORE_REQUIRED_TOOLCHAINS
        .iter()
        .map(|row| row.core)
        .find(|name| *name == canonical)
        .unwrap_or("")
}

/// Reuse the general probe arguments when a tool is already in the inventory.
/// Dùng lại đối số probe chung khi tool đã có trong inventory.
fn probe_args_for(binary: &str) -> &'static [&'static str] {
    TOOLCHAIN_PROBES
        .iter()
        .find(|&&(_, bin, _)| bin == binary)
        .map(|&(_, _, args)| args)
        .unwrap_or(&[DEFAULT_TOOL_PROBE_ARG])
}
