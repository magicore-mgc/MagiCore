//! Retrieves and serves a verified registry blob.
//! Lấy và phục vụ blob từ registry.

use super::error::DownloadFileError;
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, header, header::InvalidHeaderValue},
    response::Response,
};

/// Tải wheel/sdist — blob content-addressed (sha256)
pub(in crate::pypi) async fn download_file(
    State((store, auth)): State<AppState>,
    headers: HeaderMap,
    Path((name, filename)): Path<(String, String)>,
) -> Result<Response, DownloadFileError> {
    let package_scope = crate::trusted::scoped_package("pypi", &name)?;
    auth.authorize_package_read_async(&headers, &package_scope)
        .await?;
    let digest = store
        .get_pypi_file_digest(&name, &filename)
        .await
        .map_err(DownloadFileError::StorageRead)?
        .ok_or(DownloadFileError::FileNotFound)?;

    let data = store
        .get_blob(&digest)
        .await
        .map_err(DownloadFileError::StorageRead)?
        .ok_or(DownloadFileError::FileNotFound)?;

    let mut resp = Response::new(axum::body::Body::from(data));
    resp.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("application/octet-stream"),
    );
    let content_disposition =
        attachment_disposition(&filename).map_err(DownloadFileError::InvalidHeaderValue)?;
    resp.headers_mut()
        .insert(header::CONTENT_DISPOSITION, content_disposition);
    Ok(resp)
}

/// Builds a safe ASCII fallback and an RFC 6266 UTF-8 filename parameter.
/// Tạo filename dự phòng ASCII an toàn và tham số UTF-8 theo RFC 6266.
pub(super) fn attachment_disposition(filename: &str) -> Result<HeaderValue, InvalidHeaderValue> {
    let fallback: String = filename
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.' | ' ') {
                character
            } else {
                '_'
            }
        })
        .collect();
    let mut extended = String::with_capacity(filename.len());
    for byte in filename.bytes() {
        if byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'!' | b'#' | b'$' | b'&' | b'+' | b'-' | b'.' | b'^' | b'_' | b'`' | b'|' | b'~'
            )
        {
            extended.push(char::from(byte));
        } else {
            use std::fmt::Write as _;
            let _ = write!(extended, "%{byte:02X}");
        }
    }
    HeaderValue::from_str(&format!(
        "attachment; filename=\"{fallback}\"; filename*=UTF-8''{extended}"
    ))
}
