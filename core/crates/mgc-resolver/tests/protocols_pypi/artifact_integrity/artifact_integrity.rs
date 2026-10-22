// artifact_integrity behavior tests — test behavior thuộc owner artifact_integrity.
use super::*;
use std::io::{Cursor, Write};
use zip::write::SimpleFileOptions;

#[test]
fn pypi_materialization_isolated_by_integrity_when_url_basename_collides() {
    let temp = tempfile::tempdir().expect("test operation must succeed");
    let protocol = PypiProtocol::new("https://pypi.example.test");
    let artifact_url = "https://files.example.test/packages/demo-1.0.0-py3-none-any.whl";
    let first_bytes = b"wheel bytes from source one";
    let second_bytes = b"wheel bytes from source two";
    let first = mgc_resolver::protocols::ResolvedEntry {
        name: "demo".to_string(),
        version: "1.0.0".to_string(),
        deps: Vec::new(),
        artifact_url: artifact_url.to_string(),
        sha256: sha256_hex(first_bytes),
        extra_markers: Vec::new(),
    };
    let second = mgc_resolver::protocols::ResolvedEntry {
        sha256: sha256_hex(second_bytes),
        ..first.clone()
    };

    let first_path = protocol
        .materialize(&first, first_bytes, temp.path())
        .expect("test operation must succeed");
    let second_path = protocol
        .materialize(&second, second_bytes, temp.path())
        .expect("test operation must succeed");

    assert_ne!(
        first_path, second_path,
        "same URL basename with different integrity must not alias in a shared store"
    );
    assert_eq!(
        std::fs::read(first_path).expect("test operation must succeed"),
        first_bytes
    );
    assert_eq!(
        std::fs::read(second_path).expect("test operation must succeed"),
        second_bytes
    );
}

#[test]
fn pypi_materialization_rejects_bytes_that_disagree_with_lock_digest() {
    let temp = tempfile::tempdir().expect("test operation must succeed");
    let protocol = PypiProtocol::new("https://pypi.example.test");
    let bytes = b"artifact bytes";
    let entry = mgc_resolver::protocols::ResolvedEntry {
        name: "demo".to_string(),
        version: "1.0.0".to_string(),
        deps: Vec::new(),
        artifact_url: "https://files.example.test/demo-1.0.0.whl".to_string(),
        sha256: sha256_hex(b"different bytes"),
        extra_markers: Vec::new(),
    };

    let error = protocol
        .materialize(&entry, bytes, temp.path())
        .expect_err("must refuse bytes not authenticated by the resolved digest");

    assert!(error.to_string().contains("mismatched SHA-256"));
    assert!(
        std::fs::read_dir(temp.path())
            .expect("test operation must succeed")
            .next()
            .is_none()
    );
}

#[test]
fn pypi_importable_sites_are_isolated_by_integrity_for_same_package_version() {
    let temp = tempfile::tempdir().expect("test operation must succeed");
    let protocol = PypiProtocol::new("https://pypi.example.test");
    let artifact_url = "https://files.example.test/packages/demo-1.0.0-py3-none-any.whl";
    let first_bytes = wheel_with_module(b"VALUE = 'source one'\n");
    let second_bytes = wheel_with_module(b"VALUE = 'source two'\n");
    let mut entry = mgc_resolver::protocols::ResolvedEntry {
        name: "demo".to_string(),
        version: "1.0.0".to_string(),
        deps: Vec::new(),
        artifact_url: artifact_url.to_string(),
        sha256: sha256_hex(&first_bytes),
        extra_markers: Vec::new(),
    };

    let mut mismatched_entry = entry.clone();
    mismatched_entry.sha256 = sha256_hex(b"not the wheel bytes");
    assert!(
        protocol
            .materialize_importable(&mismatched_entry, &first_bytes, temp.path())
            .is_err()
    );
    assert!(
        !temp.path().join("site").exists(),
        "digest mismatch must be rejected before unpacking"
    );

    let first_site = protocol
        .materialize_importable(&entry, &first_bytes, temp.path())
        .expect("test operation must succeed")
        .expect("test operation must succeed");
    entry.sha256 = sha256_hex(&second_bytes);
    let second_site = protocol
        .materialize_importable(&entry, &second_bytes, temp.path())
        .expect("test operation must succeed")
        .expect("test operation must succeed");

    assert_ne!(first_site, second_site);
    assert_eq!(
        std::fs::read(first_site.join("demo.py")).expect("test operation must succeed"),
        b"VALUE = 'source one'\n"
    );
    assert_eq!(
        std::fs::read(second_site.join("demo.py")).expect("test operation must succeed"),
        b"VALUE = 'source two'\n"
    );
}

