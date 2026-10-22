// release_age_gate behavior tests — test behavior thuộc owner release_age_gate.
use super::*;

#[tokio::test]
async fn pypi_age_gate_skips_recent_release_and_uses_older_mature_candidate() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let base = server.url();
    let sha = sha256_hex(b"WHEEL");
    let releases = format!(
        r#""1.0.0": [{recent}], "0.9.0": [{mature}]"#,
        recent = file_entry_at(
            "demo-1.0.0-py3-none-any.whl",
            &format!("{base}/files/recent.whl"),
            &sha,
            "bdist_wheel",
            Some("2099-01-01T00:00:00Z")
        ),
        mature = file_entry_at(
            "demo-0.9.0-py3-none-any.whl",
            &format!("{base}/files/mature.whl"),
            &sha,
            "bdist_wheel",
            Some("2020-01-01T00:00:00Z")
        ),
    );
    server
        .mock("GET", "/pypi/demo/json")
        .with_status(200)
        .with_body(index_json(&releases))
        .create_async()
        .await;
    server
        .mock("GET", "/pypi/demo/0.9.0/json")
        .with_status(200)
        .with_body(version_json(""))
        .create_async()
        .await;

    let protocol = PypiProtocol::new(&base).with_age_gate(24, false);
    let entry = protocol
        .resolve("demo", "*")
        .await
        .expect("test operation must succeed");
    assert_eq!(entry.version, "0.9.0");
}

#[tokio::test]
async fn pypi_age_gate_rejects_unstamped_release_by_default() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let base = server.url();
    let sha = sha256_hex(b"WHEEL");
    let release = file_entry_at(
        "demo-1.0.0-py3-none-any.whl",
        &format!("{base}/files/demo.whl"),
        &sha,
        "bdist_wheel",
        None,
    );
    server
        .mock("GET", "/pypi/demo/json")
        .with_status(200)
        .with_body(index_json(&format!(r#""1.0.0": [{release}]"#)))
        .create_async()
        .await;

    let error = PypiProtocol::new(&base)
        .with_age_gate(24, false)
        .resolve("demo", "*")
        .await
        .expect_err("test operation must fail");
    assert!(error.to_string().contains("unstamped"), "{error}");
}

#[tokio::test]
async fn pypi_age_gate_allows_unstamped_release_only_with_explicit_escape() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let base = server.url();
    let sha = sha256_hex(b"WHEEL");
    let release = file_entry_at(
        "demo-1.0.0-py3-none-any.whl",
        &format!("{base}/files/demo.whl"),
        &sha,
        "bdist_wheel",
        None,
    );
    server
        .mock("GET", "/pypi/demo/json")
        .with_status(200)
        .with_body(index_json(&format!(r#""1.0.0": [{release}]"#)))
        .create_async()
        .await;
    server
        .mock("GET", "/pypi/demo/1.0.0/json")
        .with_status(200)
        .with_body(version_json(""))
        .create_async()
        .await;

    let entry = PypiProtocol::new(&base)
        .with_age_gate(24, true)
        .resolve("demo", "*")
        .await
        .expect("test operation must succeed");
    assert_eq!(entry.version, "1.0.0");
}

#[tokio::test]
async fn pypi_age_gate_zero_hours_disables_timestamp_filtering() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let base = server.url();
    let sha = sha256_hex(b"WHEEL");
    let release = file_entry_at(
        "demo-1.0.0-py3-none-any.whl",
        &format!("{base}/files/demo.whl"),
        &sha,
        "bdist_wheel",
        Some("2099-01-01T00:00:00Z"),
    );
    server
        .mock("GET", "/pypi/demo/json")
        .with_status(200)
        .with_body(index_json(&format!(r#""1.0.0": [{release}]"#)))
        .create_async()
        .await;
    server
        .mock("GET", "/pypi/demo/1.0.0/json")
        .with_status(200)
        .with_body(version_json(""))
        .create_async()
        .await;

    let entry = PypiProtocol::new(&base)
        .with_age_gate(0, false)
        .resolve("demo", "*")
        .await
        .expect("test operation must succeed");
    assert_eq!(entry.version, "1.0.0");
}

#[tokio::test]
async fn pypi_age_gate_rejects_release_with_missing_timestamp_on_any_file() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let base = server.url();
    let sha = sha256_hex(b"WHEEL");
    let release = format!(
        "{},{}",
        file_entry_at(
            "demo-1.0.0-py3-none-any.whl",
            &format!("{base}/files/demo.whl"),
            &sha,
            "bdist_wheel",
            Some("2020-01-01T00:00:00Z"),
        ),
        file_entry_at(
            "demo-1.0.0.tar.gz",
            &format!("{base}/files/demo.tar.gz"),
            &sha,
            "sdist",
            None,
        ),
    );
    server
        .mock("GET", "/pypi/demo/json")
        .with_status(200)
        .with_body(index_json(&format!(r#""1.0.0": [{release}]"#)))
        .create_async()
        .await;

    let error = PypiProtocol::new(&base)
        .with_age_gate(24, false)
        .resolve("demo", "*")
        .await
        .expect_err("test operation must fail");
    assert!(error.to_string().contains("unstamped"), "{error}");
}
