//! Content-Range validation for chunked OCI blob uploads.
//! Validate Content-Range cho upload blob OCI theo chunk.

use axum::http::{HeaderMap, StatusCode};

pub(crate) fn parse_upload_range(headers: &HeaderMap, body_len: usize) -> Result<i64, StatusCode> {
    let value = headers
        .get("content-range")
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::RANGE_NOT_SATISFIABLE)?;
    let (start, end) = value
        .split_once('-')
        .ok_or(StatusCode::RANGE_NOT_SATISFIABLE)?;
    if start.is_empty()
        || end.is_empty()
        || !start.bytes().all(|byte| byte.is_ascii_digit())
        || !end.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(StatusCode::RANGE_NOT_SATISFIABLE);
    }
    let start = start
        .parse::<u64>()
        .map_err(|_| StatusCode::RANGE_NOT_SATISFIABLE)?;
    let end = end
        .parse::<u64>()
        .map_err(|_| StatusCode::RANGE_NOT_SATISFIABLE)?;
    let declared_len = end
        .checked_sub(start)
        .and_then(|range| range.checked_add(1))
        .ok_or(StatusCode::RANGE_NOT_SATISFIABLE)?;
    if declared_len != body_len as u64 {
        return Err(StatusCode::RANGE_NOT_SATISFIABLE);
    }
    i64::try_from(start).map_err(|_| StatusCode::RANGE_NOT_SATISFIABLE)
}
