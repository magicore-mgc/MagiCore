use super::*;

#[test]
fn flutter_run_never_resolves_packages_outside_mgc() {
    let ios = super::flutter_dev_command(&TargetPlatform::IosSimulator, true);
    let android = super::flutter_dev_command(&TargetPlatform::Android, true);
    assert!(ios.args.contains(&"--no-pub".to_string()));
    assert!(android.args.contains(&"--no-pub".to_string()));
}
