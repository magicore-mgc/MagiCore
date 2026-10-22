use crate::allowlist::ScriptInvocation;

#[test]
fn invocation_keeps_program_args_and_environment_as_distinct_fields() {
    let invocation = ScriptInvocation {
        program: "vite".into(),
        args: vec!["build".into()],
        env: vec![("MODE".into(), "production".into())],
    };
    assert_eq!(invocation.program, "vite");
    assert_eq!(invocation.args, vec!["build".to_string()]);
    assert_eq!(
        invocation.env,
        vec![("MODE".to_string(), "production".to_string())]
    );
}
