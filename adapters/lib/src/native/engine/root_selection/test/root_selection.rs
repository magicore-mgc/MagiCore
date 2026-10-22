//! Root Selection source regression test.
//! Test hồi quy source cho root selection.

use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};

use crate::native::engine::engine::root_selection::selected_roots;

#[test]
fn selects_runtime_roots_and_adds_dev_roots_only_when_requested() {
    let mut manifest = Manifest::new("demo-lib", Ecosystem::Lib);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("runtime-tool").unwrap(),
            VersionRange::parse("^1.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("test-tool").unwrap(),
            VersionRange::parse("^1.0").unwrap(),
        ),
        true,
        false,
        false,
    );

    let runtime = selected_roots(&manifest, false);
    let all = selected_roots(&manifest, true);

    assert_eq!(runtime.len(), 1);
    assert_eq!(runtime[0].0.name.as_str(), "runtime-tool");
    assert_eq!(all.len(), 2);
    assert!(!all[0].1);
    assert!(all[1].1);
}
