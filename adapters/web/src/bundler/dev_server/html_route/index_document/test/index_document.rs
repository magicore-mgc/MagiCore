//! HTML escaping, fallback and HMR-injection tests.
//! Test escape HTML, dự phòng và chèn HMR.

use super::super::implementation::{html_escape_text, serve_index};
use crate::bundler::DepsCache;
use crate::bundler::dev_server::DevServerConfig;
use crate::bundler::dev_server::implementation::types::ServerState;
use crate::bundler::hmr::HmrManager;
use axum::{body::to_bytes, extract::State, response::IntoResponse};
use std::sync::Arc;

fn server_state(root: std::path::PathBuf) -> ServerState {
    let entry = root.join("src/main.ts");
    ServerState {
        config: Arc::new(DevServerConfig {
            root,
            entry,
            host: "127.0.0.1".to_string(),
            port: 0,
        }),
        hmr_manager: Arc::new(HmrManager::new()),
        deps_cache: DepsCache::default(),
    }
}

#[test]
fn html_text_escape_kills_markup_and_preserves_text_quotes() {
    assert_eq!(
        html_escape_text("<script>alert(1)</script>"),
        "&lt;script&gt;alert(1)&lt;/script&gt;"
    );
    assert_eq!(html_escape_text("a & b"), "a &amp; b");
    assert_eq!(html_escape_text("'\""), "'\"");
}

#[tokio::test]
async fn fallback_title_is_escaped_and_hmr_script_is_added() {
    let temp = tempfile::tempdir().expect("tempdir");
    let root = temp.path().join("<script>alert(1)");
    let response = serve_index(State(server_state(root))).await.into_response();
    let body = to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("fallback response body must be readable");
    let html = std::str::from_utf8(&body).expect("HTML response must be UTF-8");
    let title = html
        .split("<title>")
        .nth(1)
        .and_then(|value| value.split("</title>").next())
        .expect("fallback template must include a title");

    assert_eq!(title, "&lt;script&gt;alert(1)");
    assert!(html.contains("src=\"/src/main.ts\""));
    assert_eq!(html.matches("/@magicore/hmr.js").count(), 1);
}

#[tokio::test]
async fn existing_document_keeps_content_and_gets_hmr_before_head_close() {
    let temp = tempfile::tempdir().expect("tempdir");
    let root = temp.path().join("site");
    std::fs::create_dir_all(&root).expect("site directory");
    std::fs::write(
        root.join("index.html"),
        "<html><head><title>existing</title></head><body>body-marker</body></html>",
    )
    .expect("existing document");
    let response = serve_index(State(server_state(root))).await.into_response();
    assert_eq!(
        response
            .headers()
            .get(axum::http::header::CONTENT_TYPE)
            .expect("HTML content type must be present"),
        "text/html; charset=utf-8"
    );
    let body = to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("index response body must be readable");
    let html = std::str::from_utf8(&body).expect("HTML response must be UTF-8");

    assert!(html.contains("<title>existing</title>"));
    assert!(html.contains("body-marker"));
    assert!(html.contains("</script>\n</head>"));
}

#[cfg(unix)]
#[tokio::test]
async fn external_index_symlink_uses_fallback_without_reading_target() {
    let temp = tempfile::tempdir().expect("tempdir");
    let root = temp.path().join("site");
    std::fs::create_dir_all(&root).expect("site directory");
    let outside = temp.path().join("outside.html");
    std::fs::write(&outside, "INDEX-OUTSIDE-ROOT-SECRET").expect("outside document");
    std::os::unix::fs::symlink(&outside, root.join("index.html")).expect("external symlink");

    let response = serve_index(State(server_state(root))).await.into_response();
    let body = to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("fallback response body must be readable");
    let html = std::str::from_utf8(&body).expect("fallback HTML must be UTF-8");

    assert!(html.contains("<title>site</title>"));
    assert!(!html.contains("INDEX-OUTSIDE-ROOT-SECRET"));
}
