// Supported audit-strict command spellings — parser values only, no command execution.
// Tên lệnh hỗ trợ audit-strict — chỉ parse giá trị, không thực thi command.
use super::super::reject_unsupported_audit_strict;
use super::parse_command;

#[test]
fn every_audit_strict_supported_cli_spelling_is_accepted() {
    let supported = [
        ("audit", false),
        ("verify", false),
        ("install", false),
        ("add", true),
        ("install-web", false),
        ("install-game", false),
        ("install-ai", false),
        ("install-clo", false),
        ("install-cicd", false),
        ("install-iot", false),
        ("install-app", false),
        ("install-lib", false),
        ("install-hardware", false),
        ("add-web", true),
        ("add-game", true),
        ("add-ai", true),
        ("add-clo", true),
        ("add-cicd", true),
        ("add-iot", true),
        ("add-app", true),
        ("add-lib", true),
        ("add-hardware", true),
    ];

    assert_eq!(supported.len(), 22);
    for (name, needs_package) in supported {
        let command = parse_command(name, needs_package);
        assert!(
            reject_unsupported_audit_strict(&command).is_ok(),
            "audit-strict must accept '{name}'"
        );
    }
}
