//! Go module proxy engine tests — hermetic via mockito (no real network).
//! Test engine Go module proxy — hermetic qua mockito (không mạng thật).

#![allow(clippy::unwrap_used)]

use mgc_resolver::protocols::sha256_hex;
use mgc_resolver::protocols::{GoModProtocol, RegistryProtocol};

/// Build a minimal module zip whose layout matches dirhash.HashZip input
/// (entries carry the `{module}@{version}/` prefix). Uses the shared
/// hand-rolled zip shape (store method) — no zip-writer crate.
/// Dựng zip module tối giản với layout đúng input của dirhash.HashZip
/// (entry mang prefix `{module}@{version}/`). Dùng shape zip tự ghép dùng
/// chung (method store) — không crate zip-writer.
fn build_module_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let mut centrals: Vec<Vec<u8>> = Vec::new();
    for (name, data) in entries {
        let local_offset = out.len() as u32;
        let crc = mgc_resolver::protocols::zip_reader::crc32(data);
        out.extend_from_slice(b"PK\x03\x04");
        out.extend_from_slice(&20u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes()); // flags
        out.extend_from_slice(&0u16.to_le_bytes()); // method 0 = store
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
        cd.extend_from_slice(&0u16.to_le_bytes()); // method
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

async fn mock_server() -> Option<mockito::ServerGuard> {
    match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(listener) => drop(listener),
        Err(error) => panic!(
            "Go proxy mock tests require localhost; refusing to report skipped tests as passing: {error}"
        ),
    }
    Some(mockito::Server::new_async().await)
}

