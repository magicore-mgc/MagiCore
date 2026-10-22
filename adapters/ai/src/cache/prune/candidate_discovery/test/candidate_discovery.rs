//! Candidate discovery cases — kiểm tra sidecar, entry thường và symlink.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::super::candidate_discovery::discover_candidates;
use crate::registry::Registry;
use tempfile::TempDir;

#[test]
fn sidecars_are_not_discovered_as_independent_models() {
    let temp = TempDir::new().unwrap();
    std::fs::write(temp.path().join("weights.bin"), [1u8; 3]).unwrap();
    std::fs::write(temp.path().join("weights.meta.json"), b"{}").unwrap();

    let candidates = discover_candidates(
        &Registry::Local(temp.path().to_path_buf()),
        temp.path(),
        true,
    )
    .unwrap();

    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].path.file_name().unwrap(), "weights.bin");
    assert_eq!(
        std::fs::metadata(candidates[0].path.with_extension("meta.json"))
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn shared_sidecar_is_not_assigned_to_either_candidate() {
    let temp = TempDir::new().unwrap();
    std::fs::write(temp.path().join("weights"), [1u8; 3]).unwrap();
    std::fs::write(temp.path().join("weights.bin"), [1u8; 5]).unwrap();
    std::fs::write(temp.path().join("weights.meta.json"), b"{}").unwrap();

    let candidates = discover_candidates(
        &Registry::Local(temp.path().to_path_buf()),
        temp.path(),
        true,
    )
    .unwrap();

    assert_eq!(candidates.len(), 2);
    assert!(
        candidates
            .iter()
            .all(|candidate| candidate.metadata.is_none())
    );
    assert!(
        candidates
            .iter()
            .all(|candidate| candidate.metadata_sidecar.is_none())
    );
}

#[cfg(unix)]
#[test]
fn symbolic_links_are_not_candidates() {
    use std::os::unix::fs::symlink;

    let temp = TempDir::new().unwrap();
    let external_root = TempDir::new().unwrap();
    let external = external_root.path().join("external.bin");
    std::fs::write(&external, [1u8; 5]).unwrap();
    symlink(&external, temp.path().join("linked.bin")).unwrap();

    assert!(
        discover_candidates(
            &Registry::Local(temp.path().to_path_buf()),
            temp.path(),
            true,
        )
        .unwrap()
        .is_empty()
    );
}

#[test]
fn non_size_policy_does_not_measure_model_directory_contents() {
    let temp = TempDir::new().unwrap();
    let model = temp.path().join("model");
    std::fs::create_dir(&model).unwrap();
    std::fs::write(model.join("weights.bin"), [1u8; 64]).unwrap();

    let candidates = discover_candidates(
        &Registry::Local(temp.path().to_path_buf()),
        temp.path(),
        false,
    )
    .unwrap();

    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].content_bytes, None);
}

#[test]
fn hugging_face_discovery_skips_control_and_non_model_root_entries() {
    let temp = TempDir::new().unwrap();
    for name in ["models--org--model", "datasets--org--data", ".locks"] {
        std::fs::create_dir(temp.path().join(name)).unwrap();
    }
    std::fs::write(temp.path().join("version.txt"), b"1").unwrap();

    let candidates = discover_candidates(&Registry::HuggingFace, temp.path(), false).unwrap();

    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].model_id(), "org/model");
}
