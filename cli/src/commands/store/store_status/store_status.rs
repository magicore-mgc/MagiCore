//! Reports installed package count and virtual-store location.
//! Báo số package đã cài và vị trí virtual-store.

use super::super::store_paths::{store_db_for, vstore_root_for};

pub(in crate::commands::store) fn run_status(project_root: &std::path::Path) -> anyhow::Result<()> {
    let db = mgc_store::database::Database::open(&store_db_for(project_root)?)?;
    let installed = db.list_installed()?;
    println!("Store status: {} installed package(s)", installed.len());
    println!("virtual store: {}", vstore_root_for(project_root).display());
    Ok(())
}
