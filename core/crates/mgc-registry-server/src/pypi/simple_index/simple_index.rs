//! Serves the PEP 503 HTML package index.
//! Phục vụ package index HTML theo PEP 503.

use super::error::SimpleIndexError;
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Response,
};

/// PEP 503 HTML simple index — pip install --index-url http://host/pypi/simple/
/// (pip cũ gửi Accept JSON nhưng không parse được → luôn trả HTML)
pub(in crate::pypi) async fn simple_index(
    State((store, auth)): State<AppState>,
    headers: HeaderMap,
    Path(name): Path<String>,
) -> Result<Response, SimpleIndexError> {
    let package_scope = crate::trusted::scoped_package("pypi", &name)?;
    auth.authorize_package_read_async(&headers, &package_scope)
        .await?;
    let files = store
        .get_pypi_files(&name)
        .await
        .map_err(SimpleIndexError::StorageRead)?;
    if files.is_empty() {
        return Err(SimpleIndexError::PackageNotFound);
    }

    // PEP 503 HTML — pip cũ (21.x) và mới đều đọc được; pip 21.x gửi Accept JSON
    // nhưng không parse được → chỉ trả HTML cho tương thích tối đa
    let mut links = String::new();
    for f in &files {
        let requires = f
            .requires_python
            .as_deref()
            .map(|r| format!(" data-requires-python=\"{}\"", escape_html(r)))
            .unwrap_or_default();
        let package_path = encode_path_segment(&f.name);
        let filename_path = encode_path_segment(&f.filename);
        links.push_str(&format!(
            "<a href=\"/pypi/packages/{}/{}\"{}>{}</a><br/>\n",
            escape_html(&package_path),
            escape_html(&filename_path),
            requires,
            escape_html(&f.filename)
        ));
    }
    let html = format!(
        "<!DOCTYPE html><html><body><h1>Links for {}</h1>\n{}</body></html>",
        escape_html(&name),
        links
    );
    let mut resp = Response::new(axum::body::Body::from(html));
    resp.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("text/html; charset=utf-8"),
    );
    Ok(resp)
}
/// Escape untrusted text before placing it in an HTML node or quoted attribute.
/// Escape dữ liệu không tin cậy trước khi đưa vào node HTML hoặc thuộc tính có quote.
fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#x27;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}
/// Percent-encode one UTF-8 path segment, preserving only RFC 3986 unreserved bytes.
/// Mã hóa percent một segment UTF-8, chỉ giữ nguyên byte unreserved theo RFC 3986.
fn encode_path_segment(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            encoded.push(char::from(byte));
        } else {
            use std::fmt::Write as _;
            let _ = write!(encoded, "%{byte:02X}");
        }
    }
    encoded
}
