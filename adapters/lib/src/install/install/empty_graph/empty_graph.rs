//! Preserve no-network empty-graph install and lock publication behavior.
//! Giữ hành vi không mạng và công bố lock của graph rỗng.

use crate::install::{
    atomic_write_cargo_lock, direct_package_ids, ecosystem_for_language, prepare_empty_cargo_lock,
    write_canonical_lock_with_roots,
};
use crate::language::LibLanguage;
use mgc_types::adapter::InstallSummary;
use mgc_types::{MgResult, ResolvedGraph};
use std::path::Path;

pub(in crate::install) fn install_empty_resolved_graph(
    language: LibLanguage,
    graph: &ResolvedGraph,
    project_root: &Path,
    offline: bool,
    lock_packages: Vec<mgc_lockfile::Package>,
) -> MgResult<InstallSummary> {
    if offline {
        if language == LibLanguage::Rust {
            // An empty resolved graph is safe only when the Cargo manifest is
            // dependency-free. Validate it without publishing generated lock bytes.
            // (Graph rỗng chỉ hợp lệ khi Cargo.toml không khai báo dependency;
            // xác minh nhưng không ghi lock ở offline.)
            let _ = prepare_empty_cargo_lock(project_root, &lock_packages)?;
        }
        return Ok(InstallSummary::default());
    }

    // No artifacts need fetching. Write the selected lock but avoid opening
    // registries, stores or network probes.
    // (Không có artifact để tải: ghi lock được chọn nhưng không mở registry,
    // store hoặc dò mạng.)
    let cargo_lock_update = if language == LibLanguage::Rust {
        prepare_empty_cargo_lock(project_root, &lock_packages)?
    } else {
        None
    };
    write_canonical_lock_with_roots(
        project_root,
        ecosystem_for_language(language),
        lock_packages,
        direct_package_ids(graph),
    )?;
    if let Some(bytes) = cargo_lock_update {
        atomic_write_cargo_lock(project_root, &bytes)?;
    }
    Ok(InstallSummary::default())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
