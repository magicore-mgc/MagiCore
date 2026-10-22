//! Resolve the transitive Cargo dependency and feature graph.
//! Resolve graph dependency và feature Cargo bắc cầu.

use super::super::super::super::{
    CrateDependencyRequest, CrateIndexEntry, CrateResolutionState, CratesProtocol,
    GRAPH_RESOLVE_CONCURRENCY, RegistryRoot, ResolvedEntry,
};
use super::{FeatureGraphError, FeatureGraphResult};
use futures_util::stream::{self, StreamExt, TryStreamExt};
use std::collections::{BTreeMap, BTreeSet};

pub(in crate::protocols::crates) async fn resolve_feature_graph(
    protocol: &CratesProtocol,
    roots: &[RegistryRoot],
) -> FeatureGraphResult<Vec<ResolvedEntry>> {
    let mut states = BTreeMap::<(String, String), CrateResolutionState>::new();
    let mut index_cache = BTreeMap::<String, Vec<CrateIndexEntry>>::new();
    let mut root_requests = BTreeMap::<String, CrateDependencyRequest>::new();
    for root in roots {
        let request = root_requests.entry(root.name.clone()).or_default();
        request.ranges.push(root.range.clone());
        request.features.extend(root.features.iter().cloned());
        request.default_features |= root.default_features;
    }

    for (name, request) in root_requests {
        let ranges = unique_strings(request.ranges);
        let entries = super::super::super::super::index::index_entries(protocol, &name).await?;
        let entry =
            super::super::super::super::index::select_index_entry(&name, &ranges, entries.clone())?;
        index_cache.insert(name, entries);
        let key = (entry.name.clone(), entry.vers.clone());
        states.insert(
            key,
            CrateResolutionState {
                entry,
                requested_features: request.features,
                default_features: request.default_features,
                active: true,
                resolved_dependencies: BTreeSet::new(),
                extra_markers: BTreeSet::new(),
            },
        );
    }

    loop {
        let mut pending = Vec::new();
        for (parent, state) in &states {
            let activation = super::super::activation::activate_crate_features(
                &state.entry,
                &state.requested_features,
                state.default_features,
            )?;
            for dependency in &state.entry.deps {
                if dependency.kind.as_deref() == Some("dev") {
                    continue;
                }
                let alias = dependency.name.as_str();
                let dependency_active = state.active
                    && (!dependency.optional || activation.optional_dependencies.contains(alias));
                let weak_lock_only = activation.lock_only_optional_dependencies.contains(alias);
                let feature_selected = activation.optional_dependencies.contains(alias);
                if dependency.optional
                    && !dependency_active
                    && !weak_lock_only
                    && !(feature_selected && !state.active)
                {
                    continue;
                }
                if let Some(registry) = &dependency.registry {
                    return Err(FeatureGraphError::AlternateRegistry {
                        package: state.entry.name.clone(),
                        version: state.entry.vers.clone(),
                        dependency: dependency.name.clone(),
                        registry: registry.clone(),
                    });
                }
                let actual_name = dependency
                    .package
                    .as_deref()
                    .unwrap_or(dependency.name.as_str())
                    .to_string();
                let mut request = CrateDependencyRequest::default();
                request.ranges.push(if dependency.req.trim().is_empty() {
                    "*".to_string()
                } else {
                    dependency.req.clone()
                });
                request.features.extend(dependency.features.iter().cloned());
                if let Some(features) = activation.dependency_features.get(alias) {
                    request.features.extend(features.iter().cloned());
                }
                request.default_features = dependency_active && dependency.default_features;
                pending.push((parent.clone(), actual_name, request, dependency_active));
            }
        }

        let names_to_fetch = pending
            .iter()
            .map(|(_, name, _, _)| name.clone())
            .filter(|name| !index_cache.contains_key(name))
            .collect::<BTreeSet<_>>();
        let fetched = stream::iter(names_to_fetch.into_iter().map(|name| async move {
            let entries = super::super::super::super::index::index_entries(protocol, &name).await?;
            Ok::<_, FeatureGraphError>((name, entries))
        }))
        .buffered(GRAPH_RESOLVE_CONCURRENCY)
        .try_collect::<Vec<_>>()
        .await?;
        index_cache.extend(fetched);

        let mut changed = false;
        let mut next_edges = BTreeMap::<(String, String), BTreeSet<(String, String)>>::new();
        for (parent, name, mut request, dependency_active) in pending {
            request.ranges = unique_strings(request.ranges);
            let entries = index_cache
                .get(&name)
                .ok_or(FeatureGraphError::IndexCacheLost)?;
            let entry = super::super::super::super::index::select_index_entry(
                &name,
                &request.ranges,
                entries.clone(),
            )?;
            let key = (entry.name.clone(), entry.vers.clone());
            next_edges.entry(parent).or_default().insert(key.clone());
            if let Some(state) = states.get_mut(&key) {
                if dependency_active && !state.active {
                    state.active = true;
                    changed = true;
                }
                for feature in request.features {
                    changed |= state.requested_features.insert(feature);
                }
                if request.default_features && !state.default_features {
                    state.default_features = true;
                    changed = true;
                }
            } else {
                states.insert(
                    key.clone(),
                    CrateResolutionState {
                        entry,
                        requested_features: request.features,
                        default_features: request.default_features,
                        active: dependency_active,
                        resolved_dependencies: BTreeSet::new(),
                        extra_markers: BTreeSet::new(),
                    },
                );
                changed = true;
            }
        }
        for (key, state) in &mut states {
            state.resolved_dependencies = next_edges.remove(key).unwrap_or_default();
            state.extra_markers.clear();
            if !state.active {
                continue;
            }
            let activation = super::super::activation::activate_crate_features(
                &state.entry,
                &state.requested_features,
                state.default_features,
            )?;
            for dependency in &state.entry.deps {
                if dependency.kind.as_deref() == Some("dev") {
                    continue;
                }
                if let Some(target) = &dependency.target {
                    state.extra_markers.insert(format!("target:{target}"));
                }
                if dependency.optional
                    && !activation
                        .optional_dependencies
                        .contains(dependency.name.as_str())
                {
                    let actual_name = dependency
                        .package
                        .as_deref()
                        .unwrap_or(dependency.name.as_str());
                    state
                        .extra_markers
                        .insert(format!("optional:{actual_name}"));
                }
            }
        }
        if !changed {
            break;
        }
    }

    states
        .values()
        .map(|state| {
            super::super::resolved_entry::resolved_entry_for_state(protocol, state)
                .map_err(super::FeatureGraphError::from)
        })
        .collect()
}

fn unique_strings(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}
