//! Run optimizer for a project using detected hardware and core adapters.
//! Chạy optimizer cho project dựa trên hardware phát hiện được và adapter theo core.

use anyhow::Result;
use std::path::Path;

use crate::commands::optimizer::{detect, generators, runtime_detect};

/// Run optimization for project based on Core and Hardware detection — chạy tối ưu hóa dự án dựa trên Core và phát hiện Hardware
/// REFACTORED: Runtime detection → adapter dispatch (no hardcoded language/runtime) — đã refactor: phát hiện runtime → dispatch adapter
pub fn optimize_project(project_root: &Path, core: &str, force: bool) -> Result<()> {
    let hw = detect::HardwareInfo::detect();
    if hw.total_memory_gb.is_none() {
        mgc_ui::warning(
            "RAM size could not be detected on this machine — profile degraded to Constrained and memory-derived tuning is skipped (no fabricated values).",
        );
    }
    if hw.cpu_cores.is_none() {
        mgc_ui::warning(
            "Logical CPU count could not be detected — CPU-derived tuning is omitted (no fabricated core count).",
        );
    }
    mgc_ui::info(&format!(
        "Detected System: {} ({}), {} Logical Cores, {} RAM -> Profile: {:?}",
        hw.os,
        hw.arch,
        hw.cpu_cores
            .map(|cores| cores.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        hw.total_memory_gb
            .map(|gb| format!("~{gb}GB"))
            .unwrap_or_else(|| "unknown".to_string()),
        hw.profile
    ));
    if hw.gpus.is_empty() {
        let message = match hw.gpu_detection_status {
            detect::GpuDetectionStatus::Available => {
                "GPU inventory complete: no PCI display device detected."
            }
            detect::GpuDetectionStatus::Partial => {
                "GPU inventory is partial; no device was fully identified."
            }
            detect::GpuDetectionStatus::Unavailable => {
                "GPU inventory unavailable on this platform; no hardware claim made."
            }
        };
        mgc_ui::info(message);
    } else {
        let names: Vec<String> = hw
            .gpus
            .iter()
            .map(|g| match (g.vendor.as_deref(), g.vram_mb) {
                (Some(v), Some(mb)) => format!("{} [{} | {} MiB]", g.name, v, mb),
                (Some(v), None) => format!("{} [{}]", g.name, v),
                (None, _) => g.name.clone(),
            })
            .collect();
        mgc_ui::info(&format!("Detected GPUs: {}", names.join("; ")));
    }

    // Detect runtimes for this project — phát hiện runtimes cho project này
    let detected_runtimes = runtime_detect::detect_optimizer_runtimes(project_root, core);
    let runtime_unavailable = detected_runtimes.is_empty()
        || matches!(
            detected_runtimes[0],
            runtime_detect::DetectedRuntime::Unknown
        );
    if runtime_unavailable && !generators::optimizer_coverage(core).survives_without_runtime() {
        // Fail closed: automation reading Ok(()) here would report a
        // successful optimization that never happened (V1.2: no success
        // without verifiable state change).
        // (Fail-closed: automation đọc Ok(()) ở đây sẽ báo tối ưu thành
        // công trong khi chưa có gì xảy ra.)
        anyhow::bail!(
            "No runtime detected for `{core}` core in {} — nothing was optimized (not a success).",
            project_root.display()
        );
    }

    mgc_ui::info(&format!(
        "Detected runtimes for `{core}`: {:?}",
        detected_runtimes
    ));

    // Generate optimizations via adapters — tạo optimizations qua adapters
    let files = generators::generate_optimizations_for_core(core, &hw, project_root);
    if files.is_empty() {
        mgc_ui::info(&format!(
            "No specific hardware optimizations needed for `{core}` core."
        ));
        return Ok(());
    }

    let mut applied_count = 0;
    for file in &files {
        if generators::apply_optimized_file(project_root, file, force)? {
            applied_count += 1;
        }
    }

    mgc_ui::success(&format!(
        "MagiCore Optimizer finished: applied {applied_count}/{} configurations for `{core}`.",
        files.len()
    ));
    Ok(())
}
