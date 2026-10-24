//! Flutter App Docker lifecycle E2E through MGC.
//! E2E lifecycle Docker Flutter App qua MGC.

use std::process::Command;

use super::common::{
    assert_compose_configuration, assert_container_readme, assert_mgc_tls_roots_are_available,
    assert_project_roots_owned_before_switching_user,
};

#[test]
fn create_flutter_app_docker_profile_runs_lifecycle_through_mgc() {
    let temp = tempfile::tempdir().expect("temporary project root");
    let home = temp.path().join("home");
    let data = home.join("data");
    let cache = home.join("cache");
    let templates = home.join("templates");
    let store = home.join("store");
    for path in [&data, &cache, &templates, &store] {
        std::fs::create_dir_all(path).expect("isolated MGC directory");
    }

    // Flutter framework source comes from an existing project.
    // Source framework Flutter lấy từ project có sẵn.
    let app_source = crate::common::local_source(temp.path(), "app", "flutter");
    let app_output = Command::new(env!("CARGO_BIN_EXE_mgc"))
        .args(["create-app", "flutter", "container-app", "--docker"])
        .arg(app_source.flag())
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
