// Resolution batch — Materialize lô resolve.
// Preserve enqueue order and overlap next-wave prefetch — Giữ thứ tự và tải trước wave kế tiếp song song.
use crate::solver::resolver::profile::ResolverProfile;
use crate::solver::resolver::solve::types::QueuedDependency;
use crate::solver::resolver::solve::types::{
    PrefetchedDependencies, PrefetchedVersions, SelectedPackage,
};
use crate::solver::resolver::{Resolution, Resolver, SolveError};
use futures_util::future::join;
use mgc_types::{PackageId, PackageName};
use std::collections::VecDeque;
use std::time::Instant;

impl Resolver {
    pub(in crate::solver::resolver::solve) async fn materialize_resolution_batch(
        &self,
        selected: Vec<SelectedPackage>,
        prefetched_dependencies: PrefetchedDependencies,
        next_prefetch_names: Vec<PackageName>,
        prefetched_versions: &mut PrefetchedVersions,
        profile: &ResolverProfile,
    ) -> Result<(Vec<Resolution>, VecDeque<QueuedDependency>), SolveError> {
        let add_resolution_started_at = Instant::now();
        let add_resolution_future = async {
            let mut local_resolutions = Vec::new();
            let mut local_queue = VecDeque::new();
            for (name, name_str, version) in selected {
                let pid = PackageId::new(name.clone(), version.clone());
                let deps = prefetched_dependencies
                    .get(&pid)
                    .ok_or_else(|| SolveError {
                        message: format!("dependency prefetch missing result for '{}'", pid),
                    })?;
                Self::add_resolution(
                    &mut local_resolutions,
                    &name,
                    &name_str,
                    version,
                    deps.as_ref(),
                    self.provider.as_ref(),
                    &mut local_queue,
                )
                .await?;
            }
            Ok::<_, SolveError>((local_resolutions, local_queue))
        };
        let (add_resolution_result, prefetched_next_result) = if next_prefetch_names.is_empty() {
            (add_resolution_future.await, Ok(Vec::new()))
        } else {
            join(
                add_resolution_future,
                self.provider.prefetch_versions(&next_prefetch_names),
            )
            .await
        };
        let (new_resolutions, new_queue) = add_resolution_result?;
        profile.mark("add_resolution", add_resolution_started_at);
        prefetched_versions.extend(Self::versions_to_map(prefetched_next_result.map_err(
            |e| SolveError {
                message: format!("next-wave prefetch failed: {e}"),
            },
        )?));
        Ok((new_resolutions, new_queue))
    }
}
