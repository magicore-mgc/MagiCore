//! Source transpilation and compilation-cache orchestration.
//! Điều phối transpile source và compiled cache.

use std::path::Path;

use axum::{
    body::Body,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use tracing::{debug, error};

use super::super::compile_options::canonical_compile_options;
use super::super::compiled_cache::{cached_raw_js, save_raw_js};
use super::super::compiler::{compile_error_body, compile_transpiled_source};
use super::super::imports::rewrite_imports;
use super::super::types::ServerState;

pub(in crate::bundler::dev_server::implementation) async fn serve_transpiled(
    file_path: &Path,
    state: &ServerState,
) -> Response {
    let source = match tokio::fs::read(file_path).await {
        Ok(bytes) => bytes,
        Err(error_value) => {
            error!("Failed to read {}: {}", file_path.display(), error_value);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read file").into_response();
        }
    };

    let source_hash = blake3::Hasher::new()
        .update(&source)
        .finalize()
        .to_hex()
        .to_string();

    let loader = match file_path
        .extension()
        .and_then(|extension| extension.to_str())
        .and_then(mgc_store::cas::Loader::from_extension)
    {
        Some(loader) => loader,
        None => {
            error!(
                "no compilation loader for extension: {}",
                file_path.display()
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, "Unsupported file type").into_response();
        }
    };

    let project_root = state.config.root.clone();
    let options_json = match canonical_compile_options(&project_root, file_path) {
        Ok(options) => options,
        Err(message) => {
            error!("{message}");
            return (StatusCode::INTERNAL_SERVER_ERROR, message.to_string()).into_response();
        }
    };

    let store_root = mgc_store::default_store_root();
    if let Some(cached_js) =
        cached_raw_js(&store_root, &source_hash, loader, &options_json, file_path)
    {
        let js_with_rewrites = rewrite_imports(&cached_js);
        return js_response(js_with_rewrites);
    }

    debug!("transpiling: {}", file_path.display());

    let raw_js = match compile_transpiled_source(file_path, &project_root).await {
        Ok(raw_js) => raw_js,
        Err(diagnostics) => {
            error!("esbuild error for {}: {}", file_path.display(), diagnostics);
            return js_response(compile_error_body(&diagnostics.to_string()));
        }
    };

    let js_with_rewrites = rewrite_imports(&raw_js);
    save_raw_js(store_root, &source_hash, loader, &options_json, raw_js);

    js_response(js_with_rewrites)
}

fn js_response(js: String) -> Response {
    Response::builder()
        .header(
            header::CONTENT_TYPE,
            "application/javascript; charset=utf-8",
        )
        .header(header::CACHE_CONTROL, "no-cache")
        .body(Body::from(js))
        .unwrap_or_else(|_| (StatusCode::INTERNAL_SERVER_ERROR, "").into_response())
}
