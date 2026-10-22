//! Package Operations behavior tests.
//! Test hành vi cho nhóm package operations.

use crate::integration::support::tmp;
use mgc_game_adapter::adapter_for;
use mgc_types::PackageName;
use mgc_types::adapter::{AddOptions, PackageAdapter};
use mgc_types::capabilities::{ContentStoreProvider, DependencyResolver};

pub async fn godot_install_fails_closed_pointing_to_editor() {
    // Godot projects have no dependency install step — install must fail
    // closed with typed Unsupported instead of a silent Ok.
    // Project Godot không có bước install dependency — install phải
    // fail-closed typed Unsupported thay vì Ok im lặng.
    let dir = tmp("install-godot");
    std::fs::write(dir.join("project.godot"), "[application]\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    let manifest = a.parse_manifest(&dir).await.unwrap();

    let resolve_err = a.resolve(&manifest).await.unwrap_err();
    assert!(
        matches!(resolve_err, mgc_types::MgError::Unsupported { core, .. } if core == "game"),
        "resolve must be typed-unsupported, got: {resolve_err}"
    );

    let graph = mgc_types::ResolvedGraph::default();
    let install_err = a
        .install(&graph, &dir, Default::default())
        .await
        .unwrap_err();
    assert!(
        matches!(install_err, mgc_types::MgError::Unsupported { core, capability, .. }
            if core == "game" && capability == "install"),
        "install must be typed-unsupported, got: {install_err}"
    );
}

pub async fn godot_add_fails_closed_until_mgc_owns_dependency_resolution() {
    let dir = tmp("add-godot");
    std::fs::write(dir.join("project.godot"), "[application]\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    let name = PackageName::new("godot-plugin").unwrap();
    let err = a
        .add(&dir, &name, None, AddOptions::default())
        .await
        .unwrap_err();
    assert!(matches!(
        err,
        mgc_types::MgError::Unsupported {
            core: "game",
            capability: "add",
            ..
        }
    ));
}

pub async fn godot_list_fails_closed_instead_of_returning_an_empty_native_list() {
    let dir = tmp("list-godot");
    std::fs::write(dir.join("project.godot"), "[application]\n").unwrap();
    let adapter = adapter_for(&dir).unwrap();
    let error = adapter.list(&dir).await.unwrap_err();
    assert!(matches!(
        error,
        mgc_types::MgError::Unsupported {
            core: "game",
            capability: "list",
            ..
        }
    ));
    assert!(
        !adapter
            .capabilities()
            .contains(&mgc_types::capabilities::Capability::ContentStoreProvider)
    );
    assert!(adapter.probe_content_store().is_err());
}

pub async fn bevy_list_does_not_report_manifest_ranges_as_installed_versions() {
    let dir = tmp("list-bevy");
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"bevy-game\"\nversion = \"0.1.0\"\n[dependencies]\nserde = \"^1\"\n",
    )
    .unwrap();
    let adapter = adapter_for(&dir).unwrap();
    let error = adapter.list(&dir).await.unwrap_err();
    assert!(matches!(
        error,
        mgc_types::MgError::Unsupported {
            capability: "list",
            ..
        }
    ));
}

pub async fn bevy_adapter_dependency_operations_fail_closed_without_spawning_cargo() {
    let dir = tmp("adapter-no-delegate");
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"bevy-game\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    let adapter = adapter_for(&dir).unwrap();
    let name = PackageName::new("serde").unwrap();
    let graph = mgc_types::ResolvedGraph::empty();
    let install_error = ContentStoreProvider::install(
        &adapter,
        &graph,
        &dir,
        mgc_types::adapter::InstallOptions::default(),
    )
    .await
    .unwrap_err();
    assert!(matches!(
        install_error,
        mgc_types::MgError::Unsupported { .. }
    ));
    let add_error = adapter
        .add(&dir, &name, None, AddOptions::default())
        .await
        .unwrap_err();
    assert!(matches!(add_error, mgc_types::MgError::Unsupported { .. }));
}

pub async fn unity_add_fails_closed_until_mgc_owns_upm_resolution() {
    let dir = tmp("add-unity");
    std::fs::create_dir_all(dir.join("Packages")).unwrap();
    std::fs::write(
        dir.join("Packages").join("manifest.json"),
        "{\"dependencies\":{}}\n",
    )
    .unwrap();
    let a = adapter_for(&dir).unwrap();
    let name = PackageName::new("com.unity.cinemachine").unwrap();
    let err = a
        .add(&dir, &name, None, AddOptions::default())
        .await
        .unwrap_err();
    assert!(matches!(
        err,
        mgc_types::MgError::Unsupported {
            core: "game",
            capability: "add",
            ..
        }
    ));
}
