//! Cargo projection tests — kiểm thử projection Cargo.

use super::*;
use mgc_types::adapter::InstallOptions;
use mgc_types::{PackageId, PackageName, ResolvedGraph, ResolvedPackage, Version};
use sha2::Digest;

fn locked_package(name: &str, dependencies: &[&str]) -> Package {
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

#[test]
fn cargo_projection_contains_root_transitive_edges_and_checksums() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[dependencies]\nfixture = \"1\"\n",
    )
    .unwrap();
    let fixture = locked_package("fixture", &["child"]);
    let child = locked_package("child", &[]);
    let packages = vec![&fixture, &child];

    let bytes = project_cargo_lock(project.path(), &packages).unwrap();
    let document: toml::Value = toml::from_str(std::str::from_utf8(&bytes).unwrap()).unwrap();
    let rows = document["package"].as_array().unwrap();
    assert_eq!(rows.len(), 3);
    let fixture_row = rows
        .iter()
        .find(|row| row["name"].as_str() == Some("fixture"))
        .unwrap();
    let fixture_checksum = "ab".repeat(32);
    assert_eq!(
        fixture_row["checksum"].as_str(),
        Some(fixture_checksum.as_str())
    );
    assert_eq!(fixture_row["dependencies"][0].as_str(), Some("child"));
    assert!(rows.iter().any(|row| row["name"].as_str() == Some("demo")));

    // V4 SRI and native hex must yield byte-identical Cargo projections.
    // Toolchain Cargo vẫn nhận checksum hex khi mgc.lock V4 lưu SRI base64.
    use base64::Engine as _;
    let mut sri_fixture = fixture.clone();
    let mut sri_child = child.clone();
    for package in [&mut sri_fixture, &mut sri_child] {
        package.integrity = format!(
            "sha256-{}",
            base64::engine::general_purpose::STANDARD.encode([0xab; 32])
        );
    }
    let sri_lock = Lockfile {
        packages: vec![sri_fixture, sri_child],
        ..Lockfile::new()
    };
    let sri_packages = rust_packages(&sri_lock).unwrap();
    let sri_bytes = project_cargo_lock(project.path(), &sri_packages).unwrap();
    assert_eq!(
        sri_bytes, bytes,
        "digest encoding must not change Cargo.lock bytes"
    );
}

#[test]
fn rust_lock_projection_accepts_sibling_cores_but_lib_build_remains_scoped() {
    let mut game_package = locked_package("mgcfixture", &[]);
    game_package.owner_core = Some("game".to_string());
    let lock = Lockfile {
        packages: vec![game_package],
        ..Lockfile::new()
    };

    let projected = rust_packages(&lock).unwrap();
    assert_eq!(projected.len(), 1);
    let error = validate_lib_build_ownership(&projected).unwrap_err();
    assert!(error.to_string().contains("owned by core 'game'"));

    let lib_package = locked_package("mgcfixture", &[]);
    let lib_lock = Lockfile {
        packages: vec![lib_package],
        ..Lockfile::new()
    };
    validate_lib_build_ownership(&rust_packages(&lib_lock).unwrap()).unwrap();
}

#[test]
fn cargo_projection_rejects_feature_tables_and_ambiguous_versions() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[dependencies]\nfixture = { version = \"1\", features = [\"derive\"] }\n",
    )
    .unwrap();
    let fixture = locked_package("fixture", &[]);
    let error = project_cargo_lock(project.path(), &[&fixture]).unwrap_err();
    assert!(error.to_string().contains("plain version strings"));

    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[dependencies]\nfixture = \"1\"\n",
    )
    .unwrap();
    let mut other = locked_package("fixture", &[]);
    other.version = "2.0.0".to_string();
    let lock = Lockfile {
        packages: vec![fixture.clone(), other],
        ..Lockfile::new()
    };
    let error = rust_packages(&lock).unwrap_err();
    assert!(error.to_string().contains("multiple locked versions"));
}

#[test]
fn empty_cargo_projection_is_accepted_only_when_already_current() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"empty_demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    std::fs::write(project.path().join(".mgc.core"), "lib\n").unwrap();
    super::super::write_canonical_lock(project.path(), EcosystemTag::Rust, Vec::new()).unwrap();
    let expected = super::super::prepare_empty_cargo_lock(project.path(), &[])
        .unwrap()
        .expect("initial empty Cargo.lock projection must be created");
    super::super::atomic_write_cargo_lock(project.path(), &expected).unwrap();

    assert!(validate_build(project.path()).unwrap().is_none());

    std::fs::write(
        project.path().join("Cargo.lock"),
        "version = 4\n\n[[package]]\nname = \"unlocked_extra\"\nversion = \"1.0.0\"\n",
    )
    .unwrap();
    let error = validate_build(project.path()).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("does not match the MGC dependency graph")
    );
}

#[tokio::test]
async fn rust_install_rejects_feature_graph_before_download_or_lock_write() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[dependencies]\nfixture = { version = \"1\", features = [\"derive\"] }\n",
    )
    .unwrap();
    let archive = crate_archive("fixture", "1.0.0");
    let checksum = hex::encode(sha2::Sha256::digest(&archive));
    let package = locked_package("fixture", &[]);
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: PackageId::new(
                PackageName::new("fixture").unwrap(),
                Version::parse("1.0.0").unwrap(),
            ),
            integrity: format!("sha256-{checksum}"),
            tarball_url: "http://127.0.0.1:1/fixture.crate".to_string(),
            deps: Vec::new(),
            peer_deps: Vec::new(),
            direct: true,
            dev: false,
        }],
    };

    let error = super::super::run_install(
        crate::language::LibLanguage::Rust,
        None,
        &graph,
        project.path(),
        InstallOptions::default(),
        None,
        vec![package],
    )
    .await
    .unwrap_err();

    assert!(error.to_string().contains("plain version strings"));
    assert!(!project.path().join("mgc.lock").exists());
    assert!(!project.path().join("Cargo.lock").exists());
}

