//! Direct top-level resolution selector tests.
//! Test selector resolution top-level trực tiếp.

use super::super::{cached_root_resolutions, enforce_no_downgrade_at};
use mgc_resolver::solver::RootResolution;
use mgc_types::{
    DependencySpec, Ecosystem, Manifest, PackageId, PackageName, VersionRange,
    adapter::{ResolvedGraph, ResolvedPackage},
};

fn manifest_with_widget() -> Manifest {
    manifest_with_widget_spec("^1.0.0")
}

fn manifest_with_widget_spec(spec: &str) -> Manifest {
    let mut manifest = Manifest::new("lock-guard-test", Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("widget").expect("parse dependency name"),
            VersionRange::parse(spec).expect("parse dependency range"),
        ),
        false,
        false,
        false,
    );
    manifest
}

fn manifest_with_widget_and_unlocked_gadget() -> Manifest {
    let mut manifest = manifest_with_widget();
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("gadget").expect("parse dependency name"),
            VersionRange::parse("^1.0.0").expect("parse dependency range"),
        ),
        false,
        false,
        false,
    );
    manifest
}

fn write_widget_lock(project_root: &std::path::Path, version: &str) {
    let mut lockfile = mgc_lockfile::Lockfile::new();
    lockfile.packages.push(mgc_lockfile::Package {
        owner_core: Some("web".to_owned()),
        name: "widget".to_owned(),
        version: version.to_owned(),
        ecosystem: mgc_lockfile::EcosystemTag::Web,
        registry: Some("https://registry.invalid".to_owned()),
        ..Default::default()
    });
    lockfile.root_dependencies_by_owner.insert(
        "web".to_owned(),
        vec![mgc_lockfile::format_root_pin(
            mgc_lockfile::EcosystemTag::Web,
            &format!("widget@{version}"),
        )],
    );
    let bytes =
        mgc_lockfile::writer::serialize_lockfile(&lockfile).expect("serialize test lockfile");
    std::fs::write(project_root.join("mgc.lock"), bytes).expect("write test lockfile");
}

fn write_widget_lock_with_transitive_first(
    project_root: &std::path::Path,
    transitive_version: &str,
    direct_version: &str,
) {
    let mut lockfile = mgc_lockfile::Lockfile::new();
    for version in [transitive_version, direct_version] {
        lockfile.packages.push(mgc_lockfile::Package {
            owner_core: Some("web".to_owned()),
            name: "widget".to_owned(),
            version: version.to_owned(),
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            registry: Some("https://registry.invalid".to_owned()),
            ..Default::default()
        });
    }
    lockfile.root_dependencies_by_owner.insert(
        "web".to_owned(),
        vec![mgc_lockfile::format_root_pin(
            mgc_lockfile::EcosystemTag::Web,
            &format!("widget@{direct_version}"),
        )],
    );
    let bytes =
        mgc_lockfile::writer::serialize_lockfile(&lockfile).expect("serialize test lockfile");
    std::fs::write(project_root.join("mgc.lock"), bytes).expect("write test lockfile");
}

fn root_resolution(index: usize, package: &str, spec: &str) -> RootResolution {
    let package_id = PackageId::parse(package).expect("parse package identity");
    RootResolution {
        request_index: index,
        request_name: PackageName::new(package_id.name_str()).expect("parse request name"),
        request_spec: spec.to_owned(),
        package_id,
    }
}

#[test]
fn an_absent_lock_has_no_previous_version_to_compare() {
    let project = tempfile::tempdir().expect("create project directory");
    let manifest = Manifest::new("fresh-project", Ecosystem::Web);

    assert!(enforce_no_downgrade_at(&[], &manifest, project.path()).is_ok());
}

#[test]
fn missing_root_mapping_fails_closed_before_treating_a_missing_lock_as_fresh() {
    let project = tempfile::tempdir().expect("create project directory");
    let error = enforce_no_downgrade_at(&[], &manifest_with_widget(), project.path())
        .expect_err("a resolved manifest must have root identity for every dependency");

    assert!(matches!(
        error,
        crate::supply_chain::error::SupplyChainError::RootResolutionCount {
            expected: 1,
            actual: 0
        }
    ));
}

