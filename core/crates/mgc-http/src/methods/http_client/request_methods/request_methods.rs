//! Builds convenience HTTP requests and executes them through HttpClient.
//! Dựng request HTTP tiện ích và chuyển qua HttpClient để thực thi.

use super::super::http_client::HttpClient;
use anyhow::Result;
use reqwest::{RequestBuilder, Response, header::HeaderMap};
use std::time::Duration;

impl HttpClient {
    fn get_request(&self, url: &str) -> RequestBuilder {
        self.client.get(url)
    }

    fn put_request(&self, url: &str, body: Vec<u8>) -> RequestBuilder {
        self.client.put(url).body(body)
    }

    fn post_request(&self, url: &str, body: Vec<u8>) -> RequestBuilder {
        self.client.post(url).body(body)
    }

    fn patch_request(&self, url: &str, body: Vec<u8>) -> RequestBuilder {
        self.client.patch(url).body(body)
    }

    fn patch_with_timeout_request(
        &self,
        url: &str,
        body: Vec<u8>,
        timeout: Duration,
    ) -> RequestBuilder {
        self.client.patch(url).timeout(timeout).body(body)
    }

    fn patch_with_timeout_and_headers_request(
        &self,
        url: &str,
        body: Vec<u8>,
        timeout: Duration,
        headers: HeaderMap,
    ) -> RequestBuilder {
        self.client
            .patch(url)
            .timeout(timeout)
            .headers(headers)
            .body(body)
    }

    fn delete_request(&self, url: &str) -> RequestBuilder {
        self.client.delete(url)
    }

    /// GET a response.
    /// Gửi GET và trả response.
    pub async fn get(&self, url: &str) -> Result<Response> {
        self.execute(self.get_request(url)).await
    }

    pub async fn put(&self, url: &str, body: Vec<u8>) -> Result<Response> {
        self.execute(self.put_request(url, body)).await
    }

    pub async fn post(&self, url: &str, body: Vec<u8>) -> Result<Response> {
        self.execute(self.post_request(url, body)).await
    }

    pub async fn patch(&self, url: &str, body: Vec<u8>) -> Result<Response> {
        self.execute(self.patch_request(url, body)).await
    }

    pub async fn patch_with_timeout(
        &self,
        url: &str,
        body: Vec<u8>,
        timeout: Duration,
    ) -> Result<Response> {
        self.execute(self.patch_with_timeout_request(url, body, timeout))
            .await
    }

    /// Send PATCH with additional headers and a per-request timeout.
    /// Gửi PATCH với header bổ sung và timeout riêng.
    pub async fn patch_with_timeout_and_headers(
        &self,
        url: &str,
        body: Vec<u8>,
        timeout: Duration,
        headers: HeaderMap,
    ) -> Result<Response> {
        self.execute(self.patch_with_timeout_and_headers_request(url, body, timeout, headers))
            .await
    }

    pub async fn delete(&self, url: &str) -> Result<Response> {
        self.execute(self.delete_request(url)).await
    }
}

#[cfg(test)]
#[path = "test/request_methods.rs"]
mod tests;
