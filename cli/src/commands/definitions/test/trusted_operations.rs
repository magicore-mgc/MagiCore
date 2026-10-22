// Trusted Operations CLI definition tests.
// Test lược đồ CLI theo nhóm chức năng.
use crate::Cli;
use crate::commands::definitions::commands::CommonCommands;
use crate::commands::definitions::{AuditCmd, Commands};
use clap::Parser;

#[test]
fn trusted_publish_and_signature_audit_cli_arguments_parse() {
    let publish = Cli::try_parse_from(["mgc", "publish", "--trusted", "--dry-run"]).unwrap();
    match publish.command.unwrap() {
        Commands::Common(CommonCommands::Publish {
            trusted,
            trusted_audience,
            ..
        }) => {
            assert!(trusted);
            assert_eq!(trusted_audience, None);
        }
        _ => panic!("expected trusted publish command"),
    }
    assert!(Cli::try_parse_from(["mgc", "publish", "--trusted", "--token", "secret"]).is_err());

    let audit = Cli::try_parse_from([
        "mgc",
        "audit",
        "signatures",
        "@acme/widgets",
        "--registry",
        "https://registry.example",
        "--json",
    ])
    .unwrap();
    match audit.command.unwrap() {
        Commands::Common(CommonCommands::Audit {
            cmd:
                Some(AuditCmd::Signatures {
                    package,
                    registry,
                    json,
                    ..
                }),
            fix,
            format,
        }) => {
            assert_eq!(package, "@acme/widgets");
            assert_eq!(registry, "https://registry.example");
            assert!(json);
            assert!(!fix);
            assert_eq!(format, None);
        }
        _ => panic!("expected audit signatures command"),
    }

    let trust = Cli::try_parse_from([
        "mgc",
        "registry",
        "trust",
        "@acme/widgets",
        "acme/widgets",
        "--admin-token",
        "admin",
        "--registry",
        "https://registry.example",
    ])
    .unwrap();
    assert!(matches!(
        trust.command.unwrap(),
        Commands::Common(CommonCommands::Registry {
            cmd: crate::commands::registry::RegistryCmd::Trust { .. }
        })
    ));
}

#[test]
fn trusted_oci_artifact_and_core_binding_arguments_parse() {
    let publish = Cli::try_parse_from([
        "mgc",
        "publish",
        "--trusted",
        "--protocol",
        "oci",
        "--core",
        "ai",
        "--package",
        "models/weights",
        "--version",
        "1.0.0",
        "--artifact",
        "weights.bin",
        "--registry",
        "https://registry.example",
    ])
    .unwrap();
    assert_eq!(publish.core.as_deref(), Some("ai"));
    assert!(matches!(
        publish.command,
        Some(Commands::Common(CommonCommands::Publish { .. }))
    ));

    let trust = Cli::try_parse_from([
        "mgc",
        "registry",
        "trust",
        "models/weights",
        "acme/model-builder",
        "--protocol",
        "oci",
        "--core",
        "ai",
        "--admin-token",
        "admin",
    ])
    .unwrap();
    assert_eq!(trust.core.as_deref(), Some("ai"));
    assert!(matches!(
        trust.command,
        Some(Commands::Common(CommonCommands::Registry {
            cmd: crate::commands::registry::RegistryCmd::Trust { .. }
        }))
    ));
}
