//! `optimizer/generators.rs` — Core-Neutral Optimizer (Runtime Detection + Adapter Pattern)
//! `optimizer/generators.rs` — Bộ sinh cấu hình tối ưu trung lập Core theo runtime và adapter

use super::adapters;
use super::detect::HardwareInfo;
use super::runtime_detect;
use anyhow::Result;
use std::path::Path;

/// Optimized configuration file information — thông tin file cấu hình đã tối ưu
pub struct OptimizedConfigFile {
    /// Relative path from project root — đường dẫn tương đối từ project root
    pub relative_path: String,
    /// Generated file content — nội dung file được tạo
    pub content: String,
    /// Explanation of optimization reason — giải thích lý do tối ưu
    pub description: String,
}

/// Whether a core has a safe optimization profile that does not require a language runtime.
/// (Core nào có profile tối ưu an toàn mà không cần runtime ngôn ngữ cụ thể.)
#[path = "gpu_env/mod.rs"]
mod gpu_env;

pub(crate) use gpu_env::gpu_env_file;

/// What `mgc optimizer` can do for a core — used by the fail-closed gate.
/// (`mgc optimizer` làm được gì cho core — dùng cho gate fail-closed.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OptimizerCoverage {
    /// Machine-level files plus a deploy/container profile; no runtime needed.
    /// (File mức máy + profile deploy/container; không cần runtime.)
    CoreLevel,
    /// Only the machine-level files (profile.json + gpu.env) are guaranteed.
    /// A language runtime is required for anything beyond them.
    /// (Chỉ đảm bảo file mức máy; cần runtime cho phần sau.)
    MachineLevelOnly,
}

impl OptimizerCoverage {
    /// True when "no runtime detected" must NOT abort the run.
    /// (True khi "không detect được runtime" KHÔNG được phép abort.)
    pub(crate) fn survives_without_runtime(self) -> bool {
        matches!(self, Self::CoreLevel)
    }
}

/// Classify optimizer coverage for `core` (accepts the `cloud` host tag).
/// Phân loại độ phủ optimizer cho `core` (chấp nhận cả tag host `cloud`).
pub(crate) fn optimizer_coverage(core: &str) -> OptimizerCoverage {
    let canonical = mgc_types::Ecosystem::from_str(core)
        .map(|ecosystem| ecosystem.as_str())
        .unwrap_or(core);
    if DEPLOY_PROFILE_CORES.contains(&canonical) {
        OptimizerCoverage::CoreLevel
    } else {
        // Claimed cores without a deploy profile (and `hardware`, which claims
        // nothing) only guarantee the two machine-level files. The pre-run gate
        // is therefore left exactly as it was for them: a missing runtime still
        // aborts, so automation never reads "optimized" from a run that wrote
        // nothing beyond the manifest.
        // (Core đã claim nhưng không có deploy profile — và `hardware` — chỉ đảm
        // bảo hai file mức máy; gate giữ nguyên, thiếu runtime vẫn abort.)
        OptimizerCoverage::MachineLevelOnly
    }
}

/// Whether a core has a safe optimization profile that does not require a
/// language runtime.
/// (Core nào có profile tối ưu an toàn mà không cần runtime ngôn ngữ.)
#[cfg(test)]
pub(crate) fn has_core_level_optimization(core: &str) -> bool {
    optimizer_coverage(core) == OptimizerCoverage::CoreLevel
}

/// Cores for which a deploy/container concurrency profile is generated.
/// (Core được sinh profile concurrency deploy/container.)
pub(crate) const DEPLOY_PROFILE_CORES: &[&str] = &["clo", "cicd"];

/// Was: the optimizer capability contract (PA-1/PA-2, reviewLoop0902).
///
/// The table now lives in `mgc_types::capabilities`
/// (`OPTIMIZER_CAPABILITY_CORES` / `optimizer_capability_supported`) because
/// BOTH sides need it: the adapters' `probe_optimizer` and this CLI gate.
/// Re-exported here so the CLI keeps one import path and the two cannot drift.
/// (Bảng đã chuyển sang `mgc_types::capabilities` vì cả adapter lẫn CLI gate
/// đều cần; re-export để CLI giữ một điểm import và hai bên không thể lệch.)
#[cfg(test)]
pub(crate) use mgc_types::capabilities::{
    OPTIMIZER_CAPABILITY_CORES, optimizer_capability_supported,
};

