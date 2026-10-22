//! Lib lifecycle tests — kiểm thử lifecycle Lib.
//! Owns direct mutations and installed-package listing semantics. — Sở hữu mutation trực tiếp và ngữ nghĩa list package.

use mgc_lib_adapter::adapter_for;
use mgc_types::adapter::PackageAdapter;
use mgc_types::capabilities::DependencyResolver;
use mgc_types::{PackageName, VersionRange};

use super::support::tmp;

pub(crate) async fn direct_python_adapter_mutations_fail_closed_outside_mutation_gateway() {
    let dir = tmp("py-update-all");
    std::fs::write(
        dir.join("mgc.toml"),
        "ecosystem = \"lib\"\n\n[lib]\nlanguage = \"python\"\npip_allowed_packages = [\"requests\"]\n",
    )
    .unwrap();
    std::fs::write(dir.join("pyproject.toml"), "[project]\nname = \"py-lib\"\n").unwrap();
    let a = adapter_for(&dir, None, None).unwrap().unwrap();
    let before = std::fs::read(dir.join("pyproject.toml")).unwrap();
    let name = PackageName::new("requests").unwrap();
    let range = VersionRange::parse("^2.32.3").unwrap();
    let add_error = a
        .add(&dir, &name, Some(&range), Default::default())
        .await
        .expect_err("direct adapter add must not bypass the CLI mutation gateway");
    assert!(
        add_error
            .to_string()
            .contains("direct adapter mutation is disabled")
    );
    let remove_error = a
        .remove(&dir, &name)
        .await
        .expect_err("direct adapter remove must not bypass the CLI mutation gateway");
    assert!(
        remove_error
            .to_string()
            .contains("direct adapter mutation is disabled")
    );
    let update_error = a
        .update(&dir, None)
        .await
        .expect_err("direct adapter update must not bypass the CLI mutation gateway");
    assert!(
        update_error
            .to_string()
            .contains("direct adapter mutation is disabled")
    );
    assert_eq!(std::fs::read(dir.join("pyproject.toml")).unwrap(), before);
    assert!(!dir.join("mgc.lock").exists());
}

pub(crate) async fn rust_list_does_not_confuse_lock_pins_with_installed_packages() {
    let dir = tmp("rust-list");
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"rust-list\"\nversion = \"0.1.0\"\n\n[package.metadata.magicore]\ncore = \"lib\"\n\n[dependencies]\nserde = \"1\"\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("Cargo.lock"),
        "[[package]]\nname = \"serde\"\nversion = \"1.0.219\"\n",
    )
    .unwrap();
    let a = adapter_for(&dir, None, None).unwrap().unwrap();
    let error = a.list(&dir).await.unwrap_err();
    assert!(matches!(
        error,
        mgc_types::MgError::Unsupported {
            capability: "list installed packages",
            ..
        }
    ));
}

pub(crate) async fn rust_list_is_unsupported_even_when_manifest_has_no_dependencies() {
    let dir = tmp("rust-list-empty");
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"rust-list-empty\"\nversion = \"0.1.0\"\n\n[package.metadata.magicore]\ncore = \"lib\"\n",
    )
    .unwrap();
    let a = adapter_for(&dir, None, None).unwrap().unwrap();
    let error = a.list(&dir).await.unwrap_err();
    assert!(matches!(
        error,
        mgc_types::MgError::Unsupported {
            capability: "list installed packages",
            ..
        }
    ));
}

pub(crate) async fn python_list_does_not_claim_venv_packages_without_mgc_lock() {
    let dir = tmp("py-dist-info");
    std::fs::write(
        dir.join("pyproject.toml"),
        "[project]\nname = \"py-lib\"\ndependencies = [\"requests>=2.32.3\"]\n\n[tool.magicore]\ncore = \"lib\"\n",
    )
    .unwrap();
    let metadata = dir
        .join(".venv")
        .join("lib")
        .join("python3.11")
        .join("site-packages")
        .join("requests-2.32.3.dist-info");
    std::fs::create_dir_all(&metadata).unwrap();
    std::fs::write(
        metadata.join("METADATA"),
        "Metadata-Version: 2.1\nName: requests\nVersion: 2.32.3\n",
    )
    .unwrap();
    let a = adapter_for(&dir, None, None).unwrap().unwrap();
    let installed = a.list(&dir).await.unwrap();
    assert!(installed.is_empty());
}