async fn mock_go_module(
    server: &mut mockito::ServerGuard,
    name: &str,
    version: &str,
    requirements: &str,
) {
    let info = format!(r#"{{"Version":"v{version}","Time":"2020-01-01T00:00:00Z"}}"#);
    let require_block = if requirements.trim().is_empty() {
        String::new()
    } else {
        format!("require (\n{requirements}\n)\n")
    };
    let gomod = format!("module {name}\n{require_block}");
    for (extension, body) in [("info", info), ("mod", gomod), ("ziphash", "0".repeat(64))] {
        server
            .mock("GET", format!("/{name}/@v/v{version}.{extension}").as_str())
            .with_status(200)
            .with_body(body)
            .create_async()
            .await;
    }
}

#[tokio::test]
async fn go_resolves_graph_with_ziphash_and_materializes_cache() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let zip_bytes = build_module_zip(&[
        (
            "example.com/pkg@v1.2.3/go.mod",
            b"module example.com/pkg\n".as_slice(),
        ),
        ("example.com/pkg@v1.2.3/pkg.go", b"package pkg\n".as_slice()),
    ]);
    let zip_hash = sha256_hex(&zip_bytes);
    let gomod_body = "module example.com/pkg\n\ngo 1.22\n\nrequire (\n\tgithub.com/dep/one v1.0.0\n\tgithub.com/dep/two v0.2.0 // indirect\n)\n\nreplace github.com/dep/one => github.com/dep/one-fork v1.0.0\n\nexclude github.com/dep/two v0.2.0\n";
    let info_body = r#"{"Version":"v1.2.3","Time":"2024-01-02T03:04:05Z"}"#;

    // @v/list with a pseudo-version and a lower version that must lose.
    server
        .mock("GET", "/example.com/pkg/@v/list")
        .with_status(200)
        .with_body("v1.2.3\nv1.2.2\nv0.0.0-20230101000000-abcdef123456\n")
        .create_async()
        .await;
    server
        .mock("GET", "/example.com/pkg/@v/v1.2.3.info")
        .with_status(200)
        .with_body(info_body)
        .create_async()
        .await;
    server
        .mock("GET", "/example.com/pkg/@v/v1.2.3.mod")
        .with_status(200)
        .with_body(gomod_body)
        .create_async()
        .await;
    let ziphash_mock = server
        .mock("GET", "/example.com/pkg/@v/v1.2.3.ziphash")
        .with_status(200)
        .with_body(zip_hash.as_str())
        .create_async()
        .await;
    let zip_mock = server
        .mock("GET", "/example.com/pkg/@v/v1.2.3.zip")
        .with_status(200)
        .with_body(zip_bytes.as_slice())
        .create_async()
        .await;

    let protocol = GoModProtocol::with_sum_base(&server.url(), &server.url());
    let entry = protocol.resolve("example.com/pkg", "1.2.3").await.unwrap();
    assert_eq!(entry.version, "1.2.3");
    // Pinned exact version won over 1.2.2 and the pseudo-version.
    // (Version ghim chính xác thắng 1.2.2 và pseudo-version.)
    assert_eq!(
        entry.deps,
        vec![
            // A dependency's replace/exclude directives do not control the main module graph.
            // replace/exclude trong go.mod của dependency không điều khiển graph của project.
            ("github.com/dep/one".to_string(), "1.0.0".to_string()),
            ("github.com/dep/two".to_string(), "0.2.0".to_string()),
        ]
    );
    assert!(
        entry
            .extra_markers
            .iter()
            .any(|m| m == "indirect:github.com/dep/two"),
        "indirect dep must be recorded: {:?}",
        entry.extra_markers
    );
    assert!(
        !entry
            .extra_markers
            .iter()
            .any(|m| m.starts_with("excluded:"))
    );
    assert!(
        !entry
            .extra_markers
            .iter()
            .any(|m| m.starts_with("replace:"))
    );
    assert!(
        entry
            .extra_markers
            .iter()
            .any(|m| m == "time:2024-01-02T03:04:05Z"),
        "release time marker expected: {:?}",
        entry.extra_markers
    );

    // Download + verify against the proxy ziphash (sha256).
    let files = protocol.download_module(&entry).await.unwrap();
    protocol.verify(&entry, &files.zip).unwrap();
    ziphash_mock.assert_async().await;
    zip_mock.assert_async().await;

    // Materialize into a GOMODCACHE layout — GOPROXY=off readable.
    let cache = tempfile::tempdir().unwrap();
    let zip_path = protocol.materialize(&entry, &files, cache.path()).unwrap();
    assert_eq!(
        zip_path,
        cache
            .path()
            .join("cache/download/example.com/pkg/@v/v1.2.3.zip")
    );
    for f in ["v1.2.3.zip", "v1.2.3.mod", "v1.2.3.info", "v1.2.3.ziphash"] {
        assert!(
            cache
                .path()
                .join("cache/download/example.com/pkg/@v")
                .join(f)
                .is_file(),
            "{f} must be materialized"
        );
    }
    let stored_hash = std::fs::read_to_string(
        cache
            .path()
            .join("cache/download/example.com/pkg/@v/v1.2.3.ziphash"),
    )
    .unwrap();
    // Independent Hash1 vector: sorted per-file SHA-256 lines, then base64 SHA-256.
    // Vector tính riêng theo chuẩn Go; không dùng helper production làm oracle.
    assert_eq!(
        stored_hash.trim(),
        "h1:YJbVDjHSwcdNsZfu8Z8CdZcfdjy+e4gj0Ud51FNHtys="
    );
    assert_ne!(stored_hash.trim(), sha256_hex(&zip_bytes));
    let mut malformed = files.clone();
    malformed.zip = b"not a zip".to_vec();
    let rejected_cache = tempfile::tempdir().unwrap();
    assert!(
        protocol
            .materialize(&entry, &malformed, rejected_cache.path())
            .is_err()
    );
    assert!(!rejected_cache.path().join("cache").exists());
}