#[test]
fn root_mapping_must_match_the_manifest_request_identity() {
    let project = tempfile::tempdir().expect("create project directory");
    let error = enforce_no_downgrade_at(
        &[root_resolution(0, "gadget@1.0.0", "^1.0.0")],
        &manifest_with_widget(),
        project.path(),
    )
    .expect_err("a same-index mapping for another package must be rejected");

    assert!(matches!(
        error,
        crate::supply_chain::error::SupplyChainError::InvalidRootResolution { .. }
    ));
}

#[test]
fn resolver_package_identity_must_match_the_root_request_name() {
    let project = tempfile::tempdir().expect("create project directory");
    let mut root = root_resolution(0, "widget@1.0.0", "^1.0.0");
    root.package_id = PackageId::parse("gadget@1.0.0").expect("parse mismatched package id");

    let error = enforce_no_downgrade_at(&[root], &manifest_with_widget(), project.path())
        .expect_err("package identity must match its direct manifest request");

    assert!(matches!(
        error,
        crate::supply_chain::error::SupplyChainError::InvalidRootResolution { .. }
    ));
}

#[test]
fn malformed_lock_is_an_error_instead_of_a_silent_missing_baseline() {
    let project = tempfile::tempdir().expect("create project directory");
    std::fs::write(project.path().join("mgc.lock"), "not = [valid")
        .expect("write malformed lockfile");
    let manifest = Manifest::new("broken-project", Ecosystem::Web);

    let error = enforce_no_downgrade_at(&[], &manifest, project.path())
        .expect_err("malformed lock must not be silently ignored");

    assert!(error.to_string().contains("lockfile"));
}

#[test]
fn invalid_direct_locked_version_returns_a_typed_error() {
    let project = tempfile::tempdir().expect("create project directory");
    write_widget_lock(project.path(), "not-semver");

    let error = enforce_no_downgrade_at(
        &[root_resolution(0, "widget@1.0.0", "^1.0.0")],
        &manifest_with_widget(),
        project.path(),
    )
    .expect_err("invalid direct version must fail closed");

    assert!(matches!(
        error,
        crate::supply_chain::error::SupplyChainError::InvalidLockfileVersion { .. }
    ));
}

#[test]
fn package_from_another_ecosystem_cannot_satisfy_a_web_root_pin() {
    let project = tempfile::tempdir().expect("create project directory");
    let mut lockfile = mgc_lockfile::Lockfile::new();
    lockfile.packages.push(mgc_lockfile::Package {
        owner_core: Some("app".to_owned()),
        name: "widget".to_owned(),
        version: "1.5.0".to_owned(),
        ecosystem: mgc_lockfile::EcosystemTag::Python,
        registry: Some("https://registry.invalid".to_owned()),
        ..Default::default()
    });
    lockfile.root_dependencies_by_owner.insert(
        "web".to_owned(),
        vec![mgc_lockfile::format_root_pin(
            mgc_lockfile::EcosystemTag::Web,
            "widget@1.5.0",
        )],
    );
    let bytes = mgc_lockfile::writer::serialize_lockfile(&lockfile)
        .expect("serialize cross-ecosystem test lockfile");
    std::fs::write(project.path().join("mgc.lock"), bytes)
        .expect("write cross-ecosystem test lockfile");

    let error = enforce_no_downgrade_at(
        &[root_resolution(0, "widget@1.5.0", "^1.0.0")],
        &manifest_with_widget(),
        project.path(),
    )
    .expect_err("a Python row cannot validate a Web direct-root pin");

    assert!(matches!(
        error,
        crate::supply_chain::error::SupplyChainError::MissingLockedRootPackage { .. }
    ));
}

#[test]
fn legacy_web_v2_root_and_unqualified_package_row_remain_a_baseline() {
    let project = tempfile::tempdir().expect("create project directory");
    let mut lockfile = mgc_lockfile::Lockfile::new();
    lockfile.version = "2".to_owned();
    lockfile.packages.push(mgc_lockfile::Package {
        name: "widget".to_owned(),
        version: "1.5.0".to_owned(),
        ..Default::default()
    });
    lockfile.root_dependencies = vec!["widget@1.5.0".to_owned()];
    let bytes = mgc_lockfile::writer::serialize_lockfile(&lockfile)
        .expect("serialize legacy Web v2 test lockfile");
    std::fs::write(project.path().join("mgc.lock"), bytes)
        .expect("write legacy Web v2 test lockfile");

    let error = enforce_no_downgrade_at(
        &[root_resolution(0, "widget@1.2.0", "^1.0.0")],
        &manifest_with_widget(),
        project.path(),
    )
    .expect_err("legacy Web v2 root pin must still enforce downgrade policy");

    assert!(matches!(
        error,
        crate::supply_chain::error::SupplyChainError::Downgrade {
            ref previous,
            ref selected,
            ..
        } if previous.to_string() == "1.5.0" && selected.to_string() == "1.2.0"
    ));
}

