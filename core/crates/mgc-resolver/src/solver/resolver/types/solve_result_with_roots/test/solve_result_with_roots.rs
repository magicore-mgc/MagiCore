//! Root-aware result container tests.
//! Test container kết quả có root.

use super::super::SolveResultWithRoots;
use crate::solver::resolver::{Resolution, RootResolution, SolveResult};
use mgc_types::{PackageId, PackageName, Version};

#[test]
fn root_metadata_does_not_replace_the_standard_resolution_list() {
    let package_id = PackageId::parse("widget@1.2.0").expect("parse selected package");
    let version = Version::parse("1.2.0").expect("parse selected version");
    let result = SolveResult {
        resolutions: vec![Resolution {
            package_id: package_id.clone(),
            version,
            integrity: String::new(),
            deps: Vec::new(),
            dep_specs: Vec::new(),
            peer_deps: Vec::new(),
        }],
    };
    let output = SolveResultWithRoots {
        result,
        root_resolutions: vec![RootResolution::new(
            0,
            PackageName::new("widget").expect("parse root name"),
            "^1.0.0".to_owned(),
            package_id.clone(),
        )],
    };

    assert_eq!(output.result.resolutions[0].package_id, package_id);
    assert_eq!(output.root_resolutions.len(), 1);
}
