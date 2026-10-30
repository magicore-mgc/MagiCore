//! THE optimizer capability contract — which cores may claim `OptimizerProvider`.
//! Hợp đồng capability optimizer — core nào được phép claim `OptimizerProvider`.

use crate::ecosystem::Ecosystem;

/// Cores whose adapter may claim `Capability::OptimizerProvider`.
///
/// This table is the single source of truth for two decisions that used to be
/// written separately and disagreed (PA-1/PA-2, reviewLoop0902):
///
/// 1. the `mgc optimizer` coverage/gate in `cli/src/commands/optimizer`, and
/// 2. every adapter's `OptimizerProvider::probe_optimizer`.
///
/// Admission rule — a core belongs here only if `mgc optimizer` produces a
/// verifiable state change for a project of that core **without any language
/// runtime**:
/// - every listed core always gets `.mgc-optimizer/profile.json` (machine
///   profile) and `.mgc-optimizer/gpu.env` (GPU inventory);
/// - `clo`/`cicd` additionally get `.mgc-optimizer/deploy_concurrency.env`.
///
/// `hardware` is deliberately absent. Its v1.2.0 manifest is the
/// optimizer/bench template listing, which is `ScaffoldProvider` evidence; a
/// hardware project only receives the two machine files through the generic
/// lane and `mgc optimizer` aborts when no runtime is detected — exactly the
/// shape of a `mgc create-hardware` scaffold. See
/// `adapters/hardware/src/adapter/optimizer_provider`.
/// (Bảng nguồn chân lý duy nhất cho quyết định của cả CLI optimizer và probe
/// của adapter. Điều kiện vào bảng: `mgc optimizer` phải tạo thay đổi kiểm
/// chứng được khi KHÔNG có runtime ngôn ngữ. `hardware` cố ý vắng.)
pub const OPTIMIZER_CAPABILITY_CORES: &[&str] =
    &["web", "lib", "ai", "app", "game", "iot", "clo", "cicd"];

/// Whether `core` may claim the `OptimizerProvider` capability.
///
/// Accepts the CLI canonical name and the `cloud` host tag; both resolve
/// through [`Ecosystem`] so this is alias-safe by construction.
/// (Core có được claim `OptimizerProvider` không — nhận cả tag `cloud`.)
pub fn optimizer_capability_supported(core: &str) -> bool {
    let canonical = Ecosystem::from_str(core)
        .map(|ecosystem| ecosystem.as_str())
        .unwrap_or(core);
    OPTIMIZER_CAPABILITY_CORES.contains(&canonical)
}
