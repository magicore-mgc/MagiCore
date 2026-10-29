#![allow(clippy::unwrap_used)]

/// PAR-08: the dry-run branch must reach package validation instead of
/// returning success first.
/// (PAR-08: nhánh dry-run phải tới bước kiểm package thay vì trả thành công
/// trước.)
///
/// The behaviour is asserted on the real command entry because the defect lived
/// in the order of the branches; a unit test of the message text would pass
/// with either order.
/// (Hành vi được khẳng định trên entry của lệnh vì lỗi nằm ở thứ tự nhánh; test
/// unit trên text message sẽ pass với cả hai thứ tự.)
#[cfg(feature = "clo")]
#[tokio::test]
async fn dry_run_rejects_an_invalid_package_name_without_writing() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("cdk.out")).unwrap();
    std::fs::write(
        dir.path().join("cdk.json"),
        "{\"app\":\"npx ts-node bin/app.ts\"}\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("mgc.toml"),
        "name = \"infra\"\necosystem = \"clo\"\n\n[cloud]\ntype = \"cdk\"\n",
    )
    .unwrap();
    std::fs::write(dir.path().join(".mgc.core"), "clo\n").unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        "{\"name\":\"infra\",\"dependencies\":{}}",
    )
    .unwrap();

    let previous = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();
    let result =
        super::super::install(vec!["not a package name!".to_string()], true, false, None).await;
    std::env::set_current_dir(previous).unwrap();

    let error = result.expect_err("an invalid package name must fail the dry run");
    let message = error.to_string();
    assert!(
        message.contains("invalid package name") || message.contains("cannot be resolved"),
        "the dry run must report the real problem: {message}"
    );
    let manifest = std::fs::read_to_string(dir.path().join("package.json")).unwrap();
    assert!(
        !manifest.contains("not a package name!"),
        "a dry run must not write the manifest"
    );
}
