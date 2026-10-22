#![cfg(test)]

//! Ensure previously public registry paths remain importable.
//! Xác nhận các public path registry trước đây vẫn import được.

use crate::registry::{ModelFormat, ModelMetadata, Registry};

#[test]
fn stable_registry_types_are_exported() {
    let _: Option<Registry> = Registry::from_url("https://huggingface.co/model");
    let _: Option<ModelFormat> = None;
    let _: Option<ModelMetadata> = None;
}
