#![cfg(test)]

//! Preserve public provider-field construction for the adapter type.
//! Giữ khả năng khởi tạo adapter qua field provider công khai.

use crate::adapter::adapter::CicdAdapter;
use crate::provider::CicdProvider;

#[test]
fn adapter_type_keeps_its_public_provider_field() {
    let adapter = CicdAdapter {
        provider: CicdProvider::GithubActions,
    };
    assert_eq!(adapter.provider, CicdProvider::GithubActions);
}
