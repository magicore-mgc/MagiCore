//! Bounded breadth-first dependency graph traversal.
//! Duyệt graph dependency breadth-first có giới hạn đồng thời.

use std::sync::Arc;

use futures_util::stream::{self, StreamExt, TryStreamExt};
use mgc_types::{MgError, MgResult};
use tokio::sync::Semaphore;

use super::error::GraphResolutionError;
use crate::protocols::{RegistryProtocol, ResolvedEntry};

pub(crate) const GRAPH_RESOLVE_CONCURRENCY: usize = 16;
pub(crate) const GRAPH_ROOT_CONCURRENCY: usize = 4;

pub(crate) async fn resolve_graph_with_semaphore<P: RegistryProtocol + ?Sized>(
    protocol: &P,
    name: &str,
    range: &str,
    semaphore: Arc<Semaphore>,
) -> MgResult<Vec<ResolvedEntry>> {
    let mut entries = Vec::new();
    let mut chosen = std::collections::HashMap::<String, String>::new();
    let mut queue = std::collections::VecDeque::from([(name.to_string(), range.to_string())]);

    while !queue.is_empty() {
        // Resolve peers in this BFS layer concurrently, but process results in
        // queue order. Root graphs in this invocation share one semaphore so
        // their combined requests stay within the protocol-level limit.
        // (Resolve các dependency cùng tầng BFS song song nhưng xử lý theo
        // thứ tự queue. Các root anh em dùng chung semaphore để không nhân
        // giới hạn request của protocol.)
        let layer = queue.drain(..).collect::<Vec<_>>();
        let mut resolved_layer = stream::iter(layer.into_iter().map(|(name, range)| {
            let semaphore = Arc::clone(&semaphore);
            async move {
                let _permit = semaphore
                    .acquire_owned()
                    .await
                    .map_err(|_| GraphResolutionError::SemaphoreClosed)?;
                let entry = protocol.resolve(&name, &range).await?;
                Ok::<_, MgError>((name, range, entry))
            }
        }))
        .buffered(GRAPH_RESOLVE_CONCURRENCY);

        while let Some((name, range, entry)) = resolved_layer.try_next().await? {
            if let Some(chosen_version) = chosen.get(&name) {
                // Re-resolve conflicting constraints and fail closed rather
                // than silently choosing a version without backtracking.
                // (Resolve lại constraint xung đột và fail-closed thay vì âm
                // thầm chọn version khi chưa có backtracking.)
                if entry.version != *chosen_version {
                    return Err(MgError::DependencyConflict(format!(
                        "incompatible constraints for {name}: selected {chosen_version}, but constraint '{range}' resolves to {} (native resolver does not backtrack yet)",
                        entry.version
                    )));
                }
                continue;
            }
            chosen.insert(entry.name.clone(), entry.version.clone());
            for (dep_name, dep_range) in &entry.deps {
                if !chosen.contains_key(dep_name) {
                    queue.push_back((dep_name.clone(), dep_range.clone()));
                }
            }
            entries.push(entry);
        }
    }
    Ok(entries)
}

#[cfg(test)]
#[path = "test/graph_resolution.rs"]
mod tests;
