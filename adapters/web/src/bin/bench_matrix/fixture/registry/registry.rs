use crate::bench_matrix::fixture::sanitize_pkg_name;
// Local registry benchmark fixture
// Phục vụ metadata và tarball giả lập trên loopback.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) async fn spawn_registry_fixture(
    packages: &[ResolvedPackage],
    files_per_package: usize,
) -> anyhow::Result<RegistryFixture> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    let registry_url = format!("http://{addr}");
    let package_map = packages
        .iter()
        .map(|pkg| (pkg.id.clone(), pkg))
        .collect::<std::collections::HashMap<_, _>>();
    let mut grouped = std::collections::BTreeMap::<String, Vec<&ResolvedPackage>>::new();
    for pkg in packages {
        grouped
            .entry(pkg.id.name_str().to_string())
            .or_default()
            .push(pkg);
    }

    let metadata_map = Arc::new(grouped.into_iter().map(|(name, versions)| {
        let latest = versions
            .iter()
            .map(|pkg| pkg.id.version().clone())
            .max()
            .expect("versions should be non-empty")
            .to_string();
        let version_entries = versions
            .iter()
            .map(|pkg| {
                let dependencies = if pkg.deps.is_empty() {
                    serde_json::Value::Null
                } else {
                    serde_json::Value::Object(
                        pkg.deps
                            .iter()
                            .map(|dep_id| {
                                let dep_pkg = package_map
                                    .get(dep_id)
                                    .expect("dependency package should exist in fixture graph");
                                (
                                    dep_id.name_str().to_string(),
                                    serde_json::Value::String(format!("^{}", dep_pkg.id.version())),
                                )
                            })
                            .collect(),
                    )
                };
                (
                    pkg.id.version().to_string(),
                    serde_json::json!({
                        "version": pkg.id.version().to_string(),
                        "dependencies": dependencies,
                        "optionalDependencies": null,
                        "os": null,
                        "cpu": null,
                        "dist": {
                            "tarball": format!("{registry_url}/tarballs/{}-{}.tgz", sanitize_pkg_name(pkg.id.name_str()), pkg.id.version()),
                            "integrity": null
                        }
                    }),
                )
            })
            .collect::<serde_json::Map<String, serde_json::Value>>();

        let metadata = serde_json::json!({
            "name": name,
            "description": null,
            "versions": version_entries,
            "dist-tags": {
                "latest": latest
            }
        });
        (format!("/{}", name), serde_json::to_vec(&metadata).expect("metadata serializable"))
    }).collect::<std::collections::HashMap<_, _>>());
    let tarball_map = Arc::new(
        packages
            .iter()
            .map(|pkg| {
                (
                    format!(
                        "/tarballs/{}-{}.tgz",
                        sanitize_pkg_name(pkg.id.name_str()),
                        pkg.id.version()
                    ),
                    build_tarball_bytes(&pkg.id, files_per_package)
                        .expect("tarball build succeeds"),
                )
            })
            .collect::<std::collections::HashMap<_, _>>(),
    );

    tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            let metadata_map = metadata_map.clone();
            let tarball_map = tarball_map.clone();
            tokio::spawn(async move {
                let mut buf = [0u8; 4096];
                let Ok(read) = stream.read(&mut buf).await else {
                    return;
                };
                let request = String::from_utf8_lossy(&buf[..read]);
                let path = request
                    .lines()
                    .next()
                    .and_then(|line| line.split_whitespace().nth(1))
                    .unwrap_or("/");

                if let Some(body) = metadata_map.get(path) {
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n",
                        body.len()
                    );
                    let _ = stream.write_all(response.as_bytes()).await;
                    let _ = stream.write_all(body).await;
                    return;
                }

                if let Some(body) = tarball_map.get(path) {
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
                        body.len()
                    );
                    let _ = stream.write_all(response.as_bytes()).await;
                    let _ = stream.write_all(body).await;
                    return;
                }

                let _ = stream
                    .write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n")
                    .await;
            });
        }
    });

    Ok(RegistryFixture { registry_url })
}
