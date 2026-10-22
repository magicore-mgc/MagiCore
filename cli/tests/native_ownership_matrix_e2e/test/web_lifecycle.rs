//! Web Lifecycle lifecycle E2E cases. / Test E2E lifecycle cho web lifecycle.

use crate::native_ownership_matrix_e2e::support::{
    HermeticCrates, HermeticGo, HermeticMaven, HermeticNuget, HermeticPypi, MatrixSandbox,
    WebBackendCase, dotnet_manifest, go_manifest, java_manifest, python_manifest, rust_manifest,
    web_backend_nonempty_add_install, web_backend_project,
};
use tempfile::TempDir;

pub(crate) fn matrix_web_non_js_backends_resolve_nonempty_v3_and_v4_without_toolchains() {
    for schema in ["3", "v4"] {
        let (manifest, body) = python_manifest();
        let pypi = HermeticPypi::new(&[("six", "1.17.0")]);
        let env = [("MGC_PYPI_INDEX_URL", pypi.url.as_str())];
        web_backend_nonempty_add_install(
            WebBackendCase {
                language: "python",
                framework: "fastapi",
                manifest_name: manifest,
                manifest_body: &body,
                schema,
                package: "six",
                manifest_pin: "six",
                lock_pin: "six",
            },
            &env,
        );
        pypi.assert_required_routes_used();

        let (manifest, body) = rust_manifest();
        let crates = HermeticCrates::new();
        let env = [
            ("MGC_CRATES_INDEX_URL", crates.index_url.as_str()),
            ("MGC_CRATES_DOWNLOAD_URL", crates.download_url.as_str()),
        ];
        web_backend_nonempty_add_install(
            WebBackendCase {
                language: "rust",
                framework: "actix-web",
                manifest_name: manifest,
                manifest_body: &body,
                schema,
                package: "mgcfixture",
                manifest_pin: "mgcfixture",
                lock_pin: "mgcfixture",
            },
            &env,
        );
        for mock in &crates._mocks {
            mock.assert();
        }

        let (manifest, body) = go_manifest();
        let proxy = HermeticGo::new();
        let env = [
            ("MGC_GO_PROXY_URL", proxy.url.as_str()),
            ("MGC_GO_SUM_URL", proxy.url.as_str()),
        ];
        web_backend_nonempty_add_install(
            WebBackendCase {
                language: "go",
                framework: "gin",
                manifest_name: manifest,
                manifest_body: &body,
                schema,
                package: "github.com/google/uuid",
                manifest_pin: "github.com/google/uuid",
                lock_pin: "github.com/google/uuid",
            },
            &env,
        );
        proxy.assert_all_routes_used();

        let (manifest, body) = java_manifest();
        let maven = HermeticMaven::new();
        let env = [("MGC_MAVEN_REPO_URL", maven.url.as_str())];
        web_backend_nonempty_add_install(
            WebBackendCase {
                language: "java",
                framework: "spring-boot",
                manifest_name: manifest,
                manifest_body: &body,
                schema,
                package: "org.apache.commons:commons-lang3",
                manifest_pin: "commons-lang3",
                lock_pin: "commons-lang3",
            },
            &env,
        );
        maven.assert_all_routes_used();

        let (manifest, body) = dotnet_manifest();
        let nuget = HermeticNuget::new();
        let env = [("MGC_NUGET_INDEX_URL", nuget.index_url.as_str())];
        let lock_pin = if schema == "v4" {
            "newtonsoft.json@13.0.3"
        } else {
            "Newtonsoft.Json"
        };
        web_backend_nonempty_add_install(
            WebBackendCase {
                language: "dotnet",
                framework: "dotnet-webapi",
                manifest_name: manifest,
                manifest_body: &body,
                schema,
                package: "Newtonsoft.Json",
                manifest_pin: "Newtonsoft.Json",
                lock_pin,
            },
            &env,
        );
        nuget.assert_all_routes_used();
    }
}

pub(crate) fn matrix_web_rust_offline_reinstall_matches_capability_owners() {
    for framework in ["actix-web", "axum", "rust"] {
        for schema in ["3", "v4"] {
            let (manifest, body) = rust_manifest();
            let project = TempDir::new().unwrap();
            web_backend_project(project.path(), "rust", framework, manifest, &body, schema);
            let registry = HermeticCrates::new();
            let online_env = [
                ("MGC_CRATES_INDEX_URL", registry.index_url.as_str()),
                ("MGC_CRATES_DOWNLOAD_URL", registry.download_url.as_str()),
            ];
            let sandbox = MatrixSandbox::new();

            let (code, out) =
                sandbox.run_with_env(&["add-web", "mgcfixture"], project.path(), &online_env);
            assert_eq!(
                code,
                Some(0),
                "web/{framework} Rust add ({schema}) must succeed:\n{out}"
            );
            let (code, out) = sandbox.run_with_env(&["install-web"], project.path(), &online_env);
            assert_eq!(
                code,
                Some(0),
                "web/{framework} Rust install ({schema}) must succeed:\n{out}"
            );

            let mgc_lock_before = std::fs::read(project.path().join("mgc.lock")).unwrap();
            let cargo_lock_before = std::fs::read(project.path().join("Cargo.lock")).ok();
            let cached_source = sandbox
                .store_root()
                .join("cargo/registry/src/mgcfixture-1.0.0/Cargo.toml");
            assert!(
                cached_source.is_file(),
                "web/{framework} online install must materialize Rust source"
            );
            std::fs::remove_dir_all(cached_source.parent().unwrap()).unwrap();

            // Closed registry endpoints prove offline replay never falls back to the network.
            // Endpoint registry đóng chứng minh replay offline không rơi về truy cập mạng.
            let closed_registry = [
                ("MGC_CRATES_INDEX_URL", "http://127.0.0.1:1"),
                ("MGC_CRATES_DOWNLOAD_URL", "http://127.0.0.1:1"),
            ];
            let (code, out) = sandbox.run_with_env(
                &["install-web", "--offline"],
                project.path(),
                &closed_registry,
            );
            assert_eq!(
                code,
                Some(0),
                "web/{framework} Rust offline reinstall ({schema}) must use cache:\n{out}"
            );
            assert_eq!(
                std::fs::read(project.path().join("mgc.lock")).unwrap(),
                mgc_lock_before,
                "offline replay must preserve mgc.lock bytes"
            );
            assert_eq!(
                std::fs::read(project.path().join("Cargo.lock")).ok(),
                cargo_lock_before,
                "offline replay must preserve Cargo.lock bytes"
            );
            assert!(
                cached_source.is_file(),
                "offline replay must restore web/{framework} Rust source"
            );
            assert_eq!(
                std::fs::read_to_string(project.path().join(".mgc.core")).unwrap(),
                "web\n",
                "web/{framework} offline replay must preserve the web project owner"
            );
            sandbox.assert_no_spawn(&format!("web/{framework} Rust offline reinstall"));
            for mock in &registry._mocks {
                mock.assert();
            }
        }
    }
}
