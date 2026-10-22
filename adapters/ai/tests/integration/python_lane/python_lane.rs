//! AI native Python lane tests — kiểm thử lane Python native của AI.
//! Owns package-lane detection and fail-closed lifecycle contract cases. — Sở hữu nhận diện lane package và hợp đồng lifecycle fail-closed.

use mgc_ai_adapter::adapter_for;
use mgc_types::adapter::{AddOptions, PackageAdapter};
use mgc_types::capabilities::{ContentStoreProvider, DependencyResolver};
use mgc_types::{PackageName, ResolvedGraph};

use super::support::tmp;

pub(crate) fn native_python_lane_does_not_take_over_foreign_lockfiles() {
    let dir = tmp("native-lane-lock-policy");
    std::fs::write(dir.join("pyproject.toml"), "[project]\nname='t'\n").unwrap();
    assert!(mgc_ai_adapter::uses_native_python_lane(&dir));
    std::fs::write(dir.join("uv.lock"), "version = 1\n").unwrap();
    assert!(!mgc_ai_adapter::uses_native_python_lane(&dir));
    std::fs::remove_file(dir.join("uv.lock")).unwrap();
    std::fs::write(dir.join("requirements.lock"), "six==1.17.0\n").unwrap();
    assert!(!mgc_ai_adapter::uses_native_python_lane(&dir));
}

pub(crate) fn native_python_lane_rejects_unowned_lockfiles_and_dependency_sources() {
    let lockfiles = [
        "requirements.txt",
        "requirements-dev.txt",
        "pylock.toml",
        "pylock.production.toml",
        "poetry.lock",
        "pdm.lock",
        "Pipfile.lock",
        "pixi.lock",
        "conda-lock.yml",
    ];
    for (index, lockfile) in lockfiles.iter().enumerate() {
        let dir = tmp(&format!("foreign-python-lock-{index}"));
        std::fs::write(
            dir.join("pyproject.toml"),
            "[project]\nname='t'\ndependencies=[]\n",
        )
        .unwrap();
        std::fs::write(dir.join(lockfile), "foreign owner\n").unwrap();
        assert!(
            !mgc_ai_adapter::uses_native_python_lane(&dir),
            "must not take ownership when {lockfile} exists"
        );
    }

    let poetry = tmp("poetry-project");
    std::fs::write(
        poetry.join("pyproject.toml"),
        "[tool.poetry]\nname='t'\n[tool.poetry.dependencies]\npython='^3.12'\nrequests='*'\n",
    )
    .unwrap();
    assert!(!mgc_ai_adapter::uses_native_python_lane(&poetry));

    let dynamic = tmp("dynamic-python-deps");
    std::fs::write(
        dynamic.join("pyproject.toml"),
        "[project]\nname='t'\ndynamic=['dependencies']\n",
    )
    .unwrap();
    assert!(!mgc_ai_adapter::uses_native_python_lane(&dynamic));
}

pub(crate) async fn pypi_lane_claims_registry_capabilities_and_resolves_natively() {
    let dir = tmp("pypi-lane");
    std::fs::write(
        dir.join("pyproject.toml"),
        "[project]\nname = \"t\"\n\n[tool.magicore]\nframework = \"python-agent\"\n",
    )
    .unwrap();
    let adapter = adapter_for(&dir).unwrap();
    assert!(
        adapter.python_lane.is_some(),
        "pyproject → native PyPI lane"
    );
    let caps = adapter.capabilities();
    assert!(caps.contains(&mgc_types::capabilities::Capability::DependencyResolver));
    assert!(caps.contains(&mgc_types::capabilities::Capability::LockfileProvider));
    assert!(caps.contains(&mgc_types::capabilities::Capability::ArtifactFetcher));
    assert!(caps.contains(&mgc_types::capabilities::Capability::ContentStoreProvider));
    let identity = mgc_types::adapter::PackageAdapter::manifest_identity(&adapter).unwrap();
    assert_eq!(identity.core, "ai");
    assert_eq!(identity.language, "python");
    // Empty manifest → native resolve returns an empty graph WITHOUT any
    // network call (no deps to resolve).
    // (Manifest rỗng → resolve native trả graph rỗng KHÔNG chạm network.)
    let manifest = adapter.parse_manifest(&dir).await.unwrap();
    let graph = adapter.resolve(&manifest).await.unwrap();
    assert!(graph.packages.is_empty());
}

