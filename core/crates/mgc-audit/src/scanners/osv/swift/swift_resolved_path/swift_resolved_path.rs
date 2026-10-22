//! bounded Package.resolved lookup at project root, ios, and one parent level.
//! Tìm Package.resolved trong phạm vi project đã giới hạn.

use std::path::{Path, PathBuf};

/// Shared Package.resolved locator (R2'/R1 + REVIEW F1): adapters gate
/// their swift step on THIS, the same search the scanner runs. Scope is
/// deliberately bounded — project root, `ios/` (React Native layout),
/// and exactly one parent level (audited from a project subdir). A
/// filesystem-root walk would attribute ANOTHER scope's lockfile to this
/// audit, so anything farther stays out (honest Unsupported, not a
/// borrowed scan).
/// Locator Package.resolved dùng chung: scope giới hạn chủ ý — root,
/// `ios/`, và đúng một cấp cha. Đi xa hơn là lockfile của scope khác.
pub fn swift_resolved_path(project_root: &Path) -> Option<PathBuf> {
    let direct = project_root.join("Package.resolved");
    if direct.is_file() {
        return Some(direct);
    }
    let ios = project_root.join("ios").join("Package.resolved");
    if ios.is_file() {
        return Some(ios);
    }
    if let Some(parent) = project_root.parent() {
        let up = parent.join("Package.resolved");
        if up.is_file() {
            return Some(up);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// CocoaPods — Podfile.lock (PODS: section, deterministic indentation).
