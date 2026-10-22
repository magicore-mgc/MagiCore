//! Project detector capability contract tests.
//! Test hợp đồng capability nhận diện dự án.

use crate::capabilities::ProjectDetector;
use crate::capabilities::test_support::MockCore;
use std::path::Path;

#[test]
fn required_detector_method_accepts_the_fixture_project() {
    assert!(MockCore.can_handle(Path::new(".")));
}
