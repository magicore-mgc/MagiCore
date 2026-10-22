//! Construct the adapter only when a supported CI/CD provider is detected.
//! Chỉ tạo adapter khi nhận diện được provider CICD được hỗ trợ.

use super::super::adapter::CicdAdapter;
use crate::provider::detect_provider;
use std::path::Path;

pub fn adapter_for(root: &Path) -> Option<CicdAdapter> {
    let provider = detect_provider(root)?;
    Some(CicdAdapter { provider })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
