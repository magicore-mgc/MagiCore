//! Hermetic Pub registry fixture. / Fixture registry Pub hermetic.

use crate::native_ownership_matrix_e2e::support::write;

/// Hermetic pub.dev package API and archive fixture for Flutter E2E.
/// Fixture API + archive pub.dev cục bộ cho E2E Flutter.
pub(crate) struct HermeticPub {
    pub(crate) _server: mockito::ServerGuard,
    pub(crate) _mocks: Vec<mockito::Mock>,
    pub(crate) url: String,
}

pub(crate) fn build_test_pub_archive(name: &str, version: &str) -> Vec<u8> {
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut archive = tar::Builder::new(encoder);
    for (path, contents) in [
        (
            "pubspec.yaml".to_string(),
            format!("name: {name}\nversion: {version}\nenvironment:\n  sdk: '>=3.0.0 <4.0.0'\n"),
        ),
        (
            format!("lib/{name}.dart"),
            "const fixture = true;\n".to_string(),
        ),
    ] {
        let mut header = tar::Header::new_gnu();
        header.set_size(contents.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        archive
            .append_data(&mut header, path, contents.as_bytes())
            .expect("append fixture pub package file");
    }
    archive
        .into_inner()
        .expect("finish fixture pub tar")
        .finish()
        .expect("finish fixture pub gzip")
}

impl HermeticPub {
    pub(crate) fn new(name: &str, version: &str) -> Self {
        use sha2::{Digest, Sha256};

        let mut server = mockito::Server::new();
        let url = server.url();
        let archive = build_test_pub_archive(name, version);
        let archive_hash = hex::encode(Sha256::digest(&archive));
        let archive_path = format!("/archives/{name}-{version}.tar.gz");
        let package_doc = serde_json::json!({
            "versions": [{
                "version": version,
                "archive_url": format!("{url}{archive_path}"),
                "archive_sha256": archive_hash,
                "pubspec": {
                    "environment": {"sdk": ">=3.0.0 <4.0.0"},
                    "dependencies": {}
                }
            }]
        });
        let mocks = vec![
            server
                .mock("GET", format!("/api/packages/{name}").as_str())
                .expect_at_least(1)
                .with_status(200)
                .with_body(package_doc.to_string())
                .create(),
            server
                .mock("GET", archive_path.as_str())
                .expect_at_least(1)
                .with_status(200)
                .with_body(archive)
                .create(),
        ];
        Self {
            _server: server,
            _mocks: mocks,
            url,
        }
    }

    pub(crate) fn assert_metadata_route_used(&self) {
        self._mocks[0].assert();
    }

    pub(crate) fn assert_all_routes_used(&self) {
        for mock in &self._mocks {
            mock.assert();
        }
    }
}
