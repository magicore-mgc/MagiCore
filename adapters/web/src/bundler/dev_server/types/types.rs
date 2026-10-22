//! Shared server configuration and request state.
//! Cấu hình server và request state dùng chung.

use std::path::PathBuf;
use std::sync::Arc;

use crate::bundler::DepsCache;
use crate::bundler::hmr::HmrManager;

/// Public project and entry configuration for MgDevServer.
/// Cấu hình project và entry public của MgDevServer.
pub struct DevServerConfig {
    pub root: PathBuf,
    pub entry: PathBuf,
    pub host: String,
    pub port: u16,
}

/// Shared state cloned into each Axum request.
/// State dùng chung được clone vào từng request Axum.
#[derive(Clone)]
pub(in crate::bundler::dev_server::implementation) struct ServerState {
    pub(in crate::bundler::dev_server::implementation) config: Arc<DevServerConfig>,
    pub(in crate::bundler::dev_server::implementation) hmr_manager: Arc<HmrManager>,
    /// Pre-bundling cache for node_modules dependencies.
    /// (Cache pre-bundling cho dependency trong node_modules.)
    pub(in crate::bundler::dev_server::implementation) deps_cache: DepsCache,
}

impl axum::extract::FromRef<ServerState> for Arc<HmrManager> {
    fn from_ref(state: &ServerState) -> Self {
        state.hmr_manager.clone()
    }
}
