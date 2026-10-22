//! Selects safe concurrency for a monorepo install.
//! Chọn mức song song an toàn cho install monorepo.

use std::path::PathBuf;

pub(in crate::commands::core::web::implementation::package_lifecycle::install::monorepo_install) fn monorepo_install_concurrency(
    package_targets: &[PathBuf],
) -> usize {
    if package_targets.is_empty() {
        return 1;
    }

    if let Some(override_value) = std::env::var("MAGICORE_WEB_MONOREPO_INSTALL_CONCURRENCY")
        .ok()
        .and_then(|raw| raw.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
    {
        return override_value;
    }

    let default = std::thread::available_parallelism()
        .map(|count| count.get().min(4))
        .unwrap_or(2)
        .max(1);

    if looks_like_cold_monorepo_install(package_targets) {
        return 1;
    }

    default
}

fn looks_like_cold_monorepo_install(package_targets: &[PathBuf]) -> bool {
    if package_targets.len() <= 1 {
        return false;
    }

    package_targets.iter().all(|target| {
        !target.join("node_modules").exists()
            && !target.join("mgc.lock").exists()
            && !target.join(".magicore").join("cache").join("web").exists()
    })
}
