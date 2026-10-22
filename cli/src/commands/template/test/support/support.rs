pub(in crate::commands::template::test) fn archive(
    entries: &[(&str, &[u8], tar::EntryType)],
) -> Vec<u8> {
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut builder = tar::Builder::new(encoder);
    for (path, contents, kind) in entries {
        let mut header = tar::Header::new_gnu();
        header.set_mode(0o644);
        header.set_size(contents.len() as u64);
        header.set_entry_type(*kind);
        // Raw headers intentionally bypass Builder's safe path setter for attack fixtures.
        // Header thô giữ nguyên path độc hại để kiểm boundary đọc archive.
        header.as_mut_bytes()[..path.len()].copy_from_slice(path.as_bytes());
        if kind.is_symlink() || kind.is_hard_link() {
            header.set_link_name("../../outside").unwrap();
        }
        header.set_cksum();
        builder.append(&header, *contents).unwrap();
    }
    builder.into_inner().unwrap().finish().unwrap()
}

pub(in crate::commands::template::test) fn valid_archive() -> Vec<u8> {
    archive(&[
        (
            "package/template.toml",
            b"[[files]]\nsource = 'main.txt'\ntarget = 'main.txt'\n",
            tar::EntryType::Regular,
        ),
        (
            "package/sources/main.txt",
            b"new content",
            tar::EntryType::Regular,
        ),
    ])
}

pub(in crate::commands::template::test) struct HttpFixture {
    pub(in crate::commands::template::test) url: String,
    pub(in crate::commands::template::test) requests: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
    pub(in crate::commands::template::test) stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub(in crate::commands::template::test) worker: Option<std::thread::JoinHandle<()>>,
}

impl HttpFixture {
    pub(in crate::commands::template::test) fn new(
        respond: impl Fn(&str) -> Vec<u8> + Send + 'static,
    ) -> Self {
        use std::io::{Read, Write};
        use std::sync::{
            Arc, Mutex,
            atomic::{AtomicBool, Ordering},
        };
        const PORTS: &[u16] = &[
            5134, 5143, 5314, 5341, 5413, 5431, 4315, 4351, 4135, 4153, 4513, 4531, 3415, 3451,
            3145, 3154, 3541, 3514, 1345, 1354, 1435, 1453, 1534, 1543,
        ];
        let listener = PORTS
            .iter()
            .find_map(|port| {
                std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, *port)).ok()
            })
            .expect("an isolated permitted fixture port must be available");
        let url = format!("http://{}", listener.local_addr().unwrap());
        listener.set_nonblocking(true).unwrap();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let captured = requests.clone();
        let finished = stop.clone();
        let worker = std::thread::spawn(move || {
            while !finished.load(Ordering::Relaxed) {
                let (mut stream, _) = match listener.accept() {
                    Ok(connection) => connection,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(std::time::Duration::from_millis(5));
                        continue;
                    }
                    Err(_) => break,
                };
                // The listener is nonblocking for the accept loop. Explicitly
                // restore blocking mode on each accepted socket: some OSes
                // propagate O_NONBLOCK, which would make the first read return
                // WouldBlock before the HTTP client has written its request.
                stream.set_nonblocking(false).unwrap();
                stream
                    .set_read_timeout(Some(std::time::Duration::from_secs(2)))
                    .unwrap();
                stream
                    .set_write_timeout(Some(std::time::Duration::from_secs(2)))
                    .unwrap();
                let mut request = Vec::new();
                let mut buffer = [0u8; 1024];
                while request.len() < 16 * 1024
                    && !request.windows(4).any(|part| part == b"\r\n\r\n")
                {
                    match stream.read(&mut buffer) {
                        Ok(0) | Err(_) => break,
                        Ok(count) => request.extend_from_slice(&buffer[..count]),
                    }
                }
                if let Some(end) = request.windows(4).position(|part| part == b"\r\n\r\n") {
                    request.truncate(end + 4);
                }
                if request.is_empty() {
                    continue;
                }
                let request = String::from_utf8(request).unwrap();
                captured.lock().unwrap().push(request.clone());
                let _ = stream.write_all(&respond(&request));
            }
        });
        Self {
            url,
            requests,
            stop,
            worker: Some(worker),
        }
    }
}

impl Drop for HttpFixture {
    fn drop(&mut self) {
        self.stop.store(true, std::sync::atomic::Ordering::Relaxed);
        self.worker.take().unwrap().join().unwrap();
    }
}

pub(in crate::commands::template::test) fn http_response(
    status: &str,
    extra_headers: &str,
    body: &[u8],
) -> Vec<u8> {
    let mut response =
        format!("HTTP/1.1 {status}\r\nConnection: close\r\n{extra_headers}\r\n").into_bytes();
    response.extend_from_slice(body);
    response
}

pub(in crate::commands::template::test) fn fixture_client() -> reqwest::Client {
    reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap()
}
