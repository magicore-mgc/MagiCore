//! Local-generator tests — verify offline scaffolding per core.
//! Test local generator — xác minh scaffold offline theo từng core.
//!
//! Every case targets a fresh, not-yet-existing directory and asserts the files
//! the adapter generator owns. No network and no host toolchain is used, so a
//! green run here is proof the offline path works.
//! Mỗi ca nhắm vào thư mục mới chưa tồn tại và kiểm file generator adapter sở
//! hữu. Không dùng mạng lẫn toolchain host, nên test xanh là bằng chứng đường
//! offline chạy được.

#[cfg(feature = "iot")]
#[test]
fn iot_esp32_rust_scaffolds_offline() {
    let dir = target("iot-esp32");
    request("iot", "esp32-rust", &dir, Some("esp32c3"));
    assert!(dir.join("Cargo.toml").is_file(), "Cargo.toml missing");
    assert!(dir.join("mgc.toml").is_file(), "mgc.toml missing");
    assert!(dir.join("src/main.rs").is_file(), "src/main.rs missing");
    cleanup(&dir);
}

#[cfg(feature = "iot")]
#[test]
fn iot_default_board_for_framework_is_accepted() {
    // The create lane derives the board via `iot_default_board`; that board must
    // exist in the adapter registry or the generator would reject it.
    // (Create lane suy ra board qua `iot_default_board`; board đó phải có trong
    // registry adapter, nếu không generator sẽ từ chối.)
    let dir = target("iot-default-board");
    let board = crate::scaffold::framework_registry::iot_default_board("esp32-rust")
        .expect("esp32-rust has a registered default board");
    request("iot", "esp32-rust", &dir, Some(board));
    assert!(dir.join("mgc.toml").is_file(), "mgc.toml missing");
    cleanup(&dir);
}

#[cfg(feature = "game")]
#[test]
fn game_bevy_writes_cargo_project_offline() {
    let dir = target("game-bevy");
    request("game", "bevy", &dir, None);
    assert!(dir.join("Cargo.toml").is_file(), "Cargo.toml missing");
    assert!(dir.join("src/main.rs").is_file(), "src/main.rs missing");
    assert!(dir.join("mgc.toml").is_file(), "mgc.toml missing");
    cleanup(&dir);
}

#[cfg(feature = "clo")]
#[test]
fn cloud_terraform_writes_main_tf_offline() {
    let dir = target("clo-terraform");
    request("clo", "terraform", &dir, None);
    assert!(dir.join("main.tf").is_file(), "main.tf missing");
    cleanup(&dir);
}

#[cfg(feature = "ai")]
#[test]
fn ai_python_agent_writes_pyproject_and_package_offline() {
    let dir = target("ai-python-agent");
    request("ai", "python-agent", &dir, None);
    assert!(
        dir.join("pyproject.toml").is_file(),
        "pyproject.toml missing"
    );
    assert!(dir.join("src/agent.py").is_file(), "src/agent.py missing");
    cleanup(&dir);
}

#[cfg(feature = "ai")]
#[test]
fn ai_mcp_server_writes_stdio_server_offline() {
    let dir = target("ai-mcp-server");
    request("ai", "mcp-server", &dir, None);
    assert!(dir.join("server.py").is_file(), "server.py missing");
    assert!(
        dir.join("pyproject.toml").is_file(),
        "pyproject.toml missing"
    );
    cleanup(&dir);
}

#[test]
fn unknown_framework_has_no_local_generator() {
    // Fail-closed: a framework the adapter cannot parse returns false so the
    // caller keeps its registry/fail-closed path instead of writing nothing.
    // (Fail-closed: framework adapter không parse được trả false để caller giữ
    // đường registry/fail-closed thay vì ghi hư không.)
    let dir = target("unknown-fw");
    let generated = run("iot", "not-a-framework", &dir, None).expect("local scaffold");
    assert!(!generated, "unknown framework must not scaffold");
    cleanup(&dir);
}

#[test]
fn existing_target_fails_closed() {
    // A destination that already exists must be refused, never merged into.
    // (Đích đã tồn tại phải bị từ chối, không bao giờ hòa vào.)
    let dir = target("existing-target");
    std::fs::create_dir_all(&dir).expect("create existing target");
    let outcome = run("game", "bevy", &dir, None);
    assert!(outcome.is_err(), "existing target must fail closed");
    cleanup(&dir);
}

/// Build and run one local-scaffold request, returning whether it scaffolded.
/// Dựng và chạy một yêu cầu local scaffold, trả về đã scaffold hay chưa.
fn run(
    core: &str,
    framework: &str,
    dir: &std::path::Path,
    board: Option<&str>,
) -> anyhow::Result<bool> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    runtime.block_on(super::try_local_scaffold(&super::LocalScaffoldRequest {
        core,
        framework,
        target: dir,
        board,
    }))
}

/// Run a request that is expected to scaffold and panic otherwise.
/// Chạy yêu cầu chắc chắn scaffold, panic nếu không.
fn request(core: &str, framework: &str, dir: &std::path::Path, board: Option<&str>) {
    let generated = run(core, framework, dir, board).expect("local scaffold");
    assert!(
        generated,
        "expected '{core}/{framework}' to scaffold offline"
    );
}

/// A unique, not-yet-existing destination directory the generators may create.
/// Thư mục đích duy nhất, chưa tồn tại, để generator được phép tạo.
fn target(label: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "mgc-local-gen-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ))
}

fn cleanup(dir: &std::path::Path) {
    std::fs::remove_dir_all(dir).ok();
}
