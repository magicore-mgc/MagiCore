#![allow(clippy::unwrap_used)]
//! Dedupe preference tests — DedupePref/PreferExisting + peer merge (02 §2.1)
//! (Test ưu tiên version có sẵn trong lockfile khi prefer-dedupe)
use async_trait::async_trait;
use mgc_resolver::solver::{
    DedupePref, DependencyError, DependencyProvider, ResolvedDep, Resolver,
};
use mgc_types::{PackageId, PackageName, Version};
use std::collections::HashMap;
use std::sync::Arc;

struct VersionsProvider {
    versions: Vec<&'static str>,
}

#[async_trait]
impl DependencyProvider for VersionsProvider {
    async fn get_versions(&self, _: &PackageName) -> Result<Vec<Version>, DependencyError> {
        Ok(self
            .versions
            .iter()
            .map(|s| Version::parse(s).expect("test operation must succeed"))
            .collect())
    }

    async fn get_dependencies(&self, _: &PackageId) -> Result<Vec<ResolvedDep>, DependencyError> {
        Ok(vec![])
    }
}

#[tokio::test]
async fn prefer_existing_reuses_installed_version_over_latest() {
    let provider = Arc::new(VersionsProvider {
        versions: vec!["1.0.0", "2.0.0"],
    });
    let resolver = Resolver::new(provider);
    resolver.set_dedupe_pref(DedupePref::PreferExisting);
    let mut existing = HashMap::new();
    existing.insert(
        "react".to_string(),
        Version::parse("1.0.0").expect("test operation must succeed"),
    );
    resolver.set_existing_versions(existing);

    let wanted = vec![(
        PackageName::new("react").expect("test operation must succeed"),
        "^1.0.0".to_string(),
    )];
    let result = resolver
        .solve(&wanted)
        .await
        .expect("test operation must succeed");
    assert_eq!(result.resolutions[0].version.to_string(), "1.0.0");
}

#[tokio::test]
async fn prefer_existing_ignores_installed_version_outside_range() {
    let provider = Arc::new(VersionsProvider {
        versions: vec!["1.0.0", "2.0.0"],
    });
    let resolver = Resolver::new(provider);
    resolver.set_dedupe_pref(DedupePref::PreferExisting);
    let mut existing = HashMap::new();
    existing.insert(
        "react".to_string(),
        Version::parse("1.0.0").expect("test operation must succeed"),
    );
    resolver.set_existing_versions(existing);

    // Range ^2.0.0 — installed 1.0.0 does NOT match → must pick 2.0.0.
    let wanted = vec![(
        PackageName::new("react").expect("test operation must succeed"),
        "^2.0.0".to_string(),
    )];
    let result = resolver
        .solve(&wanted)
        .await
        .expect("test operation must succeed");
    assert_eq!(result.resolutions[0].version.to_string(), "2.0.0");
}

#[tokio::test]
async fn prefer_latest_default_picks_newest() {
    let provider = Arc::new(VersionsProvider {
        versions: vec!["1.0.0", "1.2.0"],
    });
    let resolver = Resolver::new(provider);
    let wanted = vec![(
        PackageName::new("react").expect("test operation must succeed"),
        "^1.0.0".to_string(),
    )];
    let result = resolver
        .solve(&wanted)
        .await
        .expect("test operation must succeed");
    assert_eq!(result.resolutions[0].version.to_string(), "1.2.0");
}

struct PeerVersionProvider;

#[async_trait]
impl DependencyProvider for PeerVersionProvider {
    async fn get_versions(&self, package: &PackageName) -> Result<Vec<Version>, DependencyError> {
        let versions: &[&str] = match package.as_str() {
            "widget" => &["1.2.0", "1.9.0"],
            "root" => &["1.0.0"],
            _ => &[],
        };
        versions
            .iter()
            .map(|version| {
                Version::parse(version).map_err(|error| DependencyError(error.to_string()))
            })
            .collect()
    }

    async fn get_dependencies(&self, id: &PackageId) -> Result<Vec<ResolvedDep>, DependencyError> {
        if id.name_str() != "root" {
            return Ok(Vec::new());
        }
        Ok(vec![ResolvedDep {
            package: PackageName::new("widget").expect("parse transitive package name"),
            spec: "^1.8.0".to_owned(),
            optional: false,
            peer: false,
        }])
    }
}

#[tokio::test]
async fn root_aware_result_keeps_old_direct_pin_separate_from_newer_matching_transitive() {
    let resolver = Resolver::new(Arc::new(PeerVersionProvider));
    resolver.set_dedupe_pref(DedupePref::PreferExisting);
    resolver.set_existing_versions(HashMap::from([(
        "widget".to_owned(),
        Version::parse("1.2.0").expect("test operation must succeed"),
    )]));
    let wanted = vec![
        (
            PackageName::new("widget").expect("test operation must succeed"),
            "^1.0.0".to_owned(),
        ),
        (
            PackageName::new("root").expect("test operation must succeed"),
            "^1.0.0".to_owned(),
        ),
    ];

    let result = resolver
        .solve_with_roots(&wanted)
        .await
        .expect("test operation must succeed");

    let direct_widget = result
        .root_resolutions
        .iter()
        .find(|root| root.request_index == 0)
        .expect("test operation must succeed");
    assert_eq!(direct_widget.package_id.to_string(), "widget@1.2.0");
    assert_eq!(result.root_resolutions.len(), 2);
    assert_eq!(
        result
            .root_resolutions
            .iter()
            .find(|root| root.request_index == 1)
            .expect("test operation must succeed")
            .package_id
            .to_string(),
        "root@1.0.0"
    );
    assert!(
        result
            .result
            .resolutions
            .iter()
            .any(|entry| entry.package_id.to_string() == "widget@1.9.0")
    );
}
