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
async fn cross_host_redirect_never_forwards_the_credential() {
    // SEC-8/G5: the redirect target is a DIFFERENT host than the origin.
    //
    // The property that matters is that the credential does not travel, not
    // that the hop is refused: real registries (crates.io -> static CDN) answer
    // with a cross-host redirect, and refusing it made an install impossible.
    // So the target MAY be contacted, and this test proves it never sees the
    // Authorization header.
    // (SEC-8/G5: đích redirect khác host gốc. Thuộc tính cần bảo vệ là
    // credential không đi theo, không phải hop bị từ chối: registry thật
    // (crates.io -> CDN tĩnh) trả redirect khác host, chặn nó làm install không
    // xong. Vì vậy đích CÓ THỂ được gọi tới, và test chứng minh nó không bao
    // giờ thấy header Authorization.)
    let target = TcpListener::bind("127.0.0.1:0").unwrap();
    let target_address = target.local_addr().unwrap();
    target.set_nonblocking(false).unwrap();
    // Whatever the target receives is captured here so the assertion can look
    // for the token instead of assuming no connection.
    // (Mọi thứ đích nhận được ghi lại để assert tìm token, thay vì giả định
    // không có kết nối.)
    let (seen_tx, seen_rx) = std::sync::mpsc::channel::<String>();
    let target_server = std::thread::spawn(move || {
        if let Ok((mut stream, _)) = target.accept() {
            let mut buffer = [0_u8; 4096];
            let read = stream.read(&mut buffer).unwrap_or(0);
            let seen = String::from_utf8_lossy(&buffer[..read]).to_string();
            let _ = seen_tx.send(seen);
            let _ = stream.write_all(
                b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok",
            );
        } else {
            let _ = seen_tx.send(String::new());
        }
    });

    let origin = TcpListener::bind("127.0.0.1:0").unwrap();
    let origin_address = origin.local_addr().unwrap();
    // `localhost` and `127.0.0.1` are different origins to reqwest, which is
    // what makes this a cross-origin hop.
    // (`localhost` và `127.0.0.1` khác origin với reqwest, nên đây là hop khác
    // origin.)
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

    // The result may be Ok (hop followed) or Err (hop refused); both are
    // acceptable. Leaking the token is not.
    // (Kết quả có thể Ok (đi tiếp) hoặc Err (từ chối); cả hai đều được. Rò rỉ
    // token thì không.)
    let outcome = client
        .get(&format!("http://{origin_address}/package.tgz"))
        .await;
    origin_server.join().unwrap();
    let seen = seen_rx
        .recv_timeout(Duration::from_secs(3))
        .unwrap_or_default();
    target_server.join().unwrap();

    assert!(
        !seen.to_ascii_lowercase().contains("super-secret-token"),
        "credential was forwarded across origins: {seen}"
    );
    if let Err(error) = outcome {
        let message = format!("{error:#}");
        assert!(
            !message.contains("super-secret-token"),
            "credential leaked into the error: {message}"
        );
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
