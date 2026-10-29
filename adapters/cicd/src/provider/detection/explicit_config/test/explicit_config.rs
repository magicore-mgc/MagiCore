//! Tests for explicit provider config outcomes.
//! Test các outcome của config provider tường minh.

use super::super::{ProviderConfigOutcome, read_configured_provider};
use crate::provider::CicdProvider;
use std::fs;

#[test]
fn recognized_provider_is_returned() {
    let project = tempfile::tempdir().expect("temporary directory");
    fs::write(
        project.path().join("mgc.toml"),
        "[cicd]\nprovider = \"gcp\"\n",
    )
    .expect("write project config");

    assert_eq!(
        read_configured_provider(project.path()),
        ProviderConfigOutcome::Recognized(CicdProvider::Gcp)
    );
}

#[test]
fn unsupported_string_has_a_distinct_outcome() {
    let project = tempfile::tempdir().expect("temporary directory");
    fs::write(
        project.path().join("mgc.toml"),
        "[cicd]\nprovider = \"travis\"\n",
    )
    .expect("write project config");

    assert_eq!(
        read_configured_provider(project.path()),
        ProviderConfigOutcome::UnsupportedName
    );
}

#[test]
fn absent_malformed_or_mistyped_value_is_not_configured() {
    let project = tempfile::tempdir().expect("temporary directory");
    assert_eq!(
        read_configured_provider(project.path()),
        ProviderConfigOutcome::NotConfigured
    );

    fs::write(
        project.path().join("mgc.toml"),
        "[cicd\nprovider = \"aws\"\n",
    )
    .expect("write malformed project config");
    assert_eq!(
        read_configured_provider(project.path()),
        ProviderConfigOutcome::NotConfigured
    );

    fs::write(project.path().join("mgc.toml"), "[cicd]\nprovider = 8\n")
        .expect("write mistyped project config");
    assert_eq!(
        read_configured_provider(project.path()),
        ProviderConfigOutcome::NotConfigured
    );
}
