use super::super::CratesProtocol;
use super::{HttpError, HttpResult};

pub(in crate::protocols::crates) async fn get_text(
    protocol: &CratesProtocol,
    url: &str,
) -> HttpResult<String> {
    let (status, bytes) = get_response_body(protocol, url).await?;
    let body = String::from_utf8(bytes).map_err(|error| {
        HttpError::Network(format!(
            "GET {url} returned a non-UTF-8 index body: {error}"
        ))
    })?;
    if !(200..300).contains(&status) {
        return Err(HttpError::Network(format!(
            "GET {url} returned {status}: {body}"
        )));
    }
    Ok(body)
}

pub(in crate::protocols::crates) async fn get_bytes(
    protocol: &CratesProtocol,
    url: &str,
) -> HttpResult<Vec<u8>> {
    let (status, bytes) = get_response_body(protocol, url).await?;
    if !(200..300).contains(&status) {
        return Err(HttpError::Network(format!("GET {url} returned {status}")));
    }
    Ok(bytes)
}

async fn get_response_body(protocol: &CratesProtocol, url: &str) -> HttpResult<(u16, Vec<u8>)> {
    const BODY_READ_RETRIES: u32 = 2;
    for attempt in 0..=BODY_READ_RETRIES {
        let response = protocol
            .client
            .get(url)
            .await
            .map_err(|error| HttpError::Network(format!("GET {url} failed: {error}")))?;
        let status = response.status().as_u16();
        match response.bytes().await {
            Ok(bytes) => return Ok((status, bytes.to_vec())),
            Err(error) if attempt < BODY_READ_RETRIES => {
                let delay = std::time::Duration::from_millis(200 * (1 << attempt));
                tracing::warn!(
                    url,
                    attempt = attempt + 1,
                    "registry response body failed; retrying in {:?}: {}",
                    delay,
                    error
                );
                tokio::time::sleep(delay).await;
            }
            Err(error) => {
                return Err(HttpError::Network(format!(
                    "read {url} response body failed after {} attempts: {error}",
                    BODY_READ_RETRIES + 1
                )));
            }
        }
    }
    unreachable!("bounded response-body retry loop always returns")
}
