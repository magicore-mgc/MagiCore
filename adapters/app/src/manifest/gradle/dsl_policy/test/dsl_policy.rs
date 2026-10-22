//! Tests for Gradle build-file selection and refusal policy.
//! Test chọn build file Gradle và policy từ chối thao tác.

use super::super::parse_error;
use super::super::write_error;
use crate::manifest::gradle::error::GradleManifestError;
use std::fs;

#[test]
fn missing_build_file_keeps_legacy_error() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let error = parse_error(directory.path());

    assert!(
        matches!(error, GradleManifestError::Other(message) if message == "build.gradle not found")
    );
}

#[test]
fn kotlin_dsl_has_priority_and_generic_write_stays_unsupported() {
    let directory = tempfile::tempdir().expect("temporary directory");
    fs::write(directory.path().join("build.gradle"), "plugins {}\n").expect("write Groovy DSL");
    fs::write(directory.path().join("build.gradle.kts"), "plugins {}\n").expect("write Kotlin DSL");

    let error = parse_error(directory.path());
    assert!(matches!(
        error,
        GradleManifestError::Unsupported {
            capability: "parse-gradle-manifest",
            guidance,
            ..
        } if guidance.contains("build.gradle.kts")
    ));
    assert!(matches!(
        write_error(),
        GradleManifestError::Unsupported {
            capability: "write-gradle-manifest",
            ..
        }
    ));
}
