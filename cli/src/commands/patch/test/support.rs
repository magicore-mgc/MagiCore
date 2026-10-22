//! Shared patch test project fixture. / Fixture project dùng chung cho test patch.

use mgc_config::project::ProjectConfig;
use std::path::Path;

pub(super) fn project(root: &Path) {
    let mut config = ProjectConfig::new("patch-test", "web");
    config.lock = Some(mgc_config::project::LockConfig {
        acquire_timeout_ms: Some(1),
        ..Default::default()
    });
    config.save(root).unwrap();
}
