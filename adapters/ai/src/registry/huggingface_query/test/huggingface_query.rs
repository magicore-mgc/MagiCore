#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Opt-in live test for the Hugging Face Hub endpoint.
//! Test live tùy chọn cho endpoint Hugging Face Hub.

use crate::registry::{Registry, huggingface_query::query_huggingface};

#[tokio::test]
#[ignore = "hits huggingface.co — run manually"]
async fn query_huggingface_live() {
    let metadata = query_huggingface("gpt2").await.unwrap();
    assert_eq!(metadata.id, "gpt2");
    assert_eq!(metadata.registry, Registry::HuggingFace);
}
