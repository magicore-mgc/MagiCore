// Publish mapping — ensure the full registry and policy flag set survives.
// Mapping publish — bảo đảm toàn bộ cờ registry và policy được giữ nguyên.
use super::super::common_to_dispatch;
use crate::commands::definitions::commands::common::CommonCommands;
use crate::dispatch::types::CommonCommand;
use std::path::PathBuf;

#[test]
fn maps_all_publish_fields_without_dropping_security_or_release_options() {
    let artifacts = vec![PathBuf::from("dist/a.whl"), PathBuf::from("dist/b.whl")];
    let command = CommonCommands::Publish {
        protocol: "pypi".to_owned(),
        package: Some("sample".to_owned()),
        version: Some("1.4.2".to_owned()),
        artifacts: artifacts.clone(),
        image: Some("sample:1.4.2".to_owned()),
        tag: Some("stable".to_owned()),
        access: Some("restricted".to_owned()),
        dry_run: true,
        json: true,
        otp: Some("123456".to_owned()),
        force: true,
        ignore_scripts: false,
        allow_scripts: true,
        no_git_checks: true,
        publish_branch: Some("release".to_owned()),
        batch: true,
        report_summary: true,
        patch: true,
        minor: false,
        major: false,
        registry: Some("https://registry.example".to_owned()),
        token: Some("test-token".to_owned()),
        trusted: false,
        trusted_audience: Some("https://registry.example".to_owned()),
    };

    match common_to_dispatch(command) {
        Some(CommonCommand::Publish {
            protocol,
            package,
            version,
            artifacts: actual_artifacts,
            image,
            tag,
            access,
            dry_run,
            json,
            otp,
            force,
            ignore_scripts,
            allow_scripts,
            no_git_checks,
            publish_branch,
            batch,
            report_summary,
            patch,
            minor,
            major,
            registry,
            token,
            trusted,
            trusted_audience,
        }) => {
            assert_eq!(protocol, "pypi");
            assert_eq!(package.as_deref(), Some("sample"));
            assert_eq!(version.as_deref(), Some("1.4.2"));
            assert_eq!(actual_artifacts, artifacts);
            assert_eq!(image.as_deref(), Some("sample:1.4.2"));
            assert_eq!(tag.as_deref(), Some("stable"));
            assert_eq!(access.as_deref(), Some("restricted"));
            assert!(dry_run);
            assert!(json);
            assert_eq!(otp.as_deref(), Some("123456"));
            assert!(force);
            assert!(!ignore_scripts);
            assert!(allow_scripts);
            assert!(no_git_checks);
            assert_eq!(publish_branch.as_deref(), Some("release"));
            assert!(batch);
            assert!(report_summary);
            assert!(patch);
            assert!(!minor);
            assert!(!major);
            assert_eq!(registry.as_deref(), Some("https://registry.example"));
            assert_eq!(token.as_deref(), Some("test-token"));
            assert!(!trusted);
            assert_eq!(
                trusted_audience.as_deref(),
                Some("https://registry.example")
            );
        }
        _ => panic!("publish command or payload was changed"),
    }
}
