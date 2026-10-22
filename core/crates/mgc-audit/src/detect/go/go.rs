//! Detect the existing Go manifest lane.
//! Nhận diện lane manifest Go hiện có.
use crate::contract::ScanStep;
use std::path::Path;

/// go.mod → MGC's native OSV scanner (go).
/// go.mod → scanner OSV native của MGC (Go).
pub fn go_step(project_root: &Path) -> Option<ScanStep<'static>> {
    if !project_root.join("go.mod").is_file() {
        return None;
    }
    let root = project_root.to_path_buf();
    Some(ScanStep {
        ecosystem: "go",
        scanner: "mgc-go-osv",
        run: Box::new(move || {
            let root = root.clone();
            Box::pin(async move { crate::scanners::audit_go(&root).await })
        }),
    })
}
