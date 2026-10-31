use super::*;

/// Store path override read by `mgc_platform::paths::GlobalPaths`.
/// Biến ghi đè store root mà `GlobalPaths` đọc.
const STORE_ROOT_ENV: &str = "MAGICORE_STORE_ROOT";

/// Serializes process-wide environment writes across the cargo_compat tests.
/// Tuần tự hóa ghi environment toàn cục process giữa các test cargo_compat.
static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn rust_lock_projection_accepts_sibling_cores_but_lib_build_remains_scoped() {
    let mut game_package = locked_package("mgcfixture", &[]);
    game_package.owner_core = Some("game".to_string());
    let lock = Lockfile {
        packages: vec![game_package],
        ..Lockfile::new()
    };

    let projected = rust_packages(&lock).unwrap();
    assert_eq!(projected.len(), 1);
    let error = validate_build_ownership(&projected, "lib").unwrap_err();
    assert!(error.to_string().contains("owned by core 'game'"));

    let lib_package = locked_package("mgcfixture", &[]);
    let lib_lock = Lockfile {
        packages: vec![lib_package],
        ..Lockfile::new()
    };
    validate_build_ownership(&rust_packages(&lib_lock).unwrap(), "lib").unwrap();

    let mut iot_package = locked_package("mgcfixture", &[]);
    iot_package.owner_core = Some("iot".to_string());
    let iot_lock = Lockfile {
        packages: vec![iot_package],
        ..Lockfile::new()
    };
    validate_build_ownership(&rust_packages(&iot_lock).unwrap(), "iot").unwrap();
    let error = validate_build_ownership(&rust_packages(&iot_lock).unwrap(), "lib").unwrap_err();
    assert!(error.to_string().contains("owned by core 'iot'"));
}

#[test]
fn empty_cargo_projection_is_accepted_only_when_already_current() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"empty_demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    std::fs::write(project.path().join(".mgc.core"), "lib\n").unwrap();
    crate::install::write_canonical_lock(project.path(), EcosystemTag::Rust, Vec::new()).unwrap();
    let expected = crate::install::prepare_empty_cargo_lock(project.path(), &[])
        .unwrap()
        .expect("initial empty Cargo.lock projection must be created");
    crate::install::atomic_write_cargo_lock(project.path(), &expected).unwrap();

    assert!(validate_build(project.path()).unwrap().is_none());

    std::fs::write(
        project.path().join("Cargo.lock"),
        "version = 4\n\n[[package]]\nname = \"unlocked_extra\"\nversion = \"1.0.0\"\n",
    )
    .unwrap();
    let error = validate_build(project.path()).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("does not match the MGC dependency graph")
    );
}

/// The empty-graph refusal must say what an empty graph means, because the
/// common cause is a feature-gated dependency with no feature enabled — the
/// generated ESP32 starter that shipped `esp-hal = "1.2"` (no chip feature).
/// (Lỗi từ chối graph rỗng phải nói ý nghĩa của graph rỗng, vì nguyên nhân
/// thường gặp là dependency sau feature không bật feature nào — starter ESP32
/// từng ghi `esp-hal = "1.2"` thiếu chip feature.)
#[test]
fn empty_graph_refusal_names_the_feature_gated_cause() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"firmware\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nesp-hal = \"1.2\"\n",
    )
    .unwrap();
    let error = crate::install::prepare_empty_cargo_lock(project.path(), &[]).unwrap_err();
    let rendered = error.to_string();
    assert!(
        rendered.contains("refusing to write an empty Cargo.lock"),
        "{rendered}"
    );
    assert!(rendered.contains("esp-hal"), "{rendered}");
    assert!(rendered.contains("feature"), "{rendered}");
    assert!(
        !project.path().join("Cargo.lock").exists(),
        "no lock may be written for an unresolved declaration"
    );
}

#[cfg(unix)]
#[test]
fn managed_store_preflight_blocks_only_the_offline_build() {
    // Regression (P0): the preflight required every locked crate to exist in
    // the managed store before cargo started, in BOTH modes. An online build
    // can fetch those crates, so the check turned a resolvable build into
    // "run `mgc install`"; only the offline build has no other source.
    // (Lỗi gốc P0: preflight đòi mọi crate đã có trong store trước khi chạy
    // cargo ở CẢ hai chế độ. Build online tải được nên check này biến build
    // resolve được thành "run `mgc install`"; chỉ build offline mới không còn
    // nguồn khác.)
    let project = tempfile::tempdir().expect("temporary Cargo project");
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"storeless_demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nregex = \"1.13.1\"\n",
    )
    .expect("write Cargo.toml");
    std::fs::write(project.path().join(".mgc.core"), "lib\n").expect("write core marker");
    let mut package = locked_package("regex", &[]);
    package.version = "1.13.1".to_string();
    package.resolved = "https://static.crates.io/crates/regex/regex-1.13.1.crate".to_string();
    // The root pin must exist in the lock, otherwise the projection refuses a
    // graph whose manifest root has no owner entry.
    // (Root pin phải có trong lock, nếu không projection từ chối graph có root
    // manifest không có entry sở hữu.)
    let root_id = format!("{}@{}", package.name, package.version);
    crate::install::write_canonical_lock_with_roots(
        project.path(),
        EcosystemTag::Rust,
        vec![package.clone()],
        vec![root_id.clone()],
    )
    .expect("write MGC lock");
    let expected = project_cargo_lock(project.path(), &[&package], std::slice::from_ref(&root_id))
        .expect("project Cargo.lock");
    crate::install::atomic_write_cargo_lock(project.path(), &expected).expect("write Cargo.lock");

    // Point the store at an empty directory so no managed crate can exist.
    // (Trỏ store vào thư mục rỗng để chắc chắn không có crate managed nào.)
    let store = tempfile::tempdir().expect("temporary store");
    let _guard = ENV_LOCK.lock().unwrap_or_else(|error| error.into_inner());
    // SAFETY: the test process mutates its own environment under the test lock.
    // AN TOÀN: tiến trình test chỉ đổi environment của chính nó dưới khóa test.
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var(STORE_ROOT_ENV, store.path());
    }

    let online = validate_build_for_core_with_fetch(project.path(), "lib", false);
    let offline = validate_build_for_core_with_fetch(project.path(), "lib", true);
    // SAFETY: the test process mutates its own environment under the test lock.
    // AN TOÀN: tiến trình test chỉ đổi environment của chính nó dưới khóa test.
    #[allow(unsafe_code)]
    unsafe {
        std::env::remove_var(STORE_ROOT_ENV);
    }

    assert!(
        online.is_ok(),
        "an online build must not require the managed store: {online:?}"
    );
    let error = offline.expect_err("an offline build must fail closed without the store");
    assert!(
        error
            .to_string()
            .contains("managed Cargo source is missing"),
        "{error}"
    );
}
