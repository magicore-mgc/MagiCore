// Supported recursive spellings — account for every current classifier arm.
// Tên lệnh hỗ trợ recursive — phủ mọi nhánh classifier hiện tại.
use super::super::recursive_supported;
use super::parse_command;

#[test]
fn recursive_matrix_accepts_all_current_command_variants() {
    let base = [
        ("publish", false, false),
        ("audit", false, false),
        ("outdated", false, false),
        ("install", false, false),
        ("add", true, false),
        ("remove", true, false),
        ("update", false, false),
        ("list", false, false),
        ("build", false, false),
        ("run", false, true),
        ("test", false, false),
        ("optimizer", false, false),
        ("dev", false, false),
    ];

    let mut checked = 0;
    for (name, needs_package, needs_script) in base {
        let command = parse_command(name, needs_package, needs_script);
        assert!(
            recursive_supported(&command),
            "'{name}' must remain supported"
        );
        checked += 1;
    }

    for core in [
        "web", "game", "ai", "clo", "cicd", "iot", "app", "lib", "hardware",
    ] {
        for operation in ["install", "add", "remove", "update", "list"] {
            let name = format!("{operation}-{core}");
            let needs_package = matches!(operation, "add" | "remove");
            let command = parse_command(&name, needs_package, false);
            assert!(
                recursive_supported(&command),
                "'{name}' must remain supported"
            );
            checked += 1;
        }
    }

    assert_eq!(checked, 58);
}
