//! Registry URL candidate generation / tạo danh sách khóa từ URL registry.

pub(in crate::auth::resolve_auth) fn registry_candidates(registry_url: &str) -> Vec<String> {
    let parsed = url::Url::parse(registry_url).ok();
    let host = parsed
        .as_ref()
        .map(|url| url.host_str().unwrap_or_default().to_string())
        .unwrap_or_default();

    if let Some(url) = &parsed {
        let host_with_port = url
            .port()
            .map(|port| format!("{}:{}", host, port))
            .unwrap_or_else(|| host.clone());
        let origin = url.as_str().trim_end_matches('/').to_string();
        vec![origin, format!("//{host_with_port}"), host_with_port, host]
    } else {
        vec![host]
    }
}
