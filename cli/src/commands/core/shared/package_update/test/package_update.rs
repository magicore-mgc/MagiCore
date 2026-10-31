//! `mgc update` regression tests — hồi quy cho `mgc update`.
//!
//! P0 (data loss): `resolve_update_targets` retargeted a dependency's range by
//! calling `Manifest::add_dep` with the target's dependency flags. `add_dep` is
//! an ADD contract — it removes the name from every section and re-appends it to
//! the section implied by the flags — so every `devDependencies` entry moved into
//! `dependencies` while the command still printed "Updated N package(s)".
//!
//! (P0 mất dữ liệu: `resolve_update_targets` đổi range qua `Manifest::add_dep`
//! kèm cờ dependency. `add_dep` là contract ADD — xoá tên khỏi mọi section rồi
//! thêm lại theo cờ — nên mọi entry `devDependencies` bị dời sang `dependencies`
//! trong khi lệnh vẫn in "Updated N package(s)".)

use super::super::package_resolution::{UpdateResolution, resolve_update_targets};
use super::super::post_update_install::update_summary;
use super::super::types::UpdateTarget;
use async_trait::async_trait;
use mgc_types::adapter::AddOptions;
use mgc_types::adapter::{InstalledPackage, ManifestIdentity, PackageAdapter, PreparedAdd};
use mgc_types::capabilities::Capability;
use mgc_types::ecosystem::Ecosystem;
use mgc_types::manifest::Manifest;
use mgc_types::{DependencySpec, MgResult, PackageId, PackageName, Version, VersionRange};
use std::path::Path;

/// Adapter double whose `prepare_add` returns a scripted resolution.
///
/// Only `parse_manifest`, `prepare_add` and `list` are reachable from
/// `resolve_update_targets`, so the rest of the trait is inert.
/// (Adapter giả trả kết quả resolve theo kịch bản. Chỉ `parse_manifest`,
/// `prepare_add` và `list` là tới được từ `resolve_update_targets`.)
struct ScriptedAdapter {
    /// `(name, resolved version, recorded range)` per requested package.
    responses: Vec<(String, String, String)>,
}

#[async_trait]
impl PackageAdapter for ScriptedAdapter {
    fn capabilities(&self) -> &'static [Capability] {
        &[]
    }

    async fn parse_manifest(&self, _project_root: &Path) -> MgResult<Manifest> {
        Ok(Manifest::new("scripted", Ecosystem::Web))
    }

    async fn prepare_add(
        &self,
        _project_root: &Path,
        name: &PackageName,
        _range: Option<&VersionRange>,
        _opts: AddOptions,
    ) -> MgResult<PreparedAdd> {
        let (_, version, range) = self
            .responses
            .iter()
            .find(|(candidate, _, _)| candidate == name.as_str())
            .expect("adapter asked for an unscripted package");
        Ok(PreparedAdd {
            id: PackageId::new(name.clone(), Version::parse(version)?),
            range: VersionRange::parse(range)?,
        })
    }

    async fn list(&self, _project_root: &Path) -> MgResult<Vec<InstalledPackage>> {
        Ok(Vec::new())
    }

    fn manifest_identity(&self) -> Option<ManifestIdentity> {
        None
    }

    fn supports_native_update(&self) -> bool {
        true
    }
}

impl mgc_types::capabilities::CoreIdent for ScriptedAdapter {
    fn core_id(&self) -> &'static str {
        "web"
    }

    fn name(&self) -> &str {
        "scripted adapter"
    }

    fn ecosystem(&self) -> Ecosystem {
        Ecosystem::Web
    }
}

impl mgc_types::capabilities::ProjectDetector for ScriptedAdapter {
    fn can_handle(&self, _project_root: &Path) -> bool {
        true
    }
}

macro_rules! inert_capabilities {
    ($($trait_name:ident),* $(,)?) => {
        $(impl mgc_types::capabilities::$trait_name for ScriptedAdapter {})*
    };
}

inert_capabilities!(
    DependencyResolver,
    ArtifactFetcher,
    ContentStoreProvider,
    LockfileProvider,
    AuditProvider,
    ScaffoldProvider,
    LifecycleRunner,
    OptimizerProvider,
    Materializer,
    SimulatorProvider,
    DeviceProvider,
    DeployProvider,
    ModelRuntimeProvider,
);

