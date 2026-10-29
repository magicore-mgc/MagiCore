// Batch version selection — Chọn version theo lô.
// Preserve override, dedupe and fresh-resolution order — Giữ thứ tự override, dedupe và resolve mới.
use crate::solver::resolver::{Resolver, SolveError, profile::ResolverProfile};
use crate::solver::resolver::{
    RootResolution,
    solve::types::{QueuedDependency, SelectedPackage},
};
use mgc_types::{PackageId, PackageName, Version, VersionRange};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;

impl Resolver {
    #[allow(clippy::too_many_arguments)] // Each mutable map records a distinct batch-resolution phase.
    pub(in crate::solver::resolver::solve) async fn select_batch_versions(
        &self,
        batch: Vec<QueuedDependency>,
        mut selected: Vec<SelectedPackage>,
        prefetched_versions: &mut HashMap<String, Arc<[Version]>>,
        resolved: &mut HashMap<String, Version>,
        resolved_versions: &mut HashSet<(String, Version)>,
        root_resolutions: &mut Vec<RootResolution>,
        profile: &ResolverProfile,
    ) -> Result<Vec<SelectedPackage>, SolveError> {
        // Deduplicate and prefetch batch versions — Loại trùng và tải trước version theo lô
        let mut seen = HashSet::new();
        let batch_names: Vec<PackageName> = batch
            .iter()
            .filter(|request| {
                let name = request.name.as_str().to_string();
                seen.insert(name.clone()) && !prefetched_versions.contains_key(&name)
            })
            .map(|request| request.name.clone())
            .collect();
        if !batch_names.is_empty() {
            let batch_prefetch_started_at = Instant::now();
            prefetched_versions.extend(Self::versions_to_map(
                self.provider
                    .prefetch_versions(&batch_names)
                    .await
                    .map_err(|e| SolveError {
                        message: format!("batch prefetch failed: {e}"),
                    })?,
            ));
            profile.mark("batch_prefetch_versions", batch_prefetch_started_at);
        }

        let select_started_at = Instant::now();
        for request in batch {
            let QueuedDependency {
                name,
                spec,
                root_index,
            } = request;
            let name_str = name.as_str().to_string();
            let constraint = VersionRange::parse(&spec).map_err(|e| SolveError {
                message: format!("invalid spec '{}' for '{}': {}", spec, name_str, e),
            })?;

            // Phase 1: override — Pha 1: áp dụng ghi đè version
            if let Some(override_spec) = self.overrides.get(&name_str) {
                let oc = VersionRange::parse(override_spec).map_err(|e| SolveError {
                    message: format!("invalid override '{}': {}", override_spec, e),
                })?;
                let versions = if let Some(prefetched) = prefetched_versions.get(&name_str) {
                    Arc::clone(prefetched)
                } else {
                    Arc::<[Version]>::from(self.provider.get_versions(&name).await.map_err(
                        |e| SolveError {
                            message: format!("versions fetch failed for '{}': {}", name_str, e),
                        },
                    )?)
                };
                if let Some(v) = Self::select_best_version(versions.as_ref(), &oc, override_spec)
                    .or_else(|| versions.iter().filter(|v| v.pre.is_none()).cloned().max())
                    .or_else(|| versions.iter().cloned().max())
                {
                    record_root_resolution(root_index, &name, &spec, &v, root_resolutions);
                    resolved.insert(name_str.clone(), v.clone());
                    resolved_versions.insert((name_str.clone(), v.clone()));
                    selected.push((name, name_str, v));
                }
                continue;
            }

            // Phase 2: already resolved — Pha 2: package đã resolve
            if let Some(existing) = resolved.get(&name_str).cloned() {
                if constraint.matches(&existing) {
                    record_root_resolution(root_index, &name, &spec, &existing, root_resolutions);
                    continue;
                }
                let versions = if let Some(prefetched) = prefetched_versions.get(&name_str) {
                    Arc::clone(prefetched)
                } else {
                    Arc::<[Version]>::from(self.provider.get_versions(&name).await.map_err(
                        |e| SolveError {
                            message: format!("cannot fetch versions for '{}': {}", name_str, e),
                        },
                    )?)
                };

                if let Some(other) =
                    Self::select_best_version(versions.as_ref(), &constraint, &spec)
                {
                    record_root_resolution(root_index, &name, &spec, &other, root_resolutions);
                    let key = (name_str.clone(), other.clone());
                    if !resolved_versions.contains(&key) {
                        resolved_versions.insert(key);
                        selected.push((name, name_str, other));
                    }
                }
                continue;
            }

            // Phase 3: fresh resolve — Pha 3: package mới
            let versions = if let Some(prefetched) = prefetched_versions.get(&name_str) {
                Arc::clone(prefetched)
            } else {
                Arc::<[Version]>::from(self.provider.get_versions(&name).await.map_err(|e| {
                    SolveError {
                        message: format!("cannot fetch versions for '{}': {}", name_str, e),
                    }
                })?)
            };
            let version = self.select_version(&name_str, &constraint, &spec, versions.as_ref());

            match version {
                Some(v) => {
                    record_root_resolution(root_index, &name, &spec, &v, root_resolutions);
                    resolved.insert(name_str.clone(), v.clone());
                    resolved_versions.insert((name_str.clone(), v.clone()));
                    selected.push((name, name_str, v));
                }
                None => {
                    // Name the registry's newest version. Without it the message
                    // says only "no match", so a user cannot tell a typo from a
                    // template pinned ahead of its release — the exact failure that
                    // breaks `create-astro`: upstream's `main` branch ships
                    // `astro: ^7.4.3` while npm's newest is 7.4.2.
                    // (Nêu version mới nhất của registry. Thiếu nó thì thông điệp
                    // chỉ nói "không khớp", người dùng không phân biệt được gõ sai
                    // với template đi trước bản phát hành — đúng lỗi của
                    // `create-astro`.)
                    // `max`, not `last`: the registry does not guarantee order, and
                    // a hint naming the wrong "newest" would mislead.
                    // (`max` không phải `last`: registry không đảm bảo thứ tự.)
                    let newest = versions.iter().max().map(ToString::to_string);
                    let newest_hint = match newest {
                        Some(newest) => format!(
                            " (newest published is {newest}; the required range may come from a \
                             template pinned ahead of its release)"
                        ),
                        None => " (the registry returned no versions for this package)".to_string(),
                    };
                    return Err(SolveError {
                        message: format!(
                            "no version of '{name_str}' matches '{spec}'{newest_hint}"
                        ),
                    });
                }
            }
        }
        profile.mark("select_versions", select_started_at);
        Ok(selected)
    }
}

fn record_root_resolution(
    root_index: Option<usize>,
    name: &PackageName,
    spec: &str,
    version: &Version,
    root_resolutions: &mut Vec<RootResolution>,
) {
    let Some(root_index) = root_index else {
        return;
    };
    root_resolutions.push(RootResolution::new(
        root_index,
        name.clone(),
        spec.to_owned(),
        PackageId::new(name.clone(), version.clone()),
    ));
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
