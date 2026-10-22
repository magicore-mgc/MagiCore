//! Shared constructors for Cargo compatibility behavior tests.
//! Hàm dựng dữ liệu dùng chung cho test Cargo compatibility.

use mgc_lockfile::{EcosystemTag, Package};

pub(in crate::install::cargo_compat) fn locked_package(
    name: &str,
    dependencies: &[&str],
) -> Package {
    Package {
        owner_core: Some("lib".to_string()),
        name: name.to_string(),
        version: "1.0.0".to_string(),
        integrity: format!("sha256-{}", "ab".repeat(32)),
        dependencies: dependencies
            .iter()
            .map(|name| (*name).to_string())
            .collect(),
        ecosystem: EcosystemTag::Rust,
        registry: Some("crates://sparse+https://index.crates.io".to_string()),
        ..Package::default()
    }
}

pub(in crate::install::cargo_compat) fn crate_archive(name: &str, version: &str) -> Vec<u8> {
    let root = format!("{name}-{version}");
    let manifest =
        format!("[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2021\"\n");
    let source = "pub fn fixture() {}\n";
    crate_archive_files(
        &root,
        &[
            ("Cargo.toml", manifest.as_bytes()),
            ("src/lib.rs", source.as_bytes()),
        ],
    )
}

pub(in crate::install::cargo_compat) fn crate_archive_with_readme_case_aliases(
    upper: &[u8],
    lower: &[u8],
) -> Vec<u8> {
    let manifest = b"[package]\nname = \"fixture\"\nversion = \"1.0.0\"\nedition = \"2021\"\n";
    let source = b"pub fn fixture() {}\n";
    let archive_root = "fixture-1.0.0";
    let archive_files = &[
        ("Cargo.toml", manifest.as_slice()),
        ("src/lib.rs", source.as_slice()),
        ("README.md", upper),
        ("readme.md", lower),
    ];
    crate_archive_files(archive_root, archive_files)
}

pub(in crate::install::cargo_compat) fn crate_archive_files(
    root: &str,
    files: &[(&str, &[u8])],
) -> Vec<u8> {
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut archive = tar::Builder::new(encoder);
    for (relative, contents) in files {
        let path = format!("{root}/{relative}");
        let mut header = tar::Header::new_gnu();
        header.set_size(contents.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        archive.append_data(&mut header, path, *contents).unwrap();
    }
    archive.into_inner().unwrap().finish().unwrap()
}
