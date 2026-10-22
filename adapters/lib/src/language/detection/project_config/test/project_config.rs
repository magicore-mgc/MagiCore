//! Tests for project language config outcomes.
//! Test outcome config ngôn ngữ project.

use super::super::{ProjectLanguageOutcome, read_project_language};
use crate::language::types::LibLanguage;
use std::fs;

#[test]
fn supported_alias_is_mapped_to_its_language() {
    let project = tempfile::tempdir().expect("temporary project");
    fs::write(
        project.path().join("mgc.toml"),
        "[lib]\nlanguage = \"typescript\"\n",
    )
    .expect("write project config");

    assert_eq!(
        read_project_language(project.path()),
        ProjectLanguageOutcome::Configured(LibLanguage::Ts)
    );
}

#[test]
fn unsupported_string_has_an_authoritative_outcome() {
    let project = tempfile::tempdir().expect("temporary project");
    fs::write(
        project.path().join("mgc.toml"),
        "[lib]\nlanguage = \"assembly\"\n",
    )
    .expect("write project config");

    assert_eq!(
        read_project_language(project.path()),
        ProjectLanguageOutcome::UnsupportedLanguage
    );
}

#[test]
fn foreign_ecosystem_with_regular_package_json_is_ignored() {
    let project = tempfile::tempdir().expect("temporary project");
    fs::write(project.path().join("mgc.toml"), "ecosystem = \"web\"\n")
        .expect("write foreign config");
    fs::write(project.path().join("package.json"), "{}\n").expect("write package manifest");

    assert_eq!(
        read_project_language(project.path()),
        ProjectLanguageOutcome::IgnoredJsProject
    );
}

#[test]
fn missing_malformed_or_mistyped_settings_are_not_configured() {
    let project = tempfile::tempdir().expect("temporary project");
    assert_eq!(
        read_project_language(project.path()),
        ProjectLanguageOutcome::NotConfigured
    );

    fs::write(
        project.path().join("mgc.toml"),
        "[lib\nlanguage = \"rust\"\n",
    )
    .expect("write malformed config");
    assert_eq!(
        read_project_language(project.path()),
        ProjectLanguageOutcome::NotConfigured
    );

    fs::write(project.path().join("mgc.toml"), "[lib]\nlanguage = 4\n")
        .expect("write mistyped config");
    assert_eq!(
        read_project_language(project.path()),
        ProjectLanguageOutcome::NotConfigured
    );
}
