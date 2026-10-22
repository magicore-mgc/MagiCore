//! Kubernetes development manifest E2E through the shipped MGC CLI.
//! E2E manifest Kubernetes development qua CLI MGC phát hành.

use std::process::Command;

use super::common::{assert_mgc_tls_roots_are_available, assert_valid_k8s_manifest};

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
