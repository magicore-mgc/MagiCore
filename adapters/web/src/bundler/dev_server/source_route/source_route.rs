//! Project source and static asset response handler.
//! Handler response source project và asset tĩnh.

use axum::{
    body::Body,
    extract::{Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};

use super::super::request_path::request_path_is_safe;
use super::super::source_file_resolution::{SourceFileResolutionError, resolve_source_file};
use super::super::transpile::serve_transpiled;
use super::super::types::ServerState;

/// Serve static files or transpiled sources for requests under the
/// project root — behind the P1-1 traversal gate.
/// (Phục vụ file tĩnh hoặc source transpile cho request dưới root
/// project — sau cổng traversal P1-1.)
pub(in crate::bundler::dev_server::implementation) async fn serve_source_or_static(
    Path(path): Path<String>,
    State(state): State<ServerState>,
) -> impl IntoResponse {
    // Normalize path: strip the leading slash if present.
    // (Chuẩn hoá path: bỏ leading slash nếu có.)
    let rel_path = path.trim_start_matches('/');

    // P1-1 traversal gate — BEFORE any filesystem join. A request whose
    // decoded path escapes the root must fail closed with 403.
    // (Cổng traversal P1-1 — TRƯỚC mọi phép nối filesystem. Request có
    // path decode thoát khỏi root phải fail cứng với 403.)
    if !request_path_is_safe(rel_path) {
        return (
            StatusCode::FORBIDDEN,
            "Forbidden: path escapes the project root",
        )
            .into_response();
    }

    let file_path = match resolve_source_file(&state.config.root, rel_path).await {
        Ok(Some(path)) => path,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, format!("Not found: /{}", rel_path)).into_response();
        }
        Err(SourceFileResolutionError::Forbidden) => {
            return (
                StatusCode::FORBIDDEN,
                "Forbidden: path is private or outside the project root",
            )
                .into_response();
        }
        Err(error) => {
            tracing::error!("Failed to resolve project file: {error}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to resolve project file",
            )
                .into_response();
        }
    };

    // Keep response dispatch tied to the request spelling, as before
    // canonicalization. The resolver separately requires a file symlink
    // to preserve this extension.
    let ext = std::path::Path::new(rel_path)
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("");

    match ext {
        // ─── TypeScript / JSX: transpile on-the-fly ───────────────────────
        "ts" | "tsx" | "jsx" | "js" | "mjs" => serve_transpiled(&file_path, &state).await,
        // ─── CSS: serve directly / phục vụ trực tiếp ──────────────────────
        "css" => match tokio::fs::read_to_string(&file_path).await {
            Ok(content) => Response::builder()
                .header(header::CONTENT_TYPE, "text/css; charset=utf-8")
                .header(header::CACHE_CONTROL, "no-cache")
                .body(Body::from(content))
                .unwrap_or_else(|_| (StatusCode::INTERNAL_SERVER_ERROR, "").into_response()),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read CSS").into_response(),
        },
        // ─── Static assets: serve with the detected MIME type / phục vụ theo MIME ───
        _ => match tokio::fs::read(&file_path).await {
            Ok(bytes) => {
                let mime = mime_guess::from_path(rel_path).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .header(header::CACHE_CONTROL, "public, max-age=86400")
                    .body(Body::from(bytes))
                    .unwrap_or_else(|_| {
                        (StatusCode::INTERNAL_SERVER_ERROR, "Server error").into_response()
                    })
            }
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read file").into_response(),
        },
    }
}
