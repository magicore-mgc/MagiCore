use super::*;

#[test]
fn swift_package_run_keeps_package_resolution_disabled() {
    assert_eq!(
        swift_package_args(),
        ["run", "--skip-update", "--disable-automatic-resolution"]
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>()
    );
}
