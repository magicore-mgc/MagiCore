//! Hermetic Go module proxy fixture. / Fixture Go module proxy hermetic.

use crate::native_ownership_matrix_e2e::support::write;

/// Hermetic GOPROXY fixture for native Go CLI E2E.
/// Fixture GOPROXY cục bộ cho E2E CLI Go native.
pub(crate) struct HermeticGo {
    pub(crate) _server: mockito::ServerGuard,
    pub(crate) _mocks: Vec<mockito::Mock>,
    pub(crate) url: String,
}

pub(crate) fn build_test_go_module_zip(module: &str, version: &str) -> Vec<u8> {
    use std::io::Write;

    let prefix = format!("{module}@v{version}/");
    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let options =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (path, contents) in [
        (
            format!("{prefix}go.mod"),
            format!("module {module}\n\ngo 1.21\n"),
        ),
        (
            format!("{prefix}uuid.go"),
            "package uuid\n\nfunc New() string { return \"fixture\" }\n".to_string(),
        ),
    ] {
        writer.start_file(path, options).unwrap();
        writer.write_all(contents.as_bytes()).unwrap();
    }
    writer.finish().unwrap().into_inner()
}

impl HermeticGo {
    pub(crate) fn new() -> Self {
        use sha2::{Digest, Sha256};

        let module = "github.com/google/uuid";
        let version = "v1.0.0";
        let mut server = mockito::Server::new();
        let url = server.url();
        let zip = build_test_go_module_zip(module, version.trim_start_matches('v'));
        let ziphash = hex::encode(Sha256::digest(&zip));
        let base = format!("/{module}/@v");
        let mocks = vec![
            server
                .mock("GET", format!("{base}/list").as_str())
                .expect_at_least(1)
                .with_status(200)
                .with_body(format!("{version}\n"))
                .create(),
            server
                .mock("GET", format!("{base}/{version}.info").as_str())
                .expect_at_least(1)
                .with_status(200)
                .with_body(format!(
                    "{{\"Version\":\"{version}\",\"Time\":\"2024-01-01T00:00:00Z\"}}"
                ))
                .create(),
            server
                .mock("GET", format!("{base}/{version}.mod").as_str())
                .expect_at_least(1)
                .with_status(200)
                .with_body(format!("module {module}\n\ngo 1.21\n"))
                .create(),
            server
                .mock("GET", format!("{base}/{version}.ziphash").as_str())
                .expect_at_least(1)
                .with_status(200)
                .with_body(ziphash)
                .create(),
            server
                .mock("GET", format!("{base}/{version}.zip").as_str())
                .expect_at_least(1)
                .with_status(200)
                .with_body(zip)
                .create(),
        ];
        Self {
            _server: server,
            _mocks: mocks,
            url,
        }
    }

    pub(crate) fn assert_all_routes_used(&self) {
        for mock in &self._mocks {
            mock.assert();
        }
    }

    pub(crate) fn assert_resolution_routes_used(&self) {
        for mock in self._mocks.iter().take(3) {
            mock.assert();
        }
    }
}
