//! End-to-end checks that container flags are honored by the shipped CLI.
//! Kiểm E2E rằng flag container được CLI phát hành thực thi thật.

use std::path::Path;
use std::process::Command;

use serde::Deserialize;

fn assert_valid_k8s_manifest(manifest: &str, probe_path: &str) {
    let documents = serde_yaml::Deserializer::from_str(manifest)
        .map(serde_yaml::Value::deserialize)
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|error| panic!("invalid generated Kubernetes YAML: {error}"));
    assert_eq!(
        documents.len(),
        2,
        "manifest must contain Deployment and Service"
    );

    let deployment = &documents[0];
    assert_eq!(deployment["apiVersion"], "apps/v1");
    assert_eq!(deployment["kind"], "Deployment");
    assert_eq!(
        deployment["spec"]["template"]["spec"]["containers"][0]["startupProbe"]["httpGet"]["path"],
        probe_path
    );

    let service = &documents[1];
    assert_eq!(service["apiVersion"], "v1");
    assert_eq!(service["kind"], "Service");
    assert_eq!(service["spec"]["ports"][0]["targetPort"], "http");
}

fn assert_project_roots_owned_before_switching_user(dockerfile: &str) {
    let ownership = dockerfile
        .find("RUN chown 65532:65532 /app /mgc-data")
        .expect("project and shared data roots must be writable by the runtime user");
    let user = dockerfile
        .rfind("USER 65532:65532")
        .expect("container must drop root privileges");
    assert!(
        ownership < user,
        "root ownership must be set before dropping privileges"
    );
}

fn assert_mgc_tls_roots_are_available(dockerfile: &str) {
    assert!(
        dockerfile.contains(
            "COPY --from=mgc-cli /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt"
        ),
        "runtime image must carry the CA bundle used by MGC"
    );
    assert!(
        dockerfile.contains("SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt"),
        "MGC must use the copied CA bundle for HTTPS registry access"
    );
}

fn assert_container_readme(project: &Path) {
    let readme = std::fs::read_to_string(project.join("README.md"))
        .expect("container-enabled project must document its run command");
    assert!(readme.contains("docker compose up --build"));
    assert!(readme.contains("copies that same lock"));
    assert!(readme.contains("mgc install --frozen"));
}

fn assert_compose_configuration(project: &Path) {
    let compose_path = project.join("docker-compose.yml");
    let compose_text =
        std::fs::read_to_string(&compose_path).expect("--docker must create Compose configuration");
    let compose: serde_yaml::Value = serde_yaml::from_str(&compose_text).unwrap_or_else(|error| {
        panic!(
            "invalid Compose YAML at {}: {error}",
            compose_path.display()
        )
    });

    let service = compose
        .get("services")
        .and_then(|services| services.get("app"))
        .expect("Compose must define services.app");
    let service_volumes = service
        .get("volumes")
        .and_then(serde_yaml::Value::as_sequence)
        .expect("Compose app must mount the MGC shared data volume");
    assert!(
        service_volumes
            .iter()
            .any(|volume| volume.as_str() == Some("mgc-data:/mgc-data"))
    );

    let named_volume = compose
        .get("volumes")
        .and_then(|volumes| volumes.get("mgc-data"))
        .expect("Compose must declare the MGC named data volume");
    assert_eq!(
        named_volume.get("name").and_then(serde_yaml::Value::as_str),
        Some("${MGC_DATA_VOLUME:-magicore-system-data}")
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
    assert_mgc_tls_roots_are_available(&dockerfile);
    assert!(dockerfile.contains("COPY --from=mgc-cli /usr/local/bin/mgc"));
    assert!(dockerfile.contains("RUN mgc install-web --frozen"));
    assert!(dockerfile.contains("ENTRYPOINT [\"/usr/local/bin/mgc-container-entrypoint\"]"));
    assert!(project.join("mgc-container-entrypoint").is_file());

    let manifest = std::fs::read_to_string(project.join("k8s/deployment.yaml"))
        .expect("--k8s must emit a Kubernetes deployment");
    assert_valid_k8s_manifest(&manifest, "/");
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
        assert_mgc_tls_roots_are_available(&dockerfile);
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
        assert_compose_configuration(&project);
        assert!(project.join(".dockerignore").is_file());
        assert_container_readme(&project);
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
    assert_mgc_tls_roots_are_available(&app_dockerfile);
    assert_project_roots_owned_before_switching_user(&app_dockerfile);
    assert!(app_dockerfile.contains("FLUTTER_VERSION=3.47.7"));
    assert!(
        app_dockerfile.contains("af7455f540df951184c7953fbe3cf884766e7541dacf8571f9f582db03dd6230")
    );
    let flutter_runtime_stage = app_dockerfile
        .split_once("FROM ${FLUTTER_IMAGE}\n")
        .expect("Flutter image must start a separate runtime stage")
        .1;
    for argument in [
        "ARG FLUTTER_VERSION\n",
        "ARG FLUTTER_ARCHIVE_SHA256\n",
        "ARG FLUTTER_RELEASE_BASE_URL\n",
    ] {
        assert!(
            flutter_runtime_stage.contains(argument),
            "Flutter runtime-stage argument missing: {argument}"
        );
    }
    let flutter_sdk_ownership = app_dockerfile
        .find("RUN chown -R 65532:65532 /opt/flutter")
        .expect("Flutter SDK files must belong to the non-root runtime user");
    let flutter_user = app_dockerfile
        .find("USER 65532:65532\nRUN /opt/flutter/bin/flutter config")
        .expect("Flutter SDK config and precache must run as the SDK owner");
    let image_setup_root = app_dockerfile
        .find("USER 0:0")
        .expect("privileged image setup must explicitly switch back to root");
    assert!(
        flutter_sdk_ownership < flutter_user && flutter_user < image_setup_root,
        "Flutter SDK ownership must precede non-root config/precache and root image setup"
    );
    assert!(app_dockerfile.contains("RUN mgc install --frozen && mgc test && mgc build"));
    assert!(app_dockerfile.contains("ENTRYPOINT [\"/usr/local/bin/mgc-container-entrypoint\"]"));
    let app_entrypoint = std::fs::read_to_string(app_project.join("mgc-container-entrypoint"))
        .expect("Flutter Docker profile must include its MGC entrypoint");
    assert!(app_entrypoint.contains("mgc install --frozen"));
    assert_container_readme(&app_project);
    assert!(app_dockerfile.contains("sha256sum --check --strict"));
    assert!(!app_dockerfile.contains("dart pub get"));
    assert_compose_configuration(&app_project);
}
