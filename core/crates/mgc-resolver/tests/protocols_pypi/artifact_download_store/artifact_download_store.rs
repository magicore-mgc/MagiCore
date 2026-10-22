// artifact_download_store behavior tests — test behavior thuộc owner artifact_download_store.
use super::*;

#[tokio::test]
async fn pypi_resolve_download_verify_store_ref_full_path() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let bytes = b"WHEEL_ARCHIVE";
    let sha = sha256_hex(bytes);
    let base = server.url();
    let releases = format!(
        r#""1.0.0": [{w}]"#,
        w = file_entry(
            "demo-1.0.0-py3-none-any.whl",
            &format!("{base}/files/demo-1.0.0-py3-none-any.whl"),
            &sha,
            "bdist_wheel"
        ),
    );

    let index_mock = server
        .mock("GET", "/pypi/demo/json")
        .with_status(200)
        .with_body(index_json(&releases))
        .create_async()
        .await;
    let version_mock = server
        .mock("GET", "/pypi/demo/1.0.0/json")
        .with_status(200)
        .with_body(version_json(""))
        .create_async()
        .await;
    let file_mock = server
        .mock("GET", "/files/demo-1.0.0-py3-none-any.whl")
        .with_status(200)
        .with_body(bytes.as_slice())
        .create_async()
        .await;

    let protocol = PypiProtocol::new(&base);
    let entry = protocol
        .resolve("demo", "==1.0.0")
        .await
        .expect("test operation must succeed");
    let downloaded = protocol
        .download(&entry)
        .await
        .expect("test operation must succeed");
    protocol
        .verify(&entry, &downloaded)
        .expect("test operation must succeed");
    let expected_hex = blake3::hash(bytes).to_hex().to_string();
    assert_eq!(
        protocol.store_ref(&downloaded),
        format!("files/blake3/{}/{}", &expected_hex[..2], expected_hex)
    );

    index_mock.assert_async().await;
    version_mock.assert_async().await;
    file_mock.assert_async().await;
}
