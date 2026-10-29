//! SEC-9 download tests: HTTP policy routing, byte cap, partial-file cleanup.
//! Test tải SEC-9: đi qua policy HTTP, giới hạn byte, dọn file dở.

#![allow(clippy::unwrap_used)]

use mgc_fetcher::download::download_with_progress;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::Mutex;
use std::thread::JoinHandle;
use std::time::Duration;

/// Environment variable read by `download_with_progress` for the cap.
/// Biến môi trường mà `download_with_progress` đọc để lấy giới hạn.
const CAP_ENV: &str = "MAGICORE_MAX_DOWNLOAD_BYTES";

/// The environment is process-global; serialize tests that mutate it.
/// Environment là toàn cục process; tuần tự hóa test có thay đổi biến.
static ENV_LOCK: Mutex<()> = Mutex::new(());

/// Serve one canned raw HTTP response bytes and return the captured request.
/// Phục vụ một response HTTP thô cố định và trả lại request đã nhận.
fn serve_once(response: Vec<u8>) -> (String, JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut request = Vec::new();
        let mut buffer = [0_u8; 2048];
        while !request.windows(4).any(|window| window == b"\r\n\r\n") {
            match stream.read(&mut buffer) {
                Ok(0) | Err(_) => break,
                Ok(read) => request.extend_from_slice(&buffer[..read]),
            }
        }
        stream.write_all(&response).unwrap();
        stream.flush().unwrap();
        String::from_utf8_lossy(&request).to_string()
    });
    (format!("http://{address}/pkg.tgz"), server)
}

fn http_response(head: String, body: &[u8]) -> Vec<u8> {
    let mut bytes = head.into_bytes();
    bytes.extend_from_slice(body);
    bytes
}

fn temp_dest(name: &str) -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let dest = dir.path().join(name);
    (dir, dest)
}

/// Run `download_with_progress` with a temporary cap override.
/// Chạy `download_with_progress` với giới hạn ghi đè tạm thời.
async fn download_under_cap(
    url: &str,
    dest: &std::path::Path,
    cap: u64,
) -> anyhow::Result<()> {
    let _guard = ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let previous = std::env::var_os(CAP_ENV);
    // SAFETY: test-only mutation of process-local state while `ENV_LOCK` is
    // held, so no sibling test thread reads the environment concurrently.
    // AN TOÀN: chỉ test ghi state cục bộ process khi đang giữ `ENV_LOCK` nên
    // thread test khác không đọc env đồng thời.
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var(CAP_ENV, cap.to_string())
    };
    let result = download_with_progress(url, dest, None).await;
    match previous {
        Some(value) =>
        // SAFETY: restore under the same lock.
        // AN TOÀN: khôi phục dưới cùng lock.
        {
            #[allow(unsafe_code)]
            unsafe {
                std::env::set_var(CAP_ENV, value)
            };
        }
        None =>
        // SAFETY: restore under the same lock.
        // AN TOÀN: khôi phục dưới cùng lock.
        {
            #[allow(unsafe_code)]
            unsafe {
                std::env::remove_var(CAP_ENV)
            };
        }
    }
    result
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn download_writes_the_body_under_the_cap() {
    let (_dir, dest) = temp_dest("pkg.tgz");
    let response = http_response(
        "HTTP/1.1 200 OK\r\nContent-Length: 11\r\nConnection: close\r\n\r\n".to_string(),
        b"hello world",
    );
    let (url, server) = serve_once(response);
    download_under_cap(&url, &dest, 1024).await.unwrap();
    let request = server.join().unwrap();
    assert!(request.starts_with("GET /pkg.tgz HTTP/1.1"), "{request}");
    assert_eq!(std::fs::read(&dest).unwrap(), b"hello world");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn declared_oversize_body_is_rejected_without_creating_the_file() {
    let (_dir, dest) = temp_dest("oversize.tgz");
    // Declare a body far above the cap; not a single body byte is served.
    // (Khai báo body vượt xa giới hạn; không phục vụ byte body nào.)
    let response = http_response(
        "HTTP/1.1 200 OK\r\nContent-Length: 4096\r\nConnection: close\r\n\r\n".to_string(),
        b"",
    );
    let (url, server) = serve_once(response);
    let error = download_under_cap(&url, &dest, 64).await.unwrap_err();
    server.join().unwrap();
    assert!(
        format!("{error:#}").contains("exceeds the 64 byte limit"),
        "unexpected error: {error:#}"
    );
    assert!(!dest.exists(), "oversized file must not be created");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn streamed_body_over_the_cap_is_aborted_and_deleted() {
    // No Content-Length: the cap can only be enforced mid-stream, and the
    // partially written file must be removed when the abort happens.
    // (Không có Content-Length: chỉ chặn được giữa stream, và file đang ghi dở
    // phải bị xóa khi hủy.)
    let (_dir, dest) = temp_dest("stream-oversize.tgz");
    let body = vec![b'a'; 4096];
    let mut head = "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n"
        .to_string();
    for chunk in body.chunks(1024) {
        head.push_str(&format!("{:x}\r\n", chunk.len()));
        head.push_str(std::str::from_utf8(chunk).unwrap());
        head.push_str("\r\n");
    }
    head.push_str("0\r\n\r\n");
    let response = http_response(head, b"");
    let (url, server) = serve_once(response);
    let error = download_under_cap(&url, &dest, 2048).await.unwrap_err();
    server.join().unwrap();
    let message = format!("{error:#}");
    assert!(
        message.contains("exceeded the 2048 byte limit"),
        "unexpected error: {message}"
    );
    assert!(!dest.exists(), "partial file must be removed on abort");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn non_success_status_is_reported_and_leaves_no_file() {
    let (_dir, dest) = temp_dest("missing.tgz");
    let response = http_response(
        "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_string(),
        b"",
    );
    let (url, server) = serve_once(response);
    let error = download_under_cap(&url, &dest, 1024).await.unwrap_err();
    server.join().unwrap();
    assert!(
        format!("{error:#}").contains("404"),
        "unexpected error: {error:#}"
    );
    assert!(!dest.exists());
}
