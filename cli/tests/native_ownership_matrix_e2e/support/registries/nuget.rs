//! Hermetic NuGet registry fixture. / Fixture registry NuGet hermetic.

use crate::native_ownership_matrix_e2e::support::write;

/// Hermetic NuGet v3 service index, registration, catalog, and package fixture.
/// Fixture cục bộ NuGet v3 cho E2E .NET, không gọi nuget.org.
pub(crate) struct HermeticNuget {
    pub(crate) _server: mockito::ServerGuard,
    pub(crate) _mocks: Vec<mockito::Mock>,
    pub(crate) index_url: String,
}

pub(crate) fn build_test_nuget_package() -> Vec<u8> {
    use std::io::Write;

    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let options =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    writer
        .start_file("Newtonsoft.Json.nuspec", options)
        .unwrap();
    writer
        .write_all(b"<package><metadata><id>Newtonsoft.Json</id><version>13.0.3</version></metadata></package>")
        .unwrap();
    writer
        .start_file("lib/netstandard2.0/Newtonsoft.Json.dll", options)
        .unwrap();
    writer.write_all(b"test fixture assembly").unwrap();
    writer.finish().unwrap().into_inner()
}

impl HermeticNuget {
    pub(crate) fn new() -> Self {
        use base64::Engine as _;
        use sha2::{Digest, Sha512};

        let mut server = mockito::Server::new();
        let base = server.url();
        let index_url = format!("{base}/v3/index.json");
        let package = build_test_nuget_package();
        let package_hash =
            base64::engine::general_purpose::STANDARD.encode(Sha512::digest(&package));
        let resources = serde_json::json!({
            "resources": [
                {"@id": format!("{base}/registration/"), "@type": "RegistrationsBaseUrl/3.6.0"},
                {"@id": format!("{base}/flat/"), "@type": "PackageBaseAddress/3.0.0"}
            ]
        });
        let catalog_url = format!("{base}/catalog/newtonsoft.json.13.0.3.json");
        let registration = serde_json::json!({
            "items": [{
                "items": [{
                    "catalogEntry": {
                        "@id": catalog_url,
                        "id": "Newtonsoft.Json",
                        "version": "13.0.3",
                        "listed": true
                    }
                }]
            }]
        });
        let mocks = vec![
            server
                .mock("GET", "/v3/index.json")
                .expect_at_least(1)
                .with_status(200)
                .with_body(resources.to_string())
                .create(),
            server
                .mock("GET", "/flat/newtonsoft.json/index.json")
                .expect_at_least(1)
                .with_status(200)
                .with_body(r#"{"versions":["13.0.3"]}"#)
                .create(),
            server
                .mock("GET", "/registration/newtonsoft.json/index.json")
                .expect_at_least(1)
                .with_status(200)
                .with_body(registration.to_string())
                .create(),
            server
                .mock("GET", "/catalog/newtonsoft.json.13.0.3.json")
                .expect_at_least(1)
                .with_status(200)
                .with_body(
                    serde_json::json!({
                        "packageHash": package_hash,
                        "packageHashAlgorithm": "SHA512"
                    })
                    .to_string(),
                )
                .create(),
            server
                .mock("GET", "/flat/newtonsoft.json/13.0.3/newtonsoft.json.nuspec")
                .expect_at_least(1)
                .with_status(200)
                .with_body("<package><metadata><dependencies /></metadata></package>")
                .create(),
            server
                .mock(
                    "GET",
                    "/flat/newtonsoft.json/13.0.3/newtonsoft.json.13.0.3.nupkg",
                )
                .expect_at_least(1)
                .with_status(200)
                .with_body(package)
                .create(),
        ];
        Self {
            _server: server,
            _mocks: mocks,
            index_url,
        }
    }

    pub(crate) fn assert_all_routes_used(&self) {
        for mock in &self._mocks {
            mock.assert();
        }
    }

    pub(crate) fn assert_resolution_routes_used(&self) {
        for mock in self._mocks.iter().take(5) {
            mock.assert();
        }
    }
}
