#![allow(clippy::unwrap_used)]
//! HTTP security config tests — ensure timeout/TLS config is actually wired.
//! Kiểm chứng cấu hình bảo mật không bị giữ trong struct rồi bỏ qua.

use mgc_http::{HttpClient, TlsConfig, timeout::TimeoutConfig};
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread::JoinHandle,
    time::Duration,
};

fn local_response(response: &'static str) -> (String, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = [0_u8; 1024];
        let _ = stream.read(&mut request);
        stream.write_all(response.as_bytes()).unwrap();
    });
    (format!("http://{address}/"), server)
}

#[tokio::test]
async fn bounded_identity_client_does_not_follow_redirects() {
    let (url, server) = local_response(
        "HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1:9/redirect-target\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
    );
    let timeout = TimeoutConfig {
        connect: Duration::from_secs(1),
        request: Duration::from_secs(1),
        ..TimeoutConfig::default()
    };
    let client = HttpClient::with_security_no_redirects(&timeout, &TlsConfig::default()).unwrap();

    let (status, body) = client.get_bytes_limited(&url, 128).await.unwrap();
    server.join().unwrap();

    assert_eq!(status, 302);
    assert!(body.is_empty());
}

#[tokio::test]
async fn bounded_identity_client_rejects_oversized_responses() {
    let (url, server) =
        local_response("HTTP/1.1 200 OK\r\nContent-Length: 5\r\nConnection: close\r\n\r\n12345");
    let timeout = TimeoutConfig {
        connect: Duration::from_secs(1),
        request: Duration::from_secs(1),
        ..TimeoutConfig::default()
    };
    let client = HttpClient::with_security_no_redirects(&timeout, &TlsConfig::default()).unwrap();

    let error = client.get_bytes_limited(&url, 4).await.unwrap_err();
    server.join().unwrap();

    assert!(error.to_string().contains("response exceeds byte limit"));
}

#[test]
fn http_client_applies_tls_config_errors() {
    let tls = TlsConfig {
        ca_bundle: Some("/definitely/missing/magicore-ca.pem".to_string()),
        ..TlsConfig::default()
    };

    let err = match HttpClient::with_security(&TimeoutConfig::default(), &tls) {
        Ok(_) => panic!("expected missing CA bundle to fail"),
        Err(err) => err,
    };
    assert!(
        err.to_string().contains("read CA"),
        "unexpected error: {err}"
    );
}

#[tokio::test]
async fn default_client_refuses_cross_host_redirect_before_contacting_target() {
    // SEC-8/G5: the redirect target is a DIFFERENT host than the origin. The
    // default policy must fail the chain, so the target listener never accepts
    // a connection and never sees the Authorization header.
    // (SEC-8/G5: đích redirect khác host gốc. Policy mặc định phải làm chuỗi
    // thất bại, nên listener đích không nhận kết nối và không thấy header.)
    let target = TcpListener::bind("127.0.0.1:0").unwrap();
    let target_address = target.local_addr().unwrap();
    target.set_nonblocking(true).unwrap();

    let origin = TcpListener::bind("127.0.0.1:0").unwrap();
    let origin_address = origin.local_addr().unwrap();
    let body = format!(
        "HTTP/1.1 302 Found\r\nLocation: http://localhost:{}/steal\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        target_address.port()
    );
    let origin_server = std::thread::spawn(move || {
        let (mut stream, _) = origin.accept().unwrap();
        let mut buffer = [0_u8; 4096];
        let _ = stream.read(&mut buffer);
        stream.write_all(body.as_bytes()).unwrap();
    });

    let timeout = TimeoutConfig {
        connect: Duration::from_secs(2),
        request: Duration::from_secs(2),
        ..TimeoutConfig::default()
    };
    let client = HttpClient::with_security(&timeout, &TlsConfig::default())
        .unwrap()
        .with_auth("authorization", "Bearer super-secret-token");

    let error = client
        .get(&format!("http://{origin_address}/package.tgz"))
        .await
        .expect_err("cross-origin redirect must not be followed");
    origin_server.join().unwrap();

    let message = format!("{error:#}");
    assert!(
        message.contains("refusing cross-origin redirect"),
        "unexpected error: {message}"
    );
    assert!(
        !message.contains("super-secret-token"),
        "credential leaked into the error: {message}"
    );
    // The target host must not have received any connection.
    // (Host đích không được nhận bất kỳ kết nối nào.)
    match target.accept() {
        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
        Ok(_) => panic!("redirect target was contacted despite the policy"),
        Err(error) => panic!("unexpected accept error on target: {error}"),
    }
}

#[tokio::test]
async fn default_client_follows_same_host_redirect_and_keeps_credentials() {
    // Same-host redirect is legitimate (path normalization, trailing slash) and
    // must keep working so the deny rule cannot break real registries.
    // (Redirect cùng host là hợp lệ (chuẩn hóa path, dấu / cuối) và phải tiếp
    // tục hoạt động để rule chặn không phá registry thật.)
    let origin = TcpListener::bind("127.0.0.1:0").unwrap();
    let origin_address = origin.local_addr().unwrap();
    let first = format!(
        "HTTP/1.1 302 Found\r\nLocation: http://{origin_address}/final\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
    );
    let origin_server = std::thread::spawn(move || {
        let (mut first_stream, _) = origin.accept().unwrap();
        let mut buffer = [0_u8; 4096];
        let _ = first_stream.read(&mut buffer);
        first_stream.write_all(first.as_bytes()).unwrap();
        let (mut second_stream, _) = origin.accept().unwrap();
        let mut second_buffer = [0_u8; 4096];
        let read = second_stream.read(&mut second_buffer).unwrap_or(0);
        let request = String::from_utf8_lossy(&second_buffer[..read]).to_string();
        second_stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok")
            .unwrap();
        request
    });

    let timeout = TimeoutConfig {
        connect: Duration::from_secs(2),
        request: Duration::from_secs(2),
        ..TimeoutConfig::default()
    };
    let client = HttpClient::with_security(&timeout, &TlsConfig::default())
        .unwrap()
        .with_auth("authorization", "Bearer same-host-token");

    let response = client
        .get(&format!("http://{origin_address}/start"))
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.text().await.unwrap(), "ok");
    let forwarded = origin_server.join().unwrap();
    assert!(
        forwarded.contains("/final"),
        "redirect did not reach the same-host target: {forwarded}"
    );
    assert!(
        forwarded.to_ascii_lowercase().contains("authorization: bearer same-host-token"),
        "same-host redirect lost credentials: {forwarded}"
    );
}
