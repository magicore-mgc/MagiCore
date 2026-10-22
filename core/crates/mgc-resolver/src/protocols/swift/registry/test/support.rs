//! Registry integration fixtures. / Fixture integration registry.

pub(super) async fn mock_server() -> Option<mockito::ServerGuard> {
    match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(listener) => drop(listener),
        Err(error) => panic!(
            "Swift registry mock tests require localhost; refusing to report skipped tests as passing: {error}"
        ),
    }
    Some(mockito::Server::new_async().await)
}

/// Hand-assembled store-method zip (no zip-writer crate) — the archive
/// carries the package's Package.swift at the ROOT (SPM registry shape).
/// Zip method store tự ghép (không crate zip-writer) — archive mang
/// Package.swift của package ở GỐC (shape registry SPM).
pub(super) fn build_archive(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let mut centrals: Vec<Vec<u8>> = Vec::new();
    for (name, data) in entries {
        let local_offset = out.len() as u32;
        let crc = mgc_resolver::protocols::zip_reader::crc32(data);
        out.extend_from_slice(b"PK\x03\x04");
        out.extend_from_slice(&20u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&crc.to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&(name.len() as u16).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(data);

        let mut cd = Vec::new();
        cd.extend_from_slice(b"PK\x01\x02");
        cd.extend_from_slice(&20u16.to_le_bytes());
        cd.extend_from_slice(&20u16.to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes());
        cd.extend_from_slice(&0u32.to_le_bytes());
        cd.extend_from_slice(&crc.to_le_bytes());
        cd.extend_from_slice(&(data.len() as u32).to_le_bytes());
        cd.extend_from_slice(&(data.len() as u32).to_le_bytes());
        cd.extend_from_slice(&(name.len() as u16).to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes());
        cd.extend_from_slice(&0u32.to_le_bytes());
        cd.extend_from_slice(&local_offset.to_le_bytes());
        cd.extend_from_slice(name.as_bytes());
        centrals.push(cd);
    }
    let cd_offset = out.len() as u32;
    for cd in &centrals {
        out.extend_from_slice(cd);
    }
    let cd_size = out.len() as u32 - cd_offset;
    out.extend_from_slice(b"PK\x05\x06");
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&(centrals.len() as u16).to_le_bytes());
    out.extend_from_slice(&(centrals.len() as u16).to_le_bytes());
    out.extend_from_slice(&cd_size.to_le_bytes());
    out.extend_from_slice(&cd_offset.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out
}

pub(super) const LIB_PACKAGE_SWIFT: &str = r#"// swift-tools-version:5.9
let package = Package(
    name: "Lib",
    dependencies: [
        .package(id: "other.lib", from: "2.0.0"),
        .package(url: "https://github.com/example/gitdep.git", exact: "1.0.0")
    ]
)"#;
