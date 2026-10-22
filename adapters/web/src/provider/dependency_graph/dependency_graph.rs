//! `dependency_graph.rs` — dependency graph.
//! Dependency và alias edge.

use super::super::provider::NpmDependencyProvider;
use crate::native;
use mgc_resolver::ResolvedDep;
use mgc_types::PackageName;

impl NpmDependencyProvider {
    pub fn parse_alias_spec(spec: &str) -> Option<(String, String)> {
        let remainder = spec.strip_prefix("npm:")?;
        let at = remainder.rfind('@')?;
        if at == 0 {
            return None;
        }
        let (package, range) = remainder.split_at(at);
        let range = range.strip_prefix('@')?;
        if package.trim().is_empty() || range.trim().is_empty() {
            return None;
        }
        Some((package.to_string(), range.to_string()))
    }

    pub fn collect_resolved_deps(
        &self,
        deps: Option<&std::collections::HashMap<String, String>>,
        optional: bool,
        peer: bool,
    ) -> Vec<ResolvedDep> {
        deps.into_iter()
            .flat_map(|deps| deps.iter())
            .filter_map(|(name, spec)| {
                let alias = PackageName::new(name).ok()?;
                if let Some((target, range)) = Self::parse_alias_spec(spec) {
                    let target = PackageName::new(target).ok()?;
                    self.record_alias_target(&alias, &target);
                    Some(ResolvedDep {
                        package: alias,
                        spec: range,
                        optional,
                        peer,
                    })
                } else {
                    Some(ResolvedDep {
                        package: alias,
                        spec: spec.clone(),
                        optional,
                        peer,
                    })
                }
            })
            .collect()
    }

    /// Collect installable edges from one npm version record. Optional peer
    /// dependencies are compatibility hints, not dependencies to auto-fetch;
    /// npm encodes that distinction in `peerDependenciesMeta`.
    /// (Thu thập edge cài đặt từ metadata npm. Optional peer chỉ là gợi ý
    /// tương thích, không được tự tải; npm khai báo trong `peerDependenciesMeta`.)
    pub(in crate::provider) fn collect_version_dependencies(
        &self,
        version: &native::npm_registry::VersionInfo,
    ) -> Vec<ResolvedDep> {
        let mut dependencies =
            self.collect_resolved_deps(version.dependencies.as_ref(), false, false);
        dependencies.extend(self.collect_resolved_deps(
            version.optional_dependencies.as_ref(),
            true,
            false,
        ));
        let mut peers = self.collect_resolved_deps(version.peer_dependencies.as_ref(), false, true);
        peers.retain(|peer| {
            !version
                .peer_dependencies_meta
                .get(peer.package.as_str())
                .is_some_and(|metadata| metadata.optional)
        });
        dependencies.extend(peers);
        dependencies
    }
}