#[test]
fn cargo_lock_writer_preserves_signed_lock_and_skips_identical_bytes() {
    let project = tempfile::tempdir().unwrap();
    let cargo_lock = project.path().join("Cargo.lock");
    let original = b"signed projection\n";
    std::fs::write(&cargo_lock, original).unwrap();
    let signature = mgc_lockfile::parser::signature_path_for(&cargo_lock);
    std::fs::write(signature, b"signature").unwrap();

    super::super::atomic_write_cargo_lock(project.path(), original).unwrap();
    let error =
        super::super::atomic_write_cargo_lock(project.path(), b"replacement\n").unwrap_err();

    assert!(error.to_string().contains("signature sidecar"));
    assert_eq!(std::fs::read(cargo_lock).unwrap(), original);
}

#[test]
fn rust_package_projection_rejects_path_like_locked_names() {
    let mut package = locked_package("fixture", &[]);
    package.name = "../outside".to_string();
    let lock = Lockfile {
        packages: vec![package],
        ..Lockfile::new()
    };
    let error = rust_packages(&lock).unwrap_err();
    assert!(error.to_string().contains("invalid locked crate name"));
}

#[test]
fn rust_package_projection_accepts_known_edges_but_rejects_unknown_markers() {
    let mut package = locked_package("itoa", &[]);
    package.markers = Some(vec!["optional:no-panic".to_string()]);
    let lock = Lockfile {
        packages: vec![package.clone()],
        ..Lockfile::new()
    };
    assert_eq!(rust_packages(&lock).unwrap().len(), 1);

    package.markers = Some(vec!["target:cfg(any())".to_string()]);
    let lock = Lockfile {
        packages: vec![package.clone()],
        ..Lockfile::new()
    };
    assert_eq!(rust_packages(&lock).unwrap().len(), 1);

    package.markers = Some(vec!["target:cfg(windows)".to_string()]);
    let lock = Lockfile {
        packages: vec![package],
        ..Lockfile::new()
    };
    assert_eq!(rust_packages(&lock).unwrap().len(), 1);

    let mut package = locked_package("itoa", &[]);
    package.markers = Some(vec!["unknown:marker".to_string()]);
    let lock = Lockfile {
        packages: vec![package],
        ..Lockfile::new()
    };
    let error = rust_packages(&lock).unwrap_err();
    assert!(error.to_string().contains("unknown:marker"));
}

#[test]
fn cargo_directory_source_checksum_covers_extracted_files() {
    use sha2::{Digest, Sha256};

    let archive = crate_archive("fixture", "1.0.0");
    let package_checksum = hex::encode(Sha256::digest(&archive));
    let destination = tempfile::tempdir().unwrap();
    mgc_resolver::protocols::archive::extract_crate_tar_gz(
        &archive,
        destination.path(),
        "fixture-1.0.0",
    )
    .unwrap();
    let source = destination.path().join("fixture-1.0.0");

    write_source_checksum(&source, &package_checksum, &archive).unwrap();
    let body: serde_json::Value =
        serde_json::from_slice(&std::fs::read(source.join(".cargo-checksum.json")).unwrap())
            .unwrap();
    assert_eq!(body["package"].as_str(), Some(package_checksum.as_str()));
    assert_eq!(body["files"].as_object().unwrap().len(), 2);
    verify_source_checksum(&source, &package_checksum, &archive).unwrap();

    let changed_source = "pub fn changed() {}\n";
    std::fs::write(source.join("src/lib.rs"), changed_source).unwrap();
    let error = verify_source_checksum(&source, &package_checksum, &archive).unwrap_err();
    assert!(error.to_string().contains("differ from the MGC-verified"));

    // Editing the neighboring checksum map too must not turn modified bytes into trusted source.
    // Sửa tiếp map checksum cạnh đó cũng không biến byte đã đổi thành source đáng tin.
    let mut forged = body;
    forged["files"]["src/lib.rs"] =
        serde_json::Value::String(hex::encode(Sha256::digest(changed_source.as_bytes())));
    std::fs::write(
        source.join(".cargo-checksum.json"),
        serde_json::to_vec(&forged).unwrap(),
    )
    .unwrap();
    let error = verify_source_checksum(&source, &package_checksum, &archive).unwrap_err();
    assert!(error.to_string().contains("differ from the MGC-verified"));
}

fn crate_archive(name: &str, version: &str) -> Vec<u8> {
    let root = format!("{name}-{version}");
    let manifest =
        format!("[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2021\"\n");
    let source = "pub fn fixture() {}\n";
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut archive = tar::Builder::new(encoder);
    for (path, contents) in [
        (format!("{root}/Cargo.toml"), manifest.as_bytes()),
        (format!("{root}/src/lib.rs"), source.as_bytes()),
    ] {
        let mut header = tar::Header::new_gnu();
        header.set_size(contents.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        archive.append_data(&mut header, path, contents).unwrap();
    }
    archive.into_inner().unwrap().finish().unwrap()
}
