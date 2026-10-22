//! Hermetic Maven registry fixture. / Fixture registry Maven hermetic.

/// Hermetic Maven repository metadata, POM, checksum, and artifact fixture.
/// Fixture cục bộ Maven cho E2E Java, không gọi Maven Central.
pub(crate) struct HermeticMaven {
    pub(crate) _server: mockito::ServerGuard,
    pub(crate) _mocks: Vec<mockito::Mock>,
    pub(crate) url: String,
}

impl HermeticMaven {
    pub(crate) fn new() -> Self {
        use sha2::{Digest, Sha256};

        let mut server = mockito::Server::new();
        let url = server.url();
        let jar = b"maven test artifact";
        let checksum = hex::encode(Sha256::digest(jar));
        let base = "/org/apache/commons/commons-lang3";
        let mocks = vec![
            server.mock("GET", format!("{base}/maven-metadata.xml").as_str())
                .expect_at_least(1)
                .with_status(200)
                .with_body("<metadata><versioning><versions><version>3.12.0</version><version>3.13.0</version></versions></versioning></metadata>")
                .create(),
            server.mock("GET", format!("{base}/3.13.0/commons-lang3-3.13.0.pom").as_str())
                .expect_at_least(1)
                .with_status(200)
                .with_body("<project><modelVersion>4.0.0</modelVersion><groupId>org.apache.commons</groupId><artifactId>commons-lang3</artifactId><version>3.13.0</version></project>")
                .create(),
            server.mock("GET", format!("{base}/3.13.0/commons-lang3-3.13.0.jar.sha256").as_str())
                .expect_at_least(1)
                .with_status(200)
                .with_body(checksum)
                .create(),
            server.mock("GET", format!("{base}/3.13.0/commons-lang3-3.13.0.jar").as_str())
                .expect_at_least(1)
                .with_status(200)
                .with_body(jar.as_slice())
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
