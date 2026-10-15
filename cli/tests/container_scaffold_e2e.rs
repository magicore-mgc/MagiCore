//! End-to-end checks that container flags are honored by the shipped CLI.
//! Kiểm E2E rằng flag container được CLI phát hành thực thi thật.

use std::process::Command;

fn assert_project_roots_owned_before_switching_user(dockerfile: &str) {
    let ownership = dockerfile
        .find("RUN chown 65532:65532 /app /mgc-data")
        .expect("project and shared data roots must be writable by the runtime user");
    let user = dockerfile
        .find("USER 65532:65532")
        .expect("container must drop root privileges");
    assert!(
        ownership < user,
        "root ownership must be set before dropping privileges"
    );
}

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

    let compose = std::fs::read_to_string(project.join("docker-compose.yml"))
        .expect("--docker must create Compose configuration");
    assert!(compose.contains("4315:4315"));
    assert!(compose.contains("mgc-data:/mgc-data"));
    assert!(compose.contains("magicore-system-data"));
    let readme = std::fs::read_to_string(project.join("README.md"))
        .expect("Docker usage must be part of the generated project guide");
    assert!(readme.contains("docker compose up --build"));
    assert!(readme.contains("copies that same lock"));
    let dockerignore = std::fs::read_to_string(project.join(".dockerignore"))
        .expect("--docker must create a build-context ignore list");
    assert!(dockerignore.contains("node_modules"));
    assert!(dockerignore.contains(".magicore/store"));
}

#[test]
fn create_web_k8s_implies_docker_and_generates_local_cluster_assets() {
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
        .args([
            "create-web",
            "react-vite",
            "cluster-web-project-name-that-is-longer-than-the-kubernetes-dns-label-limit-by-a-wide-margin",
            "--k8s",
        ])
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
        "mgc create-web --k8s failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let project_name = "cluster-web-project-name-that-is-longer-than-the-kubernetes-dns-label-limit-by-a-wide-margin";
    let project = temp.path().join(project_name);
    let dockerfile = std::fs::read_to_string(project.join("Dockerfile"))
        .expect("--k8s must imply the MGC-managed Docker build");
    assert!(dockerfile.contains("COPY --from=mgc-cli /usr/local/bin/mgc"));
    assert!(dockerfile.contains("RUN mgc install-web --frozen"));
    assert!(dockerfile.contains("ENTRYPOINT [\"/usr/local/bin/mgc-container-entrypoint\"]"));
    assert!(project.join("mgc-container-entrypoint").is_file());

    let manifest = std::fs::read_to_string(project.join("k8s/deployment.yaml"))
        .expect("--k8s must emit a Kubernetes deployment");
    let deployment_name = manifest
        .lines()
        .find_map(|line| line.trim().strip_prefix("name: "))
        .expect("deployment must have a Kubernetes-safe resource name");
    assert!(manifest.contains("kind: Deployment"));
    assert!(manifest.contains("kind: Service"));
    assert!(manifest.contains("startupProbe:"));
    assert!(manifest.contains("failureThreshold: 36"));
    assert!(deployment_name.len() <= 63);
    assert!(deployment_name.starts_with("magicore-"));
    assert!(manifest.contains(&format!("image: {deployment_name}:local")));
    assert!(manifest.contains("runAsNonRoot: true"));
    assert!(manifest.contains("automountServiceAccountToken: false"));
    assert!(manifest.contains("allowPrivilegeEscalation: false"));
    assert!(manifest.contains("drop: [ALL]"));
    assert!(!manifest.contains("mountPath: /mgc-data"));

    let readme = std::fs::read_to_string(project.join("k8s/README.md"))
        .expect("Kubernetes scope must be documented");
    assert!(readme.contains("not a production deployment profile"));
    assert!(readme.contains(&format!("kind load docker-image {deployment_name}:local")));
    assert!(readme.contains("mgc install-web --frozen"));
    assert!(readme.contains("copies the project lock"));
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
        let output = Command::new(env!("CARGO_BIN_EXE_mgc"))
            .args(["create-web", framework, project_name, "--docker"])
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
        if framework == "axum" {
            let source = std::fs::read_to_string(project.join("src/main.rs"))
                .expect("Axum scaffold must contain its server source");
            assert!(source.contains("0.0.0.0:4315"));
            assert!(source.contains("/api/health"));
        }
    }

    let output = Command::new(env!("CARGO_BIN_EXE_mgc"))
        .args(["create-web", "fastapi", "container-web-python-k8s", "--k8s"])
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
    assert!(manifest.contains("path: /api/health"));
    assert!(manifest.contains("mgc install-web --frozen"));
    let source = std::fs::read_to_string(temp.path().join("container-web-python-k8s/main.py"))
        .expect("FastAPI scaffold must contain its health endpoint");
    assert!(source.contains("@app.get(\"/api/health\")"));
}

