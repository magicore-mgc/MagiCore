use super::super::first_subcommand;

#[test]
fn skips_leading_flags_and_stops_skipping_after_terminator() {
    assert_eq!(
        first_subcommand(&["--offline".into(), "install".into()]),
        Some("install".into())
    );
    assert_eq!(
        first_subcommand(&["--".into(), "--version".into()]),
        Some("--version".into())
    );
    assert_eq!(first_subcommand(&["--offline".into()]), None);
}
