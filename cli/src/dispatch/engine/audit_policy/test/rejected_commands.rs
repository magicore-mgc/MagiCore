// Unsupported audit-strict command cases — preserve fail-closed diagnostics.
// Ca lệnh không hỗ trợ audit-strict — giữ nguyên chẩn đoán fail-closed.
use super::super::reject_unsupported_audit_strict;
use super::parse_command;

#[test]
fn commands_that_do_not_consult_audit_strict_are_rejected() {
    let unsupported = [
        ("build", false),
        ("remove-web", true),
        ("remove-hardware", true),
    ];
    for (name, needs_package) in unsupported {
        let command = parse_command(name, needs_package);
        let error = reject_unsupported_audit_strict(&command).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("--audit-strict has no effect on"),
            "unexpected diagnostic for '{name}': {error}"
        );
        if name == "build" {
            assert!(
                error
                    .to_string()
                    .contains("--audit-strict has no effect on 'build'")
            );
        }
    }
}
