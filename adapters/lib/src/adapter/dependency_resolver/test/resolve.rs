//! Verify the language-to-resolver lane and Java refusal mapping.
//! Kiểm tra ánh xạ ngôn ngữ sang resolver lane và từ chối Java.

use super::super::resolve::resolver_lane;
use crate::adapter::types::{JavaManifestKind, ResolveFirst};
use crate::language::LibLanguage;
use mgc_types::MgError;

#[test]
fn each_native_language_selects_its_existing_resolver_lane() {
    assert!(matches!(
        resolver_lane(LibLanguage::Rust, JavaManifestKind::None).unwrap(),
        Some(ResolveFirst::Rust)
    ));
    assert!(matches!(
        resolver_lane(LibLanguage::Python, JavaManifestKind::None).unwrap(),
        Some(ResolveFirst::Python)
    ));
    assert!(matches!(
        resolver_lane(LibLanguage::Go, JavaManifestKind::None).unwrap(),
        Some(ResolveFirst::Go)
    ));
    assert!(matches!(
        resolver_lane(LibLanguage::Java, JavaManifestKind::Pom).unwrap(),
        Some(ResolveFirst::JavaPom)
    ));
    assert!(matches!(
        resolver_lane(LibLanguage::DotNet, JavaManifestKind::None).unwrap(),
        Some(ResolveFirst::DotNet)
    ));
    assert!(
        resolver_lane(LibLanguage::Ts, JavaManifestKind::None)
            .unwrap()
            .is_none()
    );
}

#[test]
fn non_maven_java_routes_keep_typed_fail_closed_errors() {
    let ambiguous_guidance = "both Maven and Gradle manifests are present; select one build system before native dependency operations";
    for kind in [JavaManifestKind::Gradle, JavaManifestKind::None] {
        assert!(matches!(
            resolver_lane(LibLanguage::Java, kind),
            Err(MgError::Unsupported {
                core: "lib",
                capability: "resolve",
                ..
            })
        ));
    }
    assert!(matches!(
        resolver_lane(LibLanguage::Java, JavaManifestKind::Ambiguous),
        Err(MgError::Unsupported {
            core: "lib",
            capability: "resolve",
            guidance,
        }) if guidance == ambiguous_guidance
    ));
}
