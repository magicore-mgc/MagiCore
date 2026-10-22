//! Shared fixtures and test prelude for Lib installation owners.
//! Fixture và prelude dùng chung cho test install Lib.

#![allow(dead_code, unused_imports)] // This fixture prelude is included by multiple focused test owners.

pub(super) use crate::install::{
    complete_lock_packages_from_existing, install_rust_cached, native_python_path_entries,
    native_python_path_entries_from_store, preflight_rust_artifact_cache, python_site_dirname,
    read_cached_python_artifact, read_cached_rust_artifact, read_existing_lock, run_install,
    select_python_packages_for_owner, validate_lock_coverage, validate_python_importable_graph,
    write_canonical_lock, write_canonical_lock_with_roots,
};
pub(super) use mgc_lockfile::{EcosystemTag, Lockfile, Package, parser, writer};
pub(super) use mgc_types::adapter::InstallOptions;
pub(super) use mgc_types::{PackageId, PackageName, ResolvedGraph, ResolvedPackage, Version};
pub(super) use sha2::Digest;
pub(super) use std::io::{Read, Write};

pub(super) fn package(name: &str, version: &str, ecosystem: EcosystemTag) -> Package {
    Package {
        name: name.to_string(),
        version: version.to_string(),
        ecosystem,
        registry: (ecosystem != EcosystemTag::Other).then(|| format!("test://{ecosystem}")),
        ..Package::default()
    }
}

pub(super) fn resolved(name: &str, version: &str) -> ResolvedPackage {
    ResolvedPackage {
        id: PackageId::new(
            PackageName::new(name).unwrap(),
            Version::parse(version).unwrap(),
        ),
        integrity: "sha256-test".to_string(),
        tarball_url: "https://example.invalid/package.tgz".to_string(),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    }
}

pub(super) fn resolved_rust(name: &str, version: &str, archive: &[u8]) -> ResolvedPackage {
    ResolvedPackage {
        integrity: format!("sha256-{}", hex::encode(sha2::Sha256::digest(archive))),
        tarball_url: format!("https://example.invalid/{name}-{version}.crate"),
        ..resolved(name, version)
    }
}

pub(super) fn test_crate_archive(name: &str, version: &str) -> Vec<u8> {
    let manifest =
        format!("[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2021\"\n");
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut archive = tar::Builder::new(encoder);
    let path = format!("{name}-{version}/Cargo.toml");
    let mut header = tar::Header::new_gnu();
    header.set_size(manifest.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    archive
        .append_data(&mut header, path, manifest.as_bytes())
        .unwrap();
    archive.into_inner().unwrap().finish().unwrap()
}

pub(super) fn python_wheel(
    name: &str,
    version: &str,
    module: &str,
    module_bytes: &[u8],
) -> Vec<u8> {
    use base64::Engine;
    use sha2::{Digest, Sha256};
    let dist_info = format!("{name}-{version}.dist-info");
    let metadata_path = format!("{dist_info}/METADATA");
    let record_path = format!("{dist_info}/RECORD");
    let metadata = format!("Name: {name}\nVersion: {version}\n");
    let files = [
        (module, module_bytes),
        (metadata_path.as_str(), metadata.as_bytes()),
    ];
    let b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    let mut record = String::new();
    for (path, bytes) in files {
        record.push_str(&format!(
            "{path},sha256={},{}\n",
            b64.encode(Sha256::digest(bytes)),
            bytes.len()
        ));
    }
    record.push_str(&format!("{record_path},,\n"));
    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let options =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (path, bytes) in files {
        writer.start_file(path, options).unwrap();
        writer.write_all(bytes).unwrap();
    }
    writer.start_file(record_path, options).unwrap();
    writer.write_all(record.as_bytes()).unwrap();
    writer.finish().unwrap().into_inner()
}

pub(super) fn v4_lock_package(name: &str, version: &str) -> mgc_lockfile::Package {
    mgc_lockfile::Package {
        name: name.to_string(),
        version: version.to_string(),
        ecosystem: EcosystemTag::Python,
        resolved: format!("https://pypi.org/packages/{name}-{version}.whl"),
        integrity: "sha512-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=="
            .to_string(),
        registry: Some("https://pypi.org/simple".to_string()),
        ..Default::default()
    }
}