#[test]
fn rejects_a_lower_selected_version_when_the_prior_lock_still_satisfies_manifest() {
    let project = tempfile::tempdir().expect("create project directory");
    write_widget_lock(project.path(), "1.5.0");

    let error = enforce_no_downgrade_at(
        &[root_resolution(0, "widget@1.2.0", "^1.0.0")],
        &manifest_with_widget(),
        project.path(),
    )
    .expect_err("lower direct version must be rejected");

    assert!(matches!(
        error,
        crate::supply_chain::error::SupplyChainError::Downgrade { .. }
    ));
}

#[test]
fn stale_lock_that_does_not_satisfy_manifest_is_not_a_downgrade_baseline() {
    let project = tempfile::tempdir().expect("create project directory");
    write_widget_lock(project.path(), "2.0.0");

    assert!(
        enforce_no_downgrade_at(
            &[root_resolution(0, "widget@1.2.0", "^1.0.0")],
            &manifest_with_widget(),
            project.path(),
        )
        .is_ok()
    );
}

#[test]
fn missing_baseline_for_one_direct_dependency_does_not_skip_other_downgrade_checks() {
    let project = tempfile::tempdir().expect("create project directory");
    write_widget_lock(project.path(), "1.5.0");

    let error = enforce_no_downgrade_at(
        &[
            root_resolution(0, "widget@1.2.0", "^1.0.0"),
            root_resolution(1, "gadget@1.0.0", "^1.0.0"),
        ],
        &manifest_with_widget_and_unlocked_gadget(),
        project.path(),
    )
    .expect_err("an unlocked new dependency must not disable other downgrade checks");

    assert!(matches!(
        error,
        crate::supply_chain::error::SupplyChainError::Downgrade { ref name, .. }
            if name == "widget"
    ));
}

#[test]
fn direct_root_pin_is_used_when_a_transitive_same_name_row_appears_first() {
    let project = tempfile::tempdir().expect("create project directory");
    write_widget_lock_with_transitive_first(project.path(), "1.2.0", "1.5.0");

    let error = enforce_no_downgrade_at(
        &[root_resolution(0, "widget@1.2.0", "^1.0.0")],
        &manifest_with_widget(),
        project.path(),
    )
    .expect_err("a lower selection must be compared with the exact prior direct root pin");

    assert!(matches!(
        error,
        crate::supply_chain::error::SupplyChainError::Downgrade {
            ref previous,
            ref selected,
            ..
        } if previous.to_string() == "1.5.0" && selected.to_string() == "1.2.0"
    ));
}

#[test]
fn multiple_prior_root_pins_for_one_manifest_dependency_fail_closed() {
    let project = tempfile::tempdir().expect("create project directory");
    let mut lockfile = mgc_lockfile::Lockfile::new();
    for version in ["1.4.0", "1.5.0"] {
        lockfile.packages.push(mgc_lockfile::Package {
            owner_core: Some("web".to_owned()),
            name: "widget".to_owned(),
            version: version.to_owned(),
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            registry: Some("https://registry.invalid".to_owned()),
            ..Default::default()
        });
    }
    lockfile.root_dependencies_by_owner.insert(
        "web".to_owned(),
        ["1.4.0", "1.5.0"]
            .into_iter()
            .map(|version| {
                mgc_lockfile::format_root_pin(
                    mgc_lockfile::EcosystemTag::Web,
                    &format!("widget@{version}"),
                )
            })
            .collect(),
    );
    let bytes = mgc_lockfile::writer::serialize_lockfile(&lockfile)
        .expect("serialize ambiguous test lockfile");
    std::fs::write(project.path().join("mgc.lock"), bytes).expect("write ambiguous test lockfile");

    let error = enforce_no_downgrade_at(
        &[root_resolution(0, "widget@1.2.0", "^1.0.0")],
        &manifest_with_widget(),
        project.path(),
    )
    .expect_err("multiple direct root identities cannot become a baseline");

    assert!(matches!(
        error,
        crate::supply_chain::error::SupplyChainError::AmbiguousDirectRootPins { ref name }
            if name == "widget"
    ));
}

