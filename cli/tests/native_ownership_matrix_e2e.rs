//! V1.2 Capability Registry matrix: EVERY native-owned (core, language,
//! operation) cell MUST execute with ZERO toolchain spawn. Delegated /
//! unsupported cells MUST fail closed (refuse + zero spawn) without
//! --compat-runtime. One table, executable proof — a lane labeled Native
//! that spawns fails this file (false-native canary).
//! (Ma trận registry V1.2: cell Native zero-spawn, cell khác fail-closed.)
#![allow(clippy::unwrap_used)]
// The shared matrix modules intentionally import common fixtures across feature lanes.
#![allow(unused_imports)]
mod common;

#[path = "native_ownership_matrix_e2e/mod.rs"]
mod native_ownership_matrix_e2e;

#[test]
#[ignore = "requires Node.js; executed by the ownership CI matrices after setup-node"]
fn framework_npm_version_probe_uses_mgc_shim_with_canonical_windows_cwd() {
    native_ownership_matrix_e2e::test::canary::framework_npm_version_probe_uses_mgc_shim_with_canonical_windows_cwd();
}

#[test]
fn native_canary_resolves_launchers_without_recording_arguments() {
    native_ownership_matrix_e2e::test::canary::native_canary_resolves_launchers_without_recording_arguments();
}

#[test]
fn matrix_lib_python_add_install_list_frozen() {
    native_ownership_matrix_e2e::test::lib_lifecycle::matrix_lib_python_add_install_list_frozen();
}

#[test]
fn matrix_ai_python_native_list_remove_use_verified_adapter_without_tools() {
    native_ownership_matrix_e2e::test::ai_lifecycle::matrix_ai_python_native_list_remove_use_verified_adapter_without_tools();
}

#[test]
fn matrix_lib_rust_add_install() {
    native_ownership_matrix_e2e::test::lib_lifecycle::matrix_lib_rust_add_install();
}

#[test]
fn matrix_lib_rust_v4_add_install_build() {
    native_ownership_matrix_e2e::test::lib_lifecycle::matrix_lib_rust_v4_add_install_build();
}

#[test]
fn rust_lib_reuses_verified_shared_artifacts_across_project_roots() {
    native_ownership_matrix_e2e::test::lib_lifecycle::rust_lib_reuses_verified_shared_artifacts_across_project_roots();
}

#[test]
fn matrix_lib_go_add_install() {
    native_ownership_matrix_e2e::test::lib_lifecycle::matrix_lib_go_add_install();
}

#[test]
fn matrix_lib_dotnet_add_install() {
    native_ownership_matrix_e2e::test::lib_lifecycle::matrix_lib_dotnet_add_install();
}

#[test]
fn matrix_lib_java_add_install() {
    native_ownership_matrix_e2e::test::lib_lifecycle::matrix_lib_java_add_install();
}

#[test]
fn matrix_web_non_js_backends_resolve_nonempty_v3_and_v4_without_toolchains() {
    native_ownership_matrix_e2e::test::web_lifecycle::matrix_web_non_js_backends_resolve_nonempty_v3_and_v4_without_toolchains();
}

#[test]
fn matrix_web_rust_offline_reinstall_matches_capability_owners() {
    native_ownership_matrix_e2e::test::web_lifecycle::matrix_web_rust_offline_reinstall_matches_capability_owners();
}

#[test]
fn matrix_ai_python_add_install() {
    native_ownership_matrix_e2e::test::ai_lifecycle::matrix_ai_python_add_install();
}

#[test]
fn matrix_app_flutter_install() {
    native_ownership_matrix_e2e::test::app_lifecycle::matrix_app_flutter_install();
}

#[test]
fn matrix_native_remove_all_lanes() {
    native_ownership_matrix_e2e::test::native_remove::matrix_native_remove_all_lanes();
}

#[test]
fn matrix_ai_foreign_python_manifest_update_refuses_without_mutation() {
    native_ownership_matrix_e2e::test::ai_lifecycle::matrix_ai_foreign_python_manifest_update_refuses_without_mutation();
}

