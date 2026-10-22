// .NET restore and target parsing tests.
// Test restore và phân tích target .NET.

use super::*;

#[test]
fn dotnet_mutations_and_build_restore_are_guarded() {
    assert!(denial_reason(&[], "restore").is_some());
    assert!(denial_reason(&[], "run").is_some());
    assert!(denial_reason(&["--no-restore".into()], "run").is_none());
    assert!(denial_reason(&["tool".into(), "install".into()], "tool").is_some());
    assert!(denial_reason(&[], "workload").is_some());
    assert!(
        denial_reason(&["nuget".into(), "push".into()], "nuget").is_some()
    );
}

#[test]
fn dotnet_restore_target_is_denied_in_supported_target_forms() {
    for (first, args) in [
        ("msbuild", vec!["msbuild".into(), "-t:Restore".into()]),
        (
            "build",
            vec!["build".into(), "--no-restore".into(), "-target:Build;Restore".into()],
        ),
        (
            "build",
            vec!["build".into(), "--no-restore".into(), "--target".into(), "Restore".into()],
        ),
    ] {
        assert!(requests_restore_target(&args), "target args: {args:?}");
        assert!(denial_reason(&args, first).is_some(), "policy args: {args:?}");
    }
    let safe_build = ["build".into(), "--no-restore".into()];
    assert!(!requests_restore_target(&safe_build));
    assert!(denial_reason(&safe_build, "build").is_none());
}
