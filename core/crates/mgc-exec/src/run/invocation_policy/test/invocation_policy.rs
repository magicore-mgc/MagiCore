use super::super::{ExecOptions, compat_runner_declared};

#[test]
fn compat_runner_exemption_matches_only_the_declared_manager() {
    let opts = ExecOptions {
        compat_runner: Some("pnpm".to_string()),
        ..ExecOptions::default()
    };

    assert!(compat_runner_declared("/tools/pnpm.CMD", &opts));
    assert!(!compat_runner_declared("npm", &opts));
    assert!(!compat_runner_declared(
        "/tools/pnpm",
        &ExecOptions::default()
    ));
}
