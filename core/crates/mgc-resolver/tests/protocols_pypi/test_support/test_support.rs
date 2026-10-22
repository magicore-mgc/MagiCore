// Shared PyPI fixtures — fixture chung cho các test PyPI.
use std::io::{Cursor, Write};
use zip::write::SimpleFileOptions;

pub(crate) async fn mock_server() -> Option<mockito::ServerGuard> {
    match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(listener) => drop(listener),
        Err(error) => panic!(
            "PyPI mock tests require localhost; refusing to report skipped tests as passing: {error}"
        ),
    }
    Some(mockito::Server::new_async().await)
}

pub(crate) fn wheel_with_record(record: &str) -> Vec<u8> {
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (path, bytes) in [
        ("six.py", b"VALUE = 1\n".as_slice()),
        (
            "six-1.0.0.dist-info/METADATA",
            b"Name: six\nVersion: 1.0.0\n".as_slice(),
        ),
        ("six-1.0.0.dist-info/RECORD", record.as_bytes()),
    ] {
        writer
            .start_file(path, options)
            .expect("test operation must succeed");
        writer
            .write_all(bytes)
            .expect("test operation must succeed");
    }
    writer
        .finish()
        .expect("test operation must succeed")
        .into_inner()
}

pub(crate) fn wheel_with_module(module_bytes: &[u8]) -> Vec<u8> {
    use base64::Engine;
    use sha2::{Digest, Sha256};

    let metadata = b"Name: demo\nVersion: 1.0.0\n";
    let module_hash =
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(Sha256::digest(module_bytes));
    let metadata_hash =
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(Sha256::digest(metadata));
    let record = format!(
        "demo.py,sha256={module_hash},{}\ndemo-1.0.0.dist-info/METADATA,sha256={metadata_hash},{}\ndemo-1.0.0.dist-info/RECORD,,\n",
        module_bytes.len(),
        metadata.len()
    );
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (path, bytes) in [
        ("demo.py", module_bytes),
        ("demo-1.0.0.dist-info/METADATA", metadata.as_slice()),
        ("demo-1.0.0.dist-info/RECORD", record.as_bytes()),
    ] {
        writer
            .start_file(path, options)
            .expect("test operation must succeed");
        writer
            .write_all(bytes)
            .expect("test operation must succeed");
    }
    writer
        .finish()
        .expect("test operation must succeed")
        .into_inner()
}

pub(crate) fn file_entry(filename: &str, url: &str, sha: &str, ptype: &str) -> String {
    format!(
        r#"{{"filename":"{filename}","url":"{url}","digests":{{"sha256":"{sha}"}},"packagetype":"{ptype}","requires_python":">=3.9"}}"#
    )
}

pub(crate) fn file_entry_at(
    filename: &str,
    url: &str,
    sha: &str,
    ptype: &str,
    upload_time: Option<&str>,
) -> String {
    let mut value = serde_json::json!({
        "filename": filename,
        "url": url,
        "digests": {"sha256": sha},
        "packagetype": ptype,
        "requires_python": ">=3.9"
    });
    if let Some(upload_time) = upload_time {
        value["upload_time_iso_8601"] = serde_json::Value::String(upload_time.to_string());
    }
    value.to_string()
}

pub(crate) fn index_json(releases: &str) -> String {
    format!(r#"{{"info":{{"requires_python":">=3.9"}},"releases":{{{releases}}}}}"#)
}

pub(crate) fn version_json(requires_dist: &str) -> String {
    format!(r#"{{"info":{{"requires_dist":[{requires_dist}]}},"releases":{{}}}}"#)
}
