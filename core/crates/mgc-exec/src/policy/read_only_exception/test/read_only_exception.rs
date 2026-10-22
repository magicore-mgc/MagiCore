use crate::policy::ReadOnlyException;

#[test]
fn read_only_exception_keeps_exactness_expiry_and_answer() {
    let value = ReadOnlyException {
        program: "npm".into(),
        args_exact: vec!["--version".into()],
        unexpired: true,
        answer: Some("0.0.0".into()),
    };
    assert_eq!(value.program, "npm");
    assert_eq!(value.args_exact[0], "--version");
    assert!(value.unexpired);
    assert_eq!(value.answer.as_deref(), Some("0.0.0"));
}
