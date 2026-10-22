//! Symlink Safety behavior tests.
//! Test hành vi cho nhóm symlink safety.

use crate::integration::support::tmp;
use mgc_cloud_adapter::detect_type;

#[cfg(unix)]
pub fn type_detection_does_not_follow_external_mgc_config_symlink() {
    let project = tmp("external-mgc-config-link");
    let external = tmp("external-mgc-config-target");
    std::fs::write(external.join("mgc.toml"), "[cloud]\ntype = 'terraform'\n").unwrap();
    std::os::unix::fs::symlink(external.join("mgc.toml"), project.join("mgc.toml")).unwrap();

    assert_eq!(detect_type(&project), None);
    let _ = std::fs::remove_dir_all(project);
    let _ = std::fs::remove_dir_all(external);
}

#[cfg(unix)]
pub fn type_detection_does_not_follow_external_package_json_symlink() {
    let project = tmp("external-package-json-link");
    let external = tmp("external-package-json-target");
    std::fs::write(
        external.join("package.json"),
        r#"{"dependencies":{"aws-cdk-lib":"^2.0.0"}}"#,
    )
    .unwrap();
    std::os::unix::fs::symlink(external.join("package.json"), project.join("package.json"))
        .unwrap();

    assert_eq!(detect_type(&project), None);
    let _ = std::fs::remove_dir_all(project);
    let _ = std::fs::remove_dir_all(external);
}
