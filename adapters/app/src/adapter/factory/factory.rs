//! App project detection and adapter construction.
//! Nhận diện project App và tạo adapter.

use crate::adapter::{AppAdapter, PendingInstallState};
use crate::language::detect_language;
use std::path::Path;

pub fn adapter_for(root: &Path) -> Option<AppAdapter> {
    let language = detect_language(root)?;
    Some(AppAdapter {
        language,
        project_root: root.to_path_buf(),
        pending_install: PendingInstallState::default(),
    })
}
