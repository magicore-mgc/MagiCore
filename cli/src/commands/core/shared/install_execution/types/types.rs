//! Carry the resolved graph and install summary across preparation and execution.
//! Mang resolved graph và install summary giữa bước chuẩn bị và thực thi.

use mgc_types::ResolvedGraph;

pub(crate) struct InstallExecution {
    pub graph: ResolvedGraph,
    pub summary: mgc_types::adapter::InstallSummary,
    pub used_lockfile: bool,
}
