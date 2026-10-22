use crate::policy::Invocation;

#[test]
fn invocation_retains_all_observed_fields() {
    let value = Invocation {
        program: "npm".into(),
        args: vec!["--version".into()],
        env: vec![("NO_COLOR".into(), "1".into())],
        in_compat_subtree: true,
    };
    assert_eq!(value.program, "npm");
    assert_eq!(value.args[0], "--version");
    assert_eq!(value.env, vec![("NO_COLOR".into(), "1".into())]);
    assert!(value.in_compat_subtree);
}
