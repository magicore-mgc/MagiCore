// ── PA-1/PA-2: the manifest and the dispatch table must agree ───────────
// `Capability::OptimizerProvider` was claimed by `hardware` (probe always
// `Ok(())`) while `mgc optimizer` aborted for it whenever no runtime was
// detected — which is the normal shape of a `mgc create-hardware` scaffold.
// These two tests pin both directions so a third core cannot copy the block
// without proving the dispatch it claims.
// (`OptimizerProvider` từng được claim trong khi `mgc optimizer` abort — hai test
// dưới khoá cả hai chiều để core sao chép khối đó phải chứng minh được.)

#[test]
fn a_claimed_optimizer_core_produces_its_files_without_any_runtime() {
    use crate::commands::optimizer::detect::{GpuDetectionStatus, HardwareInfo, SystemProfile};
    use crate::commands::optimizer::generators::{
        OPTIMIZER_CAPABILITY_CORES, generate_optimizations_for_core,
    };

    let hw = HardwareInfo {
        cpu_cores: Some(8),
        arch: "aarch64".to_string(),
        os: "linux".to_string(),
        total_memory_gb: Some(16),
        profile: SystemProfile::Standard,
        gpus: Vec::new(),
        gpu_detection_status: GpuDetectionStatus::Unavailable,
    };
    // An EMPTY directory: no package.json, no pyproject.toml, no Cargo.toml —
    // exactly what `mgc create-hardware` leaves behind, and the case that made
    // the old hardware claim false.
    // (Thư mục RỖNG: đúng trạng thái scaffold hardware — đúng ca làm claim cũ sai.)
    let root = tempfile::tempdir().expect("temp dir");

    for core in OPTIMIZER_CAPABILITY_CORES {
        let files = generate_optimizations_for_core(core, &hw, root.path());
        let paths: Vec<&str> = files.iter().map(|f| f.relative_path.as_str()).collect();
        for expected in [".mgc-optimizer/profile.json", ".mgc-optimizer/gpu.env"] {
            assert!(
                paths.contains(&expected),
                "{core}: claims OptimizerProvider but {expected} is missing when no \
                 language runtime exists"
            );
        }
        let deploy_profile = paths.contains(&".mgc-optimizer/deploy_concurrency.env");
        assert_eq!(
            deploy_profile,
            crate::commands::optimizer::generators::DEPLOY_PROFILE_CORES.contains(core),
            "{core}: deploy_concurrency.env is generated exactly for the deploy cores"
        );
    }
}

#[test]
fn every_claimed_optimizer_core_is_survivable_without_a_runtime() {
    use crate::commands::optimizer::generators::{
        OPTIMIZER_CAPABILITY_CORES, has_core_level_optimization, optimizer_coverage,
    };

    for core in OPTIMIZER_CAPABILITY_CORES {
        assert!(
            has_core_level_optimization(core),
            "{core}: claims OptimizerProvider but `mgc optimizer` still aborts on \
             'no runtime detected' — the claim would advertise a run that always bails"
        );
        assert!(optimizer_coverage(core).survives_without_runtime());
    }
    // `hardware` is outside the contract; its gate keeps the pre-existing
    // fail-closed behaviour (no runtime ⇒ nothing was optimized).
    // (`hardware` ngoài contract; gate giữ nguyên hành vi fail-closed cũ.)
    assert!(!crate::commands::optimizer::generators::optimizer_capability_supported("hardware"));
    assert!(!has_core_level_optimization("hardware"));
}
