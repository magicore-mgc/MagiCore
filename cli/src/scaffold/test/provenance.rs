//! Tests for scaffold provenance records.
//! Test cho record provenance của scaffold.
use super::*;
use tempfile::TempDir;

#[test]
fn test_provenance_write_read() {
    let dir = TempDir::new().unwrap();
    let prov = ScaffoldProvenance::new(
        "nextjs".to_string(),
        "web".to_string(),
        "15.5.0".to_string(),
        Some("https://registry.npmjs.org".to_string()),
        vec!["web/frontend/nextjs".to_string()],
    );

    prov.write(dir.path()).unwrap();

    let read = ScaffoldProvenance::read(dir.path()).unwrap();
    assert_eq!(read.template, "nextjs");
    assert_eq!(read.version, "15.5.0");
    assert_eq!(read.core, "web");
}

#[test]
fn test_provenance_includes_timestamp() {
    let prov = ScaffoldProvenance::new(
        "fastapi".to_string(),
        "ai".to_string(),
        "0.115.0".to_string(),
        None,
        vec![],
    );
    assert!(!prov.generated_at.is_empty());
    assert!(prov.generated_at.contains('T')); // ISO 8601
}

#[test]
fn test_provenance_includes_mgc_version() {
    let prov = ScaffoldProvenance::new(
        "flutter".to_string(),
        "app".to_string(),
        "stable".to_string(),
        None,
        vec![],
    );
    assert!(!prov.mgc_version.is_empty());
}
