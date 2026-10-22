//! Verify native Maven POM project resolution and unsupported Gradle behavior.
//! Kiểm tra resolve project Maven POM và hành vi Gradle chưa hỗ trợ.

use super::super::support::environment_override::{
    EnvironmentOverrideGuard, lock_environment_access,
};
use super::super::support::mock_server;
use mgc_resolver::protocols::sha256_hex;
use mgc_types::{DependencyResolver, Ecosystem, PackageAdapter};

pub(crate) async fn lib_adapter_maven_pom_project_resolves_natively_via_env() {
    let Some(mut server) = mock_server().await else {
        return;
    };
    let jar_bytes = b"MOCKITO_JAR_BYTES";
    let jar_sha256 = sha256_hex(jar_bytes);
    let base = server.url();

    server
        .mock("GET", "/com/demo/core/maven-metadata.xml")
        .with_status(200)
        .with_body(
            "<metadata><versioning><versions><version>1.0.0</version></versions></versioning></metadata>",
        )
        .create_async()
        .await;
    server
        .mock("GET", "/com/demo/core/1.0.0/core-1.0.0.pom")
        .with_status(200)
        .with_body(
            "<project><packaging>jar</packaging><dependencies><dependency><groupId>com.demo</groupId><artifactId>util</artifactId><version>1.0.0</version></dependency></dependencies></project>",
        )
        .create_async()
        .await;
    server
        .mock("GET", "/com/demo/core/1.0.0/core-1.0.0.jar.sha256")
        .with_status(200)
        .with_body(jar_sha256.as_str())
        .create_async()
        .await;
    // Transitive dep com.demo:util — full mock set.
    // (Dep bắc cầu com.demo:util — bộ mock đầy đủ.)
    let util_bytes = b"MOCKITO_UTIL_JAR";
    let util_sha256 = sha256_hex(util_bytes);
    server
        .mock("GET", "/com/demo/util/maven-metadata.xml")
        .with_status(200)
        .with_body(
            "<metadata><versioning><versions><version>1.0.0</version></versions></versioning></metadata>",
        )
        .create_async()
        .await;
    server
        .mock("GET", "/com/demo/util/1.0.0/util-1.0.0.pom")
        .with_status(200)
        .with_body("<project><packaging>jar</packaging><dependencies/></project>")
        .create_async()
        .await;
    server
        .mock("GET", "/com/demo/util/1.0.0/util-1.0.0.jar.sha256")
        .with_status(200)
        .with_body(util_sha256.as_str())
        .create_async()
        .await;

    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(
        tmp.path().join("pom.xml"),
        "<project><groupId>com.demo</groupId><artifactId>app</artifactId><version>0.1.0</version><dependencies><dependency><groupId>com.demo</groupId><artifactId>core</artifactId><version>1.0.0</version></dependency></dependencies></project>",
    )
    .unwrap();

    let _environment_lock = lock_environment_access();
    let _environment =
        EnvironmentOverrideGuard::set(&_environment_lock, &[("MGC_MAVEN_REPO_URL", base)]);
    let adapter = mgc_lib_adapter::adapter_for(tmp.path(), None, None)
        .unwrap()
        .unwrap();
    let manifest = adapter.parse_manifest(tmp.path()).await.unwrap();
    assert_eq!(manifest.name, "com.demo:app");
    let graph = adapter.resolve(&manifest).await.unwrap();
    drop(_environment);

    assert_eq!(graph.packages.len(), 2, "core + transitive util");
    assert_eq!(graph.packages[0].id.name_str(), "com.demo:core");
    assert_eq!(graph.packages[0].id.version().to_string(), "1.0.0");
    assert_eq!(graph.packages[1].id.name_str(), "com.demo:util");

    // Gradle projects fail closed with honest guidance.
    // (Project gradle fail-closed kèm hướng dẫn trung thực.)
    let gradle = tempfile::tempdir().unwrap();
    std::fs::write(gradle.path().join("build.gradle"), "plugins {}\n").unwrap();
    let gradle_adapter = mgc_lib_adapter::adapter_for(gradle.path(), None, None)
        .unwrap()
        .unwrap();
    let err = gradle_adapter
        .resolve(&mgc_types::Manifest::new("g", Ecosystem::Lib))
        .await
        .unwrap_err();
    assert!(
        matches!(err, mgc_types::MgError::Unsupported { .. }),
        "{err:?}"
    );
}
