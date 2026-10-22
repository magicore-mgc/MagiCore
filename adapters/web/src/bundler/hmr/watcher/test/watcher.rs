//! HMR noise filter and watcher attachment tests.
//! Test noise filter và gắn watcher HMR.

use std::{
    path::PathBuf,
    sync::{Arc, atomic::AtomicU64},
};

use tempfile::tempdir;
use tokio::sync::broadcast;

use super::{paths_are_signal, watch_directory};
use crate::bundler::hmr::HmrEvent;

#[test]
fn filters_all_existing_generated_path_components() {
    for path in [
        "/project/node_modules/pkg/index.js",
        "/project/.magicore/cache/file",
        "/project/dist/app.js",
        "/project/build/output.js",
    ] {
        assert!(
            !paths_are_signal(&[PathBuf::from(path)]),
            "noise path: {path}"
        );
    }
}

#[test]
fn accepts_project_paths_and_preserves_empty_path_behavior() {
    assert!(paths_are_signal(&[PathBuf::from("/project/src/main.ts")]));
    assert!(paths_are_signal(&[]));
}

#[test]
fn attaches_a_watcher_to_an_existing_project_directory() {
    let project = tempdir().unwrap();
    let (tx, _receiver) = broadcast::channel::<HmrEvent>(100);

    let watcher = watch_directory(tx, Arc::new(AtomicU64::new(0)), project.path()).unwrap();

    drop(watcher);
}
