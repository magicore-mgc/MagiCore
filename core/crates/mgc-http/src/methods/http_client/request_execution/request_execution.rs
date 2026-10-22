//! Executes HTTP requests with the existing retry and rate-limit policy.
//! Thực thi HTTP request theo policy retry và rate-limit hiện tại.

use super::super::http_client::HttpClient;
use super::error::RequestExecutionError;
use anyhow::Result;
use reqwest::{RequestBuilder, Response, StatusCode};

impl HttpClient {
    /// Execute request with retry and rate limit.
    /// Thực thi request với retry và rate limit.
    pub async fn execute(&self, req: RequestBuilder) -> Result<Response> {
        // Rate limit once before the retry loop.
        // Chờ rate limit một lần trước vòng lặp retry.
        if let Some(ref rl) = self.ratelimit {
            rl.wait().await;
        }

        // The current policy allows three retries after the first send (four sends total).
        // Policy hiện tại cho ba lần retry sau lần gửi đầu (tổng cộng bốn lần gửi).
        const MAX_RETRIES: u32 = 3;
        let mut attempt = 0;
        loop {
            let mut req = req
                .try_clone()
                .ok_or_else(|| anyhow::Error::new(RequestExecutionError::RequestNotCloneable))?;
            if let Some((name, value)) = &self.auth {
                req = req.header(name, value);
            }
            let resp = req.send().await;

            match resp {
                Ok(r) if self.should_retry(r.status()) => {
                    if attempt >= MAX_RETRIES {
                        return Ok(r);
                    }
                    let delay = self.retry.delay(attempt);
                    tracing::warn!("HTTP {} - retry in {:?}", r.status(), delay);
                    tokio::time::sleep(delay).await;
                    attempt += 1;
                    continue;
                }
                Ok(r) => return Ok(r),
                Err(e) if self.is_retryable_error(&e) => {
                    if attempt >= MAX_RETRIES {
                        return Err(e.into());
                    }
                    let delay = self.retry.delay(attempt);
                    let error_kind = if e.is_timeout() {
                        "timeout"
                    } else if e.is_connect() {
                        "connect"
                    } else {
                        "request"
                    };
                    tracing::warn!(error_kind, "HTTP request failed; retry in {:?}", delay);
                    tokio::time::sleep(delay).await;
                    attempt += 1;
                    continue;
                }
                Err(e) => return Err(e.into()),
            }
        }
    }

    fn should_retry(&self, status: StatusCode) -> bool {
        matches!(status.as_u16(), 429 | 500..=599)
    }

    fn is_retryable_error(&self, error: &reqwest::Error) -> bool {
        error.is_timeout() || error.is_connect() || error.is_request()
    }
}

#[cfg(test)]
#[path = "test/request_execution.rs"]
mod tests;
