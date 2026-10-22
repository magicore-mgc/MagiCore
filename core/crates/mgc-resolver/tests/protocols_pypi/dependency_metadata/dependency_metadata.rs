// dependency_metadata behavior tests — test behavior thuộc owner dependency_metadata.
use super::*;

#[tokio::test]
async fn pypi_malformed_requires_dist_fails_closed() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let sha = sha256_hex(b"WHEEL");
    let base = server.url();
    let releases = format!(
        r#""1.0.0": [{wheel}]"#,
        wheel = file_entry(
            "demo-1.0.0-py3-none-any.whl",
            &format!("{base}/files/demo.whl"),
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
        .mock("GET", "/pypi/demo/1.0.0/json")
        .with_status(200)
        .with_body(version_json(r#"">=1""#))
        .create_async()
        .await;

    let error = PypiProtocol::new(&base)
        .resolve("demo", "==1.0.0")
        .await
        .expect_err("malformed Requires-Dist must not disappear from the graph");
    assert!(error.to_string().contains("Requires-Dist"), "{error}");
}

#[tokio::test]
async fn pypi_requested_dependency_extras_fail_closed() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let sha = sha256_hex(b"WHEEL");
    let base = server.url();
    let releases = format!(
        r#""1.0.0": [{wheel}]"#,
        wheel = file_entry(
            "demo-1.0.0-py3-none-any.whl",
            &format!("{base}/files/demo.whl"),
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
        .mock("GET", "/pypi/demo/1.0.0/json")
        .with_status(200)
        .with_body(version_json(r#""idna[socks]>=3""#))
        .create_async()
        .await;

    let error = PypiProtocol::new(&base)
        .resolve("demo", "==1.0.0")
        .await
        .expect_err("requested Python extras must not be stripped from the graph");
    assert!(error.to_string().contains("extras"), "{error}");
}

#[tokio::test]
async fn pypi_skips_unrequested_extra_marked_transitive_requirements() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let sha = sha256_hex(b"WHEEL");
    let base = server.url();
    let releases = format!(
        r#""1.0.0": [{wheel}]"#,
        wheel = file_entry(
            "demo-1.0.0-py3-none-any.whl",
            &format!("{base}/files/demo.whl"),
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
        .mock("GET", "/pypi/demo/1.0.0/json")
        .with_status(200)
        .with_body(version_json(r#""attrs[tests]>=22; extra == \"cov\"""#))
        .create_async()
        .await;

    let entry = PypiProtocol::new(&base)
        .resolve("demo", "==1.0.0")
        .await
        .expect("an unrequested optional extra must not block base installation");

    assert!(
        entry.deps.is_empty(),
        "unrequested extra leaked into the base graph"
    );
}
