//! App package lifecycle tests — kiểm thử lifecycle package App.
//! Owns parse, resolve/install, mutation refusal and list. — Sở hữu parse, cài đặt, từ chối mutation, list.

use mgc_app_adapter::adapter_for;
use mgc_types::adapter::{AddOptions, PackageAdapter};
use mgc_types::capabilities::{ContentStoreProvider, DependencyResolver};
use mgc_types::{PackageName, ResolvedGraph};

use super::support::tmp;

pub(crate) async fn parse_manifest_derives_name_from_dir() {
    let dir = tmp("my-flutter-app");
    std::fs::write(dir.join("pubspec.yaml"), "name: a\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    let manifest = a.parse_manifest(&dir).await.unwrap();
    // Name now comes from pubspec.yaml content, not directory
    assert_eq!(manifest.name, "a");
}

pub(crate) async fn flutter_resolve_empty_deps_returns_empty_graph() {
    let dir = tmp("resolve");
    std::fs::write(dir.join("pubspec.yaml"), "name: a\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    let manifest = a.parse_manifest(&dir).await.unwrap();
    // Phase 2: app (Flutter) now claims DependencyResolver via the native
    // pub.dev engine; an empty pubspec resolves to an empty graph (honest —
    // nothing to resolve, never a fake error and never a fake network call).
    // (Phase 2: app (Flutter) giờ claim DependencyResolver qua engine pub.dev
    // native; pubspec rỗng resolve ra graph rỗng (trung thực — không gì để
    // resolve, không lỗi giả, không gọi mạng giả).)
    let graph = a.resolve(&manifest).await.unwrap();
    assert!(graph.packages.is_empty());
}

pub(crate) async fn kotlin_resolve_fails_closed_toolchain_owned() {
    let dir = tmp("resolve-kotlin");
    std::fs::write(dir.join("build.gradle.kts"), "").unwrap();
    let a = adapter_for(&dir).unwrap();
    // Gradle build files are executable DSL, not a parsed dependency
    // manifest. Refuse at the parser boundary before any resolver path.
    // (Gradle DSL có thể thực thi; từ chối ngay ở parser boundary.)
    let result = a.parse_manifest(&dir).await;
    assert!(
        result.is_err(),
        "Kotlin Gradle DSL must not become an empty manifest"
    );
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("executable Gradle DSL"),
        "error must state why the Gradle manifest is not parsed"
    );
}

pub(crate) async fn native_flutter_empty_install_writes_mgc_lock_without_provider_resolution() {
    let dir = tmp("install-ok");
    std::fs::write(dir.join("pubspec.yaml"), "name: a\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    // An empty native graph must not trigger Flutter's package resolver.
    // Graph native rỗng không được kích hoạt Flutter resolve package.
    let graph = ResolvedGraph::default();
    let result = a.install(&graph, &dir, Default::default()).await;
    assert!(
        result.is_ok(),
        "empty native graph should finish without provider tools: {result:?}"
    );
    assert!(
        dir.join("mgc.lock").is_file(),
        "an empty native graph must write the lock required by dev freshness"
    );
    assert!(!dir.join("pubspec.lock").exists());
}

pub(crate) async fn add_fails_closed_directs_to_tooling() {
    let dir = tmp("add-fail");
    std::fs::write(dir.join("pubspec.yaml"), "name: a\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    let name = PackageName::new("http").unwrap();
    let err = a
        .add(&dir, &name, None, AddOptions::default())
        .await
        .unwrap_err();
    let msg = err.to_string();
    // Global Gate 1: app does NOT claim DependencyResolver — add fails
    // closed with the unified capability message naming the core.
    // (Global Gate 1: app KHÔNG claim DependencyResolver — add fail
    // closed với message capability thống nhất nêu tên core.)
    assert!(
        msg.contains("app core does not support 'add'"),
        "error must be the capability default naming the core: {msg}"
    );
}

pub(crate) async fn remove_fails_closed_directs_to_tooling() {
    let dir = tmp("remove-fail");
    std::fs::write(dir.join("pubspec.yaml"), "name: a\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    let name = PackageName::new("http").unwrap();
    assert!(a.remove(&dir, &name).await.is_err());
}

pub(crate) async fn update_fails_closed_directs_to_tooling() {
    let dir = tmp("update-fail");
    std::fs::write(dir.join("pubspec.yaml"), "name: a\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    let name = PackageName::new("http").unwrap();
    assert!(a.update(&dir, Some(&name)).await.is_err());
}

pub(crate) async fn list_returns_empty_for_no_deps() {
    let dir = tmp("list");
    std::fs::write(dir.join("pubspec.yaml"), "name: a\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    let pkgs = a.list(&dir).await.unwrap();
    assert!(pkgs.is_empty());
}

pub(crate) async fn list_does_not_claim_unresolved_manifest_ranges_are_installed_versions() {
    let dir = tmp("list-unresolved");
    std::fs::write(
        dir.join("pubspec.yaml"),
        "name: a\ndependencies:\n  http: ^1.0.0\n",
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
