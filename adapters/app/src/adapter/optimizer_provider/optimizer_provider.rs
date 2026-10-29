//! `OptimizerProvider` for the App core.
//! `OptimizerProvider` cho core App.

use crate::adapter::AppAdapter;
use mgc_types::MgResult;
use mgc_types::capabilities::OptimizerProvider;

impl OptimizerProvider for AppAdapter {
    /// Evidence: `mgc optimizer` dispatches this core and writes its profile.
    /// Dẫn chứng: `mgc optimizer` dispatch core này và ghi profile.
    fn probe_optimizer(&self) -> MgResult<()> {
        Ok(())
    }
}
