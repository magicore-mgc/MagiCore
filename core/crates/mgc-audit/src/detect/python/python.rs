//! Detect the existing Python manifest lane.
//! Nhận diện lane manifest Python hiện có.
use crate::contract::ScanStep;
use std::path::Path;

/// Any Python dependency manifest → MGC's native OSV scanner (python). Recognized set
/// mirrors the scanner's own routing: requirements variants, PEP 751
/// pylock variants, uv.lock, pyproject.toml. A manifest the scanner
/// cannot consume (uv.lock without export, pyproject without lockfile)
/// still yields a step — the SCANNER reports the honest Failed, the
/// gate never hides a recognized manifest.
/// Mọi manifest dependency Python → scanner OSV native của MGC. Manifest scanner không
/// đọc được vẫn cho step — SCANNER báo Failed trung thực, gate không
/// giấu manifest đã nhận diện.
pub fn python_step(project_root: &Path) -> Option<ScanStep<'static>> {
    let recognized = crate::scanners::find_requirements_file(project_root).is_some()
        || crate::scanners::find_pylock_file(project_root).is_some()
        || project_root.join("uv.lock").is_file()
        || project_root.join("pyproject.toml").is_file();
    if !recognized {
        return None;
    }
    let root = project_root.to_path_buf();
    Some(ScanStep {
        ecosystem: "python",
        scanner: "mgc-python-osv",
        run: Box::new(move || {
            let root = root.clone();
            Box::pin(async move { crate::scanners::audit_python(&root).await })
        }),
    })
}
