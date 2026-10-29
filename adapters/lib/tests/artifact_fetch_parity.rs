//! Standalone `ArtifactFetcher` contract for the Lib core.
//! Contract `ArtifactFetcher` độc lập cho core Lib.
//!
//! These tests assert the capability CLAIM matches what the language can do:
//! a language that declares `artifact-fetcher` must have a protocol behind it,
//! and a graph with nothing to fetch must still answer honestly.
//! (Test khẳng định claim capability khớp khả năng thật: ngôn ngữ khai
//! `artifact-fetcher` phải có protocol phía sau, và graph rỗng vẫn phải trả lời
//! trung thực.)

use mgc_lib_adapter::{LibLanguage, adapter_for_language};
use mgc_types::ResolvedGraph;
use mgc_types::adapter::PackageAdapter;
use mgc_types::capabilities::ArtifactFetcher;
use mgc_types::capabilities::Capability;
use std::path::Path;

/// A project root carrying no language manifest, used only to build adapters.
/// (Root không có manifest ngôn ngữ, chỉ dùng để dựng adapter.)
fn bare_project() -> tempfile::TempDir {
    tempfile::tempdir().expect("temp dir")
}

/// A project that declares the language as MagiCore-owned.
///
/// Ownership is a property of the on-disk manifest, not of the language: a
/// Python directory without `pyproject.toml`/`setup.py` is "foreign" by design,
/// and its manifest deliberately withholds the fetch capability.
/// (Ownership là thuộc tính của manifest trên đĩa, không của ngôn ngữ: thư mục
/// Python thiếu `pyproject.toml`/`setup.py` bị coi là "foreign" theo thiết kế và
/// manifest cố ý không cấp capability fetch.)
fn owned_project(language: LibLanguage) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("temp dir");
    let (name, body) = match language {
        LibLanguage::Rust => (
            "Cargo.toml",
            "[package]\nname = \"probe\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        ),
        LibLanguage::Python => (
            "pyproject.toml",
            "[project]\nname = \"probe\"\nversion = \"0.1.0\"\n",
        ),
        LibLanguage::Go => ("go.mod", "module probe\n\ngo 1.22\n"),
        LibLanguage::Java => (
            "pom.xml",
            "<project><modelVersion>4.0.0</modelVersion><groupId>p</groupId><artifactId>probe</artifactId><version>0.1.0</version></project>",
        ),
        LibLanguage::DotNet => (
            "probe.csproj",
            "<Project Sdk=\"Microsoft.NET.Sdk\"><PropertyGroup><TargetFramework>net8.0</TargetFramework></PropertyGroup></Project>",
        ),
        LibLanguage::Ts => ("package.json", "{\"name\":\"probe\",\"version\":\"0.1.0\"}"),
    };
    std::fs::write(dir.path().join(name), body).expect("write manifest");
    dir
}

/// Every lib language must accept `probe_artifact_fetcher`.
///
/// TypeScript rides the Web engine and the rest own a protocol, so no language
/// is left behind — which is the whole point of the parity rule.
/// (Mọi ngôn ngữ lib phải qua `probe_artifact_fetcher`: TS qua Web engine, còn
/// lại có protocol riêng — không ngôn ngữ nào bị bỏ lại.)
#[test]
fn every_language_probes_artifact_fetcher_successfully() {
    for language in [
        LibLanguage::Rust,
        LibLanguage::Python,
        LibLanguage::Go,
        LibLanguage::Java,
        LibLanguage::DotNet,
        LibLanguage::Ts,
    ] {
        let root = owned_project(language);
        let adapter = adapter_for_language(language, root.path(), None, None)
            .unwrap_or_else(|error| panic!("{language:?} adapter: {error}"));
        adapter.probe_artifact_fetcher().unwrap_or_else(|error| {
            panic!(
                "{language:?} claims artifact-fetcher in CAPABILITIES but the probe \
                 refuses: {error}"
            )
        });
    }
}

/// `artifact-fetcher` appears in the manifest exactly when the probe succeeds.
///
/// A manifest entry without a working probe is the failure mode this test
/// exists for: it would advertise a capability the product cannot perform.
/// (Entry manifest chỉ xuất hiện khi probe thành công — entry mà probe hỏng
/// chính là lỗi test này canh.)
#[test]
fn the_manifest_entry_matches_the_probe() {
    for language in [
        LibLanguage::Rust,
        LibLanguage::Python,
        LibLanguage::Go,
        LibLanguage::DotNet,
    ] {
        let root = owned_project(language);
        let adapter = adapter_for_language(language, root.path(), None, None).expect("adapter");
        let claimed = adapter
            .capabilities()
            .contains(&Capability::ArtifactFetcher);
        let works = adapter.probe_artifact_fetcher().is_ok();
        assert_eq!(
            claimed, works,
            "{language:?}: claimed={claimed} but probe_ok={works}"
        );
    }
}

