//! security origin.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

// Explicit registry credentials are restricted to the same HTTP origin.
// Token của registry không được gửi sang scheme/host/port khác.
pub(in crate::native::npm_registry) fn registry_origin_matches(
    registry_url: &str,
    request_url: &str,
) -> bool {
    let (Ok(registry), Ok(request)) = (
        reqwest::Url::parse(registry_url),
        reqwest::Url::parse(request_url),
    ) else {
        return false;
    };
    matches!(registry.scheme(), "http" | "https")
        && matches!(request.scheme(), "http" | "https")
        && registry.username().is_empty()
        && request.username().is_empty()
        && registry.password().is_none()
        && request.password().is_none()
        && registry.host_str().is_some()
        && registry.scheme() == request.scheme()
        && registry.host_str() == request.host_str()
        && registry.port_or_known_default() == request.port_or_known_default()
}
