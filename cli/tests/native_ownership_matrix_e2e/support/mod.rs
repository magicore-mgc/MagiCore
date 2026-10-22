//! Shared test support owner exports. / Export owner helper dùng chung cho test.

mod cli;
mod lifecycle;
mod projects;
mod registries;
mod sandbox;

pub(crate) use cli::{mgc_binary, run_product_cli_direct_bounded};

pub(crate) use sandbox::{ALL_TOOLS, MatrixSandbox, canary_executable};

pub(crate) use projects::{
    WebBackendCase, ai_project, bevy_project, bevy_with_dep, dotnet_manifest, esp32_project,
    esp32_with_dep, flutter_project, flutter_with_meta, go_manifest, java_manifest,
    kotlin_with_dep, lib_project, python_manifest, rust_manifest, swift_with_dep,
    web_backend_nonempty_add_install, web_backend_project, write,
};

pub(crate) use lifecycle::{
    blocked_mutation_unchanged, native_add_cell_with_env, native_install_cell_with_env,
    native_remove_cell, native_remove_cell_with_env, native_remove_only,
    native_remove_only_with_env, native_update_cell_with_env, outdated_ai, outdated_bevy,
    outdated_dotnet, outdated_esp32, outdated_flutter, outdated_go, outdated_java, outdated_python,
    outdated_rust,
};

pub(crate) use registries::{
    HermeticCrates, HermeticGo, HermeticMaven, HermeticNuget, HermeticPub, HermeticPypi,
    PYPI_MOCKS_PER_PACKAGE, build_test_crate_archive, build_test_go_module_zip,
    build_test_nuget_package, build_test_pub_archive, build_test_wheel,
};