#[tokio::test]
async fn go_sumdb_dirhash_fallback_verifies_and_fails_closed() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let zip_bytes = build_module_zip(&[
        (
            "example.com/sum@v0.1.0/go.mod",
            b"module example.com/sum\n".as_slice(),
        ),
        (
            "example.com/sum@v0.1.0/main.go",
            b"package main\n".as_slice(),
        ),
    ]);
    // Compute the REAL sumdb dirhash of the zip for the mocked record.
    // (Tính dirhash sumdb THẬT của zip cho bản ghi mock.)
    let entries = mgc_resolver::protocols::zip_reader::read_zip_entries(&zip_bytes).unwrap();
    let h1 = mgc_resolver::protocols::go::dirhash_zip(&entries).unwrap();

    server
        .mock("GET", "/example.com/sum/@v/list")
        .with_status(200)
        .with_body("v0.1.0\n")
        .create_async()
        .await;
    server
        .mock("GET", "/example.com/sum/@v/v0.1.0.info")
        .with_status(200)
        .with_body(r#"{"Version":"v0.1.0","Time":"2024-05-06T07:08:09Z"}"#)
        .create_async()
        .await;
    server
        .mock("GET", "/example.com/sum/@v/v0.1.0.mod")
        .with_status(200)
        .with_body("module example.com/sum\n")
        .create_async()
        .await;
    // No .ziphash on this proxy → sumdb fallback.
    // (Proxy này không có .ziphash → fallback sumdb.)
    server
        .mock("GET", "/example.com/sum/@v/v0.1.0.ziphash")
        .with_status(404)
        .create_async()
        .await;
    server
        .mock("GET", "/lookup/example.com/sum@v0.1.0")
        .with_status(200)
        .with_body(format!(
            // Genuine sumdb lookup shape (space-separated tokens, one
            // record per line, sig tail) — captured from sum.golang.org.
            // (Dạng lookup sumdb thật: token cách dấu cách, mỗi bản ghi
            // một dòng.)
            "42\nexample.com/sum v0.1.0 {h1}\nexample.com/sum v0.1.0/go.mod h1:AAAA=\n\ngo.sum database tree\n99\nSIGNATURE=\n"
        ))
        .create_async()
        .await;

    let protocol = GoModProtocol::with_sum_base(&server.url(), &server.url());
    let entry = protocol.resolve("example.com/sum", "*").await.unwrap();
    assert_eq!(entry.version, "0.1.0");
    assert!(entry.sha256.is_empty(), "sumdb path must not fake a sha256");
    assert!(
        entry
            .extra_markers
            .iter()
            .any(|m| m.starts_with("sumdb-ziphash:h1:")),
        "sumdb dirhash must be recorded: {:?}",
        entry.extra_markers
    );

    // Genuine zip verifies through the h1 directory hash.
    // (Zip nguyên bản được xác minh qua directory hash h1.)
    protocol.verify(&entry, &zip_bytes).unwrap();

    // Tampered payload fails closed with Integrity — flip a byte inside the
    // first entry's stored data (located via the local header, not a fixed
    // offset).
    // (Payload đã can thiệp fail-closed với Integrity — lật một byte trong
    // dữ liệu entry đầu (định vị qua local header, không dùng offset cứng).)
    let mut tampered = zip_bytes.clone();
    let first_local = zip_bytes
        .windows(4)
        .position(|w| w == b"PK\x03\x04")
        .unwrap();
    let name_len =
        u16::from_le_bytes([zip_bytes[first_local + 26], zip_bytes[first_local + 27]]) as usize;
    let data_idx = first_local + 30 + name_len + 2;
    tampered[data_idx] ^= 0xFF;
    let err = protocol.verify(&entry, &tampered).unwrap_err();
    assert!(matches!(err, mgc_types::MgError::Integrity(_)), "{err:?}");
}

