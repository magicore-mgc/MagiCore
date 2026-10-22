//! Adapter Discovery behavior tests.
//! Test hành vi cho nhóm adapter discovery.

use crate::integration::support::tmp;
use mgc_cloud_adapter::adapter_for;

pub fn adapter_for_returns_some_for_terraform() {
    let dir = tmp("af-tf");
    std::fs::write(dir.join("main.tf"), "provider \"aws\" {}\n").unwrap();
    assert!(adapter_for(&dir).unwrap().is_some());
}

pub fn adapter_for_returns_none_without_any_marker() {
    let dir = tmp("af-none");
    assert!(adapter_for(&dir).unwrap().is_none());
}

pub fn adapter_cloud_type_method_returns_correct_str() {
    let dir = tmp("ct-str");
    std::fs::write(dir.join("main.tf"), "provider \"aws\" {}\n").unwrap();
    let a = adapter_for(&dir).unwrap().unwrap();
    assert_eq!(a.cloud_type(), "terraform");
}
