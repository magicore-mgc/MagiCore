// platform_fallback behavior tests — test behavior thuộc owner platform_fallback.
use super::*;

#[tokio::test]
async fn pypi_sdist_fallback_when_no_wheel() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let sha = sha256_hex(b"SDIST");
    let base = server.url();
    let releases = format!(
        r#""1.0.0": [{s}]"#,
        s = file_entry(
            "demo-1.0.0.tar.gz",
            &format!("{base}/files/demo-1.0.0.tar.gz"),
            &sha,
            "sdist"
        ),
    );
    server
        .mock("GET", "/pypi/demo/json")
        .with_status(200)
        .with_body(index_json(&releases))
        .create_async()
        .await;
    server
        .mock("GET", "/pypi/demo/1.0.0/json")
        .with_status(200)
        .with_body(version_json(""))
        .create_async()
        .await;

    let protocol = PypiProtocol::new(&base);
    let entry = protocol
        .resolve("demo", "==1.0.0")
        .await
        .expect("test operation must succeed");
    assert_eq!(
        entry.artifact_url,
        format!("{base}/files/demo-1.0.0.tar.gz")
    );
    assert!(entry.extra_markers.iter().any(|m| m.starts_with("sdist:")));
}

#[tokio::test]
async fn pypi_foreign_wheel_loses_to_sdist() {
    // FreeBSD wheels are foreign on every Linux/macOS/Windows CI runner.
    // Wheel FreeBSD không phù hợp các OS CI; phải chọn sdist thay vì cài lặng lẽ.
    let Some(mut server) = mock_server().await else {
        return;
    };
    let wheel_sha = sha256_hex(b"FOREIGN_WHEEL");
    let sdist_sha = sha256_hex(b"SDIST_FALLBACK");
    let base = server.url();
    let releases = format!(
        r#""1.0.0": [{w}, {s}]"#,
        w = file_entry(
            "demo-1.0.0-cp312-cp312-freebsd_13_0_x86_64.whl",
            &format!("{base}/files/demo-1.0.0-cp312-cp312-freebsd_13_0_x86_64.whl"),
            &wheel_sha,
            "bdist_wheel"
        ),
        s = file_entry(
            "demo-1.0.0.tar.gz",
            &format!("{base}/files/demo-1.0.0.tar.gz"),
            &sdist_sha,
            "sdist"
        ),
    );
    server
        .mock("GET", "/pypi/demo/json")
        .with_status(200)
        .with_body(index_json(&releases))
        .create_async()
        .await;
    server
        .mock("GET", "/pypi/demo/1.0.0/json")
        .with_status(200)
        .with_body(version_json(""))
        .create_async()
        .await;

    let protocol = PypiProtocol::new(&base);
    let entry = protocol
        .resolve("demo", "==1.0.0")
        .await
        .expect("test operation must succeed");
    assert_eq!(
        entry.artifact_url,
        format!("{base}/files/demo-1.0.0.tar.gz"),
        "foreign wheel must lose to sdist"
    );
    assert_eq!(entry.sha256, sdist_sha);
}

#[tokio::test]
async fn pypi_only_foreign_wheel_fails_closed() {
    // Nothing installable and no sdist: resolve MUST fail, never hand
    // back the foreign wheel.
    let Some(mut server) = mock_server().await else {
        return;
    };
    let wheel_sha = sha256_hex(b"FOREIGN_ONLY");
    let base = server.url();
    let releases = format!(
        r#""1.0.0": [{w}]"#,
        w = file_entry(
            "demo-1.0.0-cp312-cp312-freebsd_13_0_x86_64.whl",
            &format!("{base}/files/demo-1.0.0-cp312-cp312-freebsd_13_0_x86_64.whl"),
            &wheel_sha,
            "bdist_wheel"
        ),
    );
    server
        .mock("GET", "/pypi/demo/json")
        .with_status(200)
        .with_body(index_json(&releases))
        .create_async()
        .await;
    server
        .mock("GET", "/pypi/demo/1.0.0/json")
        .with_status(200)
        .with_body(version_json(""))
        .create_async()
        .await;

    let protocol = PypiProtocol::new(&base);
    let err = protocol
        .resolve("demo", "==1.0.0")
        .await
        .expect_err("test operation must fail");
    assert!(
        err.to_string().contains("no downloadable file")
            || err
                .to_string()
                .contains("no artifact compatible with Python"),
        "unexpected error: {err}"
    );
}
