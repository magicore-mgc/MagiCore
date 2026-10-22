//! Game Iot Hardware lifecycle E2E cases. / Test E2E lifecycle cho game iot hardware.

use crate::native_ownership_matrix_e2e::support::{
    HermeticCrates, MatrixSandbox, bevy_project, esp32_project, write,
};
use tempfile::TempDir;

pub(crate) fn matrix_game_bevy_add_install_native() {
    let project = TempDir::new().unwrap();
    bevy_project(project.path());
    let registry = HermeticCrates::new();
    let env = [
        ("MGC_CRATES_INDEX_URL", registry.index_url.as_str()),
        ("MGC_CRATES_DOWNLOAD_URL", registry.download_url.as_str()),
    ];
    let sandbox = MatrixSandbox::new();
    let (code, out) = sandbox.run_with_env(&["add-game", "mgcfixture"], project.path(), &env);
    assert_eq!(code, Some(0), "native bevy `add-game` must succeed:\n{out}");
    let body = std::fs::read_to_string(project.path().join("Cargo.toml")).unwrap();
    assert!(
        body.contains("mgcfixture"),
        "Cargo.toml must pin mgcfixture:\n{body}"
    );
    sandbox.assert_no_spawn("add-game");
    let (code, out) = sandbox.run_with_env(&["install-game"], project.path(), &env);
    assert_eq!(
        code,
        Some(0),
        "native bevy `install-game` must succeed:\n{out}"
    );
    sandbox.assert_no_spawn("install-game");
    let lock = std::fs::read_to_string(project.path().join("mgc.lock")).unwrap_or_default();
    assert!(
        lock.contains("mgcfixture"),
        "mgc.lock must record mgcfixture:\n{lock}"
    );
}

pub(crate) fn matrix_iot_esp32_add_install_native() {
    let project = TempDir::new().unwrap();
    esp32_project(project.path());
    let registry = HermeticCrates::new();
    let env = [
        ("MGC_CRATES_INDEX_URL", registry.index_url.as_str()),
        ("MGC_CRATES_DOWNLOAD_URL", registry.download_url.as_str()),
    ];
    let sandbox = MatrixSandbox::new();
    let (code, out) = sandbox.run_with_env(&["add-iot", "mgcfixture"], project.path(), &env);
    assert_eq!(code, Some(0), "native esp32 `add-iot` must succeed:\n{out}");
    let body = std::fs::read_to_string(project.path().join("Cargo.toml")).unwrap();
    assert!(
        body.contains("mgcfixture"),
        "Cargo.toml must pin mgcfixture:\n{body}"
    );
    sandbox.assert_no_spawn("add-iot");
    let (code, out) = sandbox.run_with_env(&["install-iot"], project.path(), &env);
    assert_eq!(
        code,
        Some(0),
        "native esp32 `install-iot` must succeed:\n{out}"
    );
    sandbox.assert_no_spawn("install-iot");
    let lock = std::fs::read_to_string(project.path().join("mgc.lock")).unwrap_or_default();
    assert!(
        lock.contains("mgcfixture"),
        "mgc.lock must record mgcfixture:\n{lock}"
    );
}

pub(crate) fn matrix_iot_platformio_build_refuses_external_package_build_tool() {
    let project = TempDir::new().unwrap();
    write(
        project.path(),
        "mgc.toml",
        "name = \"m\"\necosystem = \"iot\"\n[iot]\nframework = \"platformio\"\n",
    );
    write(
        project.path(),
        "platformio.ini",
        "[env:esp32dev]\nplatform = espressif32\n",
    );
    let sandbox = MatrixSandbox::new();
    let (code, output) = sandbox.run(&["build"], project.path());
    assert_ne!(
        code,
        Some(0),
        "PlatformIO build must not be reported as a MagiCore-native build:\n{output}"
    );
    assert!(
        output.contains("native PlatformIO build backend"),
        "the refusal must identify the missing MagiCore builder:\n{output}"
    );
    sandbox.assert_no_spawn("build iot/platformio");
}

pub(crate) fn matrix_hardware_platformio_build_refuses_external_package_build_tool() {
    let project = TempDir::new().unwrap();
    write(
        project.path(),
        "mgc.toml",
        "name = \"m\"\necosystem = \"hardware\"\n",
    );
    write(
        project.path(),
        "platformio.ini",
        "[env:board]\nplatform = native\n",
    );
    let sandbox = MatrixSandbox::new();
    let (code, output) = sandbox.run(&["build"], project.path());
    assert_ne!(
        code,
        Some(0),
        "Hardware PlatformIO build must not be reported as MagiCore-native:\n{output}"
    );
    assert!(
        output.contains("native PlatformIO build backend"),
        "the refusal must identify the missing MagiCore builder:\n{output}"
    );
    sandbox.assert_no_spawn("build hardware/platformio");
}
