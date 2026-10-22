//! Adapter Contract behavior tests.
//! Test hành vi cho nhóm adapter contract.

use crate::integration::support::tmp;
use mgc_cloud_adapter::adapter_for;
use mgc_types::capabilities::{CoreIdent, ProjectDetector};

pub fn adapter_name_and_ecosystem() {
    let dir = tmp("name-eco");
    std::fs::write(dir.join("main.tf"), "provider \"aws\" {}\n").unwrap();
    let a = adapter_for(&dir).unwrap().unwrap();
    assert_eq!(a.name(), "cloud");
    assert_eq!(format!("{:?}", a.ecosystem()), "Cloud");
}

pub fn can_handle_returns_true_for_known_marker() {
    let dir = tmp("ch-true");
    std::fs::write(dir.join("main.tf"), "provider \"aws\" {}\n").unwrap();
    let a = adapter_for(&dir).unwrap().unwrap();
    assert!(a.can_handle(&dir));
}
