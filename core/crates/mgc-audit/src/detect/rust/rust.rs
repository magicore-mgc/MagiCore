//! Detect the existing Rust manifest lane.
//! Nhận diện lane manifest Rust hiện có.
use crate::contract::ScanStep;
use std::path::Path;

/// Cargo.toml → MGC's native OSV scanner (rust).
/// Cargo.toml → scanner OSV native của MGC (Rust).
pub fn rust_step(project_root: &Path) -> Option<ScanStep<'static>> {
    if !project_root.join("Cargo.toml").is_file() {
        return None;
    }
    let root = project_root.to_path_buf();
    Some(ScanStep {
        ecosystem: "rust",
        scanner: "mgc-rust-osv",
        run: Box::new(move || {
            let root = root.clone();
            Box::pin(async move { crate::scanners::audit_rust(&root).await })
        }),
    })
}
