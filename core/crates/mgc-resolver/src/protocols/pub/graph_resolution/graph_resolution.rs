//! Iterative joint-root selection with bounded requests and convergence checks.
//! Chọn version lặp cho nhiều root, giới hạn request và kiểm tra hội tụ.

use super::super::package_catalog::PubPackage;
use super::GraphResolutionError;
use crate::protocols::{GRAPH_RESOLVE_CONCURRENCY, ResolvedEntry};
use futures_util::stream::{self, StreamExt, TryStreamExt};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Semaphore;

const MAX_PUB_GRAPH_SOLVE_ROUNDS: usize = 128;

impl super::super::PubProtocol {
    pub(in crate::protocols::pubdev) async fn resolve_joint_roots(
        &self,
        roots: &[(String, String)],
    ) -> Result<Vec<ResolvedEntry>, GraphResolutionError> {
        let mut docs = HashMap::<String, PubPackage>::new();
        let mut selected = HashMap::<String, ResolvedEntry>::new();
        let mut seen_states = HashSet::<Vec<(String, String)>>::new();
        let request_limit = Arc::new(Semaphore::new(GRAPH_RESOLVE_CONCURRENCY));

        for _ in 0..MAX_PUB_GRAPH_SOLVE_ROUNDS {
            let mut constraints = BTreeMap::<String, BTreeSet<String>>::new();
            for (name, range) in roots {
                constraints
                    .entry(name.clone())
                    .or_default()
                    .insert(range.clone());
            }
            for entry in selected.values() {
                for (dependency, range) in &entry.deps {
                    constraints
                        .entry(dependency.clone())
                        .or_default()
                        .insert(range.clone());
                }
            }

            let missing_docs = constraints
                .keys()
                .filter(|name| !docs.contains_key(*name))
                .cloned()
                .collect::<Vec<_>>();
            let mut fetched_docs = stream::iter(missing_docs.into_iter().map(|name| {
                let request_limit = Arc::clone(&request_limit);
                async move {
                    let _permit = request_limit
                        .acquire_owned()
                        .await
                        .map_err(|_| GraphResolutionError::SemaphoreClosed)?;
                    let doc = self.package_doc(&name).await?;
                    Ok::<_, GraphResolutionError>((name, doc))
                }
            }))
            .buffered(GRAPH_RESOLVE_CONCURRENCY);
            while let Some((name, doc)) = fetched_docs.try_next().await? {
                docs.insert(name, doc);
            }

            let mut next = HashMap::<String, ResolvedEntry>::new();
            for (name, ranges) in constraints {
                let constraints = ranges.into_iter().collect::<Vec<_>>();
                let doc = docs
                    .get(&name)
                    .ok_or_else(|| GraphResolutionError::MissingCachedMetadata(name.clone()))?;
                let entry = super::super::package_catalog::select_entry(&name, doc, &constraints)?;
                next.insert(name, entry);
            }

            let signature = selection_signature(&next);
            let previous_signature = selection_signature(&selected);
            if signature == previous_signature {
                let ordered = next
                    .into_iter()
                    .collect::<BTreeMap<_, _>>()
                    .into_values()
                    .collect();
                return Ok(ordered);
            }
            if !seen_states.insert(signature) {
                return Err(GraphResolutionError::OscillatingSelection);
            }
            selected = next;
        }

        Err(GraphResolutionError::DidNotConverge(
            MAX_PUB_GRAPH_SOLVE_ROUNDS,
        ))
    }
}

fn selection_signature(selected: &HashMap<String, ResolvedEntry>) -> Vec<(String, String)> {
    let mut signature = selected
        .iter()
        .map(|(name, entry)| (name.clone(), entry.version.clone()))
        .collect::<Vec<_>>();
    signature.sort();
    signature
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
