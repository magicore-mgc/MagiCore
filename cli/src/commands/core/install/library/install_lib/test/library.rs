#![allow(clippy::unwrap_used)]
#![allow(unsafe_code)]

use super::*;

/// E-3: an existing binary on disk is installed, sealed and activated, and the
/// keg copy runs on its own.
/// (E-3: binary có sẵn trên đĩa được cài, seal và kích hoạt; bản trong keg
/// chạy được độc lập.)
#[test]
fn installs_an_existing_binary_into_the_keg() {
    let root = tempfile::tempdir().unwrap();
    let data = root.path().join("data");
    std::fs::create_dir_all(&data).unwrap();
    // SAFETY: test-only env mutation; this binary's tests run single-threaded
    // and each restores the variable through `ScopedEnv`.
    // AN TOÀN: chỉ test đổi env; test của binary này chạy đơn luồng và mỗi lần
    // đều khôi phục biến qua `ScopedEnv`.
    // The keg store derives from MGC_DATA_DIR, so every test that installs a
    // fixture holds the process-wide lock instead of racing another test's
    // environment.
    // (Keg store suy ra từ MGC_DATA_DIR, nên mọi test cài fixture giữ lock toàn
    // process thay vì tranh env với test khác.)
    let _lock = test_env_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let _data = ScopedEnv::set("MGC_DATA_DIR", data.to_str().unwrap());

    let source = root.path().join("mgc-probe-runner");
    std::fs::write(&source, b"#!/bin/sh\necho probe-ok\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&source, std::fs::Permissions::from_mode(0o755)).unwrap();
    }

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let outcome = runtime
        .block_on(execute(InstallLibRequest {
            target: source.to_str().unwrap(),
            manifest: None,
            key_id: None,
            trust_keys: Vec::new(),
        }))
        .expect("install must succeed for an existing executable");

    assert_eq!(outcome.name, "mgc-probe-runner");
    assert!(!outcome.reused);
    let installed = outcome.keg_dir.join("mgc-probe-runner");
    assert!(installed.is_file(), "the keg must hold the artifact");

    // The installed copy runs; a keg that only records metadata would fail
    // here.
    // (Bản đã cài chạy được; keg chỉ ghi metadata sẽ fail ở đây.)
    #[cfg(unix)]
    {
        let output = std::process::Command::new(&installed)
            .output()
            .expect("installed binary must run");
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "probe-ok");
    }

    // Re-installing the same version reuses the sealed keg instead of writing
    // it again.
    // (Cài lại cùng version dùng lại keg đã seal thay vì ghi lại.)
    let again = runtime
        .block_on(execute(InstallLibRequest {
            target: source.to_str().unwrap(),
            manifest: None,
            key_id: None,
            trust_keys: Vec::new(),
        }))
        .expect("re-install must succeed");
    assert!(again.reused, "the second install must reuse the keg");
}

/// A target that exists nowhere fails with guidance and installs nothing.
/// (Target không tồn tại ở đâu báo lỗi có hướng dẫn và không cài gì.)
#[test]
fn missing_target_fails_without_creating_a_keg() {
    let root = tempfile::tempdir().unwrap();
    let data = root.path().join("data");
    std::fs::create_dir_all(&data).unwrap();
    // The keg store derives from MGC_DATA_DIR, so every test that installs a
    // fixture holds the process-wide lock instead of racing another test's
    // environment.
    // (Keg store suy ra từ MGC_DATA_DIR, nên mọi test cài fixture giữ lock toàn
    // process thay vì tranh env với test khác.)
    let _lock = test_env_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let _data = ScopedEnv::set("MGC_DATA_DIR", data.to_str().unwrap());

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let error = runtime
        .block_on(execute(InstallLibRequest {
            target: "mgc-probe-definitely-missing",
            manifest: None,
            key_id: None,
            trust_keys: Vec::new(),
        }))
        .expect_err("a missing binary must fail");
    assert!(error.to_string().contains("is not on PATH"));
    assert!(!data.join("lib").join("Cellar").exists());
}

/// A manifest without a pinned trust key never reaches the download step.
/// (Manifest không có key pin không bao giờ tới bước tải.)
#[test]
fn manifest_without_trust_key_is_refused_before_download() {
    let root = tempfile::tempdir().unwrap();
    let data = root.path().join("data");
    std::fs::create_dir_all(&data).unwrap();
    // The keg store derives from MGC_DATA_DIR, so every test that installs a
    // fixture holds the process-wide lock instead of racing another test's
    // environment.
    // (Keg store suy ra từ MGC_DATA_DIR, nên mọi test cài fixture giữ lock toàn
    // process thay vì tranh env với test khác.)
    let _lock = test_env_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let _data = ScopedEnv::set("MGC_DATA_DIR", data.to_str().unwrap());
    let _key = ScopedEnv::set(TRUST_KEY_ENV, "");

    let manifest_path = root.path().join("manifest.json");
    std::fs::write(
        &manifest_path,
        r#"{"entry":{"name":"x","version":"1.0.0","target":"t","url":"https://example.invalid/x","blake3":"00"},"signature":"AA==","public_key":"AA=="}"#,
    )
    .unwrap();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let error = runtime
        .block_on(execute(InstallLibRequest {
            target: "x@1.0.0",
            manifest: Some(&manifest_path),
            key_id: None,
            trust_keys: Vec::new(),
        }))
        .expect_err("untrusted manifest must fail");
    assert!(error.to_string().contains("binary manifest rejected"));
    assert!(!data.join("lib").join("Cellar").exists());
}

/// Version banners are parsed without inventing a version.
/// (Banner version được parse mà không bịa version.)
#[test]
fn version_banner_parsing_is_strict() {
    assert_eq!(parse_version("cargo 1.94.0 (abc 2026-01-01)"), "1.94.0");
    assert_eq!(parse_version("no version here"), "0.0.0");
    assert_eq!(parse_version("v2"), "0.0.0");
}

/// Scope guard that restores an environment variable.
/// Guard khôi phục biến môi trường.
struct ScopedEnv {
    name: &'static str,
    previous: Option<std::ffi::OsString>,
}

impl ScopedEnv {
    fn set(name: &'static str, value: &str) -> Self {
        let previous = std::env::var_os(name);
        // SAFETY: test-only env mutation; the owning test restores the value on
        // drop so no other test observes a torn environment.
        // AN TOÀN: chỉ test đổi env; test sở hữu khôi phục giá trị khi drop để
        // test khác không thấy môi trường ghi dở.
        unsafe { std::env::set_var(name, value) };
        Self { name, previous }
    }
}

impl Drop for ScopedEnv {
    fn drop(&mut self) {
        match self.previous.take() {
            Some(value) => unsafe { std::env::set_var(self.name, value) },
            None => unsafe { std::env::remove_var(self.name) },
        }
    }
}