/// Generate optimized configuration files for each Core and actual Hardware specs — sinh các file cấu hình tối ưu phù hợp với từng Core và thông số Hardware thực tế
/// REFACTORED: Runtime detection + adapter pattern (no hardcoding per core) — đã refactor: phát hiện runtime + pattern adapter
pub fn generate_optimizations_for_core(
    core: &str,
    hw: &HardwareInfo,
    project_root: &Path,
) -> Vec<OptimizedConfigFile> {
    let mut files = vec![];
    // One canonical name for every branch below (accepts the `cloud` host tag).
    // (Một tên chuẩn cho mọi nhánh dưới — chấp nhận tag `cloud`.)
    let core = mgc_types::Ecosystem::from_str(core)
        .map(|ecosystem| ecosystem.as_str())
        .unwrap_or(core);

    // 0. Metadata Optimizer Manifest for all Cores — metadata manifest chung cho mọi Core
    files.push(OptimizedConfigFile {
        relative_path: ".mgc-optimizer/profile.json".to_string(),
        content: serde_json::to_string_pretty(hw).unwrap_or_default(),
        description: format!(
            "System hardware snapshot & assigned profile ({:?})",
            hw.profile
        ),
    });

    // 0a. GPU facts as data (all cores) — downstream consumers can distinguish
    // a complete zero-device inventory from a partial/unavailable probe.
    // (Dữ liệu GPU phân biệt inventory đủ, partial và chưa hỗ trợ.)
    files.push(gpu_env_file(hw));

    // Cloud and CI/CD profiles tune deployment/build concurrency without
    // depending on a language runtime being present.
    // (Profile Cloud và CI/CD tối ưu concurrency mà không cần runtime ngôn ngữ.)
    if DEPLOY_PROFILE_CORES.contains(&core) {
        let upload_limit = hw.cpu_cores.map(|cores| cores.max(2));
        let upload_line = upload_limit
            .map(|limit| format!("MGC_PARALLEL_UPLOADS={limit}\n"))
            .unwrap_or_else(|| "# CPU count unknown; upload concurrency unchanged.\n".to_string());
        files.push(OptimizedConfigFile {
            relative_path: ".mgc-optimizer/deploy_concurrency.env".to_string(),
            content: format!(
                "# Generated by MagiCore Optimizer for Cloud/CICD Core ({:?})\n\
                 {upload_line}\
                 DOCKER_BUILDKIT=1\n",
                hw.profile
            ),
            description: upload_limit
                .map(|limit| format!("Deploy & container build concurrency ({limit})"))
                .unwrap_or_else(|| {
                    "Deploy and container build defaults; CPU count unknown".to_string()
                }),
        });
    }

    // 0b. Unknown RAM guard: memory-derived adapter values (heap sizes,
    // container limits) have no honest source — emit the honest manifest
    // only, never tuned files from a guessed size.
    // (Chặn RAM unknown: không nguồn trung thực — chỉ manifest.)
    if hw.total_memory_gb.is_none() {
        mgc_ui::warning(
            "RAM size unknown — skipping memory-derived adapter configs (manifest only).",
        );
        return files;
    }

    // 1. Detect runtimes for this project + core — phát hiện runtimes cho project + core này
    let detected_runtimes = runtime_detect::detect_optimizer_runtimes(project_root, core);

    // 2. Find matching adapters — tìm adapters khớp
    let matched_adapters = adapters::find_adapters(&detected_runtimes);

    // 3. Generate optimizations via adapters — tạo optimizations qua adapters
    for adapter in matched_adapters {
        let adapter_files = adapter.generate(hw);
        mgc_ui::info(&format!(
            "Optimizer: {} runtime detected, generating {} config(s)",
            adapter.name(),
            adapter_files.len()
        ));
        files.extend(adapter_files);
    }

    files
}

/// Write optimized configuration file with Hash Guard — ghi file cấu hình tối ưu với Hash Guard
pub fn apply_optimized_file(
    project_root: &Path,
    opt_file: &OptimizedConfigFile,
    force: bool,
) -> Result<bool> {
    let target_path = project_root.join(&opt_file.relative_path);

    if target_path.exists() && !force {
        let existing = std::fs::read_to_string(&target_path)?;
        // If JSON or config file without header marker -> check user modifications — nếu JSON hoặc file config không có header marker → kiểm tra sửa đổi user
        if target_path.extension().is_none_or(|ext| ext != "json")
            && !existing.starts_with("# Generated by MagiCore Optimizer")
        {
            mgc_ui::warning(&format!(
                "Skipping {} (modified by user, use --force to overwrite)",
                opt_file.relative_path
            ));
            return Ok(false);
        }
    }

    if let Some(parent) = target_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(&target_path, &opt_file.content)?;
    mgc_ui::success(&format!(
        "Optimized: {} ({})",
        opt_file.relative_path, opt_file.description
    ));
    Ok(true)
}
