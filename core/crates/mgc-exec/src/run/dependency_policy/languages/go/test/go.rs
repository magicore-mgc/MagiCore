// Go module and effective GOPROXY tests.
// Test module Go và GOPROXY hiệu lực.

use super::*;

#[test]
fn go_mutation_is_denied_and_compile_requires_readonly_offline_mode() {
    assert!(denial_reason(&["get".into()], "get", true).is_some());
    assert!(denial_reason(&["list".into(), "-m".into(), "all".into()], "list", false).is_some());
    assert!(denial_reason(&["build".into(), "-mod=readonly".into()], "build", false).is_some());
    assert!(denial_reason(&["build".into(), "-mod=readonly".into()], "build", true).is_none());
}

#[test]
fn go_allows_only_the_exact_builtin_version_query() {
    assert!(is_go_toolchain_version_probe(&["tool".into(), "asm".into(), "-v=full".into()]));
    assert!(!is_go_toolchain_version_probe(&[
        "tool".into(),
        "compile".into(),
        "-v=full".into(),
        "extra".into(),
    ]));
}
