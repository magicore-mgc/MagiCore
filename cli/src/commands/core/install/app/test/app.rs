//! App install argument tests.
//! Test tham số cài App.

use crate::commands::core::install::app::install_packages_error;

#[tokio::test]
async fn flutter_install_with_package_arguments_requires_add() {
    let project = tempfile::tempdir().expect("tempdir");
    let packages = vec!["http".to_string()];
    let error = crate::commands::core::install::app::install_flutter_native(
        project.path(),
        packages.clone(),
        false,
        None,
        false,
        false,
    )
    .await
    .expect_err("install must not accept package arguments");

    assert_eq!(
        error.to_string(),
        install_packages_error(&packages).to_string()
    );
}

#[test]
fn install_package_argument_error_uses_the_app_error_owner() {
    assert!(
        install_packages_error(&["http".to_string()])
            .to_string()
            .contains("add-app <package>")
    );
}
