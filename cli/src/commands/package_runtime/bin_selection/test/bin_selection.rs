//! Focused bin selection regression tests.
//! Test hồi quy tập trung cho bin selection.

use super::*;
use std::path::PathBuf;

fn entries(rows: &[(&str, &str)]) -> Vec<(String, PathBuf)> {
    rows.iter()
        .map(|(name, target)| ((*name).into(), (*target).into()))
        .collect()
}
#[test]
fn single_nonmatching_name() {
    assert_eq!(
        select_bin_name(&entries(&[("serve", "cli.js")]), "some-package", None).unwrap(),
        "serve"
    );
}
#[test]
fn scoped_basename_wins() {
    assert_eq!(
        select_bin_name(
            &entries(&[("helper", "helper.js"), ("proof", "proof.js")]),
            "@mgc/proof",
            None
        )
        .unwrap(),
        "proof"
    );
}
#[test]
fn explicit_typescript_selection() {
    assert_eq!(
        select_bin_name(
            &entries(&[("tsc", "bin/tsc"), ("tsserver", "bin/tsserver")]),
            "typescript",
            Some("tsc")
        )
        .unwrap(),
        "tsc"
    );
}
#[test]
fn ambiguous_typescript_is_not_guessed() {
    assert!(
        select_bin_name(
            &entries(&[("tsc", "bin/tsc"), ("tsserver", "bin/tsserver")]),
            "typescript",
            None
        )
        .is_err()
    );
}
#[test]
fn aliases_choose_deterministically() {
    assert_eq!(
        select_bin_name(&entries(&[("z", "cli.js"), ("a", "cli.js")]), "pkg", None).unwrap(),
        "a"
    );
}
#[test]
fn undeclared_dependency_bin_and_path_are_denied() {
    for selected in ["other", "../bin", "/bin", "..", "", "bad\nname"] {
        assert!(select_bin_name(&entries(&[("safe", "cli.js")]), "pkg", Some(selected)).is_err());
    }
}
#[test]
fn no_bin_and_unsafe_declared_name_are_denied() {
    assert!(select_bin_name(&[], "pkg", None).is_err());
    assert!(select_bin_name(&entries(&[("../escape", "cli.js")]), "pkg", None).is_err());
}
