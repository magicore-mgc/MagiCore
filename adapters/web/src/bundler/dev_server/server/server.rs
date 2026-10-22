//! Public server lifecycle and ordered route registration.
//! Vòng đời server public và đăng ký route theo thứ tự.

use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tracing::info;

use super::super::dependency_route::serve_dep;
use super::super::html_route::{serve_hmr_client, serve_index};
use super::super::source_route::serve_source_or_static;
use super::super::types::{DevServerConfig, ServerState};
use crate::bundler::DepsCache;
use crate::bundler::hmr::{HmrManager, hmr_ws_handler};

/// Native Web development server.
/// Dev server Web native.
pub struct MgDevServer {
    config: DevServerConfig,
}

impl MgDevServer {
    pub fn new(config: DevServerConfig) -> Self {
        Self { config }
    }

    pub async fn serve(&self) -> anyhow::Result<()> {
        let hmr_manager = Arc::new(HmrManager::new());
        // Watcher LIFETIME (P0, vòng-11 — found by the WS HMR evidence
        // test): the watcher binding previously lived INSIDE the
        // `if src_dir.exists() {}` block, so it was DROPPED at the end
        // of that block — before the server even bound its port. HMR
        // production never fired a single event; only tests that kept
        // the watcher in the enclosing scope passed. The watcher must
        // outlive `axum::serve` — hold it in THIS frame for the whole
        // serve() call.
        // (VÒNG ĐỜI watcher (P0 vòng-11 — test bằng chứng WS HMR tìm
        // ra): binding watcher trước đây nằm TRONG khối `if
        // src_dir.exists() {}`, nên bị DROP cuối khối đó — trước cả khi
        // server bind port. HMR production chưa từng fire event nào;
        // chỉ test giữ watcher ở scope ngoài mới pass. Watcher phải
        // sống lâu hơn `axum::serve` — giữ nó trong frame NÀY suốt
        // lời gọi serve().)
        let _watcher = if self.config.root.join("src").exists() {
            hmr_manager.watch_dir(&self.config.root.join("src"))
        } else {
            hmr_manager.watch_dir(&self.config.root)
        };
        if let Err(e) = &_watcher {
            // Non-fatal: a project the watcher cannot attach to still
            // serves — HMR is disabled, plain reload still works.
            // (Không chặn: project watcher không gắn được vẫn serve —
            // HMR tắt, reload thường vẫn chạy.)
            eprintln!(
                "warning: HMR file watcher failed to start ({e}) — \
                 hot reload disabled, serving without it"
            );
        }

        let node_modules = self.config.root.join("node_modules");
        let deps_cache = DepsCache::new(node_modules);

        let state = ServerState {
            config: Arc::new(DevServerConfig {
                root: self.config.root.clone(),
                entry: self.config.entry.clone(),
                host: self.config.host.clone(),
                port: self.config.port,
            }),
            hmr_manager,
            deps_cache,
        };

        let app = Router::new()
            // Home page / Trang chủ.
            .route("/", get(serve_index))
            .route("/index.html", get(serve_index))
            // HMR client and WebSocket / HMR client và WebSocket.
            .route("/@magicore/hmr.js", get(serve_hmr_client))
            .route("/@magicore/hmr", get(hmr_ws_handler))
            // Dependency pre-bundling / pre-bundling dependency: /@magicore/deps/{package}.
            .route("/@magicore/deps/{*pkg}", get(serve_dep))
            // Source files are transpiled on demand / transpile source khi có request.
            .route("/{*path}", get(serve_source_or_static))
            .with_state(state);

        let bind_host = if self.config.host == "localhost" {
            "127.0.0.1"
        } else {
            self.config.host.as_str()
        };
        let bind_addr = format!("{}:{}", bind_host, self.config.port);
        info!(
            "MgDevServer (Native ESM) listening at http://{}:{}",
            self.config.host, self.config.port
        );

        let listener = TcpListener::bind(&bind_addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}
