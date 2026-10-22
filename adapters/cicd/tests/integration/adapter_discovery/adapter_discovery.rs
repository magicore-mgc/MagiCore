//! Adapter Discovery behavior tests.
//! Test hành vi cho nhóm adapter discovery.

use crate::integration::support::tmp;
use mgc_cicd_adapter::adapter_for;
use mgc_types::capabilities::ProjectDetector;

pub fn adapter_for_returns_some_with_wrangler() {
    let dir = tmp("af-cf");
    std::fs::write(dir.join("wrangler.toml"), "name = \"w\"\n").unwrap();
    assert!(adapter_for(&dir).is_some());
}

pub fn adapter_for_returns_none_without_markers() {
    let dir = tmp("af-none");
    assert!(adapter_for(&dir).is_none());
}

pub fn adapter_provider_method_returns_correct_str() {
    let dir = tmp("af-gha");
    std::fs::create_dir_all(dir.join(".github").join("workflows")).unwrap();
    std::fs::write(
        dir.join(".github").join("workflows").join("ci.yml"),
        "name: CI\n",
    )
    .unwrap();
    let a = adapter_for(&dir).unwrap();
    assert_eq!(a.provider(), "github-actions");
}

pub fn can_handle_returns_true_for_known_marker() {
    let dir = tmp("ch-true");
    std::fs::write(dir.join("wrangler.toml"), "name = \"w\"\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    assert!(a.can_handle(&dir));
}
