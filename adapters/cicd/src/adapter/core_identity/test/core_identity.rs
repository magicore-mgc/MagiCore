#![cfg(test)]

//! Keep the CI/CD core id, name and ecosystem stable.
//! Giữ core id, tên và ecosystem CICD ổn định.

use crate::adapter::CicdAdapter;
use crate::provider::CicdProvider;
use mgc_types::capabilities::CoreIdent;

#[test]
fn identity_uses_cicd_id_name_and_ecosystem() {
    let adapter = CicdAdapter {
        provider: CicdProvider::GithubActions,
    };
    assert_eq!(adapter.core_id(), "cicd");
    assert_eq!(adapter.name(), "cicd");
    assert_eq!(format!("{:?}", adapter.ecosystem()), "Cicd");
}