#[test]
fn matrix_cloud_dependency_commands_fail_closed_without_native_manifest() {
    native_ownership_matrix_e2e::test::cloud_capabilities::matrix_cloud_dependency_commands_fail_closed_without_native_manifest();
}

#[test]
fn cloud_dev_and_deploy_fail_closed_without_provider_cli_spawn() {
    native_ownership_matrix_e2e::test::cloud_capabilities::cloud_dev_and_deploy_fail_closed_without_provider_cli_spawn();
}

#[test]
fn capabilities_accepts_explicit_json_format_flag() {
    native_ownership_matrix_e2e::test::capabilities::capabilities_accepts_explicit_json_format_flag(
    );
}

#[test]
fn matrix_capabilities_binary_emits_closed_owner_for_every_framework_operation() {
    native_ownership_matrix_e2e::test::capabilities::matrix_capabilities_binary_emits_closed_owner_for_every_framework_operation();
}

#[test]
fn matrix_python_cached_reinstall_survives_registry_outage() {
    native_ownership_matrix_e2e::test::cache_registry_failure::matrix_python_cached_reinstall_survives_registry_outage();
}

#[test]
fn offline_flag_is_not_silently_dropped_for_other_cores() {
    native_ownership_matrix_e2e::test::cache_registry_failure::offline_flag_is_not_silently_dropped_for_other_cores();
}

#[test]
fn matrix_registry_outage_fails_closed_no_hang() {
    native_ownership_matrix_e2e::test::cache_registry_failure::matrix_registry_outage_fails_closed_no_hang();
}

#[test]
fn matrix_tampered_lock_fails_closed() {
    native_ownership_matrix_e2e::test::cache_registry_failure::matrix_tampered_lock_fails_closed();
}

#[test]
fn matrix_tampered_integrity_fails_verification() {
    native_ownership_matrix_e2e::test::cache_registry_failure::matrix_tampered_integrity_fails_verification();
}

#[test]
fn matrix_native_update_python_rust() {
    native_ownership_matrix_e2e::test::native_update::matrix_native_update_python_rust();
}

#[test]
fn matrix_native_update_go_dotnet_java() {
    native_ownership_matrix_e2e::test::native_update::matrix_native_update_go_dotnet_java();
}

#[test]
fn matrix_native_update_flutter_ai() {
    native_ownership_matrix_e2e::test::native_update::matrix_native_update_flutter_ai();
}

#[test]
fn matrix_game_bevy_add_install_native() {
    native_ownership_matrix_e2e::test::game_iot_hardware::matrix_game_bevy_add_install_native();
}

#[test]
fn matrix_iot_esp32_add_install_native() {
    native_ownership_matrix_e2e::test::game_iot_hardware::matrix_iot_esp32_add_install_native();
}

#[test]
fn matrix_iot_platformio_build_refuses_external_package_build_tool() {
    native_ownership_matrix_e2e::test::game_iot_hardware::matrix_iot_platformio_build_refuses_external_package_build_tool();
}

#[test]
fn matrix_hardware_platformio_build_refuses_external_package_build_tool() {
    native_ownership_matrix_e2e::test::game_iot_hardware::matrix_hardware_platformio_build_refuses_external_package_build_tool();
}

#[test]
fn matrix_native_update_game_iot() {
    native_ownership_matrix_e2e::test::native_update::matrix_native_update_game_iot();
}

#[test]
fn matrix_native_remove_flutter_swift_kotlin() {
    native_ownership_matrix_e2e::test::native_remove::matrix_native_remove_flutter_swift_kotlin();
}

#[test]
fn matrix_native_remove_game_iot() {
    native_ownership_matrix_e2e::test::native_remove::matrix_native_remove_game_iot();
}

#[test]
fn matrix_remove_rolls_back_on_injected_post_write_failure() {
    native_ownership_matrix_e2e::test::rollback::matrix_remove_rolls_back_on_injected_post_write_failure();
}

#[test]
fn matrix_remove_rolls_back_hermetic_registry() {
    native_ownership_matrix_e2e::test::rollback::matrix_remove_rolls_back_hermetic_registry();
}