#[test]
fn unrelated_root_pin_version_does_not_make_a_matching_range_ambiguous() {
    let project = tempfile::tempdir().expect("create project directory");
    let mut lockfile = mgc_lockfile::Lockfile::new();
    for version in ["1.5.0", "2.0.0"] {
        lockfile.packages.push(mgc_lockfile::Package {
            owner_core: Some("web".to_owned()),
            name: "widget".to_owned(),
            version: version.to_owned(),
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            registry: Some("https://registry.invalid".to_owned()),
            ..Default::default()
        });
    }
    lockfile.root_dependencies_by_owner.insert(
        "web".to_owned(),
        ["1.5.0", "2.0.0"]
            .into_iter()
            .map(|version| {
                mgc_lockfile::format_root_pin(
                    mgc_lockfile::EcosystemTag::Web,
                    &format!("widget@{version}"),
                )
            })
            .collect(),
    );
    let bytes = mgc_lockfile::writer::serialize_lockfile(&lockfile)
        .expect("serialize multi-version test lockfile");
    std::fs::write(project.path().join("mgc.lock"), bytes)
        .expect("write multi-version test lockfile");

    let error = enforce_no_downgrade_at(
        &[root_resolution(0, "widget@1.2.0", "^1.0.0")],
        &manifest_with_widget(),
        project.path(),
    )
    .expect_err("the matching 1.x direct pin must remain the downgrade baseline");

    assert!(matches!(
        error,
        crate::supply_chain::error::SupplyChainError::Downgrade {
            ref previous,
            ref selected,
            ..
        } if previous.to_string() == "1.5.0" && selected.to_string() == "1.2.0"
    ));
}

#[test]
fn cached_graph_selection_is_checked_against_the_prior_direct_root_pin() {
    let project = tempfile::tempdir().expect("create project directory");
    write_widget_lock(project.path(), "1.5.0");
    let package_id = PackageId::parse("widget@1.2.0").expect("parse cached package id");
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: package_id,
            integrity: String::new(),
            tarball_url: String::new(),
            deps: Vec::new(),
            peer_deps: Vec::new(),
            direct: true,
            dev: false,
        }],
    };
    let manifest = manifest_with_widget();
    let cached_roots = cached_root_resolutions(&graph, &manifest)
        .expect("cache graph has one explicit direct root");

    let error = enforce_no_downgrade_at(&cached_roots, &manifest, project.path())
        .expect_err("shared cache must not bypass direct-root downgrade policy");

    assert!(matches!(
        error,
        crate::supply_chain::error::SupplyChainError::Downgrade {
            ref previous,
            ref selected,
            ..
        } if previous.to_string() == "1.5.0" && selected.to_string() == "1.2.0"
    ));
}

#[test]
fn moved_dist_tag_keeps_the_previous_direct_pin_as_a_downgrade_baseline() {
    let project = tempfile::tempdir().expect("create project directory");
    write_widget_lock(project.path(), "1.5.0");

    let error = enforce_no_downgrade_at(
        &[root_resolution(0, "widget@1.2.0", "1.2.0")],
        &manifest_with_widget_spec("latest"),
        project.path(),
    )
    .expect_err("a moved dist-tag must not discard its previous root pin");

    assert!(matches!(
        error,
        crate::supply_chain::error::SupplyChainError::Downgrade {
            ref previous,
            ref selected,
            ..
        } if previous.to_string() == "1.5.0" && selected.to_string() == "1.2.0"
    ));
}

#[test]
fn moved_dist_tag_keeps_the_previous_direct_pin_for_an_npm_alias() {
    let project = tempfile::tempdir().expect("create project directory");
    write_widget_lock(project.path(), "1.5.0");

    let error = enforce_no_downgrade_at(
        &[root_resolution(0, "widget@1.2.0", "1.2.0")],
        &manifest_with_widget_spec("npm:@scope/source@latest"),
        project.path(),
    )
    .expect_err("an aliased moving dist-tag must retain its previous root pin");

    assert!(matches!(
        error,
        crate::supply_chain::error::SupplyChainError::Downgrade {
            ref previous,
            ref selected,
            ..
        } if previous.to_string() == "1.5.0" && selected.to_string() == "1.2.0"
    ));
}
