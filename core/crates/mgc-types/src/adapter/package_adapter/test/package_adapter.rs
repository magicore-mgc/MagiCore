//! Package Adapter nearest-owner contract tests.
//! Test contract gần owner package adapter.

use crate as mgc_types;
use crate::PackageAdapter;

/// R5: the default audit_fix must name the core so callers know WHAT to
/// ask for (or which core to switch to) instead of a generic refusal.
/// Default audit_fix phải nêu tên core thay vì từ chối chung chung.
#[test]
fn audit_fix_default_names_the_core() {
    use async_trait::async_trait;
    use mgc_types::capabilities::*;
    use std::path::Path;

    struct NoFixAdapter;
    impl CoreIdent for NoFixAdapter {
        fn core_id(&self) -> &'static str {
            "test-nofix"
        }
        fn name(&self) -> &str {
            "test-nofix"
        }
        fn ecosystem(&self) -> mgc_types::Ecosystem {
            mgc_types::Ecosystem::Lib
        }
    }
    impl ProjectDetector for NoFixAdapter {
        fn can_handle(&self, _project_root: &Path) -> bool {
            false
        }
    }
    #[async_trait]
    impl DependencyResolver for NoFixAdapter {
        async fn add(
            &self,
            _project_root: &Path,
            _name: &mgc_types::PackageName,
            _range: Option<&mgc_types::VersionRange>,
            _opts: mgc_types::adapter::AddOptions,
        ) -> mgc_types::MgResult<mgc_types::PackageId> {
            panic!("default prepare_add must not call provider add")
        }
    }
    impl ArtifactFetcher for NoFixAdapter {}
    impl ContentStoreProvider for NoFixAdapter {}
    impl LockfileProvider for NoFixAdapter {}
    impl AuditProvider for NoFixAdapter {}
    impl ScaffoldProvider for NoFixAdapter {}
    impl LifecycleRunner for NoFixAdapter {}
    impl OptimizerProvider for NoFixAdapter {}
    impl Materializer for NoFixAdapter {}
    impl SimulatorProvider for NoFixAdapter {}
    impl DeviceProvider for NoFixAdapter {}
    impl DeployProvider for NoFixAdapter {}
    impl ModelRuntimeProvider for NoFixAdapter {}
    #[async_trait]
    impl PackageAdapter for NoFixAdapter {
        async fn parse_manifest(
            &self,
            _project_root: &Path,
        ) -> mgc_types::MgResult<mgc_types::Manifest> {
            Err(mgc_types::MgError::Other("stub".to_string()))
        }
        async fn list(
            &self,
            _project_root: &Path,
        ) -> mgc_types::MgResult<Vec<mgc_types::adapter::InstalledPackage>> {
            Ok(vec![])
        }
    }

    let fut = NoFixAdapter.audit_fix(Path::new("/tmp"), &[]);
    let err = futures_util::future::FutureExt::now_or_never(Box::pin(fut))
        .expect("default audit_fix resolves on first poll")
        .expect_err("default audit_fix must fail");
    assert!(
        err.to_string().contains("test-nofix"),
        "default refusal must name the core, got: {err}"
    );

    let name = mgc_types::PackageName::new("example").unwrap();
    let prepare = NoFixAdapter.prepare_add(
        Path::new("/tmp"),
        &name,
        None,
        mgc_types::adapter::AddOptions::default(),
    );
    let prepare_err = futures_util::future::FutureExt::now_or_never(Box::pin(prepare))
        .expect("default prepare_add resolves immediately")
        .expect_err("default prepare_add must fail closed");
    assert!(prepare_err.to_string().contains("native prepare-add"));
}
