//! Validated dependency bundle response handler.
//! Handler response dependency bundle đã kiểm tra.

use axum::{
    body::Body,
    extract::{Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use tracing::debug;

use super::super::types::ServerState;
use super::response_payload::{css_injector, missing_dependency_diagnostic};

/// Serve a pre-bundled dependency.
/// Route: GET /@magicore/deps/{*pkg}
/// E.g. /@magicore/deps/react → bundle react from node_modules/react
///      /@magicore/deps/lodash/fp → package subpath
///      /@magicore/deps/@tanstack/react-query → scoped package
/// (Phục vụ dependency đã pre-bundled: /@magicore/deps/{*pkg} — đứng sau
/// cổng validator tên package P1-1.)
pub(in crate::bundler::dev_server::implementation) async fn serve_dep(
    Path(pkg): Path<String>,
    State(state): State<ServerState>,
) -> impl IntoResponse {
    // P1-1 traversal gate (fresh-context review 2026-09-15): the raw
    // route segment was joined onto node_modules — `..%2f..` escaped the
    // project and read outside files. A package name may ONLY be
    // `name` or `@scope/name`, each with an optional package subpath, with
    // every segment a valid npm package-name segment: no `..`, no `.`,
    // no separators beyond `/`, no Windows drive prefix, no empty
    // segment. Fail-closed 403.
    // (Cổng traversal P1-1 cho deps: segment route thô từng được nối
    // vào node_modules — `..%2f..` thoát project đọc file ngoài. Tên
    // package chỉ được là `name` hoặc `@scope/name` (+ subpath thường/scoped), mỗi segment
    // là segment tên npm hợp lệ: không `..`, không `.`, không dấu phân
    // tách ngoài `/`, không prefix ổ đĩa Windows, không segment rỗng.
    // Fail-closed 403.)
    if !crate::bundler::dep_name_is_valid(&pkg) {
        return (StatusCode::FORBIDDEN, "Forbidden: invalid dependency name").into_response();
    }
    debug!(dependency = ?pkg, "deps request");

    match state.deps_cache.get_or_bundle(&pkg).await {
        Some(dep) => {
            // If generated CSS accompanies the dep, inject the loader —
            // P1-3: the CSS rides as a JS STRING LITERAL (JSON-escaped,
            // see js_string_literal) — never inside a template literal.
            // (CSS kèm theo được bơm qua string literal JS (JSON-escape,
            // xem js_string_literal) — không bao giờ trong template
            // literal.)
            let mut js = dep.js.clone();
            // Edition 2024 let-chain keeps the optional CSS check together.
            // Let-chain edition 2024 gộp kiểm tra CSS tùy chọn vào cùng điều kiện.
            if let Some(css) = &dep.css
                && !css.is_empty()
            {
                // Inject CSS via the CSSStyleSheet API (modern browsers)
                // — the payload is DATA inside a JSON string literal.
                // (Bơm CSS qua API CSSStyleSheet — payload là DỮ LIỆU
                // trong JSON string literal.)
                js.push_str(&css_injector(css));
            }

            Response::builder()
                .header(
                    header::CONTENT_TYPE,
                    "application/javascript; charset=utf-8",
                )
                .header(header::CACHE_CONTROL, "public, max-age=31536000, immutable")
                .body(Body::from(js))
                .unwrap_or_else(|_| {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Server error").into_response()
                })
        }
        None => {
            let err_js = missing_dependency_diagnostic(&pkg);
            Response::builder()
                .header(header::CONTENT_TYPE, "application/javascript")
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(err_js))
                .unwrap_or_else(|_| (StatusCode::NOT_FOUND, "").into_response())
        }
    }
}
