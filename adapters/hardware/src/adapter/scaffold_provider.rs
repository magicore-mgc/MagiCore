//! `ScaffoldProvider` for the Hardware core: the CLI lane scaffolds it.
//! `ScaffoldProvider` cho core Hardware: lane CLI scaffold core này.

use super::adapter::HardwareAdapter;
use mgc_types::MgResult;
use mgc_types::capabilities::ScaffoldProvider;

impl ScaffoldProvider for HardwareAdapter {
    /// Evidence: `mgc create-hardware <optimizer|bench>` writes `mgc.toml`,
    /// `optimizer.json` and `README.md`.
    /// Dẫn chứng: `mgc create-hardware` ghi `mgc.toml`, `optimizer.json`,
    /// `README.md`.
    fn probe_scaffold(&self) -> MgResult<()> {
        Ok(())
    }
}
