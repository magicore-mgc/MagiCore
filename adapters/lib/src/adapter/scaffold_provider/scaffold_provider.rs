use crate::adapter::types::LibAdapter;
use mgc_types::MgResult;
use mgc_types::capabilities::ScaffoldProvider;

impl ScaffoldProvider for LibAdapter {
    /// Evidence: `mgc create-lib <language>` scaffold lane (CLI create
    /// commands). Dẫn chứng: lane scaffold `mgc create-lib <language>`.
    fn probe_scaffold(&self) -> MgResult<()> {
        Ok(())
    }
}
