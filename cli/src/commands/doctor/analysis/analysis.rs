//! Combines inventory snapshots into a stable diagnostic report.
//! Kết hợp snapshot inventory thành diagnostic report ổn định.

use super::super::{
    model::{DoctorReport, RegistryStatus, Tool},
    storage::StoreSnapshot,
};

pub(in crate::commands::doctor) fn build(
    toolchain: Vec<Tool>,
    store: StoreSnapshot,
    registries_reachable: Vec<RegistryStatus>,
) -> DoctorReport {
    let (mut detected_issues, mut suggested_actions) =
        super::toolchain_issues::diagnostics(&toolchain);
    let (store_issues, store_actions) = super::store_issues::diagnostics(&store);
    detected_issues.extend(store_issues);
    suggested_actions.extend(store_actions);
    let health_status = super::health_status::classify(&detected_issues);

    DoctorReport {
        toolchain,
        store_dir: store.directory.display().to_string(),
        store_writable: store.writable,
        disk_free_bytes: store.free_bytes,
        registries_reachable,
        health_status,
        detected_issues,
        suggested_actions,
    }
}
