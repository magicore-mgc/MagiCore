//! Adapter Contract behavior tests.
//! Test hành vi cho nhóm adapter contract.

use mgc_cicd_adapter::{CicdAdapter, CicdProvider};
use mgc_types::capabilities::CoreIdent;

pub fn adapter_name_and_ecosystem() {
    let a = CicdAdapter {
        provider: CicdProvider::GithubActions,
    };
    assert_eq!(a.name(), "cicd");
    assert_eq!(format!("{:?}", a.ecosystem()), "Cicd");
}
