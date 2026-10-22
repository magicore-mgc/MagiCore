//! Registry security.
// Các quy tắc xác thực, origin, redirect và scope.

/// 401/403 = auth fail — KHÔNG fallback sang registry khác (fail-closed).
pub(in crate::native::npm_registry) fn is_auth_error(err: &anyhow::Error) -> bool {
    err.chain()
        .find_map(|cause| {
            cause
                .downcast_ref::<reqwest::Error>()
                .and_then(|re| re.status())
        })
        .is_some_and(|status| {
            status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN
        })
}