/// Top-level `lib` capability list (what `mgc capabilities` prints) includes
/// `artifact-fetcher`, matching `web`.
/// (Danh sách capability `lib` mà `mgc capabilities` in ra có
/// `artifact-fetcher`, ngang `web`.)
#[test]
fn the_lib_core_advertises_artifact_fetcher() {
    let root = bare_project();
    let adapter =
        adapter_for_language(LibLanguage::Rust, root.path(), None, None).expect("adapter");
    assert!(
        adapter
            .capabilities()
            .contains(&Capability::ArtifactFetcher),
        "lib must advertise artifact-fetcher; got {:?}",
        adapter.capabilities()
    );
}

/// An empty graph fetches nothing and succeeds — the no-work case must not be
/// an error, otherwise a prefetch of an already-satisfied lock would fail.
/// (Graph rỗng: không tải gì và thành công — trường hợp không có việc không
/// được là lỗi, nếu không prefetch một lock đã đủ sẽ fail.)
#[tokio::test]
async fn an_empty_graph_is_a_successful_no_op() {
    let root = bare_project();
    let adapter =
        adapter_for_language(LibLanguage::Rust, root.path(), None, None).expect("adapter");
    let graph = ResolvedGraph::empty();
    adapter
        .fetch(&graph)
        .await
        .expect("an empty graph needs no download");
}

/// The project root is irrelevant to the fetch path: nothing is written there.
/// (Project root không liên quan đường fetch: không ghi gì vào đó.)
#[test]
fn fetching_does_not_touch_the_project_root() {
    let root = bare_project();
    let before: Vec<_> = std::fs::read_dir(root.path())
        .expect("read dir")
        .map(|entry| entry.expect("entry").file_name())
        .collect();
    let adapter =
        adapter_for_language(LibLanguage::Rust, root.path(), None, None).expect("adapter");
    let _ = adapter.probe_artifact_fetcher();
    let after: Vec<_> = std::fs::read_dir(root.path())
        .expect("read dir")
        .map(|entry| entry.expect("entry").file_name())
        .collect();
    assert_eq!(
        before, after,
        "probing must not create files in the project"
    );
    assert!(Path::new(root.path()).exists());
}

/// A foreign-owned lane must not claim or serve the fetch capability.
///
/// A Python directory without `pyproject.toml`/`setup.py` belongs to another
/// manager; claiming `artifact-fetcher` there would tell a caller MagiCore can
/// populate a store it deliberately does not own.
/// (Lane do tool khác sở hữu không được claim hay phục vụ capability fetch —
/// claim ở đó sẽ báo caller rằng MagiCore nạp được store mà nó cố ý không sở hữu.)
#[test]
fn a_foreign_python_lane_neither_claims_nor_serves_fetch() {
    let root = bare_project();
    let adapter =
        adapter_for_language(LibLanguage::Python, root.path(), None, None).expect("adapter");
    assert!(
        !PackageAdapter::capabilities(&adapter).contains(&Capability::ArtifactFetcher),
        "a foreign Python lane must not advertise artifact-fetcher"
    );
    let error = adapter
        .probe_artifact_fetcher()
        .expect_err("a foreign lane must refuse the probe");
    let message = error.to_string();
    assert!(
        message.contains("owned by another toolchain"),
        "the refusal must name the real reason, got: {message}"
    );
}

/// The probe and the manifest agree for a foreign lane too.
/// (Probe và manifest khớp nhau cả với lane foreign.)
#[test]
fn a_foreign_lane_probe_matches_its_manifest() {
    let root = bare_project();
    let adapter =
        adapter_for_language(LibLanguage::Python, root.path(), None, None).expect("adapter");
    let claimed = PackageAdapter::capabilities(&adapter).contains(&Capability::ArtifactFetcher);
    let works = adapter.probe_artifact_fetcher().is_ok();
    assert_eq!(
        claimed, works,
        "the foreign Python lane disagrees with itself: claimed={claimed} probe_ok={works}"
    );
    assert!(!claimed, "the foreign lane must withhold the claim");
}
