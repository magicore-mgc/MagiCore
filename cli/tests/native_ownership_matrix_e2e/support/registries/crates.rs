//! Hermetic Cargo registry fixture. / Fixture registry Cargo hermetic.

use crate::native_ownership_matrix_e2e::support::write;

/// Hermetic crates sparse index + archive for native Bevy/ESP32 CLI E2E.
/// Index và archive crates cục bộ để E2E Bevy/ESP32 không cần Internet.
pub(crate) struct HermeticCrates {
    pub(crate) _server: mockito::ServerGuard,
    pub(crate) _mocks: Vec<mockito::Mock>,
    pub(crate) index_url: String,
    pub(crate) download_url: String,
}

pub(crate) fn build_test_crate_archive() -> Vec<u8> {
    let name = "mgcfixture";
    let version = "1.0.0";
    let manifest =
        format!("[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2021\"\n");
    let source = "pub fn fixture() {}\n";
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut archive = tar::Builder::new(encoder);
    for (path, contents) in [
        (format!("{name}-{version}/Cargo.toml"), manifest.as_bytes()),
        (format!("{name}-{version}/src/lib.rs"), source.as_bytes()),
    ] {
        let mut header = tar::Header::new_gnu();
        header.set_size(contents.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        archive
            .append_data(&mut header, path, contents)
            .expect("append fixture crate file");
    }
    archive
        .into_inner()
        .expect("finish fixture crate tar")
        .finish()
        .expect("finish fixture crate gzip")
}

impl HermeticCrates {
    pub(crate) fn new() -> Self {
        Self::with_download_expectation(None)
    }

    pub(crate) fn with_download_expectation(download_count: Option<usize>) -> Self {
        use sha2::{Digest, Sha256};

        let mut server = mockito::Server::new();
        let index_url = server.url();
        let download_url = format!("{index_url}/api/v1/crates");
        let archive = build_test_crate_archive();
        let checksum = hex::encode(Sha256::digest(&archive));
        let index_entry = serde_json::json!({
            "name": "mgcfixture",
            "vers": "1.0.0",
            "deps": [],
            "cksum": checksum,
            "features": {},
            "yanked": false,
            "links": null,
        })
        .to_string();
        let index_mock = server
            .mock("GET", "/mg/cf/mgcfixture")
            .expect_at_least(1)
            .with_status(200)
            .with_header("content-type", "application/octet-stream")
            .with_body(index_entry)
            .create();
        let download_builder = server
            .mock("GET", "/api/v1/crates/mgcfixture/1.0.0/download")
            .with_status(200)
            .with_header("content-type", "application/octet-stream")
            .with_body(archive);
        let download_mock = match download_count {
            Some(count) => download_builder.expect(count).create(),
            None => download_builder.expect_at_least(1).create(),
        };
        let mocks = vec![index_mock, download_mock];
        Self {
            _server: server,
            _mocks: mocks,
            index_url,
            download_url,
        }
    }
}
