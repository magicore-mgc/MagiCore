//! `resolution.rs` — Dependency resolution coordinator / điều phối resolve dependency.
use super::{
    lockfile_lookup, offline_gate, package_graph, peer_validation, publication, request,
    shared_cache_lookup, solver,
};
use crate::profile::ResolveProfile;
use crate::supply_chain::enforce_resolution_supply_chain_guards;
use crate::{WebAdapter, manifest_resolution_cache_key};
use async_trait::async_trait;
use mgc_types::{
    Manifest, MgError, MgResult, adapter::ResolvedGraph, capabilities::DependencyResolver,
};
use std::time::Instant;

impl WebAdapter {
    async fn resolve_inner(
        &self,
        manifest: &Manifest,
        allow_shortcircuit: bool,
    ) -> MgResult<ResolvedGraph> {
        eprintln!(
            "[magicore:debug] resolve:packages={}",
            manifest.all_dependencies().count()
        );
        let started_at = Instant::now();
        let mut profile = ResolveProfile::from_env();
        let Some(prepared) =
            request::prepare_request(self, manifest, &mut profile, started_at).await?
        else {
            return Ok(ResolvedGraph::empty());
        };

        if let Some(graph) = lockfile_lookup::find_lockfile_graph(
            &prepared.resolved_manifest,
            allow_shortcircuit,
            prepared.age_gate_active,
            &mut profile,
            started_at,
        )? {
            return Ok(graph);
        }

        let resolution_cache_key = self.shared_cache.as_ref().map(|_| {
            manifest_resolution_cache_key(&prepared.resolved_manifest, &self.registry_url)
        });
        if let Some(graph) = shared_cache_lookup::read_shared_graph(
            self,
            manifest,
            &prepared.resolved_manifest,
            allow_shortcircuit,
            prepared.age_gate_active,
            resolution_cache_key.as_deref(),
            &mut profile,
            started_at,
        )? {
            return Ok(graph);
        }

        // An offline miss cannot proceed to registry metadata.
        // Offline miss không được đi tiếp tới metadata registry.
        if offline_gate::mgc_env_flag("MGC_OFFLINE_MODE") {
            return Err(MgError::Other(
                "offline mode: cannot resolve dependencies (no lockfile match and no cached \
                 resolution). Run 'mgc install' online first to create the lockfile."
                    .to_string(),
            ));
        }

        let snapshot = solver::solve(self, &prepared.wanted, &mut profile).await?;
        let index_started_at = Instant::now();
        let index = package_graph::build_resolution_index(&snapshot.resolutions);
        profile.mark("build_resolution_index", index_started_at);
        peer_validation::validate_required_peers(&snapshot, &index)?;

        let package_started_at = Instant::now();
        let packages = package_graph::assemble_resolved_packages(
            self,
            &prepared.resolved_manifest,
            &snapshot.resolutions,
            &snapshot.root_resolutions,
            &snapshot.metadata,
            &index,
        )?;
        profile.mark("assemble_resolved_packages", package_started_at);

        enforce_resolution_supply_chain_guards(
            &snapshot.resolutions,
            &snapshot.root_resolutions,
            &snapshot.metadata,
            manifest,
        )?;

        publication::publish(
            self,
            &prepared.resolved_manifest,
            packages,
            resolution_cache_key.as_deref(),
            &mut profile,
            started_at,
        )
    }
}

#[async_trait]
impl DependencyResolver for WebAdapter {
    fn probe_dependency_resolver(&self) -> MgResult<()> {
        Ok(())
    }

    async fn resolve(&self, manifest: &Manifest) -> MgResult<ResolvedGraph> {
        self.resolve_inner(manifest, true).await
    }

    /// Fresh resolve for bumpers (audit --fix): never short-circuit a locked vulnerable version.
    /// Resolve tươi cho bumper (audit --fix): không short-circuit version lỗi đã lock.
    async fn resolve_fresh(&self, manifest: &Manifest) -> MgResult<ResolvedGraph> {
        self.resolve_inner(manifest, false).await
    }
}
