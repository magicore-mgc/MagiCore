//! Native deployment result fields — field kết quả deploy native.

#[derive(Debug, Clone)]
pub struct DeployResult {
    pub dry_run: bool,
    pub changes: Vec<String>,
    pub duration_ms: u64,
}
