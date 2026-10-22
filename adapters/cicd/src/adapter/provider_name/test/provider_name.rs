#![cfg(test)]

//! Keep the public provider accessor mapped to the provider label.
//! Giữ accessor provider công khai ánh xạ đúng nhãn provider.

use crate::adapter::CicdAdapter;
use crate::provider::CicdProvider;

#[test]
fn provider_accessor_returns_the_canonical_provider_label() {
    let adapter = CicdAdapter {
        provider: CicdProvider::GithubActions,
    };
    assert_eq!(adapter.provider(), "github-actions");
}
