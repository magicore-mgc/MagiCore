//! `OptimizerProvider` for the cicd core.
//! `OptimizerProvider` cho core cicd.

use crate::adapter::CicdAdapter;
use mgc_types::MgResult;
use mgc_types::capabilities::OptimizerProvider;

impl OptimizerProvider for CicdAdapter {
    /// Evidence: `mgc optimizer` dispatches this core and writes its profile.
    ///
    /// This capability was withheld while `ai` declared it with the SAME
    /// evidence — "the optimizer CLI dispatches this core". The dispatch is
    /// shared infrastructure (`optimizer/generators.rs` writes `profile.json`
    /// and `gpu.env` for every core), so withholding it here advertised less
    /// than the product does.
    /// (Capability từng bị giữ lại trong khi `ai` khai với CÙNG dẫn chứng —
    /// "optimizer CLI dispatch core này". Dispatch là hạ tầng dùng chung nên giữ
    /// lại là quảng cáo ít hơn thực tế.)
    fn probe_optimizer(&self) -> MgResult<()> {
        Ok(())
    }
}
