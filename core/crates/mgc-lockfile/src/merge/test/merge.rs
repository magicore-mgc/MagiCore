// Merge behavior test for removal on both branches.
// Test hành vi khi hai nhánh cùng xóa package.
use crate::merge::merge3;
use crate::{Lockfile, Package};

fn package(name: &str, version: &str) -> Package {
    Package {
        name: name.to_string(),
        version: version.to_string(),
        ..Default::default()
    }
}

fn lock(packages: &[Package]) -> Lockfile {
    let mut lockfile = Lockfile::new();
    lockfile.packages = packages.to_vec();
    lockfile
}

#[test]
fn merge3_keeps_a_package_removed_by_both_branches_absent() {
    let base = lock(&[package("removed", "1.0.0")]);
    let ours = lock(&[]);
    let theirs = lock(&[]);

    let merged = merge3(&base, &ours, &theirs).expect("both removals should merge");

    assert!(merged.packages.is_empty());
}