pub(crate) async fn parse_manifest_uses_dir_name_as_project_name() {
    let dir = tmp("my-ai-project");
    std::fs::write(dir.join("mgc.toml"), "[ai]\nframework = \"python-agent\"\n").unwrap();
    let adapter = adapter_for(&dir).unwrap();
    let manifest = adapter.parse_manifest(&dir).await.unwrap();
    assert!(manifest.name.contains("my-ai-project"));
}

pub(crate) async fn resolve_fails_closed_without_native_python_lane() {
    let dir = tmp("resolve");
    std::fs::write(dir.join("mgc.toml"), "[ai]\nframework = \"python-agent\"\n").unwrap();
    let adapter = adapter_for(&dir).unwrap();
    let manifest = adapter.parse_manifest(&dir).await.unwrap();
    // AI does NOT claim DependencyResolver (Global Gate 1) — resolve must
    // now fail closed instead of returning an empty graph.
    // (AI KHÔNG claim DependencyResolver (Global Gate 1) — resolve phải
    // fail closed thay vì trả graph rỗng.)
    let result = adapter.resolve(&manifest).await;
    assert!(
        result.is_err(),
        "ai resolve must fail closed (no DependencyResolver claim)"
    );
}

pub(crate) async fn install_fails_closed_with_descriptive_error() {
    let dir = tmp("install-fail");
    std::fs::write(dir.join("mgc.toml"), "[ai]\nframework = \"python-agent\"\n").unwrap();
    let adapter = adapter_for(&dir).unwrap();
    let manifest = adapter.parse_manifest(&dir).await.unwrap();
    // resolve itself fails closed first (no DependencyResolver claim) —
    // install cannot even be reached without a graph.
    // (resolve đã fail closed trước (không claim DependencyResolver) —
    // install không thể gọi khi chưa có graph.)
    assert!(
        adapter.resolve(&manifest).await.is_err(),
        "ai resolve must fail closed"
    );
    let graph = ResolvedGraph::default();
    let result = adapter.install(&graph, &dir, Default::default()).await;
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("ai core"),
        "error message should name the ai core: {msg}"
    );
}

pub(crate) async fn add_fails_closed_with_descriptive_error() {
    let dir = tmp("add-fail");
    std::fs::write(dir.join("mgc.toml"), "[ai]\nframework = \"python-agent\"\n").unwrap();
    let adapter = adapter_for(&dir).unwrap();
    let name = PackageName::new("openai").unwrap();
    let result = adapter.add(&dir, &name, None, AddOptions::default()).await;
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("ai core"),
        "error message should name the ai core: {msg}"
    );
}

pub(crate) async fn update_fails_closed_with_descriptive_error() {
    let dir = tmp("update-fail");
    std::fs::write(dir.join("mgc.toml"), "[ai]\nframework = \"python-agent\"\n").unwrap();
    let adapter = adapter_for(&dir).unwrap();
    let name = PackageName::new("openai").unwrap();
    let result = adapter.update(&dir, Some(&name)).await;
    assert!(result.is_err());
}

pub(crate) async fn remove_fails_closed_without_mutating_through_adapter_api() {
    let dir = tmp("remove-fail");
    let source =
        "[project]\nname = \"ai-test\"\nversion = \"0.1.0\"\ndependencies = [\"openai>=1.0\"]\n";
    std::fs::write(dir.join("pyproject.toml"), source).unwrap();
    std::fs::write(dir.join("mgc.toml"), "[ai]\nframework = \"python-agent\"\n").unwrap();
    let adapter = adapter_for(&dir).unwrap();
    let name = PackageName::new("openai").unwrap();

    let error = adapter
        .remove(&dir, &name)
        .await
        .expect_err("direct adapter remove must fail closed outside CLI gateway");
    assert!(
        error
            .to_string()
            .contains("direct adapter mutation is disabled")
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("pyproject.toml")).unwrap(),
        source
    );
}

pub(crate) async fn list_refuses_to_claim_an_mgc_owned_set_for_foreign_lane() {
    let dir = tmp("list");
    std::fs::write(dir.join("mgc.toml"), "[ai]\nframework = \"mcp-server\"\n").unwrap();
    let adapter = adapter_for(&dir).unwrap();
    let error = adapter.list(&dir).await.unwrap_err();
    assert!(error.to_string().contains("foreign Python lock/manifest"));
}
