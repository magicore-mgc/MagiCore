// Solve orchestration — Điều phối vòng resolve dependency.
// Keeps the public resolver entry point stable — Giữ nguyên điểm vào public của resolver.
use super::super::{
    Resolution, Resolver, RootResolution, SolveError, SolveResult, SolveResultWithRoots,
    profile::ResolverProfile,
};
use super::types::{PrefetchedVersions, QueuedDependency};
use mgc_types::{PackageName, Version};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

impl Resolver {
    /// Resolve all dependencies. Fails with `SolveError` on provider error or
    /// unresolvable constraint.
    /// Resolve toàn bộ dependency và trả lỗi khi provider hoặc constraint thất bại.
    pub async fn solve(&self, wanted: &[(PackageName, String)]) -> Result<SolveResult, SolveError> {
        let result = self.solve_inner(wanted, false).await?;
        Ok(result.result)
    }

    /// Resolve all dependencies and retain the exact selection for each root request.
    /// Resolve dependency và giữ lựa chọn chính xác tương ứng từng root request.
    pub async fn solve_with_roots(
        &self,
        wanted: &[(PackageName, String)],
    ) -> Result<SolveResultWithRoots, SolveError> {
        self.solve_inner(wanted, true).await
    }

    async fn solve_inner(
        &self,
        wanted: &[(PackageName, String)],
        include_root_metadata: bool,
    ) -> Result<SolveResultWithRoots, SolveError> {
        const MAX_PACKAGES: usize = 10000;
        const MAX_QUEUE_SIZE: usize = 50000;

        let solve_started_at = Instant::now();
        let profile = ResolverProfile::from_env();
        let mut resolutions: Vec<Resolution> = Vec::new();
        let mut resolved: HashMap<String, Version> = HashMap::new();
        let mut queue: VecDeque<QueuedDependency> = wanted
            .iter()
            .enumerate()
            .map(|(index, (name, spec))| {
                if include_root_metadata {
                    QueuedDependency::root(index, name.clone(), spec.clone())
                } else {
                    QueuedDependency::transitive(name.clone(), spec.clone())
                }
            })
            .collect();
        let mut resolved_versions: HashSet<(String, Version)> = HashSet::new();
        let mut root_resolutions: Vec<RootResolution> = Vec::new();

        // Pre-fetch initial batch — Tải trước lô dependency ban đầu.
        let initial_names: Vec<PackageName> = wanted.iter().map(|(name, _)| name.clone()).collect();
        let initial_prefetch_started_at = Instant::now();
        let initial_versions = self
            .provider
            .prefetch_versions(&initial_names)
            .await
            .map_err(|e| SolveError {
                message: format!("initial prefetch failed: {e}"),
            })?;
        profile.mark("initial_prefetch_versions", initial_prefetch_started_at);
        let mut prefetched_versions: PrefetchedVersions = Self::versions_to_map(initial_versions);

        while !queue.is_empty() {
            if resolutions.len() >= MAX_PACKAGES {
                return Err(SolveError {
                    message: format!(
                        "dependency graph too large: exceeded {} packages (possible circular dependency or dep bomb)",
                        MAX_PACKAGES
                    ),
                });
            }

            if queue.len() > MAX_QUEUE_SIZE {
                return Err(SolveError {
                    message: format!(
                        "dependency queue overflow: exceeded {} entries (possible dep bomb attack)",
                        MAX_QUEUE_SIZE
                    ),
                });
            }

            let batch_limit = std::env::var("MAGICORE_RESOLVER_BATCH_SIZE")
                .ok()
                .and_then(|raw| raw.trim().parse::<usize>().ok())
                .filter(|limit| *limit > 0)
                .unwrap_or(1024);
            let batch_size = queue.len().min(batch_limit);
            let batch: Vec<QueuedDependency> = queue.drain(..batch_size).collect();
            let selected = Vec::new();
            let batch_started_at = Instant::now();
            let selected = self
                .select_batch_versions(
                    batch,
                    selected,
                    &mut prefetched_versions,
                    &mut resolved,
                    &mut resolved_versions,
                    &mut root_resolutions,
                    &profile,
                )
                .await?;

            if !selected.is_empty() {
                let (prefetched_dependencies, next_prefetch_names) = self
                    .prefetch_selected_dependencies(&selected, &prefetched_versions, &profile)
                    .await?;
                let (new_resolutions, new_queue) = self
                    .materialize_resolution_batch(
                        selected,
                        prefetched_dependencies,
                        next_prefetch_names,
                        &mut prefetched_versions,
                        &profile,
                    )
                    .await?;
                resolutions.extend(new_resolutions);
                queue.extend(new_queue);
            }
            profile.mark("batch_total", batch_started_at);
        }

        profile.mark("solve_total", solve_started_at);
        if include_root_metadata {
            root_resolutions.sort_by_key(|root| root.request_index);
            if root_resolutions.len() != wanted.len()
                || root_resolutions.iter().zip(wanted).enumerate().any(
                    |(index, (root, (name, spec)))| {
                        root.request_index != index
                            || root.request_name.as_str() != name.as_str()
                            || root.request_spec.as_str() != spec.as_str()
                    },
                )
            {
                return Err(SolveError {
                    message: "resolver did not produce one exact package selection for every root request"
                        .to_owned(),
                });
            }
        }

        Ok(SolveResultWithRoots {
            result: SolveResult {
                resolutions: Self::merge_peer_instances(resolutions),
            },
            root_resolutions,
        })
    }
}