#[test]
fn create_non_web_docker_profiles_run_lifecycle_through_mgc() {
    let temp = tempfile::tempdir().expect("temporary project root");
    let home = temp.path().join("home");
    let data = home.join("data");
    let cache = home.join("cache");
    let templates = home.join("templates");
    let store = home.join("store");
    for path in [&data, &cache, &templates, &store] {
        std::fs::create_dir_all(path).expect("isolated MGC directory");
    }

    for (args, project_name, expected_runtime, expected_command) in [
        (
            vec!["create-ai", "python-agent", "container-ai", "--docker"],
            "container-ai",
            "python:3.15.0-slim-trixie",
            "RUN mgc install --frozen",
        ),
        (
            vec!["create-clo", "cdk-typescript", "container-clo", "--docker"],
            "container-clo",
            "node:26.12.0-trixie-slim",
            "RUN mgc install --frozen",
        ),
        (
            vec!["create-lib", "rust", "container-lib", "--docker"],
            "container-lib",
            "rust:1.99.0-slim-trixie",
            "RUN mgc install --frozen",
        ),
        (
            vec!["create-lib", "go", "container-lib-go", "--docker"],
            "container-lib-go",
            "golang:1.27.2-trixie",
            "RUN mgc install --frozen",
        ),
        (
            vec!["create-lib", "python", "container-lib-python", "--docker"],
            "container-lib-python",
            "python:3.15.0-slim-trixie",
            "RUN mgc install --frozen",
        ),
        (
            vec![
                "create-lib",
                "typescript",
                "container-lib-typescript",
                "--docker",
            ],
            "container-lib-typescript",
            "node:26.12.0-trixie-slim",
            "RUN mgc install --frozen",
        ),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_mgc"))
            .args(args)
            .current_dir(temp.path())
            .env("HOME", &home)
            .env("USERPROFILE", &home)
            .env("MGC_DATA_DIR", &data)
            .env("MGC_CACHE_DIR", &cache)
            .env("MGC_TEMPLATES_DIR", &templates)
            .env("MAGICORE_STORE_ROOT", &store)
            .output()
            .expect("run MGC core scaffold");
        assert!(
            output.status.success(),
            "MGC Docker scaffold failed:\n{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );

        let project = temp.path().join(project_name);
        let dockerfile = std::fs::read_to_string(project.join("Dockerfile"))
            .expect("core --docker must generate a Dockerfile");
        assert!(dockerfile.contains(expected_runtime));
        assert!(dockerfile.contains("COPY --from=mgc-cli /usr/local/bin/mgc"));
        assert!(dockerfile.contains("MGC_DATA_DIR=/mgc-data"));
        assert!(dockerfile.contains("USER 65532:65532"));
        assert_project_roots_owned_before_switching_user(&dockerfile);
        assert!(dockerfile.contains("ENTRYPOINT [\"/usr/local/bin/mgc-container-entrypoint\"]"));
        assert!(dockerfile.contains(expected_command));
        if expected_runtime.starts_with("python:") {
            let pyproject = std::fs::read_to_string(project.join("pyproject.toml"))
                .expect("Python scaffold must include its PEP 735 dependency group");
            assert!(pyproject.contains("[dependency-groups]"));
            assert!(pyproject.contains("pytest==9.1.1"));
            assert!(pyproject.contains("build==1.6.1"));
            assert!(pyproject.contains("setuptools==84.0.0"));
        }
        assert!(dockerfile.contains("RUN mgc test"));
        assert!(dockerfile.contains("RUN mgc build"));
        assert!(!dockerfile.contains("pip install"));
        let entrypoint = std::fs::read_to_string(project.join("mgc-container-entrypoint"))
            .expect("core Docker profile must include its MGC entrypoint");
        assert!(entrypoint.contains("mgc install --frozen"));
        assert!(entrypoint.contains("exec mgc \"$@\""));
        let compose = std::fs::read_to_string(project.join("docker-compose.yml"))
            .expect("--docker must create Compose configuration");
        assert!(compose.contains("mgc-data:/mgc-data"));
        assert!(compose.contains("magicore-system-data"));
        assert!(project.join(".dockerignore").is_file());
    }

    let app_output = Command::new(env!("CARGO_BIN_EXE_mgc"))
        .args(["create-app", "flutter", "container-app", "--docker"])
        .current_dir(temp.path())
        .env("HOME", &home)
        .env("USERPROFILE", &home)
        .env("MGC_DATA_DIR", &data)
        .env("MGC_CACHE_DIR", &cache)
        .env("MGC_TEMPLATES_DIR", &templates)
        .env("MAGICORE_STORE_ROOT", &store)
        .output()
        .expect("run the MGC Flutter app scaffold");
    assert!(
        app_output.status.success(),
        "MGC Flutter Docker scaffold failed:\n{}\n{}",
        String::from_utf8_lossy(&app_output.stdout),
        String::from_utf8_lossy(&app_output.stderr)
    );
    let app_project = temp.path().join("container-app");
    let app_dockerfile = std::fs::read_to_string(app_project.join("Dockerfile"))
        .expect("Flutter --docker must write a Dockerfile");
    assert!(app_dockerfile.contains("debian:trixie-slim"));
    assert_project_roots_owned_before_switching_user(&app_dockerfile);
    assert!(app_dockerfile.contains("FLUTTER_VERSION=3.47.7"));
    assert!(
        app_dockerfile.contains("af7455f540df951184c7953fbe3cf884766e7541dacf8571f9f582db03dd6230")
    );
    assert!(app_dockerfile.contains("RUN mgc install --frozen && mgc test && mgc build"));
    assert!(app_dockerfile.contains("ENTRYPOINT [\"/usr/local/bin/mgc-container-entrypoint\"]"));
    let app_entrypoint = std::fs::read_to_string(app_project.join("mgc-container-entrypoint"))
        .expect("Flutter Docker profile must include its MGC entrypoint");
    assert!(app_entrypoint.contains("mgc install --frozen"));
    assert!(app_dockerfile.contains("sha256sum --check --strict"));
    assert!(!app_dockerfile.contains("dart pub get"));
    let app_compose = std::fs::read_to_string(app_project.join("docker-compose.yml"))
        .expect("Flutter --docker must write Compose configuration");
    assert!(app_compose.contains("mgc-data:/mgc-data"));
    assert!(app_compose.contains("magicore-system-data"));
}
