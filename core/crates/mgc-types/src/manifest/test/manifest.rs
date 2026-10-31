//! Verify the shared Manifest aggregate behavior.
//! Kiểm tra hành vi aggregate Manifest dùng chung.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::Manifest;
use crate::ecosystem::Ecosystem;
use crate::package::{DependencySpec, PackageName, VersionRange};

fn dep(name: &str) -> DependencySpec {
    DependencySpec::new(PackageName::new(name).unwrap(), VersionRange::star())
}

#[test]
fn manifest_new_sets_fields() {
    let m = Manifest::new("test", Ecosystem::Web);
    assert_eq!(m.name, "test");
    assert_eq!(m.ecosystem, Ecosystem::Web);
    assert!(m.version.is_none());
    assert!(m.dependencies.is_empty());
}

#[test]
fn add_dep_puts_in_correct_group() {
    let mut m = Manifest::new("test", Ecosystem::Lib);
    let d = dep("foo");
    assert!(m.add_dep(d, false, false, false));
    assert_eq!(m.dependencies.len(), 1);
    assert!(m.dev_dependencies.is_empty());
}

#[test]
fn add_dep_with_dev_goes_to_dev() {
    let mut m = Manifest::new("test", Ecosystem::Lib);
    assert!(m.add_dep(dep("foo"), true, false, false));
    assert_eq!(m.dev_dependencies.len(), 1);
    assert!(m.dependencies.is_empty());
}

#[test]
fn add_dep_with_optional_goes_to_optional() {
    let mut m = Manifest::new("test", Ecosystem::Lib);
    assert!(m.add_dep(dep("foo"), false, true, false));
    assert_eq!(m.optional_dependencies.len(), 1);
}

#[test]
fn add_dep_with_peer_goes_to_peer() {
    let mut m = Manifest::new("test", Ecosystem::Lib);
    assert!(m.add_dep(dep("foo"), false, false, true));
    assert_eq!(m.peer_dependencies.len(), 1);
}

#[test]
fn add_dep_dedup_replaces_existing() {
    let mut m = Manifest::new("test", Ecosystem::Lib);
    let d1 = DependencySpec {
        name: PackageName::new("foo").unwrap(),
        range: VersionRange::parse("1.0.0").unwrap(),
        dev: false,
        optional: false,
        peer: false,
        features: Vec::new(),
        default_features: true,
    };
    let d2 = DependencySpec {
        name: PackageName::new("foo").unwrap(),
        range: VersionRange::parse("2.0.0").unwrap(),
        dev: false,
        optional: false,
        peer: false,
        features: Vec::new(),
        default_features: true,
    };
    assert!(m.add_dep(d1, false, false, false));
    assert!(m.add_dep(d2, false, false, false));
    assert_eq!(m.dependencies.len(), 1);
    assert_eq!(m.dependencies[0].range.as_str(), "2.0.0");
}

#[test]
fn add_dep_reports_when_nothing_changed() {
    let mut m = Manifest::new("test", Ecosystem::Lib);
    assert!(m.add_dep(dep("foo"), false, false, false));
    assert!(!m.add_dep(dep("foo"), false, false, false));
    assert_eq!(m.dependencies.len(), 1);
}

#[test]
fn add_dep_moves_dependency_between_groups() {
    let mut m = Manifest::new("test", Ecosystem::Lib);
    assert!(m.add_dep(dep("foo"), false, false, false));
    assert!(m.add_dep(dep("foo"), true, false, false));
    assert!(m.dependencies.is_empty());
    assert_eq!(m.dev_dependencies.len(), 1);
}

// P0 regression: `update` retargeted ranges by calling `add_dep` with the
// target's flags, which relocated `devDependencies` into `dependencies`.
// (`Hồi quy P0: `update` đổi range qua `add_dep` kèm cờ, làm dời
// `devDependencies` sang `dependencies`.)

fn spec(name: &str, range: &str, dev: bool, optional: bool, peer: bool) -> DependencySpec {
    DependencySpec {
        name: PackageName::new(name).unwrap(),
        range: VersionRange::parse(range).unwrap(),
        dev,
        optional,
        peer,
        features: vec!["extra".to_string()],
        default_features: false,
    }
}

#[test]
fn retarget_dep_keeps_dev_dependencies_in_dev() {
    let mut m = Manifest::new("test", Ecosystem::Web);
    m.add_dep(
        spec("typescript", "~6.0.2", true, false, false),
        true,
        false,
        false,
    );

    assert!(m.retarget_dep("typescript", VersionRange::parse("~6.1.0").unwrap()));

    assert!(
        m.dependencies.is_empty(),
        "devDependency must not leak into dependencies"
    );
    assert_eq!(m.dev_dependencies.len(), 1);
    assert_eq!(m.dev_dependencies[0].range.as_str(), "~6.1.0");
}

