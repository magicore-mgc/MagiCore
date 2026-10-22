// version_selection behavior tests — test behavior thuộc owner version_selection.
use super::*;

#[tokio::test]
async fn pypi_selects_universal_wheel_over_sdist_and_resolves_deps() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let wheel_sha = sha256_hex(b"WHEEL_BYTES");
    let sdist_sha = sha256_hex(b"SDIST_BYTES");
    let base = server.url();

    let releases = format!(
        r#""1.0.0": [{wheel}, {sdist}], "0.9.0": [{sdist0}]"#,
        wheel = file_entry(
            "demo-1.0.0-py3-none-any.whl",
            &format!("{base}/files/demo-1.0.0-py3-none-any.whl"),
            &wheel_sha,
            "bdist_wheel"
        ),
        sdist = file_entry(
            "demo-1.0.0.tar.gz",
            &format!("{base}/files/demo-1.0.0.tar.gz"),
            &sdist_sha,
            "sdist"
        ),
        sdist0 = file_entry(
            "demo-0.9.0.tar.gz",
            &format!("{base}/files/demo-0.9.0.tar.gz"),
            &sdist_sha,
            "sdist"
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
        .with_body(version_json(
            r#""dep-a>=1.0", "extras-only ; extra == \"security\"""#,
        ))
        .create_async()
        .await;

    let protocol = PypiProtocol::new(&base);
    let entry = protocol
        .resolve("demo", ">=0.9")
        .await
        .expect("test operation must succeed");

    assert_eq!(entry.version, "1.0.0");
    assert_eq!(
        entry.artifact_url,
        format!("{base}/files/demo-1.0.0-py3-none-any.whl")
    );
    assert_eq!(entry.sha256, wheel_sha);
    assert_eq!(entry.deps, vec![("dep-a".to_string(), ">=1.0".to_string())]);
    assert!(
        entry
            .extra_markers
            .iter()
            .any(|m| m == "wheel:py3-none-any"),
        "universal wheel tag must be recorded: {:?}",
        entry.extra_markers
    );
    // extra dep marker recorded, dep skipped
    assert!(
        entry.extra_markers.iter().any(|m| m.contains("extra")),
        "extra dep marker must be recorded"
    );
    assert!(
        !entry.deps.iter().any(|(n, _)| n == "extras-only"),
        "extra dep must be dropped from graph"
    );

    index_mock.assert_async().await;
    version_mock.assert_async().await;
}

#[tokio::test]
async fn pypi_compatible_release_range() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let sha = sha256_hex(b"WHEEL");
    let base = server.url();
    let releases = format!(
        r#""1.4.9": [{w}], "1.5.0": [{w}], "1.4.4": [{w}]"#,
        w = file_entry(
            "demo-1.0.0-py3-none-any.whl",
            &format!("{base}/files/w.whl"),
            &sha,
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
        .mock("GET", "/pypi/demo/1.4.9/json")
        .with_status(200)
        .with_body(version_json(""))
        .create_async()
        .await;

    let protocol = PypiProtocol::new(&base);
    // ~=1.4.5 → >=1.4.5,<1.5 → 1.4.9 (1.5.0 excluded, 1.4.4 excluded)
    assert_eq!(
        protocol
            .resolve("demo", "~=1.4.5")
            .await
            .expect("test operation must succeed")
            .version,
        "1.4.9"
    );
}
