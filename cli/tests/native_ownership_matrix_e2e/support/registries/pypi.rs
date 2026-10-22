//! Hermetic PyPI registry fixture. / Fixture registry PyPI hermetic.

use crate::native_ownership_matrix_e2e::support::write;

/// Hermetic PyPI registry (mockito): serves JSON metadata + wheels for
/// fake distributions that provably do NOT exist on the real PyPI — a
/// passing test proves every byte came from the fixture, never the
/// network. (Registry PyPI hermetic — package giả, không mạng thật.)
pub(crate) struct HermeticPypi {
    pub(crate) _server: mockito::ServerGuard,
    pub(crate) _mocks: Vec<mockito::Mock>,
    pub(crate) url: String,
}

pub(crate) const PYPI_MOCKS_PER_PACKAGE: usize = 5;

/// Build a minimal pure-python wheel with a CORRECT RECORD (sha256
/// base64url-nopad + sizes, RECORD row self-empty) so the PEP 376
/// verifier accepts it exactly like a registry wheel.
/// (Dựng wheel tối thiểu với RECORD đúng chuẩn.)
pub(crate) fn build_test_wheel(dist: &str, version: &str) -> Vec<u8> {
    use base64::Engine;
    use sha2::{Digest, Sha256};
    use std::io::Write;
    let b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    let dist_info = format!("{dist}-{version}.dist-info");
    let pkg_dir = dist.to_string();
    let files: Vec<(String, Vec<u8>)> = vec![
        (
            format!("{pkg_dir}/__init__.py"),
            b"VALUE = 1\n".to_vec(),
        ),
        (
            format!("{dist_info}/METADATA"),
            format!("Metadata-Version: 2.1\nName: {}\nVersion: {version}\n", dist.replace('_', "-"))
                .into_bytes(),
        ),
        (
            format!("{dist_info}/WHEEL"),
            b"Wheel-Version: 1.0\nGenerator: mgc-hermetic-test\nRoot-Is-Purelib: true\nTag: py3-none-any\n"
                .to_vec(),
        ),
    ];
    let mut record = String::new();
    for (rel, data) in &files {
        let mut hasher = Sha256::new();
        hasher.update(data);
        record.push_str(&format!(
            "{rel},sha256={},{}\n",
            b64.encode(hasher.finalize()),
            data.len()
        ));
    }
    record.push_str(&format!("{dist_info}/RECORD,,\n"));
    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let options =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (rel, data) in &files {
        writer.start_file(rel, options).unwrap();
        writer.write_all(data).unwrap();
    }
    writer
        .start_file(format!("{dist_info}/RECORD"), options)
        .unwrap();
    writer.write_all(record.as_bytes()).unwrap();
    writer.finish().unwrap().into_inner()
}

impl HermeticPypi {
    /// Serve the given (dist, version) fake distributions with valid
    /// wheels + metadata (project + per-version JSON, like the real API).
    pub(crate) fn new(server_pkgs: &[(&str, &str)]) -> Self {
        let mut server = mockito::Server::new();
        let url = server.url();
        let mut mocks = Vec::new();
        for (dist, version) in server_pkgs {
            let wheel = build_test_wheel(dist, version);
            let filename = format!("{dist}-{version}-py3-none-any.whl");
            let wheel_url = format!("{url}/files/{filename}");
            let hex = {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(&wheel);
                hex::encode(hasher.finalize())
            };
            let doc = serde_json::json!({
                "info": { "requires_python": ">=3.8" },
                "releases": { version.to_string(): [{
                    "filename": filename,
                    "url": wheel_url,
                    "packagetype": "bdist_wheel",
                    "digests": { "sha256": hex },
                }] },
            })
            .to_string();
            for path in [
                format!("/pypi/{}/json", dist.replace('_', "-")),
                format!("/pypi/{}/{}/json", dist.replace('_', "-"), version),
                // Underscored twin: normalization must never 404.
                // (Đường gạch dưới dự phòng — chuẩn hóa không được 404.)
                format!("/pypi/{dist}/json"),
                format!("/pypi/{dist}/{version}/json"),
            ] {
                mocks.push(
                    server
                        .mock("GET", path.as_str())
                        .expect_at_least(1)
                        .with_status(200)
                        .with_header("content-type", "application/json")
                        .with_body(doc.clone())
                        .create(),
                );
            }
            mocks.push(
                server
                    .mock("GET", format!("/files/{filename}").as_str())
                    .expect_at_least(1)
                    .with_status(200)
                    .with_header("content-type", "application/octet-stream")
                    .with_body(wheel)
                    .create(),
            );
        }
        Self {
            _server: server,
            _mocks: mocks,
            url,
        }
    }

    pub(crate) fn assert_metadata_routes_used(&self) {
        for package_mocks in self._mocks.chunks(PYPI_MOCKS_PER_PACKAGE) {
            assert_eq!(package_mocks.len(), PYPI_MOCKS_PER_PACKAGE);
            package_mocks[0].assert();
        }
    }

    pub(crate) fn assert_required_routes_used(&self) {
        for package_mocks in self._mocks.chunks(PYPI_MOCKS_PER_PACKAGE) {
            assert_eq!(package_mocks.len(), PYPI_MOCKS_PER_PACKAGE);
            package_mocks[0].assert();
            package_mocks[4].assert();
        }
    }
}
