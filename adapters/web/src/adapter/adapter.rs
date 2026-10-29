//! `adapter.rs` — Web adapter state and capabilities / state và capability của adapter web.
use crate::cache::SharedWebCache;
use crate::provider::NpmDependencyProvider;
use mgc_resolver::Resolver as CoreResolver;
use mgc_store::ContentStore;
use mgc_types::{MgResult, capabilities::Capability};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

pub struct WebAdapter {
    pub registry_url: String,
    pub provider: Arc<NpmDependencyProvider>,
    pub resolver: Arc<CoreResolver>,
    pub store: Option<ContentStore>,
    pub shared_cache: Option<SharedWebCache>,
    pub prefetch_handle: Mutex<Option<tokio::task::JoinHandle<MgResult<u64>>>>,
    pub(super) dedupe_pref: AtomicBool,
    pub(super) existing_versions: Mutex<std::collections::HashMap<String, String>>,
}

impl WebAdapter {
    pub const CAPABILITIES: &'static [Capability] = &[
        Capability::ProjectDetector,
        Capability::ScaffoldProvider,
        Capability::DependencyResolver,
        Capability::LockfileProvider,
        Capability::ArtifactFetcher,
        Capability::ContentStoreProvider,
        Capability::Materializer,
        Capability::LifecycleRunner,
        Capability::AuditProvider,
        // Shared optimizer dispatch (`mgc optimizer`) applies to this core.
        Capability::OptimizerProvider,
    ];
}
