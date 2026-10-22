//! Converts the store snapshot and low-space advice into diagnostics.
//! Chuyển snapshot store và cảnh báo disk thấp thành diagnostic.

use crate::commands::doctor::DiagnosticIssue;

use super::super::super::{
    low_disk_space,
    storage::{StoreSnapshot, StoreStatus},
};

pub(in crate::commands::doctor::analysis) fn diagnostics(
    store: &StoreSnapshot,
) -> (Vec<DiagnosticIssue>, Vec<String>) {
    let mut issues = Vec::new();
    let mut actions = Vec::new();

    match &store.status {
        StoreStatus::Missing => {
            issues.push(DiagnosticIssue {
                code: "ERR_STORE_MISSING",
                severity: "CRITICAL",
                message: format!(
                    "MagiCore global store '{}' does not exist.",
                    store.directory.display()
                ),
                fix_command: Some("mgc doctor check --fix".to_string()),
            });
            actions.push(format!(
                "Create the missing store directory with `mgc doctor check --fix`: {}",
                store.directory.display()
            ));
        }
        StoreStatus::BrokenSymlink { path } => {
            issues.push(DiagnosticIssue {
                code: "ERR_STORE_BROKEN_SYMLINK",
                severity: "CRITICAL",
                message: format!(
                    "MagiCore global store path '{}' contains a symbolic link whose target does not exist at '{}'.",
                    store.directory.display(),
                    path.display()
                ),
                fix_command: None,
            });
            actions.push(format!(
                "Inspect and repair the existing broken symbolic link manually: {}",
                path.display()
            ));
        }
        StoreStatus::Writable => {}
        StoreStatus::NotDirectory => {
            issues.push(DiagnosticIssue {
                code: "ERR_STORE_NOT_DIRECTORY",
                severity: "CRITICAL",
                message: format!(
                    "MagiCore global store path '{}' exists but is not a directory.",
                    store.directory.display()
                ),
                fix_command: None,
            });
            actions.push(format!(
                "Inspect the existing non-directory store path manually: {}",
                store.directory.display()
            ));
        }
        StoreStatus::NotWritable(error) => {
            issues.push(DiagnosticIssue {
                code: "ERR_STORE_NOT_WRITABLE",
                severity: "CRITICAL",
                message: format!(
                    "MagiCore global store '{}' is not writable: {error}",
                    store.directory.display()
                ),
                fix_command: None,
            });
            actions.push(format!(
                concat!(
                    "Inspect the owner and permissions for the existing store; ",
                    "`--fix` will not change them: {}"
                ),
                store.directory.display()
            ));
        }
        StoreStatus::InspectionFailed(error) => {
            issues.push(DiagnosticIssue {
                code: "WARN_STORE_STATE_UNKNOWN",
                severity: "WARNING",
                message: format!(
                    "Could not determine the state of MagiCore global store '{}': {error}",
                    store.directory.display()
                ),
                fix_command: None,
            });
            actions.push(format!(
                "Check whether the current user can inspect the configured store path: {}",
                store.directory.display()
            ));
        }
    }

    if let Some(error) = &store.free_space_error {
        issues.push(DiagnosticIssue {
            code: "WARN_STORE_DISK_UNKNOWN",
            severity: "WARNING",
            message: format!(
                "Could not determine free space for configured MagiCore store '{}': {}",
                store.directory.display(),
                error
            ),
            fix_command: None,
        });
        let action = match &store.volume_path {
            Some(volume_path) => format!(
                "Inspect free space at existing volume path '{}' for configured store '{}'.",
                volume_path.display(),
                store.directory.display()
            ),
            None => format!(
                "Inspect the configured store path and its mounted volumes manually: {}",
                store.directory.display()
            ),
        };
        actions.push(action);
    }

    if let Some(advice) = low_disk_space::advice(
        store.free_bytes,
        &store.directory,
        store.volume_path.as_deref().unwrap_or(&store.directory),
    ) {
        issues.push(DiagnosticIssue {
            code: "WARN_LOW_DISK_SPACE",
            severity: "WARNING",
            message: advice.message,
            fix_command: None,
        });
        actions.push(advice.suggested_action);
    }
    (issues, actions)
}
