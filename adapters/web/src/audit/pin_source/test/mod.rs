//! Wire the pin-source behavior tests.
//! Nối các test hành vi của pin source.

#![allow(clippy::unwrap_used)]

use crate::audit::pin_source::PinSourceError;

/// Run the public audit entry without a Tokio test-macro dependency.
/// Chạy entry audit công khai không cần Tokio test-macro.
fn run_audit_blocking(
    directory: &std::path::Path,
) -> Result<mgc_types::adapter::AuditReport, String> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("test runtime");
    runtime
        .block_on(crate::audit::run_audit(
            directory,
            "https://registry.npmjs.org",
        ))
        .map_err(|error| error.to_string())
}

#[path = "p0_3_clean_project_without_any_lockfile_reports_clean_zero.rs"]
mod p0_3_clean_project_without_any_lockfile_reports_clean_zero;
#[path = "p0_3_deno_lock_without_mgc_lock_fails_closed_with_import_remediation.rs"]
mod p0_3_deno_lock_without_mgc_lock_fails_closed_with_import_remediation;
#[path = "p0_3_rival_lockfile_without_mgc_lock_fails_closed_with_import_remediation.rs"]
mod p0_3_rival_lockfile_without_mgc_lock_fails_closed_with_import_remediation;
#[path = "pin_source_error_preserves_rival_remediation.rs"]
mod pin_source_error_preserves_rival_remediation;
