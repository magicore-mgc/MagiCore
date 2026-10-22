#![cfg(test)]

//! Keep the existing MagiCore scaffold capability available.
//! Giữ capability scaffold MagiCore hiện có.

use crate::adapter::CicdAdapter;
use crate::provider::CicdProvider;
use mgc_types::capabilities::ScaffoldProvider;

#[test]
fn scaffold_probe_succeeds_for_the_native_scaffold_lane() {
    let adapter = CicdAdapter {
        provider: CicdProvider::GithubActions,
    };
    assert!(adapter.probe_scaffold().is_ok());
}
