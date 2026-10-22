//! Shared mock-server and Rust manifest fixtures for Lib engine tests.
//! Fixture Rust và mock server dùng chung cho test engine Lib.

use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};

pub(crate) async fn mock_server() -> Option<mockito::ServerGuard> {
    match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(listener) => drop(listener),
        Err(error) => panic!(
            "adapter wiring tests require localhost; refusing to report skipped tests as passing: {error}"
        ),
    }
    Some(mockito::Server::new_async().await)
}

pub(crate) fn crate_line(name: &str, vers: &str, cksum: &str, deps: &str) -> String {
    format!(
        r#"{{"name":"{name}","vers":"{vers}","deps":[{deps}],"cksum":"sha256:{cksum}","features":{{}},"yanked":false,"links":null}}"#
    )
}

pub(crate) fn rust_manifest() -> Manifest {
    let mut manifest = Manifest::new("demo-lib", Ecosystem::Lib);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("serde").unwrap(),
            VersionRange::parse("^1.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    manifest
}
