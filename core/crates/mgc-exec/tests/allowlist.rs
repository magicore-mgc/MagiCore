//! Public-crate compatibility smoke for allowlist exports.
#![allow(clippy::unwrap_used)]

#[test]
fn established_allowlist_paths_remain_public() {
    use mgc_exec::allowlist::{
        ALLOWED_TOOLS, ExecutionScope, ScriptInvocation, check_tool, parse_simple_script,
    };
    assert!(ALLOWED_TOOLS.contains(&"cargo"));
    check_tool("cargo").expect("cargo remains allowlisted");
    assert!(!ExecutionScope::Install.allows_pm_tools());
    let value = ScriptInvocation {
        program: "cargo".into(),
        args: vec!["check".into()],
        env: Vec::new(),
    };
    assert_eq!(
        parse_simple_script("cargo check").unwrap(),
        (value.program, value.args)
    );
}
