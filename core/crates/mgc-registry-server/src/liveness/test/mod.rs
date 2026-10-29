//! Liveness probe tests — test probe sống.
//! The probe exists so a user reading "is the registry running?" can answer it
//! without replaying a failing publish. It must stay unauthenticated and must not
//! expose registry content.
//! (Probe để người đọc "registry có chạy không?" trả lời được mà không phải chạy
//! lại lệnh hỏng.)

use super::routes;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

/// A throwaway registry state; the probe handler never reads it, but the router
/// type requires it before `with_state`.
/// (State registry tạm; handler probe không đọc nhưng kiểu router cần trước
/// `with_state`.)
async fn test_state() -> (crate::AppState, tempfile::TempDir) {
    let dir = tempfile::tempdir().expect("temp dir");
    let store = std::sync::Arc::new(
        crate::storage::RegistryStore::new(dir.path())
            .await
            .expect("store"),
    );
    let auth = std::sync::Arc::new(crate::auth::AuthService::new(None, store.clone()));
    ((store, auth), dir)
}

#[tokio::test]
async fn ping_answers_without_credentials() {
    let (state, _dir) = test_state().await;
    let response = routes()
        .with_state(state)
        .oneshot(
            Request::builder()
                .uri("/-/ping")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("router answers");

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "ping must not require auth"
    );
}

#[tokio::test]
async fn ping_body_is_an_empty_object() {
    let (state, _dir) = test_state().await;
    let response = routes()
        .with_state(state)
        .oneshot(
            Request::builder()
                .uri("/-/ping")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("router answers");

    let bytes = axum::body::to_bytes(response.into_body(), 1024)
        .await
        .expect("body");
    assert_eq!(&bytes[..], b"{}", "npm clients expect an empty object");
}

#[tokio::test]
async fn only_the_ping_path_is_served() {
    let (state, _dir) = test_state().await;
    let response = routes()
        .with_state(state)
        .oneshot(
            Request::builder()
                .uri("/")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("router answers");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
