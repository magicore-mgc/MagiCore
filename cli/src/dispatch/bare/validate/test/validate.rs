//! Bare-command preflight contracts across supported cores.
//! Test contract preflight lệnh bare trên các core được hỗ trợ.
use crate::commands::definitions::commands::BareCommands;
use crate::dispatch::bare::validate::validate_bare_request;

fn install_command(dry_run: bool, offline: bool) -> BareCommands {
    BareCommands::Install {
        packages: Vec::new(),
        frozen: false,
        ignore_scripts: false,
        allow_scripts: false,
        prefer_dedupe: false,
        repair: false,
        dry_run,
        offline,
        compat_runtime: None,
    }
}

fn global_add_command() -> BareCommands {
    BareCommands::Add {
        packages: vec!["fixture".to_string()],
        version: None,
        dev: false,
        global: true,
        exact: false,
        optional: false,
        peer: false,
        no_save: false,
        no_install: false,
        compat_runtime: None,
    }
}

#[test]
fn global_add_fails_before_core_validation() {
    let error = validate_bare_request(&global_add_command(), None)
        .expect_err("global add must fail closed");

    assert_eq!(
        error.to_string(),
        "`mgc add --global` is not supported: `mgc add` installs in the current project. Use `mgc tool install npm:<package>` for a persistent npm CLI or `mgc dlx <package>` for temporary execution; no project changes were made"
    );
}

#[test]
fn dry_run_is_allowed_only_for_ai_app_and_cloud() {
    for core in ["ai", "app", "clo"] {
        assert!(validate_bare_request(&install_command(true, false), Some(core)).is_ok());
    }

    for core in [
        "web",
        "lib",
        "game",
        "cicd",
        "iot",
        "hardware",
        "unknown-core",
    ] {
        let error = validate_bare_request(&install_command(true, false), Some(core))
            .expect_err("unsupported dry-run core must be rejected");
        assert!(
            error
                .to_string()
                .contains(&format!("dry-run is not supported for core '{core}'"))
        );
    }

    let error = validate_bare_request(&install_command(true, false), None)
        .expect_err("missing core must be rejected for dry-run");
    assert!(
        error
            .to_string()
            .contains("dry-run is not supported for core 'unknown'")
    );
}

#[test]
fn offline_is_allowed_only_for_web_lib_ai_and_app() {
    for core in ["web", "lib", "ai", "app"] {
        assert!(validate_bare_request(&install_command(false, true), Some(core)).is_ok());
    }

    for core in ["game", "clo", "cicd", "iot", "hardware", "unknown-core"] {
        let error = validate_bare_request(&install_command(false, true), Some(core))
            .expect_err("unsupported offline core must be rejected");
        assert!(error.to_string().contains(&format!(
            "offline install is not implemented for core '{core}'"
        )));
    }

    let error = validate_bare_request(&install_command(false, true), None)
        .expect_err("missing core must be rejected for offline install");
    assert!(
        error
            .to_string()
            .contains("offline install is not implemented for core 'unknown'")
    );
}