#[test]
fn pypi_import_materialization_rejects_a_wheel_with_a_stale_record() {
    let wheel = wheel_with_record(
        "six.py,sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA,23\nsix-1.0.0.dist-info/METADATA,sha256-BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB,11\nsix-1.0.0.dist-info/RECORD,,\n",
    );
    let entry = mgc_resolver::protocols::ResolvedEntry {
        name: "six".to_string(),
        version: "1.0.0".to_string(),
        deps: Vec::new(),
        artifact_url: "https://files.example.test/six-1.0.0-py3-none-any.whl".to_string(),
        sha256: sha256_hex(&wheel),
        extra_markers: Vec::new(),
    };
    let temp = tempfile::tempdir().expect("test operation must succeed");

    let error = PypiProtocol::new("https://pypi.example.test")
        .materialize_importable(&entry, &wheel, temp.path())
        .expect_err("tampered member must fail closed");

    assert!(error.to_string().contains("RECORD"), "{error}");
}

#[test]
fn pypi_install_rejects_unhashed_non_record_wheel_files() {
    use base64::Engine;
    use sha2::{Digest, Sha256};

    let module = b"VALUE = 1\n";
    let metadata = b"Name: six\nVersion: 1.0.0\n";
    let b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    let record = format!(
        "six.py,,{}\nsix-1.0.0.dist-info/METADATA,sha256={},{}\nsix-1.0.0.dist-info/RECORD,,\n",
        module.len(),
        b64.encode(Sha256::digest(metadata)),
        metadata.len(),
    );
    let wheel = wheel_with_record(&record);
    let temp = tempfile::tempdir().expect("test operation must succeed");
    let protocol = PypiProtocol::new("https://pypi.org");
    let entry = mgc_resolver::protocols::ResolvedEntry {
        name: "six".to_string(),
        version: "1.0.0".to_string(),
        deps: Vec::new(),
        artifact_url: "https://files.pythonhosted.org/six-1.0.0-py3-none-any.whl".to_string(),
        sha256: sha256_hex(&wheel),
        extra_markers: Vec::new(),
    };

    assert!(
        protocol
            .materialize_importable(&entry, &wheel, temp.path())
            .is_err(),
        "a regular wheel member without a SHA-256 RECORD entry must not install successfully"
    );
}

#[test]
fn pypi_vendored_dist_info_is_verified_as_payload() {
    use base64::Engine;
    use sha2::{Digest, Sha256};
    let protocol = PypiProtocol::new("https://pypi.org");
    for (unhashed_vendor, duplicate_root) in [(false, false), (true, false), (false, true)] {
        let mut files = vec![
            ("demo.py", b"VALUE = 1\n".as_slice()),
            (
                "demo-1.0.0.dist-info/METADATA",
                b"Name: demo\nVersion: 1.0.0\n".as_slice(),
            ),
            (
                "demo/_vendor/other-2.0.0.dist-info/METADATA",
                b"Name: other\nVersion: 2.0.0\n".as_slice(),
            ),
            (
                "demo/_vendor/other-2.0.0.dist-info/RECORD",
                b"vendored record contents\n".as_slice(),
            ),
        ];
        if duplicate_root {
            files.push((
                "other-2.0.0.dist-info/METADATA",
                b"Name: other\nVersion: 2.0.0\n".as_slice(),
            ));
        }
        let mut record = String::new();
        for &(path, data) in &files {
            if unhashed_vendor && path.contains("_vendor/") && path.ends_with("/RECORD") {
                record.push_str(&format!("{path},,\n"));
            } else {
                let hash =
                    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(Sha256::digest(data));
                record.push_str(&format!("{path},sha256={hash},{}\n", data.len()));
            }
        }
        record.push_str("demo-1.0.0.dist-info/RECORD,,\n");
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
        let options =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
        for (path, data) in files.into_iter().chain(std::iter::once((
            "demo-1.0.0.dist-info/RECORD",
            record.as_bytes(),
        ))) {
            writer
                .start_file(path, options)
                .expect("test operation must succeed");
            writer.write_all(data).expect("test operation must succeed");
        }
        let wheel = writer
            .finish()
            .expect("test operation must succeed")
            .into_inner();
        let entry = mgc_resolver::protocols::ResolvedEntry {
            name: "demo".to_string(),
            version: "1.0.0".to_string(),
            deps: Vec::new(),
            artifact_url: "https://files.pythonhosted.org/demo-1.0.0-py3-none-any.whl".to_string(),
            sha256: sha256_hex(&wheel),
            extra_markers: Vec::new(),
        };
        let root = tempfile::tempdir().expect("test operation must succeed");
        let result = protocol.materialize_importable(&entry, &wheel, root.path());
        assert_eq!(
            result.is_ok(),
            !unhashed_vendor && !duplicate_root,
            "vendored metadata must be ordinary authenticated payload: {result:?}"
        );
    }
}
