//! Packagist catalog projection tests against the captured p2 document.
//! Test chiếu catalog Packagist trên tài liệu p2 đã capture.

use crate::protocols::composer::package_catalog::PackagistDocument;
use crate::protocols::composer::package_catalog::dependency_metadata::{
    DependencyKind, parse_composer_dependencies,
};
use crate::protocols::composer::package_catalog::{
    DIST_REFERENCE_MARKER_PREFIX, project_entry, select_entry,
};
use crate::protocols::composer::test::common::monolog_document;
use mgc_types::MgError;

fn select(range: &str) -> crate::protocols::ResolvedEntry {
    select_entry("monolog/monolog", &monolog_document(), &[range.to_string()])
        .expect("captured Packagist fixture must resolve")
}

#[test]
fn highest_matching_version_is_selected() {
    assert_eq!(select("*").version, "3.12.1");
    assert_eq!(select("^3.0").version, "3.12.1");
    assert_eq!(select(">=3.10 <3.12").version, "3.11.0");
    let max_2 = select("^2.0");
    assert_eq!(max_2.version, "2.10.0");
    assert_eq!(select("1.0.0").version, "1.0.0");
    // A pre-release only wins when the constraint pins it: `^3.0` takes the
    // stable 3.x release, not the newer 3.0.0-RC1 in the fixture.
    // Pre-release chỉ thắng khi ràng buộc ghim đúng nó: `^3.0` lấy bản ổn
    // định 3.x chứ không lấy 3.0.0-RC1 mới hơn trong fixture.
    assert_eq!(select("^3.0.0-RC1").version, "3.12.1");
    assert_eq!(select("3.0.0-RC1").version, "3.0.0-RC1");
    assert_eq!(select("3.0.0-RC1").deps.len(), 0);
    assert_eq!(select("3.10.0").version, "3.10.0");
}

#[test]
fn no_matching_version_is_a_dependency_conflict() {
    let error = select_entry(
        "monolog/monolog",
        &monolog_document(),
        &["^4.0".to_string()],
    )
    .expect_err("no 4.x release exists in the captured fixture");
    assert!(matches!(error, MgError::DependencyConflict(_)), "{error:?}");
}

