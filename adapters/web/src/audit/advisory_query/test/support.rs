//! Serve one local HTTP response for advisory-query behavior tests.
//! Phục vụ một HTTP response cục bộ cho test query advisory.

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

pub(super) async fn start_registry(
    status: &'static str,
    body: &'static str,
) -> (String, JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind test registry");
    let address = listener.local_addr().expect("read test registry address");
    let task = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.expect("accept audit request");
        let mut request = Vec::new();
        let mut chunk = [0_u8; 2048];
        loop {
            let count = stream.read(&mut chunk).await.expect("read audit request");
            if count == 0 {
                break;
            }
            request.extend_from_slice(&chunk[..count]);
            if request.windows(4).any(|window| window == b"\r\n\r\n") {
                break;
            }
        }
        let header_end = request
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .map(|position| position + 4)
            .expect("complete test request headers");
        let header_text = String::from_utf8_lossy(&request[..header_end]).into_owned();
        let body_length = header_text
            .lines()
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse::<usize>().expect("valid content length"))
            })
            .expect("JSON request content length");
        while request.len() < header_end + body_length {
            let count = stream
                .read(&mut chunk)
                .await
                .expect("read audit request body");
            if count == 0 {
                break;
            }
            request.extend_from_slice(&chunk[..count]);
        }
        assert_eq!(request.len(), header_end + body_length);
        let response = format!(
            concat!(
                "HTTP/1.1 {}\r\nContent-Type: application/json\r\n",
                "Content-Length: {}\r\nConnection: close\r\n\r\n{}"
            ),
            status,
            body.len(),
            body
        );
        stream
            .write_all(response.as_bytes())
            .await
            .expect("write test response");
        String::from_utf8_lossy(&request).into_owned()
    });
    (format!("http://{address}"), task)
}
