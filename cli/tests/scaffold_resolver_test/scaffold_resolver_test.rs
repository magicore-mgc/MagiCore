use mgc::scaffold::resolver::{
    MissingLayersReport, ScaffoldResolveError, ScaffoldResolveStatus, spec_to_layer_path,
};
use mgc::scaffold::spec::{CoreKind, ScaffoldRef, ScaffoldSpec};
use std::path::PathBuf;

#[test]
fn existing_public_module_path_exposes_all_resolver_contracts() {
    let status = ScaffoldResolveStatus::CacheHit {
        layer: "web/frontend/app".to_string(),
        version: Some("1.0.0".to_string()),
        path: PathBuf::from("cache/app"),
    };
    assert!(status.is_available());
    assert_eq!(status.layer(), "web/frontend/app");

    let error = ScaffoldResolveError::UnsupportedCore {
        core: "other".to_string(),
    };
    assert!(error.to_string().contains("hardware"));

    let mut report = MissingLayersReport::new();
    report.add_required("app/flutter".to_string());
    assert!(report.has_required_missing());

    let spec = ScaffoldSpec {
        core: CoreKind::App,
        name: "Flutter".to_string(),
        normalized_name: "flutter".to_string(),
        requested_ref: ScaffoldRef::Default,
    };
    assert_eq!(spec_to_layer_path(&spec), "app/flutter");
}