#[tokio::test]
async fn go_latest_fallback_and_range_selection() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    // list 404 → @latest fallback.
    server
        .mock("GET", "/example.com/only/@v/list")
        .with_status(404)
        .create_async()
        .await;
    server
        .mock("GET", "/example.com/only/@v/latest")
        .with_status(200)
        .with_body(r#"{"Version":"v0.3.1","Time":"2024-02-03T04:05:06Z"}"#)
        .create_async()
        .await;
    server
        .mock("GET", "/example.com/only/@v/v0.3.1.mod")
        .with_status(200)
        .with_body("module example.com/only\n")
        .create_async()
        .await;
    server
        .mock("GET", "/example.com/only/@v/v0.3.1.info")
        .with_status(200)
        .with_body(r#"{"Version":"v0.3.1","Time":"2024-02-03T04:05:06Z"}"#)
        .create_async()
        .await;
    server
        .mock("GET", "/example.com/only/@v/v0.3.1.ziphash")
        .with_status(200)
        .with_body("a".repeat(64))
        .create_async()
        .await;

    let protocol = GoModProtocol::with_sum_base(&server.url(), &server.url());
    let entry = protocol.resolve("example.com/only", "*").await.unwrap();
    assert_eq!(entry.version, "0.3.1");

    // Range that does not match the latest version fails closed.
    // (Range không khớp version mới nhất fail-closed.)
    assert!(protocol.resolve("example.com/only", "1.0.0").await.is_err());
}

#[test]
#[ignore = "requires an explicitly configured real Go SDK"]
fn materialized_go_module_imports_with_real_sdk_without_rewriting_hash() {
    use mgc_resolver::protocols::ResolvedEntry;
    use mgc_resolver::protocols::go::GoModuleFiles;
    let sdk = std::env::var("MGC_E2E_GO_BINARY").expect("MGC_E2E_GO_BINARY is required");
    assert!(
        std::path::Path::new(&sdk).is_file(),
        "Go SDK executable is missing"
    );
    let root = tempfile::tempdir().unwrap();
    let gomod = b"module example.com/pkg\ngo 1.21\n";
    let zip = build_module_zip(&[
        ("example.com/pkg@v1.2.3/go.mod", gomod.as_slice()),
        (
            "example.com/pkg@v1.2.3/pkg.go",
            b"package pkg\nconst Marker = \"MGC_GO_IMPORTED\"\n".as_slice(),
        ),
    ]);
    let files = GoModuleFiles {
        zip: zip.clone(),
        gomod: gomod.to_vec(),
        info: br#"{"Version":"v1.2.3","Time":"2020-01-01T00:00:00Z"}"#.to_vec(),
    };
    let entry = ResolvedEntry {
        name: "example.com/pkg".into(),
        version: "1.2.3".into(),
        deps: vec![],
        artifact_url: String::new(),
        sha256: sha256_hex(&zip),
        extra_markers: vec![],
    };
    let protocol = GoModProtocol::from_env();
    protocol.verify(&entry, &files.zip).unwrap();
    let cache = root.path().join("modules");
    let zip_path = protocol.materialize(&entry, &files, &cache).unwrap();
    let hash_path = zip_path.with_extension("ziphash");
    let before = std::fs::read(&hash_path).unwrap();
    assert_eq!(before, b"h1:bg40Pl8hXM1h5nYr/dKdz2D7lBOBkGzSsFhpsZht/qY=");
    std::fs::write(
        root.path().join("go.mod"),
        "module example.com/probe\ngo 1.21\nrequire example.com/pkg v1.2.3\n",
    )
    .unwrap();
    // Independent sums cover both the zip content and the go.mod artifact.
    // Sum tính độc lập cho zip và go.mod; không cho SDK fetch hoặc sửa manifest.
    std::fs::write(root.path().join("go.sum"), "example.com/pkg v1.2.3 h1:bg40Pl8hXM1h5nYr/dKdz2D7lBOBkGzSsFhpsZht/qY=\nexample.com/pkg v1.2.3/go.mod h1:P80OUskPLmuRx31utNM9j/sgmpvOrZ/ta6nccuJ2wHM=\n").unwrap();
    std::fs::write(
        root.path().join("main.go"),
        "package main\nimport(\"fmt\";\"example.com/pkg\")\nfunc main(){fmt.Println(pkg.Marker)}\n",
    )
    .unwrap();
    let output = std::process::Command::new(sdk)
        .args(["run", "-mod=readonly", "main.go"])
        .current_dir(root.path())
        .env("GOMODCACHE", &cache)
        .env("GOCACHE", root.path().join("build"))
        .env("GOPROXY", "off")
        .env("GOSUMDB", "off")
        .env("GOTOOLCHAIN", "local")
        .env("GOENV", "off")
        .env("CGO_ENABLED", "0")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "Go cached import failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("MGC_GO_IMPORTED"));
    assert_eq!(std::fs::read(hash_path).unwrap(), before);
}

#[test]
fn go_dirhash_rejects_newline_names_instead_of_hashing_ambiguous_lines() {
    use mgc_resolver::protocols::{go::dirhash_zip, zip_reader::ZipEntry};
    let entry = ZipEntry {
        name: "example.com/pkg@v1.2.3/bad\nname.go".into(),
        data: b"package pkg\n".to_vec(),
    };
    let error = dirhash_zip(&[entry]).unwrap_err();
    assert!(error.to_string().contains("newlines"));
    let ordinary = ZipEntry {
        name: "example.com/pkg@v1.2.3/ordinary.go".into(),
        data: b"package pkg\n".to_vec(),
    };
    assert!(dirhash_zip(&[ordinary]).unwrap().starts_with("h1:"));
}

#[test]
fn go_materializer_rejects_escaping_coordinates_before_cache_writes() {
    use mgc_resolver::protocols::{ResolvedEntry, go::GoModuleFiles};
    let protocol = GoModProtocol::from_env();
    let files = GoModuleFiles {
        zip: build_module_zip(&[("example.com/pkg@v1.2.3/pkg.go", b"package pkg\n")]),
        gomod: vec![],
        info: vec![],
    };
    for (module, version) in [
        ("/absolute/module", "1.2.3"),
        ("../escaped", "1.2.3"),
        ("example.com/../escaped", "1.2.3"),
        ("C:\\escaped", "1.2.3"),
        ("example.com\\escaped", "1.2.3"),
        ("example.com/pkg", "../../escaped"),
        ("example.com/pkg", "1.2.3\\escaped"),
        ("example.com//pkg", "1.2.3"),
    ] {
        let root = tempfile::tempdir().unwrap();
        let entry = ResolvedEntry {
            name: module.into(),
            version: version.into(),
            deps: vec![],
            artifact_url: String::new(),
            sha256: sha256_hex(&files.zip),
            extra_markers: vec![],
        };
        let error = protocol
            .materialize(&entry, &files, root.path())
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("unsafe Go module cache coordinates")
        );
        assert_eq!(std::fs::read_dir(root.path()).unwrap().count(), 0);
    }
}

