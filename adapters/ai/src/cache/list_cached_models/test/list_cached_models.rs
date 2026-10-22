//! Cached model listing cases — xác nhận lọc control entries và thứ tự output.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::list_models_at;
use crate::registry::Registry;
use tempfile::TempDir;

#[test]
fn hugging_face_listing_includes_only_model_repositories() {
    let temp = TempDir::new().unwrap();
    for name in [
        "models--zoo--z-model",
        "models--openai--gpt-2",
        "datasets--org--dataset",
        "spaces--org--demo",
        ".locks",
    ] {
        std::fs::create_dir(temp.path().join(name)).unwrap();
    }
    std::fs::write(temp.path().join("version.txt"), b"1").unwrap();

    let models = list_models_at(&Registry::HuggingFace, temp.path()).unwrap();

    assert_eq!(
        models,
        vec!["openai/gpt-2".to_string(), "zoo/z-model".to_string()]
    );
}

#[test]
fn local_listing_preserves_flattened_model_names() {
    let temp = TempDir::new().unwrap();
    std::fs::create_dir(temp.path().join("model-a--v1")).unwrap();
    std::fs::write(temp.path().join("model-b.bin"), b"weights").unwrap();

    let models = list_models_at(&Registry::Local(temp.path().to_path_buf()), temp.path()).unwrap();

    assert_eq!(
        models,
        vec!["model-a/v1".to_string(), "model-b.bin".to_string()]
    );
}

#[test]
fn missing_cache_root_is_an_empty_listing() {
    let temp = TempDir::new().unwrap();
    let missing = temp.path().join("missing");

    assert!(
        list_models_at(&Registry::Local(missing.clone()), &missing)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn non_directory_cache_root_is_reported() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("cache-file");
    std::fs::write(&root, b"not a directory").unwrap();

    assert!(list_models_at(&Registry::Local(root.clone()), &root).is_err());
}
