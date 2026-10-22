//! Tests for the Java manifest detector.
//! Kiểm thử detector manifest Java.

#![allow(clippy::unwrap_used)]

use super::*;

fn tmp(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("mgc-detect-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn java_step_fires_on_either_gradle_shape() {
    for file in ["build.gradle", "build.gradle.kts"] {
        let dir = tmp(&format!("jv-{}", file.replace('.', "-")));
        std::fs::write(dir.join(file), "").unwrap();
        let step = java_step(&dir).unwrap_or_else(|| panic!("{file} must yield a java step"));
        assert_eq!(step.ecosystem, "java");
        assert_eq!(step.scanner, "osv-maven");
    }
}

#[test]
fn java_step_fires_on_pom_xml() {
    let dir = tmp("jv-pom");
    std::fs::write(dir.join("pom.xml"), "<project/>\n").unwrap();
    let step = java_step(&dir).expect("pom.xml must yield a java step");
    assert_eq!(step.ecosystem, "java");
    assert_eq!(step.scanner, "osv-maven");
}
