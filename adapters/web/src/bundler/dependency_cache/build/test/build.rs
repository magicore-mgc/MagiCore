//! Local dependency bundle output test.
//! Test output bundle dependency local.

use std::fs;

use tempfile::tempdir;

use super::super::super::error::BundleError;
use super::super::MetafileInputsError;
use super::bundle_dependency;

#[tokio::test]
async fn bundles_a_local_entry_to_nonempty_javascript() {
    let root = tempdir().unwrap();
    let entry = root.path().join("index.js");
    fs::write(
        &entry,
        "module.exports = function sample(){ return 'dependency-payload'; };",
    )
    .unwrap();

    let working_dir = root.path().to_string_lossy().to_string();
    let allowed_roots = vec![std::fs::canonicalize(root.path()).unwrap()];
    let result = bundle_dependency(&entry, &working_dir, &allowed_roots)
        .await
        .unwrap();

    assert!(!result.js.trim().is_empty());
    assert!(result.js.contains("dependency-payload"));
    assert_eq!(result.css, None);
}

#[tokio::test]
async fn rejects_output_when_esbuild_reports_a_project_file_as_an_input() {
    let project = tempdir().unwrap();
    let modules = project.path().join("node_modules");
    let package = modules.join("sample");
    fs::create_dir_all(&package).unwrap();
    fs::write(
        package.join("index.js"),
        "import secret from '../../private.js'; export default secret;",
    )
    .unwrap();
    fs::write(
        project.path().join("private.js"),
        "export default 'private';",
    )
    .unwrap();

    let allowed_roots =
        super::super::super::package_entry::allowed_dependency_roots(&modules).unwrap();
    let working_dir = project.path().to_string_lossy().to_string();
    let result = bundle_dependency(&package.join("index.js"), &working_dir, &allowed_roots).await;

    assert!(matches!(
        result,
        Err(BundleError::MetafileInputs(
            MetafileInputsError::OutsideAllowedRoots
        ))
    ));
}
