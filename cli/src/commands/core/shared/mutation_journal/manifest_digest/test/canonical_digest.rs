//! Canonical manifest digest contract.
//! Contract digest canonical cho manifest.

use super::super::manifest_canonical_digest;

#[test]
fn canonical_digest_distinguishes_ranges_groups_and_project() {
    // P0-D: digest phải bắt range/group/project — không chỉ tên.
    // (Digest must catch range/group/project drift, not just names.)
    use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};
    let mut base = Manifest::new("m", Ecosystem::Lib);
    base.add_dep(
        DependencySpec::new(
            PackageName::new("six").unwrap(),
            VersionRange::parse("==1.17.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let digest = manifest_canonical_digest(&base);
    let mut bumped = base.clone();
    bumped.dependencies[0].range = VersionRange::parse("==1.18.0").unwrap();
    assert_ne!(
        manifest_canonical_digest(&bumped),
        digest,
        "range change must alter the digest"
    );
    let mut regrouped = base.clone();
    let spec = regrouped.dependencies.pop().unwrap();
    regrouped.dev_dependencies.push(spec);
    assert_ne!(
        manifest_canonical_digest(&regrouped),
        digest,
        "group change must alter the digest"
    );
    let mut renamed = base.clone();
    renamed.name = "other".to_string();
    assert_ne!(
        manifest_canonical_digest(&renamed),
        digest,
        "project rename must alter the digest"
    );
    assert_eq!(
        manifest_canonical_digest(&base.clone()),
        digest,
        "identical manifests share the digest"
    );
}