#[tokio::test]
async fn go_resolves_multiple_roots_with_minimum_version_selection_and_cycles() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    mock_go_module(
        &mut server,
        "example.test/a",
        "1.0.0",
        "\texample.test/shared v1.0.0",
    )
    .await;
    mock_go_module(
        &mut server,
        "example.test/b",
        "1.0.0",
        "\texample.test/shared v1.2.0",
    )
    .await;
    mock_go_module(
        &mut server,
        "example.test/shared",
        "1.0.0",
        "\texample.test/old v1.0.0",
    )
    .await;
    mock_go_module(
        &mut server,
        "example.test/shared",
        "1.2.0",
        "\texample.test/new v1.0.0",
    )
    .await;
    mock_go_module(
        &mut server,
        "example.test/old",
        "1.0.0",
        "\texample.test/a v1.0.0",
    )
    .await;
    mock_go_module(&mut server, "example.test/new", "1.0.0", "").await;

    let protocol = GoModProtocol::with_sum_base(&server.url(), &server.url());
    let roots = vec![
        ("example.test/a".to_string(), "1.0.0".to_string()),
        ("example.test/b".to_string(), "1.0.0".to_string()),
    ];
    let entries = protocol.resolve_graph_roots(&roots).await.unwrap();
    let mut reversed_roots = roots;
    reversed_roots.reverse();
    let reversed_entries = protocol.resolve_graph_roots(&reversed_roots).await.unwrap();
    assert_eq!(entries, reversed_entries);
    assert_eq!(entries.len(), 5);
    assert_eq!(
        entries
            .iter()
            .find(|entry| entry.name == "example.test/shared")
            .unwrap()
            .version,
        "1.2.0"
    );
    assert!(entries.iter().any(|entry| entry.name == "example.test/old"));
    assert!(entries.iter().any(|entry| entry.name == "example.test/new"));
}