#[test]
fn retarget_dep_never_merges_sections() {
    let mut m = Manifest::new("test", Ecosystem::Web);
    m.add_dep(
        spec("runtime", "^1.0.0", false, false, false),
        false,
        false,
        false,
    );
    m.add_dep(
        spec("build", "^1.0.0", true, false, false),
        true,
        false,
        false,
    );
    m.add_dep(
        spec("native", "^1.0.0", false, true, false),
        false,
        true,
        false,
    );
    m.add_dep(
        spec("host", "^1.0.0", false, false, true),
        false,
        false,
        true,
    );

    for name in ["runtime", "build", "native", "host"] {
        assert!(m.retarget_dep(name, VersionRange::parse("^2.0.0").unwrap()));
    }

    assert_eq!(m.dependencies.len(), 1);
    assert_eq!(m.dev_dependencies.len(), 1);
    assert_eq!(m.optional_dependencies.len(), 1);
    assert_eq!(m.peer_dependencies.len(), 1);
    // Each entry stays in the section it was declared in.
    // (Mỗi entry ở đúng section đã khai.)
    assert_eq!(m.dependencies[0].name.as_str(), "runtime");
    assert_eq!(m.dev_dependencies[0].name.as_str(), "build");
    assert_eq!(m.optional_dependencies[0].name.as_str(), "native");
    assert_eq!(m.peer_dependencies[0].name.as_str(), "host");
}

#[test]
fn retarget_dep_preserves_extra_fields() {
    let mut m = Manifest::new("test", Ecosystem::Web);
    m.add_dep(
        spec("typescript", "~6.0.2", true, false, false),
        true,
        false,
        false,
    );

    assert!(m.retarget_dep("typescript", VersionRange::parse("~6.1.0").unwrap()));

    let entry = &m.dev_dependencies[0];
    assert!(entry.dev);
    assert_eq!(entry.features, vec!["extra".to_string()]);
    assert!(!entry.default_features);
}

#[test]
fn retarget_dep_reports_no_change_for_same_range() {
    let mut m = Manifest::new("test", Ecosystem::Web);
    m.add_dep(
        spec("typescript", "~6.0.2", true, false, false),
        true,
        false,
        false,
    );

    assert!(
        !m.retarget_dep("typescript", VersionRange::parse("~6.0.2").unwrap()),
        "an unchanged range must not be counted as an update"
    );
    assert_eq!(m.dev_dependencies.len(), 1);
}

#[test]
fn retarget_dep_reports_false_for_unknown_name() {
    let mut m = Manifest::new("test", Ecosystem::Web);
    assert!(!m.retarget_dep("missing", VersionRange::parse("^1.0.0").unwrap()));
    assert_eq!(m.all_dependencies().count(), 0);
}

#[test]
fn remove_dep_removes_from_all_groups() {
    let mut m = Manifest::new("test", Ecosystem::Lib);
    let d = dep("foo");
    assert!(m.add_dep(d.clone(), false, false, false));
    assert!(m.add_dep(d.clone(), true, false, false));
    assert!(m.add_dep(d.clone(), false, true, false));
    assert!(m.add_dep(d, false, false, true));
    assert_eq!(m.all_dependencies().count(), 1);
    assert!(m.remove_dep("foo"));
    assert_eq!(m.all_dependencies().count(), 0);
}

#[test]
fn remove_dep_only_removes_target() {
    let mut m = Manifest::new("test", Ecosystem::Lib);
    assert!(m.add_dep(dep("foo"), false, false, false));
    assert!(m.add_dep(dep("bar"), true, false, false));
    assert!(m.remove_dep("foo"));
    assert_eq!(m.dependencies.len(), 0);
    assert_eq!(m.dev_dependencies.len(), 1);
}

#[test]
fn remove_dep_reports_when_nothing_changed() {
    let mut m = Manifest::new("test", Ecosystem::Lib);
    assert!(m.add_dep(dep("bar"), false, false, false));
    assert!(!m.remove_dep("foo"));
    assert_eq!(m.dependencies.len(), 1);
}

#[test]
fn find_dep_searches_all_groups() {
    let mut m = Manifest::new("test", Ecosystem::Lib);
    assert!(m.add_dep(dep("foo"), false, false, true));
    assert!(m.find_dep("foo").is_some());
    assert!(m.find_dep("missing").is_none());
}

#[test]
fn all_dependencies_yields_all_groups() {
    let mut m = Manifest::new("test", Ecosystem::Lib);
    assert!(m.add_dep(dep("a"), false, false, false));
    assert!(m.add_dep(dep("b"), true, false, false));
    assert!(m.add_dep(dep("c"), false, true, false));
    assert!(m.add_dep(dep("d"), false, false, true));
    assert_eq!(m.all_dependencies().count(), 4);
}

#[test]
fn dep_groups_returns_four_entries() {
    let m = Manifest::new("test", Ecosystem::Lib);
    let groups = m.dep_groups();
    assert_eq!(groups.len(), 4);
    assert_eq!(groups[0].0, "dependencies");
    assert_eq!(groups[1].0, "devDependencies");
    assert_eq!(groups[2].0, "peerDependencies");
    assert_eq!(groups[3].0, "optionalDependencies");
}
