//! Lib capability contract tests — kiểm thử hợp đồng capability Lib.
//! Owns lane capability truth and core identity. — Sở hữu capability theo lane và định danh core.

use mgc_lib_adapter::adapter_for;
use mgc_types::adapter::PackageAdapter;
use mgc_types::capabilities::{
    ArtifactFetcher, CoreIdent, DependencyResolver, LifecycleRunner, Materializer,
};

use super::support::tmp;

pub(crate) fn ts_lane_claims_materializer_and_lifecycle_via_web_engine() {
    let dir = tmp("ts-caps");
    std::fs::write(
        dir.join("mgc.toml"),
        "ecosystem = \"lib\"\n\n[lib]\nlanguage = \"ts\"\n",
    )
    .unwrap();
    let a = adapter_for(&dir, None, None).unwrap().unwrap();
    let caps = a.capabilities();
    assert!(caps.contains(&mgc_types::capabilities::Capability::Materializer));
    assert!(caps.contains(&mgc_types::capabilities::Capability::LifecycleRunner));
    // Claim ⇒ backed: TS rides the embedded web engine for both.
    // (Claim ⇒ có thật: TS đi trên web engine nhúng cho cả hai.)
    assert!(a.probe_materializer().is_ok());
    assert!(a.probe_lifecycle_runner().is_ok());
}

pub(crate) fn toolchain_languages_do_not_claim_materializer_or_lifecycle() {
    let dir = tmp("rust-caps");
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    let a = adapter_for(&dir, None, None).unwrap().unwrap();
    let caps = a.capabilities();
    assert!(!caps.contains(&mgc_types::capabilities::Capability::Materializer));
    assert!(!caps.contains(&mgc_types::capabilities::Capability::LifecycleRunner));
    // ArtifactFetcher IS claimed: every native language owns a standalone
    // `fetch(graph)` in `artifact_fetcher/native/<language>` (download →
    // verify → CAS import). It used to be withheld on the false premise that
    // "the toolchain fetches during its own install" — MagiCore is the one
    // downloading `.crate`/wheel/go-mod bytes into its store.
    // (ArtifactFetcher ĐƯỢC claim: mọi ngôn ngữ native có `fetch(graph)` riêng.
    // Trước đây bị giữ lại dựa trên tiền đề sai "toolchain tự tải".)
    assert!(caps.contains(&mgc_types::capabilities::Capability::ArtifactFetcher));
    assert!(a.probe_artifact_fetcher().is_ok());
    // Unclaimed ⇒ fail-closed Unsupported (trung thực).
    // (Không claim ⇒ fail-closed Unsupported.)
    let err = a.probe_materializer().unwrap_err();
    assert!(matches!(err, mgc_types::MgError::Unsupported { .. }));
}

pub(crate) fn java_gradle_does_not_claim_native_dependency_operations() {
    let dir = tmp("java-gradle-caps");
    std::fs::write(dir.join("build.gradle"), "plugins { id 'java' }\n").unwrap();
    let adapter = adapter_for(&dir, None, None).unwrap().unwrap();
    let caps = adapter.capabilities();
    assert!(!caps.contains(&mgc_types::capabilities::Capability::DependencyResolver));
    assert!(!caps.contains(&mgc_types::capabilities::Capability::LockfileProvider));
    assert!(matches!(
        adapter.probe_dependency_resolver(),
        Err(mgc_types::MgError::Unsupported { .. })
    ));
}

pub(crate) fn adapter_name_and_ecosystem() {
    let dir = tmp("name-eco");
    std::fs::write(
        dir.join("mgc.toml"),
        "ecosystem = \"lib\"\n\n[lib]\nlanguage = \"rust\"\n",
    )
    .unwrap();
    let a = adapter_for(&dir, None, None).unwrap().unwrap();
    assert_eq!(a.name(), "lib");
    assert_eq!(format!("{:?}", a.ecosystem()), "Lib");
}
