//! Forwards native dependency graph resolution and capability probing to the Python lane.
//! Chuyển tiếp resolve graph dependency và capability probe sang lane Python.

use crate::adapter::AiAdapter;
use async_trait::async_trait;
use mgc_types::{Manifest, MgResult, ResolvedGraph};

#[async_trait]
impl mgc_types::capabilities::DependencyResolver for AiAdapter {
    fn probe_dependency_resolver(&self) -> MgResult<()> {
        match &self.python_lane {
            Some(py) => py.probe_dependency_resolver(),
            None => Err(mgc_types::capabilities::unsupported_capability(
                "ai",
                "resolve",
                "no pyproject.toml — native PyPI resolution requires an MGC-owned pyproject; foreign lockfiles are unsupported until an explicit migration exists",
            )),
        }
    }

    async fn resolve(&self, manifest: &Manifest) -> MgResult<ResolvedGraph> {
        match &self.python_lane {
            Some(py) => py.resolve(manifest).await,
            None => Err(mgc_types::capabilities::unsupported_capability(
                "ai",
                "resolve",
                "no pyproject.toml — ai has no native registry graph for this project",
            )),
        }
    }
}
