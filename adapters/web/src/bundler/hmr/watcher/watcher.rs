//! Filesystem watcher and noise-filtered event publication.
//! Watcher filesystem và phát event sau khi lọc nhiễu.

use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::broadcast;
use tracing::{error, info, trace};

use super::super::{error::HmrWatchError, event::HmrEvent};

/// Attach notify and retain the existing event/filter/log behavior.
/// Gắn notify và giữ nguyên hành vi event/filter/log hiện có.
pub(in crate::bundler::hmr) fn watch_directory(
    tx: broadcast::Sender<HmrEvent>,
    version: Arc<AtomicU64>,
    path: &Path,
) -> Result<RecommendedWatcher, HmrWatchError> {
    let mut watcher =
        notify::recommended_watcher(move |result: notify::Result<Event>| match result {
            Ok(event) => {
                trace!("HMR File Event: {:?}", event);
                if (event.kind.is_modify() || event.kind.is_create() || event.kind.is_remove())
                    && paths_are_signal(&event.paths)
                {
                    let next_version = version.fetch_add(1, Ordering::SeqCst) + 1;
                    let files = event
                        .paths
                        .iter()
                        .map(|path| path.display().to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    println!("[MgDevServer] hmr update {files} (version {next_version})");
                    let _ = tx.send(HmrEvent::Reload);
                }
            }
            Err(error) => error!("watch error: {:?}", error),
        })
        .map_err(HmrWatchError::from)?;

    watcher
        .watch(path, RecursiveMode::Recursive)
        .map_err(HmrWatchError::from)?;
    info!("HMR watching directory: {}", path.display());
    Ok(watcher)
}

/// Ignore events whose paths include machine-generated workspace folders.
/// Bỏ event có path chứa folder workspace do máy tạo.
fn paths_are_signal(paths: &[PathBuf]) -> bool {
    paths.iter().all(|path| {
        !path.components().any(|component| {
            matches!(
                component.as_os_str().to_str(),
                Some("node_modules" | ".magicore" | "dist" | "build")
            )
        })
    })
}

#[cfg(test)]
#[path = "test/watcher.rs"]
mod tests;
