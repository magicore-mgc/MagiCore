//! `OptimizerProvider` for the web core.
//! `OptimizerProvider` cho core web.

use crate::adapter::WebAdapter;
use mgc_types::MgResult;
use mgc_types::capabilities::OptimizerProvider;

impl OptimizerProvider for WebAdapter {
    /// Evidence: `mgc optimizer` for `web` completes WITHOUT a language
    /// runtime — it always writes `.mgc-optimizer/profile.json` and
    /// `.mgc-optimizer/gpu.env`, and for `clo`/`cicd` also
    /// `.mgc-optimizer/deploy_concurrency.env` (generators step 0/0a/`deploy_concurrency`).
    ///
    /// PA-1/PA-2 (reviewLoop0902): the claim and the CLI dispatch table used to
    /// be written independently, so the manifest could advertise a core the gate
    /// then refused. The probe now reads the contract that
    /// `OPTIMIZER_CAPABILITY_CORES` pins, so the two cannot disagree:
    /// if this line ever fails, the core must be added there (or the claim dropped).
    /// (Dẫn chứng: `mgc optimizer` cho `web` hoàn tất khi KHÔNG có runtime —
    /// luôn ghi profile.json + gpu.env. Probe đọc chính bảng contract.)
    fn probe_optimizer(&self) -> MgResult<()> {
        mgc_types::capabilities::optimizer_capability_supported("web")
            .then_some(())
            .ok_or_else(|| {
                mgc_types::capabilities::unsupported_capability(
                    "web",
                    "optimizer",
                    "this core is not in the optimizer capability contract",
                )
            })
    }
}