#[test]
fn missing_package_in_document_fails_closed() {
    let document: PackagistDocument =
        serde_json::from_str(r#"{"minified":"composer/2.0","packages":{}}"#)
            .expect("empty document must parse");
    let error = select_entry("monolog/monolog", &document, &["*".to_string()])
        .expect_err("a document without the package cannot resolve");
    assert!(matches!(error, MgError::Other(_)), "{error:?}");
}

#[test]
fn dist_identity_uses_zip_url_and_reference() {
    let entry = select("3.10.0");
    assert_eq!(entry.version, "3.10.0");
    // GitHub zipball dists are rewritten to codeload (no User-Agent needed).
    // Dist zipball GitHub được đổi sang codeload (không cần User-Agent).
    assert_eq!(
        entry.artifact_url,
        "https://codeload.github.com/Seldaek/monolog/legacy.zip/b321dd6749f0bf7189444158a3ce785cc16d69b0"
    );
    // Packagist publishes an empty shasum for this release, so the integrity
    // source is the VCS dist reference.
    // Packagist để shasum rỗng cho bản này nên nguồn toàn vẹn là reference.
    assert!(entry.sha256.is_empty());
    assert!(entry.extra_markers.contains(&format!(
        "{DIST_REFERENCE_MARKER_PREFIX}b321dd6749f0bf7189444158a3ce785cc16d69b0"
    )));
}

#[test]
fn require_and_require_dev_are_separated_and_platform_skipped() {
    let entry = select("3.12.1");
    let deps: Vec<(String, String)> = entry.deps.clone();
    assert!(
        deps.contains(&("psr/log".to_string(), "^2.0 || ^3.0".to_string())),
        "{deps:?}"
    );
    // `require-dev` never enters the production graph; it is a marker.
    // `require-dev` không vào graph production mà là marker.
    assert!(
        !deps
            .iter()
            .any(|(name, _)| name.contains("guzzlehttp/guzzle")),
        "{deps:?}"
    );
    assert!(
        entry
            .extra_markers
            .contains(&"require-dev:guzzlehttp/guzzle:^7.4.5".to_string()),
        "{:?}",
        entry.extra_markers
    );
    // `php` and `ext-json` are platform packages, never registry edges.
    // `php` và `ext-json` là package platform, không phải cạnh registry.
    assert!(deps.iter().all(|(name, _)| name != "php"), "{deps:?}");
    assert!(
        deps.iter().all(|(name, _)| !name.contains("ext-json")),
        "{deps:?}"
    );
    assert!(
        entry
            .extra_markers
            .contains(&"require:php:>=8.1".to_string())
    );
    assert!(
        entry
            .extra_markers
            .contains(&"require-dev:ext-json:*".to_string())
    );
    // Composer development edges carry the `dev:` prefix, so they sort
    // ahead of bare production names; the prefix is what marks them.
    // Cạnh development mang tiền tố `dev:` nên xếp trước tên production
    // trần; chính tiền tố đánh dấu loại cạnh.
    assert!(deps.iter().any(|(name, _)| name == "psr/log"), "{deps:?}");
    assert!(
        deps.iter().all(|(name, _)| name != "guzzlehttp/guzzle"),
        "{deps:?}"
    );
}

#[test]
fn platform_packages_are_classified() {
    let mut require = std::collections::BTreeMap::new();
    let mut require_dev = std::collections::BTreeMap::new();
    require.insert("php-64bit".to_string(), ">=8.1".to_string());
    require.insert("lib-curl".to_string(), ">=7".to_string());
    require_dev.insert("ext-mbstring".to_string(), "*".to_string());
    require_dev.insert("psr/log".to_string(), "^3.0".to_string());
    let parsed = parse_composer_dependencies(&require, &require_dev);
    assert!(
        parsed
            .platform_markers
            .contains(&"require:php-64bit:>=8.1".to_string())
    );
    assert!(
        parsed
            .platform_markers
            .contains(&"require:lib-curl:>=7".to_string())
    );
    assert!(
        parsed
            .platform_markers
            .contains(&"require-dev:ext-mbstring:*".to_string())
    );
    assert_eq!(
        parsed.edges,
        vec![(
            DependencyKind::Development,
            "psr/log".to_string(),
            "^3.0".to_string()
        )]
    );
}

#[test]
fn empty_dependency_sections_leave_no_edges() {
    let entry = select("3.10.0");
    assert!(entry.deps.is_empty(), "{:?}", entry.deps);
}

#[test]
fn missing_dist_url_and_reference_fail_closed() {
    let mut version = monolog_document()
        .packages
        .get("monolog/monolog")
        .and_then(|versions| versions.first())
        .cloned()
        .expect("fixture has a first entry");
    version
        .dist
        .as_mut()
        .expect("fixture has a dist")
        .url
        .clear();
    let error = project_entry("monolog/monolog", "3.12.1", &version, &mut Vec::new())
        .expect_err("a dist without a url cannot resolve");
    assert!(matches!(error, MgError::Other(_)), "{error:?}");

    let mut version = monolog_document()
        .packages
        .get("monolog/monolog")
        .and_then(|versions| versions.first())
        .cloned()
        .expect("fixture has a first entry");
    let dist = version.dist.as_mut().expect("fixture has a dist");
    dist.reference = None;
    let error = project_entry("monolog/monolog", "3.12.1", &version, &mut Vec::new())
        .expect_err("no shasum and no reference means unverifiable");
    assert!(matches!(error, MgError::Integrity(_)), "{error:?}");
}

#[test]
fn published_shasum_becomes_the_integrity_marker() {
    let mut version = monolog_document()
        .packages
        .get("monolog/monolog")
        .and_then(|versions| versions.first())
        .cloned()
        .expect("fixture has a first entry");
    let shasum = "A".repeat(40);
    version.dist.as_mut().expect("fixture has a dist").shasum = Some(shasum.clone());
    let entry = project_entry("monolog/monolog", "3.12.1", &version, &mut Vec::new())
        .expect("a valid shasum resolves");
    assert!(
        entry
            .extra_markers
            .contains(&format!("checksum-sha1:{}", shasum.to_ascii_lowercase()))
    );

    version.dist.as_mut().expect("fixture has a dist").shasum = Some("not-a-sha1".to_string());
    let error = project_entry("monolog/monolog", "3.12.1", &version, &mut Vec::new())
        .expect_err("a malformed shasum fails closed");
    assert!(matches!(error, MgError::Integrity(_)), "{error:?}");
}

#[test]
fn non_zip_dist_is_unsupported() {
    let mut version = monolog_document()
        .packages
        .get("monolog/monolog")
        .and_then(|versions| versions.first())
        .cloned()
        .expect("fixture has a first entry");
    version
        .dist
        .as_mut()
        .expect("fixture has a dist")
        .archive_type = "tar".to_string();
    let error = project_entry("monolog/monolog", "3.12.1", &version, &mut Vec::new())
        .expect_err("only zip dists are supported");
    assert!(matches!(error, MgError::Unsupported { .. }), "{error:?}");
}

#[test]
fn replace_and_provide_become_markers() {
    let mut require = std::collections::BTreeMap::new();
    require.insert("replace:psr/log".to_string(), "3.0.0".to_string());
    require.insert(
        "provide:psr/log-implementation".to_string(),
        "3.0.0".to_string(),
    );
    let parsed = parse_composer_dependencies(&require, &std::collections::BTreeMap::new());
    assert!(parsed.edges.is_empty());
    assert_eq!(
        parsed.substituted,
        vec!["psr/log".to_string(), "psr/log-implementation".to_string()]
    );
}
