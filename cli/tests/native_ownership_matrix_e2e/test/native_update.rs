//! Native Update lifecycle E2E cases. / Test E2E lifecycle cho native update.

use crate::native_ownership_matrix_e2e::support::{
    HermeticCrates, HermeticGo, HermeticMaven, HermeticNuget, HermeticPub, HermeticPypi,
    native_update_cell_with_env, outdated_ai, outdated_bevy, outdated_dotnet, outdated_esp32,
    outdated_flutter, outdated_go, outdated_java, outdated_python, outdated_rust,
};

pub(crate) fn matrix_native_update_python_rust() {
    let pypi = HermeticPypi::new(&[("six", "1.17.0")]);
    let python_env = [("MGC_PYPI_INDEX_URL", pypi.url.as_str())];
    native_update_cell_with_env(
        outdated_python,
        &["update-lib", "six"],
        "pyproject.toml",
        "1.15.0",
        &python_env,
    );
    let crates = HermeticCrates::new();
    let rust_env = [
        ("MGC_CRATES_INDEX_URL", crates.index_url.as_str()),
        ("MGC_CRATES_DOWNLOAD_URL", crates.download_url.as_str()),
    ];
    native_update_cell_with_env(
        outdated_rust,
        &["update-lib", "mgcfixture"],
        "Cargo.toml",
        "0.9.0",
        &rust_env,
    );
}

pub(crate) fn matrix_native_update_go_dotnet_java() {
    let proxy = HermeticGo::new();
    let go_env = [
        ("MGC_GO_PROXY_URL", proxy.url.as_str()),
        ("MGC_GO_SUM_URL", proxy.url.as_str()),
    ];
    native_update_cell_with_env(
        outdated_go,
        &["update-lib", "github.com/google/uuid"],
        "go.mod",
        "v0.9.0",
        &go_env,
    );
    let nuget = HermeticNuget::new();
    let nuget_env = [("MGC_NUGET_INDEX_URL", nuget.index_url.as_str())];
    native_update_cell_with_env(
        outdated_dotnet,
        &["update-lib", "Newtonsoft.Json"],
        "m.csproj",
        "13.0.1",
        &nuget_env,
    );
    let maven = HermeticMaven::new();
    let maven_env = [("MGC_MAVEN_REPO_URL", maven.url.as_str())];
    native_update_cell_with_env(
        outdated_java,
        &["update-lib", "org.apache.commons:commons-lang3"],
        "pom.xml",
        "3.12.0",
        &maven_env,
    );
    proxy.assert_resolution_routes_used();
    nuget.assert_resolution_routes_used();
    maven.assert_resolution_routes_used();
}

pub(crate) fn matrix_native_update_flutter_ai() {
    let pub_registry = HermeticPub::new("meta", "1.13.0");
    let pub_env = [("MGC_PUB_INDEX_URL", pub_registry.url.as_str())];
    native_update_cell_with_env(
        outdated_flutter,
        &["update-app", "meta"],
        "pubspec.yaml",
        "1.9.1",
        &pub_env,
    );
    pub_registry.assert_metadata_route_used();
    let pypi = HermeticPypi::new(&[("six", "1.17.0")]);
    let pypi_env = [("MGC_PYPI_INDEX_URL", pypi.url.as_str())];
    native_update_cell_with_env(
        outdated_ai,
        &["update-ai", "six"],
        "pyproject.toml",
        "1.15.0",
        &pypi_env,
    );
    pypi.assert_metadata_routes_used();
}

pub(crate) fn matrix_native_update_game_iot() {
    let registry = HermeticCrates::new();
    let env = [
        ("MGC_CRATES_INDEX_URL", registry.index_url.as_str()),
        ("MGC_CRATES_DOWNLOAD_URL", registry.download_url.as_str()),
    ];
    native_update_cell_with_env(
        outdated_bevy,
        &["update-game", "mgcfixture"],
        "Cargo.toml",
        "0.9.0",
        &env,
    );
    native_update_cell_with_env(
        outdated_esp32,
        &["update-iot", "mgcfixture"],
        "Cargo.toml",
        "0.9.0",
        &env,
    );
}
