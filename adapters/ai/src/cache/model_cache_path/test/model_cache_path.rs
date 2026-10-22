//! Model path cases — kiểm tra mapping tương thích và chặn ID nguy hiểm.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use crate::cache::model_cache_path::{
    cache_entry_name, model_cache_path, model_id_for_cache_entry,
};
use crate::registry::Registry;
use std::path::PathBuf;

#[test]
fn hugging_face_model_path_uses_native_repository_folder() {
    let registry = Registry::HuggingFace;

    assert_eq!(
        model_cache_path(&registry, "openai/gpt-2").unwrap(),
        mgc_platform::paths::mgc_data_dir().join("cache/ai/huggingface/hub/models--openai--gpt-2")
    );
}

#[test]
fn local_model_cache_path_keeps_existing_flattened_layout() {
    let registry = Registry::Local(PathBuf::from("/tmp/mgc-ai-cache"));

    assert_eq!(
        model_cache_path(&registry, "openai/gpt-2").unwrap(),
        PathBuf::from("/tmp/mgc-ai-cache/openai--gpt-2")
    );
}

#[test]
fn hugging_face_entry_mapping_ignores_non_model_root_entries() {
    let registry = Registry::HuggingFace;

    assert_eq!(
        cache_entry_name(&registry, "openai/gpt-2"),
        "models--openai--gpt-2"
    );
    assert_eq!(
        model_id_for_cache_entry(&registry, "models--openai--gpt-2", true),
        Some("openai/gpt-2".into())
    );
    for (name, is_directory) in [
        (".locks", true),
        ("version.txt", false),
        ("weights.META.JSON", false),
        ("datasets--org--dataset", true),
        ("spaces--org--demo", true),
        ("models--openai--gpt-2", false),
    ] {
        assert_eq!(
            model_id_for_cache_entry(&registry, name, is_directory),
            None,
            "{name}"
        );
    }
}

#[test]
fn rejects_empty_and_path_shaped_model_ids() {
    let registry = Registry::Local(PathBuf::from("/tmp/mgc-ai-cache"));

    for model_id in [
        "",
        ".",
        "..",
        "owner/../outside",
        "owner//model",
        "/outside",
        "owner/",
        r"owner\..\outside",
        r"C:\outside",
        "NUL",
        "NUL .txt",
        "COM1.txt",
        "COM¹",
        "model.meta.json",
    ] {
        assert!(model_cache_path(&registry, model_id).is_err(), "{model_id}");
    }
}
