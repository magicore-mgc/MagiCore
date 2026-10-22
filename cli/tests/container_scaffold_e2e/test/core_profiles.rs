//! AI, CLO and Lib Docker lifecycle E2E through MGC.
//! E2E lifecycle Docker cho AI, CLO và Lib qua MGC.

use std::process::Command;

use super::common::{
    assert_compose_configuration, assert_container_readme, assert_mgc_tls_roots_are_available,
    assert_project_roots_owned_before_switching_user,
};

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
}
