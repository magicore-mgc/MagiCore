#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! A project without Podfile files contributes an empty iOS tier.
//! Project không có file Podfile tạo tier iOS rỗng.

use crate::native::rn_layers::resolve_ios::resolve_ios;
use tempfile::TempDir;

#[tokio::test]
async fn missing_ios_files_return_empty_tier() {
    let temp = TempDir::new().unwrap();
    let result = resolve_ios(temp.path()).await.unwrap();
    assert!(result.graph.packages.is_empty());
    assert!(result.lock_packages.is_empty());
}
