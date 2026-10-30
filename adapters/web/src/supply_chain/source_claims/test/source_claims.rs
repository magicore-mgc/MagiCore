//! Project `[[sources]]` claim enforcement and lockfile source tests.
//! Test thực thi claim `[[sources]]` của project và source ghi vào lockfile.

use super::super::{
    RegistryOrigin, ensure_resolved_sources_satisfy_claims, project_lockfile_sources,
};
use crate::supply_chain::error::SupplyChainError;
use mgc_types::adapter::ResolvedPackage;
use mgc_types::{PackageId, Version};
use std::path::Path;

const REGISTRY: &str = "https://registry.npmjs.org";

fn package(name: &str, version: &str, tarball_url: &str) -> ResolvedPackage {
    ResolvedPackage {
        id: PackageId::new(
            mgc_types::PackageName::new(name).expect("valid package name"),
            Version::parse(version).expect("valid version"),
        ),
        integrity: String::new(),
        tarball_url: tarball_url.to_string(),
        deps: Vec::new(),
        peer_deps: Vec::new(),
        direct: true,
        dev: false,
    }
}

fn origin<'a>(url: &'a str) -> RegistryOrigin<'a> {
    RegistryOrigin {
        url,
        ecosystem: "web",
    }
}

fn project(root: &Path, sources: &str) {
    std::fs::write(
        root.join("mgc.toml"),
        format!("name = \"claims-test\"\necosystem = \"web\"\n{sources}"),
    )
    .expect("write project config");
}

const CORP_CLAIM: &str = r#"
[[sources]]
id = "corp"
url = "https://npm.corp.example"
ecosystem = "web"
priority = 0
claims = ["@corp/*"]
trusted = true
allow_hosts = ["npm.corp.example"]
"#;

#[test]
fn a_claimed_scope_served_by_another_registry_is_refused() {
    let root = tempfile::tempdir().expect("create project");
    project(root.path(), CORP_CLAIM);
    let packages = [package(
        "@corp/internal",
        "1.0.0",
        "https://registry.npmjs.org/@corp/internal/-/internal-1.0.0.tgz",
    )];

    let error = ensure_resolved_sources_satisfy_claims(root.path(), &packages, origin(REGISTRY))
        .expect_err("a claimed scope from the public chain must be refused");

    assert!(matches!(
        &error,
        SupplyChainError::SourceClaimNotServed { source_id, .. } if source_id == "corp"
    ));
    let message = error.to_string();
    assert!(message.contains("@corp/internal@1.0.0"), "{message}");
    assert!(message.contains("dependency confusion"), "{message}");
}

#[test]
fn a_claimed_scope_served_by_its_own_source_is_accepted() {
    let root = tempfile::tempdir().expect("create project");
    project(root.path(), CORP_CLAIM);
    let packages = [package(
        "@corp/internal",
        "1.0.0",
        "https://npm.corp.example/@corp/internal/-/internal-1.0.0.tgz",
    )];

    ensure_resolved_sources_satisfy_claims(
        root.path(),
        &packages,
        origin("https://npm.corp.example"),
    )
    .expect("the claiming source really served the package");
}

#[test]
fn an_unclaimed_public_package_is_not_affected_by_a_corp_claim() {
    let root = tempfile::tempdir().expect("create project");
    project(root.path(), CORP_CLAIM);
    let packages = [package(
        "react",
        "18.2.0",
        "https://registry.npmjs.org/react/-/react-18.2.0.tgz",
    )];

    ensure_resolved_sources_satisfy_claims(root.path(), &packages, origin(REGISTRY))
        .expect("no source claims react, so the chain may serve it");
}

#[test]
fn a_project_without_sources_makes_no_claim_to_enforce() {
    let root = tempfile::tempdir().expect("create project");
    project(root.path(), "");
    let packages = [package(
        "@corp/internal",
        "1.0.0",
        "https://registry.npmjs.org/@corp/internal/-/internal-1.0.0.tgz",
    )];

    ensure_resolved_sources_satisfy_claims(root.path(), &packages, origin(REGISTRY))
        .expect("no [[sources]] means no claim");
}

#[test]
fn a_malformed_project_config_fails_closed() {
    let root = tempfile::tempdir().expect("create project");
    std::fs::write(
        root.path().join("mgc.toml"),
        "name = \"broken\"\n[[sources]\n",
    )
    .expect("write malformed config");

    let error = ensure_resolved_sources_satisfy_claims(root.path(), &[], origin(REGISTRY))
        .expect_err("broken config must not silently disable the claim check");

    assert!(error.to_string().contains("mgc.toml"));
}

#[test]
fn lockfile_sources_carry_project_claims_instead_of_a_wildcard() {
    let root = tempfile::tempdir().expect("create project");
    project(root.path(), CORP_CLAIM);

    let sources = project_lockfile_sources(root.path(), origin(REGISTRY)).expect("load sources");

    assert_eq!(sources.len(), 1);
    let corp = &sources[0];
    assert_eq!(corp.id, "corp");
    assert_eq!(corp.claims, vec!["@corp/*".to_string()]);
    assert!(corp.trusted);
    assert_eq!(corp.allow_hosts, vec!["npm.corp.example".to_string()]);
    assert!(
        !sources
            .iter()
            .any(|source| source.claims.iter().any(|c| c == "*")),
        "a blanket wildcard claim must never be written"
    );
}

#[test]
fn a_source_serving_the_configured_registry_adopts_the_registry_id() {
    let root = tempfile::tempdir().expect("create project");
    project(
        root.path(),
        r#"
[[sources]]
id = "public"
url = "https://registry.npmjs.org/"
ecosystem = "web"
claims = ["registry.npmjs.org"]
"#,
    );

    let sources = project_lockfile_sources(root.path(), origin(REGISTRY)).expect("load sources");
    let expected = mgc_lockfile::v4_graph::source_id_for(REGISTRY);

    assert_eq!(sources[0].id, expected);
    assert_eq!(sources[0].url, "https://registry.npmjs.org/");
}

#[test]
fn absent_project_sources_record_the_real_registry_not_src_native() {
    let root = tempfile::tempdir().expect("create project");
    project(root.path(), "");

    let sources = project_lockfile_sources(root.path(), origin(REGISTRY)).expect("load sources");

    assert_eq!(sources.len(), 1);
    assert_ne!(sources[0].id, "src-native");
    assert_eq!(
        sources[0].id,
        mgc_lockfile::v4_graph::source_id_for(REGISTRY)
    );
    assert_eq!(sources[0].claims, vec!["registry.npmjs.org".to_string()]);
}
