//! Local-only fixtures shared by download orchestration tests.
//! Fixture cục bộ dùng chung bởi test điều phối download.

use mgc_types::MgResult;
use tempfile::TempDir;

pub(super) fn tmp() -> TempDir {
    TempDir::new().unwrap()
}

/// Helper: block_on cho test đồng bộ.
// (Helper: block_on wrapper for sync tests.)
pub(super) fn block_on<T>(fut: impl std::future::Future<Output = MgResult<T>>) -> T {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(fut)
        .unwrap()
}

/// Server HTTP cục bộ tối giản — trả 1 body cố định rồi đóng.
// (Minimal local HTTP server — serves one fixed body then closes.)
pub(super) fn serve_once(body: &'static [u8]) -> Option<(String, std::thread::JoinHandle<()>)> {
    use std::io::{Read, Write};
    let listener = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(listener) => listener,
        Err(err) => panic!(
            "AI download tests require localhost; refusing to report skipped tests as passing: {err}"
        ),
    };
    let port = listener.local_addr().unwrap().port();
    let handle = std::thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf = [0u8; 1024];
            let _ = stream.read(&mut buf);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            let _ = stream.write_all(response.as_bytes());
            let _ = stream.write_all(body);
        }
    });
    Some((format!("http://127.0.0.1:{port}/model.bin"), handle))
}
