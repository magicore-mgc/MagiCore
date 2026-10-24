//! Web and native-backend container lifecycle E2E.
//! E2E lifecycle container Web và backend native.

use std::process::Command;

use super::common::{
    assert_compose_configuration, assert_container_readme, assert_mgc_tls_roots_are_available,
    assert_project_roots_owned_before_switching_user, assert_valid_k8s_manifest,
};

#[test]
fn create_web_without_docker_does_not_emit_container_assets() {
    let temp = tempfile::tempdir().expect("temporary project root");
    let home = temp.path().join("home");
    let data = home.join("data");
    let cache = home.join("cache");
    let templates = home.join("templates");
    let store = home.join("store");
    for path in [&data, &cache, &templates, &store] {
        std::fs::create_dir_all(path).expect("isolated MGC directory");
    }

    let output = Command::new(env!("CARGO_BIN_EXE_mgc"))
        .args(["create-web", "react-vite", "plain-web"])
        .current_dir(temp.path())
        .env("HOME", &home)
        .env("USERPROFILE", &home)
        .env("MGC_DATA_DIR", &data)
        .env("MGC_CACHE_DIR", &cache)
        .env("MGC_TEMPLATES_DIR", &templates)
        .env("MAGICORE_STORE_ROOT", &store)
        .output()
        .expect("run the MagiCore CLI scaffold");
    assert!(
        output.status.success(),
        "mgc create-web without --docker failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let project = temp.path().join("plain-web");
    for asset in [
        "Dockerfile",
        ".dockerignore",
        "docker-compose.yml",
        "mgc-container-entrypoint",
    ] {
        assert!(
            !project.join(asset).exists(),
            "container asset appeared without --docker: {asset}"
        );
    }
    assert!(!project.join("k8s").exists());
}

#[test]
fn create_web_docker_generates_mgc_managed_container_assets() {
    let temp = tempfile::tempdir().expect("temporary project root");
    let home = temp.path().join("home");
    let data = home.join("data");
    let cache = home.join("cache");
    let templates = home.join("templates");
    let store = home.join("store");
    for path in [&data, &cache, &templates, &store] {
        std::fs::create_dir_all(path).expect("isolated MGC directory");
    }

    let output = Command::new(env!("CARGO_BIN_EXE_mgc"))
        .args(["create-web", "react-vite", "container-web", "--docker"])
        .current_dir(temp.path())
        .env("HOME", &home)
        .env("USERPROFILE", &home)
        .env("MGC_DATA_DIR", &data)
        .env("MGC_CACHE_DIR", &cache)
        .env("MGC_TEMPLATES_DIR", &templates)
        .env("MAGICORE_STORE_ROOT", &store)
        .output()
        .expect("run the MagiCore CLI scaffold");
    assert!(
        output.status.success(),
        "mgc create-web --docker failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let project = temp.path().join("container-web");
    let dockerfile = std::fs::read_to_string(project.join("Dockerfile"))
        .expect("--docker must create a Dockerfile from the product CLI");
    assert!(dockerfile.contains("node:26.12.0-trixie-slim"));
    assert!(dockerfile.contains("COPY --from=mgc-cli /usr/local/bin/mgc"));
    assert!(dockerfile.contains("ENV MGC_DATA_DIR=/mgc-data"));
    assert_mgc_tls_roots_are_available(&dockerfile);
    assert!(dockerfile.contains("USER 65532:65532"));
    assert_project_roots_owned_before_switching_user(&dockerfile);
    assert!(dockerfile.contains("RUN mgc install-web --frozen"));
    assert!(dockerfile.contains("RUN mgc test"));
    assert!(dockerfile.contains("ENTRYPOINT [\"/usr/local/bin/mgc-container-entrypoint\"]"));
    assert!(dockerfile.contains("CMD [\"dev\", \"--host\", \"0.0.0.0\""));
    let entrypoint = std::fs::read_to_string(project.join("mgc-container-entrypoint"))
        .expect("Docker profile must include its MGC entrypoint");
    assert!(entrypoint.contains("mgc install-web --frozen"));
    assert!(entrypoint.contains("exec mgc \"$@\""));
    for forbidden in [
        "RUN npm install",
        "RUN pnpm install",
        "RUN yarn install",
        "RUN bun install",
    ] {
        assert!(
            !dockerfile.contains(forbidden),
            "unmanaged install in Dockerfile: {forbidden}"
        );
    }

    assert_compose_configuration(&project);
    let compose = std::fs::read_to_string(project.join("docker-compose.yml"))
        .expect("--docker must create Compose configuration");
    assert!(compose.contains("4315:4315"));
    assert_container_readme(&project);
    let dockerignore = std::fs::read_to_string(project.join(".dockerignore"))
        .expect("--docker must create a build-context ignore list");
    assert!(dockerignore.contains("node_modules"));
    assert!(dockerignore.contains(".magicore/store"));
}

#[test]
fn create_web_native_backend_docker_profiles_keep_dependencies_under_mgc() {
    let temp = tempfile::tempdir().expect("temporary project root");
    let home = temp.path().join("home");
    let data = home.join("data");
    let cache = home.join("cache");
    let templates = home.join("templates");
    let store = home.join("store");
    for path in [&data, &cache, &templates, &store] {
        std::fs::create_dir_all(path).expect("isolated MGC directory");
    }

    for (framework, project_name, runtime_image, runtime_contract) in [
        (
            "axum",
            "container-web-rust",
            "rust:1.99.0-slim-trixie",
            "ENV CARGO_NET_OFFLINE=true",
        ),
        (
            "fastapi",
            "container-web-python",
            "python:3.15.0-slim-trixie",
            "ENV PYTHONDONTWRITEBYTECODE=1 PYTHONUNBUFFERED=1",
        ),
    ] {
        // Web framework source comes from an existing project of the same framework.
        // Source framework Web lấy từ project có sẵn cùng framework.
        let source = crate::common::local_source(temp.path(), "web", framework);
        let output = Command::new(env!("CARGO_BIN_EXE_mgc"))
            .args(["create-web", framework, project_name, "--docker"])
            .arg(source.flag())
            .current_dir(temp.path())
            .env("HOME", &home)
            .env("USERPROFILE", &home)
            .env("MGC_DATA_DIR", &data)
            .env("MGC_CACHE_DIR", &cache)
            .env("MGC_TEMPLATES_DIR", &templates)
            .env("MAGICORE_STORE_ROOT", &store)
            .output()
            .expect("run MGC native web container scaffold");
        assert!(
            output.status.success(),
            "{framework} Docker scaffold failed:\n{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );

        let project = temp.path().join(project_name);
        let dockerfile = std::fs::read_to_string(project.join("Dockerfile"))
            .expect("native Web --docker must produce a Dockerfile");
        assert!(dockerfile.contains(runtime_image));
        assert!(dockerfile.contains(runtime_contract));
        assert!(dockerfile.contains("COPY --from=mgc-cli /usr/local/bin/mgc"));
        assert_mgc_tls_roots_are_available(&dockerfile);
        assert!(dockerfile.contains("RUN mgc install-web --frozen"));
        assert!(dockerfile.contains("USER 65532:65532"));
        assert_project_roots_owned_before_switching_user(&dockerfile);
        assert!(dockerfile.contains("ENTRYPOINT [\"/usr/local/bin/mgc-container-entrypoint\"]"));
        assert!(!dockerfile.contains("pip install"));
        assert!(!dockerfile.contains("go mod download"));
        assert!(!dockerfile.contains("cargo fetch"));
        let entrypoint = std::fs::read_to_string(project.join("mgc-container-entrypoint"))
            .expect("native Web container must include the MGC entrypoint");
        assert!(entrypoint.contains("mgc install-web --frozen"));
        assert!(entrypoint.contains("exec mgc \"$@\""));
        assert_container_readme(&project);
        if framework == "axum" {
            let source = std::fs::read_to_string(project.join("src/main.rs"))
                .expect("Axum scaffold must contain its server source");
            assert!(source.contains("0.0.0.0:4315"));
            assert!(source.contains("/api/health"));
            let manifest = std::fs::read_to_string(project.join("Cargo.toml"))
                .expect("Axum scaffold must declare its Cargo dependencies");
            assert!(manifest.contains("axum"));
            assert!(manifest.contains("tokio"));
        } else {
            let source = std::fs::read_to_string(project.join("main.py"))
                .expect("FastAPI scaffold must contain its server source");
            assert!(source.contains("@app.get(\"/api/health\")"));
            let manifest = std::fs::read_to_string(project.join("pyproject.toml"))
                .expect("FastAPI scaffold must declare its Python dependencies");
            assert!(manifest.contains("fastapi"));
            assert!(manifest.contains("uvicorn"));
        }
    }

    let k8s_source = crate::common::local_source(temp.path(), "web", "fastapi");
    let output = Command::new(env!("CARGO_BIN_EXE_mgc"))
        .args(["create-web", "fastapi", "container-web-python-k8s", "--k8s"])
        .arg(k8s_source.flag())
        .current_dir(temp.path())
        .env("HOME", &home)
        .env("USERPROFILE", &home)
        .env("MGC_DATA_DIR", &data)
        .env("MGC_CACHE_DIR", &cache)
        .env("MGC_TEMPLATES_DIR", &templates)
        .env("MAGICORE_STORE_ROOT", &store)
        .output()
        .expect("run MGC Web Kubernetes scaffold");
    assert!(
        output.status.success(),
        "FastAPI Kubernetes scaffold failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let manifest = std::fs::read_to_string(
        temp.path()
            .join("container-web-python-k8s/k8s/deployment.yaml"),
    )
    .expect("native Web Kubernetes profile must generate deployment manifest");
    assert_valid_k8s_manifest(&manifest, "/api/health");
    assert!(manifest.contains("path: /api/health"));
    let dockerfile =
        std::fs::read_to_string(temp.path().join("container-web-python-k8s/Dockerfile"))
            .expect("native Web Kubernetes profile must generate its MGC Dockerfile");
    assert!(dockerfile.contains("RUN mgc install-web --frozen"));
    let source = std::fs::read_to_string(temp.path().join("container-web-python-k8s/main.py"))
        .expect("FastAPI scaffold must contain its health endpoint");
    assert!(source.contains("@app.get(\"/api/health\")"));
}
