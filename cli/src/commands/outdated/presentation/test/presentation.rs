//! Tests stable outdated labels and terminal text.
//! Kiểm tra nhãn và output terminal outdated ổn định.

#![allow(clippy::unwrap_used)]

use super::super::{outdated_line, outdated_summary, severity_label};
use crate::commands::outdated::model::OutdatedPkg;

#[test]
fn severity_labels_keep_the_existing_major_minor_and_patch_branches() {
    assert_eq!(severity_label("^1.0.0", 2), "major");
    assert_eq!(severity_label("~2.1.0", 2), "minor");
    assert_eq!(severity_label("3.0.0", 2), "patch");
}

#[test]
fn terminal_row_and_summary_keep_the_existing_text() {
    let package = OutdatedPkg {
        name: "widget".to_string(),
        current: "^1.2.0".to_string(),
        latest: "2.0.0".to_string(),
        major: 2,
        minor: 0,
        patch: 0,
    };
    assert_eq!(outdated_line(&package), "  widget: ^1.2.0 → 2.0.0 (major)");
    assert_eq!(
        outdated_summary(1),
        "1 package(s) outdated. Run 'mgc update' to update."
    );
}
