use super::*;

#[test]
fn explicit_dependency_command_mapping_covers_all_45_public_routes() {
    let cases = [
        ("install-web", "web", false),
        ("install-game", "game", false),
        ("install-ai", "ai", false),
        ("install-clo", "cloud", false),
        ("install-cicd", "cicd", false),
        ("install-iot", "iot", false),
        ("install-app", "app", false),
        ("install-lib", "lib", false),
        ("install-hardware", "hardware", false),
        ("add-web", "web", true),
        ("add-game", "game", true),
        ("add-ai", "ai", true),
        ("add-clo", "cloud", true),
        ("add-cicd", "cicd", true),
        ("add-iot", "iot", true),
        ("add-app", "app", true),
        ("add-lib", "lib", true),
        ("add-hardware", "hardware", true),
        ("remove-web", "web", true),
        ("remove-game", "game", true),
        ("remove-ai", "ai", true),
        ("remove-clo", "cloud", true),
        ("remove-cicd", "cicd", true),
        ("remove-iot", "iot", true),
        ("remove-app", "app", true),
        ("remove-lib", "lib", true),
        ("remove-hardware", "hardware", true),
        ("list-web", "web", false),
        ("list-game", "game", false),
        ("list-ai", "ai", false),
        ("list-clo", "cloud", false),
        ("list-cicd", "cicd", false),
        ("list-iot", "iot", false),
        ("list-app", "app", false),
        ("list-lib", "lib", false),
        ("list-hardware", "hardware", false),
        ("update-web", "web", false),
        ("update-game", "game", false),
        ("update-ai", "ai", false),
        ("update-clo", "cloud", false),
        ("update-cicd", "cicd", false),
        ("update-iot", "iot", false),
        ("update-app", "app", false),
        ("update-lib", "lib", false),
        ("update-hardware", "hardware", false),
    ];

    assert_eq!(cases.len(), 45);
    for (name, expected_core, needs_package) in cases {
        let command = parse_command(name, needs_package);
        assert_eq!(
            explicit_dependency_core(&command),
            Some(expected_core),
            "public command '{name}' must be bound to core '{expected_core}'"
        );
    }

    let build = parse_command("build", false);
    assert_eq!(explicit_dependency_core(&build), None);
}