fn dependency(name: &str, range: &str, dev: bool) -> DependencySpec {
    let mut spec = DependencySpec::new(
        PackageName::new(name.to_string()).expect("valid name"),
        VersionRange::parse(range).expect("valid range"),
    );
    spec.dev = dev;
    spec
}

/// `devDependencies` is declared first so any relocation into `dependencies`
/// would be obvious. (Khai `devDependencies` trước để dời section sẽ lộ ngay.)
fn project_manifest() -> Manifest {
    let mut manifest = Manifest::new("demo", Ecosystem::Web);
    manifest
        .dev_dependencies
        .push(dependency("typescript", "~6.0.2", true));
    manifest
        .dev_dependencies
        .push(dependency("vitest", "^3.0.0", true));
    manifest
        .dependencies
        .push(dependency("react", "^18.2.0", false));
    manifest
}

/// The `current_version` `target_selection` would produce: the version the
/// dependency is actually installed at, not the range text.
/// (`current_version` mà `target_selection` tạo ra: version đang cài thật, không
/// phải chuỗi range.)
fn target(manifest: &Manifest, name: &str, installed: &str) -> UpdateTarget {
    let declared = manifest.find_dep(name).expect("dependency exists");
    UpdateTarget {
        name: name.to_string(),
        current_version: installed.to_string(),
        dev: declared.dev,
        optional: declared.optional,
        peer: declared.peer,
    }
}

fn scripted(responses: &[(&str, &str, &str)]) -> ScriptedAdapter {
    ScriptedAdapter {
        responses: responses
            .iter()
            .map(|(name, version, range)| {
                (name.to_string(), version.to_string(), range.to_string())
            })
            .collect(),
    }
}

#[tokio::test]
async fn update_keeps_dev_dependencies_in_the_dev_section() {
    // Regression: this exact shape moved BOTH dev entries into `dependencies`.
    // (Hồi quy: đúng hình dạng này từng dời CẢ HAI entry dev sang `dependencies`.)
    let mut manifest = project_manifest();
    let adapter = scripted(&[
        ("typescript", "6.1.0", "~6.1.0"),
        ("vitest", "3.1.0", "^3.1.0"),
    ]);
    let targets = vec![
        target(&manifest, "typescript", "6.0.2"),
        target(&manifest, "vitest", "3.0.0"),
    ];

    let resolution = resolve_update_targets(&adapter, Path::new("."), &mut manifest, &targets)
        .await
        .expect("resolution succeeds");

    assert_eq!(resolution.updated.len(), 2);
    assert_eq!(resolution.unchanged, 0);
    assert_eq!(
        manifest.dev_dependencies.len(),
        2,
        "no dev dependency may be lost"
    );
    assert_eq!(
        manifest.dependencies.len(),
        1,
        "dependencies must not absorb dev entries"
    );
    assert_eq!(
        manifest.dependencies[0].name.as_str(),
        "react",
        "the real dependency must not be rewritten"
    );
}

#[test]
fn update_does_not_merge_sections() {
    // All four sections populated; retargeting each must leave four sections.
    // (Điền cả bốn section; đổi range từng cái phải để lại bốn section.)
    let mut manifest = Manifest::new("demo", Ecosystem::Web);
    manifest
        .dependencies
        .push(dependency("react", "^18.2.0", false));
    manifest
        .dev_dependencies
        .push(dependency("typescript", "~6.0.2", true));
    let mut optional = dependency("fsevents", "^2.0.0", false);
    optional.optional = true;
    manifest.optional_dependencies.push(optional);
    let mut peer = dependency("react-dom", "^18.2.0", false);
    peer.peer = true;
    manifest.peer_dependencies.push(peer);

    assert!(manifest.retarget_dep("react", VersionRange::parse("^19.0.0").unwrap()));
    assert!(manifest.retarget_dep("typescript", VersionRange::parse("~6.1.0").unwrap()));
    assert!(manifest.retarget_dep("fsevents", VersionRange::parse("^2.4.0").unwrap()));
    assert!(manifest.retarget_dep("react-dom", VersionRange::parse("^19.0.0").unwrap()));

    assert_eq!(manifest.dependencies.len(), 1);
    assert_eq!(manifest.dev_dependencies.len(), 1);
    assert_eq!(manifest.optional_dependencies.len(), 1);
    assert_eq!(manifest.peer_dependencies.len(), 1);
    assert_eq!(manifest.dependencies[0].name.as_str(), "react");
    assert_eq!(manifest.dev_dependencies[0].name.as_str(), "typescript");
    assert_eq!(manifest.optional_dependencies[0].name.as_str(), "fsevents");
    assert_eq!(manifest.peer_dependencies[0].name.as_str(), "react-dom");
}

