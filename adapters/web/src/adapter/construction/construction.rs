//! `construction.rs` — Web adapter constructors / constructor adapter web.
use super::super::WebAdapter;
use crate::cache::SharedWebCache;
use crate::provider::NpmDependencyProvider;
use crate::registry_config::{
    DEFAULT_NPM_REGISTRY, effective_registry_url, validate_registry_allowed,
};
use anyhow::Result;
use mgc_resolver::Resolver as CoreResolver;
use mgc_store::ContentStore;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

impl WebAdapter {
    // P0-4 (2026-09-15): constructors are FALLIBLE now — the registry-URL
    // guards are typed errors (fail-closed kept), so an invalid
    // MAGICORE_WEB_REGISTRY_URL / unallowed registry surfaces as an Err
    // instead of aborting the process.
    // (P0-4: constructor giờ có thể lỗi — guard URL registry là typed
    // error (vẫn fail-closed), URL sai sẽ trả Err thay vì abort process.)
    //
    // The legacy `impl Default` (new() was infallible back then) is gone:
    // a panicking Default would regress P0-4 and a silent-fallback Default
    // would not be fail-closed. Use new()/with_registry() explicitly.
    // (Đã bỏ `impl Default` cũ (thời new() chưa thể lỗi): Default panic
    // làm hồi quy P0-4, Default fallback âm thầm thì mất fail-closed.
    // Dùng new()/with_registry() tường minh.)
    pub fn new() -> Result<Self> {
        let registry_url = effective_registry_url(DEFAULT_NPM_REGISTRY)?;
        let shared_cache = SharedWebCache::discover();
        Ok(Self::build(registry_url, None, Vec::new(), shared_cache))
    }

    pub fn with_registry(registry_url: String) -> Result<Self> {
        let registry_url = effective_registry_url(&registry_url)?;
        let shared_cache = SharedWebCache::discover();
        Ok(Self::build(registry_url, None, Vec::new(), shared_cache))
    }

    /// Build an adapter with a caller-owned shared-cache root.
    /// The root is validated and used directly; process cache environment
    /// discovery cannot override it. This is useful for isolated workspaces.
    ///
    /// Tạo adapter với shared-cache root do caller quản lý.
    /// Root được kiểm tra và dùng trực tiếp; biến môi trường không thể ghi đè.
    pub fn with_registry_and_shared_cache_root(
        registry_url: String,
        shared_root: PathBuf,
    ) -> Result<Self> {
        let registry_url = effective_registry_url(&registry_url)?;
        let shared_cache = SharedWebCache { root: shared_root };
        if !shared_cache.is_usable() {
            return Err(anyhow::anyhow!("shared cache root is not writable"));
        }
        Ok(Self::build(
            registry_url,
            None,
            Vec::new(),
            Some(shared_cache),
        ))
    }

    pub fn with_registry_and_token(registry_url: String, token: Option<String>) -> Result<Self> {
        let registry_url = effective_registry_url(&registry_url)?;
        let shared_cache = SharedWebCache::discover();
        Ok(Self::build(registry_url, token, Vec::new(), shared_cache))
    }

    pub fn with_registry_chain(
        primary: String,
        token: Option<String>,
        fallbacks: Vec<(String, Option<String>)>,
    ) -> Result<Self> {
        let primary = effective_registry_url(&primary)?;
        let shared_cache = SharedWebCache::discover();
        Ok(Self::build(primary, token, fallbacks, shared_cache))
    }

    pub(crate) fn build(
        registry_url: String,
        token: Option<String>,
        fallbacks: Vec<(String, Option<String>)>,
        shared_cache: Option<SharedWebCache>,
    ) -> Self {
        // NOTE (P0/F6): the age gate is armed PER OPERATION from that
        // operation's project (see `arm_age_gate_for`) — never once per
        // process from the cwd. A fresh adapter starts unarmed (no
        // filtering = historical behavior) until its first operation.
        // (Cổng tuổi nạp theo từng operation, không phải một lần cwd.)
        let provider = Arc::new(NpmDependencyProvider::new_with_chain(
            &registry_url,
            token,
            fallbacks,
            shared_cache.clone(),
        ));
        Self::from_provider(registry_url, provider, shared_cache)
    }

    /// Use an already-resolved chain without rereading environment overrides.
    /// Chain typed là cấu hình đã chọn; không đổi endpoint rồi giữ nhầm token cũ.
    pub fn with_scoped_registry_chain(
        entries: Vec<mgc_config::chain::ScopedRegistry>,
    ) -> Result<Self> {
        for entry in &entries {
            let url = url::Url::parse(&entry.registry.url)
                .map_err(|_| anyhow::anyhow!("invalid registry endpoint"))?;
            if url.host_str().is_none()
                || !url.username().is_empty()
                || url.password().is_some()
                || (url.scheme() != "https"
                    && !crate::audit::allow_insecure_loopback_url(&entry.registry.url))
            {
                return Err(anyhow::anyhow!(
                    "registry endpoint must use HTTPS without userinfo"
                ));
            }
            validate_registry_allowed(&entry.registry.url)?;
        }
        let registry_url = entries
            .first()
            .ok_or_else(|| anyhow::anyhow!("no registry configured"))?
            .registry
            .url
            .clone();
        let shared_cache = SharedWebCache::discover();
        let provider = Arc::new(NpmDependencyProvider::new_with_scoped_chain(
            entries,
            shared_cache.clone(),
        )?);
        Ok(Self::from_provider(registry_url, provider, shared_cache))
    }

    fn from_provider(
        registry_url: String,
        provider: Arc<NpmDependencyProvider>,
        shared_cache: Option<SharedWebCache>,
    ) -> Self {
        Self {
            registry_url,
            provider: provider.clone(),
            resolver: Arc::new(CoreResolver::new(provider)),
            store: None,
            shared_cache,
            prefetch_handle: Mutex::new(None),
            dedupe_pref: AtomicBool::new(false),
            existing_versions: Mutex::new(std::collections::HashMap::new()),
        }
    }

    #[cfg(test)]
    pub fn with_registry_and_shared_cache(registry_url: String, shared_root: PathBuf) -> Self {
        Self::build(
            registry_url,
            None,
            Vec::new(),
            Some(SharedWebCache { root: shared_root }),
        )
    }

    pub fn with_store(mut self, store: ContentStore) -> Self {
        self.store = Some(store);
        self
    }
}
