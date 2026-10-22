// Runtime command mapping — preserve runtime, runner and argv options.
// Mapping lệnh runtime — giữ tùy chọn runtime, runner và argv.
use super::super::engine_to_common;
use crate::commands::definitions::commands::engine::EngineCommands;
use crate::dispatch::types::CommonCommand;

fn mapped(command: EngineCommands) -> CommonCommand {
    match engine_to_common(command) {
        Some(command) => command,
        None => panic!("engine command must be mapped"),
    }
}

#[test]
fn maps_dev_host_port_and_compatibility_options() {
    match mapped(EngineCommands::Dev {
        host: Some("127.0.0.1".to_owned()),
        port: Some(4315),
        clear: true,
        compat_runtime: Some("bun".to_owned()),
        compat_runner: Some("pnpm".to_owned()),
    }) {
        CommonCommand::Dev {
            host,
            port,
            clear,
            compat_runtime,
            compat_runner,
        } => {
            assert_eq!(host.as_deref(), Some("127.0.0.1"));
            assert_eq!(port, Some(4315));
            assert!(clear);
            assert_eq!(compat_runtime.as_deref(), Some("bun"));
            assert_eq!(compat_runner.as_deref(), Some("pnpm"));
        }
        _ => panic!("dev command mapped to a different variant"),
    }
}

#[test]
fn maps_run_script_arguments_and_both_runner_options() {
    match mapped(EngineCommands::Run {
        script: "build".to_owned(),
        args: vec!["--release".to_owned()],
        compat_runtime: Some("deno".to_owned()),
        compat_runner: Some("yarn".to_owned()),
    }) {
        CommonCommand::Run {
            script,
            args,
            compat_runtime,
            compat_runner,
        } => {
            assert_eq!(script, "build");
            assert_eq!(args, vec!["--release".to_owned()]);
            assert_eq!(compat_runtime.as_deref(), Some("deno"));
            assert_eq!(compat_runner.as_deref(), Some("yarn"));
        }
        _ => panic!("run command mapped to a different variant"),
    }
}

#[test]
fn maps_test_arguments_and_compatibility_options() {
    match mapped(EngineCommands::Test {
        args: vec!["--nocapture".to_owned()],
        compat_runtime: Some("bun".to_owned()),
        compat_runner: Some("npm".to_owned()),
    }) {
        CommonCommand::Test {
            args,
            compat_runtime,
            compat_runner,
        } => {
            assert_eq!(args, vec!["--nocapture".to_owned()]);
            assert_eq!(compat_runtime.as_deref(), Some("bun"));
            assert_eq!(compat_runner.as_deref(), Some("npm"));
        }
        _ => panic!("test command mapped to a different variant"),
    }
}

#[test]
fn maps_build_target_and_compatibility_options() {
    match mapped(EngineCommands::Build {
        target: Some("browser".to_owned()),
        compat_runtime: Some("deno".to_owned()),
        compat_runner: Some("pnpm".to_owned()),
    }) {
        CommonCommand::Build {
            target,
            compat_runtime,
            compat_runner,
        } => {
            assert_eq!(target.as_deref(), Some("browser"));
            assert_eq!(compat_runtime.as_deref(), Some("deno"));
            assert_eq!(compat_runner.as_deref(), Some("pnpm"));
        }
        _ => panic!("build command mapped to a different variant"),
    }
}

#[test]
fn maps_exec_command_arguments_and_delegated_runner() {
    match mapped(EngineCommands::Exec {
        command: "cargo".to_owned(),
        args: vec!["check".to_owned(), "--workspace".to_owned()],
        compat_runner: Some("npm".to_owned()),
    }) {
        CommonCommand::Exec {
            command,
            args,
            compat_runner,
        } => {
            assert_eq!(command, "cargo");
            assert_eq!(args, vec!["check".to_owned(), "--workspace".to_owned()]);
            assert_eq!(compat_runner.as_deref(), Some("npm"));
        }
        _ => panic!("exec command mapped to a different variant"),
    }
}