#[tokio::test]
async fn update_bumps_a_version_that_can_bump() {
    // The version must actually move: this is the half that used to report an
    // update while writing the old range back.
    // (Version phải thực sự đổi: đây là nửa từng báo cập nhật mà ghi lại range cũ.)
    let mut manifest = project_manifest();
    let adapter = scripted(&[("typescript", "6.1.9", "~6.1.9")]);
    let targets = vec![target(&manifest, "typescript", "6.0.2")];

    let UpdateResolution { updated, unchanged } =
        resolve_update_targets(&adapter, Path::new("."), &mut manifest, &targets)
            .await
            .expect("resolution succeeds");

    assert_eq!(unchanged, 0);
    assert_eq!(updated.len(), 1);
    assert_eq!(updated[0].name, "typescript");
    assert_eq!(updated[0].to_version, "6.1.9");
    let saved = manifest
        .find_dep("typescript")
        .expect("typescript is still declared");
    assert_eq!(
        saved.range.as_str(),
        "~6.1.9",
        "the range must carry the new version"
    );
    assert!(saved.dev, "the retargeted entry stays a dev dependency");
}

#[tokio::test]
async fn update_reports_an_unchanged_version_honestly() {
    // Resolved version == installed version: count it as unchanged, never as an
    // update. The P0 bug printed "Updated 8 package(s)" for exactly this case.
    // (Version resolve == version đang cài: tính là không đổi, không bao giờ là
    // cập nhật. Lỗi P0 in "Updated 8 package(s)" đúng cho trường hợp này.)
    let mut manifest = project_manifest();
    let adapter = scripted(&[("typescript", "6.0.2", "~6.0.2")]);
    let targets = vec![target(&manifest, "typescript", "6.0.2")];

    let UpdateResolution { updated, unchanged } =
        resolve_update_targets(&adapter, Path::new("."), &mut manifest, &targets)
            .await
            .expect("resolution succeeds");

    assert!(
        updated.is_empty(),
        "an unchanged package must not be reported as updated"
    );
    assert_eq!(unchanged, 1);
    assert_eq!(
        manifest
            .find_dep("typescript")
            .expect("still declared")
            .range
            .as_str(),
        "~6.0.2",
        "an unchanged package must not be rewritten"
    );
    assert_eq!(manifest.dev_dependencies.len(), 2);
}

#[tokio::test]
async fn update_reports_a_range_that_cannot_express_the_target_honestly() {
    // `~6.0.2` cannot express a 1.x jump of the minor line. A move the new range
    // does not cover must not be counted, and the entry must be left alone.
    // (`~6.0.2` không biểu diễn được version khác dòng minor. Move mà range mới
    // không bao được thì không được tính, và entry phải giữ nguyên.)
    let mut manifest = project_manifest();
    // Resolved 6.1.9 but the adapter (mis)records an exact range pinned to
    // something else — the resolver must notice the range cannot express it.
    // (Resolve 6.1.9 nhưng adapter (cố ý) ghi range exact trỏ chỗ khác — bước
    // resolve phải nhận ra range không biểu diễn được.)
    let adapter = scripted(&[("typescript", "6.1.9", "6.0.2")]);
    let targets = vec![target(&manifest, "typescript", "6.0.2")];

    let UpdateResolution { updated, unchanged } =
        resolve_update_targets(&adapter, Path::new("."), &mut manifest, &targets)
            .await
            .expect("resolution succeeds");

    assert!(updated.is_empty());
    assert_eq!(unchanged, 1);
    assert_eq!(
        manifest
            .find_dep("typescript")
            .expect("still declared")
            .range
            .as_str(),
        "~6.0.2",
        "a range that cannot express the target must not be rewritten"
    );
}

#[test]
fn update_summary_names_both_halves() {
    assert_eq!(update_summary(0, 0), "No packages updated");
    assert_eq!(
        update_summary(0, 8),
        "0 package(s) updated; 8 already at latest",
        "a no-op run must never read as a successful update"
    );
    assert_eq!(update_summary(3, 0), "Updated 3 package(s)");
    assert_eq!(
        update_summary(2, 5),
        "Updated 2 package(s); 5 already at latest"
    );
}
