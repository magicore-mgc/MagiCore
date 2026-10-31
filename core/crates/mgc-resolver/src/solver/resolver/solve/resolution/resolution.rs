use futures_util::future::join_all;
use mgc_types::{PackageId, PackageName, Version};
use std::collections::{HashMap, VecDeque};

use super::super::types::QueuedDependency;
use crate::solver::provider::{DependencyProvider, ResolvedDep};
use crate::solver::resolver::{Resolution, Resolver, SolveError};

impl Resolver {
    /// Record a resolved package and enqueue its transitive dependencies.
    pub(in crate::solver::resolver::solve) async fn add_resolution(
        resolutions: &mut Vec<Resolution>,
        name: &PackageName,
        _name_str: &str,
        version: Version,
        deps: &[ResolvedDep],
        provider: &dyn DependencyProvider,
        queue: &mut VecDeque<QueuedDependency>,
    ) -> Result<(), SolveError> {
        let pid = PackageId::new(name.clone(), version.clone());
        let dep_names: Vec<String> = deps
            .iter()
            .map(|d| d.package.as_str().to_string())
            .collect();
        let dep_specs: Vec<(String, String)> = deps
            .iter()
            .map(|d| (d.package.as_str().to_string(), d.spec.clone()))
            .collect();
        let peer_deps: Vec<String> = deps
            .iter()
            .filter(|d| d.peer)
            .map(|d| d.package.as_str().to_string())
            .collect();
        let enqueue_results = join_all(deps.iter().map(|dep| provider.should_enqueue(dep))).await;
        for (dep, should_enqueue) in deps.iter().zip(enqueue_results) {
            if should_enqueue.map_err(|e| SolveError {
                message: format!(
                    "dependency enqueue check failed for '{}@{}': {}",
                    dep.package.as_str(),
                    dep.spec,
                    e
                ),
            })? {
                queue.push_front(QueuedDependency::transitive(
                    dep.package.clone(),
                    dep.spec.clone(),
                ));
            }
        }

        resolutions.push(Resolution {
            package_id: pid,
            version,
            integrity: String::new(),
            deps: dep_names,
            dep_specs,
            peer_deps,
        });

        Ok(())
    }

    /// Peer instance merge post-solve (02 §2.1): merge resolutions that share
    /// the same (name, version) — same peer signature — into one instance.
    /// Safe: the solver already dedupes (name, version) pairs; this pass merges
    /// dep lists when the same instance was recorded more than once.
    pub(in crate::solver::resolver::solve) fn merge_peer_instances(
        resolutions: Vec<Resolution>,
    ) -> Vec<Resolution> {
        let mut merged: HashMap<(String, String), Resolution> = HashMap::new();
        for r in resolutions {
            let key = (
                r.package_id.name_str().to_string(),
                r.package_id.version().to_string(),
            );
            match merged.get_mut(&key) {
                Some(existing) => {
                    for dep in &r.deps {
                        if !existing.deps.contains(dep) {
                            existing.deps.push(dep.clone());
                        }
                    }
                    for (name, spec) in &r.dep_specs {
                        if !existing.dep_specs.contains(&(name.clone(), spec.clone())) {
                            existing.dep_specs.push((name.clone(), spec.clone()));
                        }
                    }
                }
                None => {
                    merged.insert(key, r);
                }
            }
        }
        // Sort by package identity so downstream serializers (esp. the v3
        // lockfile writer) emit a deterministic order; an unsorted HashMap
        // drain makes the lockfile non-reproducible across runs.
        // Sắp theo danh tính package để serializer phía sau (nhất là writer v3)
        // phát ra thứ tự tất định; drain HashMap không sort làm lockfile không
        // tái lập được giữa các lần chạy.
        let mut packages: Vec<_> = merged.into_values().collect();
        packages.sort_by(|a, b| {
            a.package_id
                .name_str()
                .cmp(b.package_id.name_str())
                .then_with(|| {
                    a.package_id
                        .version()
                        .to_string()
                        .cmp(&b.package_id.version().to_string())
                })
        });
        packages
    }
}
