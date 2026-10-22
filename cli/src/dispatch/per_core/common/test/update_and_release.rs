// Update/release mapping — retain nested audit and signing options.
// Mapping update/release — giữ tùy chọn audit và signing lồng nhau.
use super::super::common_to_dispatch;
use crate::commands::definitions::audit::AuditCmd;
use crate::commands::definitions::commands::common::CommonCommands;
use crate::dispatch::types::CommonCommand;

fn mapped(command: CommonCommands) -> CommonCommand {
    match common_to_dispatch(command) {
        Some(command) => command,
        None => panic!("common command must be mapped"),
    }
}

#[test]
fn maps_audit_with_and_without_its_optional_subcommand() {
    match mapped(CommonCommands::Audit {
        cmd: Some(AuditCmd::Signatures {
            package: "sample".to_owned(),
            protocol: "npm".to_owned(),
            registry: "https://registry.example".to_owned(),
            token: Some("redacted-test-value".to_owned()),
            json: true,
        }),
        fix: true,
        format: Some("sarif".to_owned()),
    }) {
        CommonCommand::Audit { cmd, fix, format } => {
            assert!(fix);
            assert_eq!(format.as_deref(), Some("sarif"));
            match cmd {
                Some(AuditCmd::Signatures {
                    package,
                    protocol,
                    registry,
                    token,
                    json,
                }) => {
                    assert_eq!(package, "sample");
                    assert_eq!(protocol, "npm");
                    assert_eq!(registry, "https://registry.example");
                    assert_eq!(token.as_deref(), Some("redacted-test-value"));
                    assert!(json);
                }
                None => panic!("audit subcommand was dropped"),
            }
        }
        _ => panic!("audit command mapped to a different variant"),
    }

    match mapped(CommonCommands::Audit {
        cmd: None,
        fix: false,
        format: None,
    }) {
        CommonCommand::Audit { cmd, fix, format } => {
            assert!(cmd.is_none());
            assert!(!fix);
            assert!(format.is_none());
        }
        _ => panic!("audit command mapped to a different variant"),
    }
}

#[test]
fn maps_self_update_and_release_signing_options() {
    match mapped(CommonCommands::SelfUpdate {
        version: Some("1.2.3".to_owned()),
        variant: Some("magicore-web".to_owned()),
        dry_run: true,
        trust_root: vec!["root-a".to_owned(), "root-b".to_owned()],
        allow_unsigned: false,
    }) {
        CommonCommand::SelfUpdate {
            version,
            variant,
            dry_run,
            trust_root,
            allow_unsigned,
        } => {
            assert_eq!(version.as_deref(), Some("1.2.3"));
            assert_eq!(variant.as_deref(), Some("magicore-web"));
            assert!(dry_run);
            assert_eq!(trust_root, vec!["root-a".to_owned(), "root-b".to_owned()]);
            assert!(!allow_unsigned);
        }
        _ => panic!("self-update command mapped to a different variant"),
    }

    match mapped(CommonCommands::SignRelease {
        manifest: Some("release.json".to_owned()),
        key_hex: Some("aabb".to_owned()),
    }) {
        CommonCommand::SignRelease { manifest, key_hex } => {
            assert_eq!(manifest.as_deref(), Some("release.json"));
            assert_eq!(key_hex.as_deref(), Some("aabb"));
        }
        _ => panic!("sign-release command mapped to a different variant"),
    }
}
