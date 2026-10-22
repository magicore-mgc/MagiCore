//! Local chunked HTTP fixture for streaming quota tests.
//! Fixture HTTP chunked cục bộ cho test quota streaming.

use tempfile::TempDir;

pub(super) fn tmp() -> TempDir {
    TempDir::new().unwrap()
}

/// Serve chunked HTTP without Content-Length to exercise streaming quota checks.
/// Phục vụ HTTP chunked không Content-Length để kiểm tra quota theo từng chunk.
pub(super) fn serve_chunked_once(body: &'static [u8]) -> (String, std::thread::JoinHandle<()>) {
    use std::io::{Read, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap_or_else(|error| panic!("AI download tests require localhost: {error}"));
    let port = listener.local_addr().unwrap().port();
    let handle = std::thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut request = [0_u8; 1024];
            let _ = stream.read(&mut request);
            let response = format!(
                "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n{:X}\r\n",
                body.len()
            );
            let _ = stream.write_all(response.as_bytes());
            let _ = stream.write_all(body);
            let _ = stream.write_all(b"\r\n0\r\n\r\n");
        }
    });
    (format!("http://127.0.0.1:{port}/model.bin"), handle)
}
