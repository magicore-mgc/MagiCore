//! HTTP request builder tests.
//! Test bộ dựng request HTTP.

use super::*;
use reqwest::Method;

fn assert_method_and_url(builder: RequestBuilder, method: Method, url: &str) {
    let request = builder.build().expect("build request without network I/O");
    assert_eq!(request.method(), &method);
    assert_eq!(request.url().as_str(), url);
}

#[test]
fn request_builders_preserve_http_verbs_and_url() {
    let client = HttpClient::new().expect("build local HTTP client");
    let url = "https://registry.example.invalid/packages/test";

    assert_method_and_url(client.get_request(url), Method::GET, url);
    assert_method_and_url(client.put_request(url, vec![1]), Method::PUT, url);
    assert_method_and_url(client.post_request(url, vec![1]), Method::POST, url);
    assert_method_and_url(client.patch_request(url, vec![1]), Method::PATCH, url);
    assert_method_and_url(
        client.patch_with_timeout_request(url, vec![1], Duration::from_secs(1)),
        Method::PATCH,
        url,
    );
    assert_method_and_url(
        client.patch_with_timeout_and_headers_request(
            url,
            vec![1],
            Duration::from_secs(1),
            HeaderMap::new(),
        ),
        Method::PATCH,
        url,
    );
    assert_method_and_url(client.delete_request(url), Method::DELETE, url);
}

#[test]
fn patch_builder_keeps_caller_supplied_headers() {
    let client = HttpClient::new().expect("build local HTTP client");
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-request-tag",
        reqwest::header::HeaderValue::from_static("test-value"),
    );
    let request = client
        .patch_with_timeout_and_headers_request(
            "https://registry.example.invalid/packages/test",
            vec![1],
            Duration::from_secs(1),
            headers,
        )
        .build()
        .expect("build request without network I/O");
    assert_eq!(
        request
            .headers()
            .get("x-request-tag")
            .expect("request header exists"),
        "test-value"
    );
}