#[tokio::test]
async fn go_resolution_rejects_release_metadata_for_another_version() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    server
        .mock("GET", "/example.test/wrong/@v/v1.2.3.info")
        .with_status(200)
        .with_body(r#"{"Version":"v9.9.9","Time":"2024-01-01T00:00:00Z"}"#)
        .create_async()
        .await;

    let protocol = GoModProtocol::with_sum_base(&server.url(), &server.url());
    let error = protocol
        .resolve("example.test/wrong", "1.2.3")
        .await
        .unwrap_err();
    assert!(
        error.to_string().contains("release metadata identifies"),
        "unexpected error: {error}"
    );
}

#[tokio::test]
async fn go_exact_incompatible_version_is_resolved_and_preserved() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    mock_go_module(&mut server, "example.test/legacy", "3.2.2+incompatible", "").await;

    let protocol = GoModProtocol::with_sum_base(&server.url(), &server.url());
    let entry = protocol
        .resolve("example.test/legacy", "3.2.2+incompatible")
        .await
        .unwrap();
    assert_eq!(entry.version, "3.2.2+incompatible");
    assert!(entry.artifact_url.ends_with("v3.2.2+incompatible.zip"));
}

#[tokio::test]
async fn go_download_rejects_changed_mod_metadata_after_resolution() {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    let Some(mut server) = mock_server().await else {
        return;
    };
    let module = "example.test/frozen";
    let version = "1.2.3";
    let original_mod = "module example.test/frozen\nrequire example.test/safe v1.0.0\n";
    let changed_mod = "module example.test/frozen\nrequire example.test/unselected v9.9.9\n";
    let mod_hits = Arc::new(AtomicUsize::new(0));
    server
        .mock("GET", "/example.test/frozen/@v/v1.2.3.info")
        .with_status(200)
        .with_body(r#"{"Version":"v1.2.3","Time":"2024-01-01T00:00:00Z"}"#)
        .create_async()
        .await;
    server
        .mock("GET", "/example.test/frozen/@v/v1.2.3.mod")
        .with_status(200)
        .with_body_from_request({
            let mod_hits = Arc::clone(&mod_hits);
            let original_mod = original_mod.to_string();
            let changed_mod = changed_mod.to_string();
            move |_| {
                if mod_hits.fetch_add(1, Ordering::SeqCst) == 0 {
                    original_mod.clone().into_bytes()
                } else {
                    changed_mod.clone().into_bytes()
                }
            }
        })
        .expect(2)
        .create_async()
        .await;
    server
        .mock("GET", "/example.test/frozen/@v/v1.2.3.ziphash")
        .with_status(200)
        .with_body("0".repeat(64))
        .create_async()
        .await;
    server
        .mock("GET", "/example.test/frozen/@v/v1.2.3.zip")
        .with_status(200)
        .with_body(b"zip fixture".as_slice())
        .create_async()
        .await;

    let protocol = GoModProtocol::with_sum_base(&server.url(), &server.url());
    let entry = protocol.resolve(module, version).await.unwrap();
    assert!(entry.extra_markers.iter().any(|marker| {
        marker == &format!("go-mod-sha256:{}", sha256_hex(original_mod.as_bytes()))
    }));
    let error = protocol.download_module(&entry).await.unwrap_err();
    assert!(
        error
            .to_string()
            .contains(".mod metadata changed after resolution"),
        "unexpected error: {error}"
    );
}
