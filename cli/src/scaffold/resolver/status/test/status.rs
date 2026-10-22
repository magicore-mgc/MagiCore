use crate::scaffold::resolver::ScaffoldResolveStatus;
use std::path::PathBuf;

#[test]
fn embedded_status_is_available_and_exposes_its_layer() {
    let status = ScaffoldResolveStatus::Embedded {
        layer: "web/frontend/vanilla".to_string(),
    };
    assert!(status.is_available());
    assert_eq!(status.layer(), "web/frontend/vanilla");
}

#[test]
fn cache_hit_status_is_available_and_exposes_its_layer() {
    let status = ScaffoldResolveStatus::CacheHit {
        layer: "web/frontend/nextjs".to_string(),
        version: Some("15.5.0".to_string()),
        path: PathBuf::from("/tmp/cache/nextjs"),
    };
    assert!(status.is_available());
    assert_eq!(status.layer(), "web/frontend/nextjs");
}

#[test]
fn fetched_status_is_available_and_exposes_its_layer() {
    let status = ScaffoldResolveStatus::Fetched {
        layer: "ai/fastapi".to_string(),
        version: "0.115.0".to_string(),
        path: PathBuf::from("/tmp/cache/fastapi"),
    };
    assert!(status.is_available());
    assert_eq!(status.layer(), "ai/fastapi");
}

#[test]
fn optional_missing_status_is_not_available_and_exposes_its_layer() {
    let status = ScaffoldResolveStatus::OptionalMissing {
        layer: "web/partial/extra".to_string(),
        reason: "not in cache".to_string(),
    };
    assert!(!status.is_available());
    assert_eq!(status.layer(), "web/partial/extra");
}
