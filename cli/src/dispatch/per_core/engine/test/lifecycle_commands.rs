// Lifecycle command mapping — preserve build, flash and server options.
// Mapping lệnh lifecycle — giữ tùy chọn build, flash và server.
use super::super::engine_to_common;
use crate::commands::definitions::commands::engine::EngineCommands;
use crate::dispatch::types::CommonCommand;

#[test]
fn maps_optimizer_overwrite_flag() {
    assert!(matches!(
        engine_to_common(EngineCommands::Optimizer { force: true }),
        Some(CommonCommand::Optimizer { force: true })
    ));
}

#[test]
fn maps_flash_board_and_build_bypass_flag() {
    match engine_to_common(EngineCommands::Flash {
        board: Some("esp32s3".to_owned()),
        skip_build: true,
    }) {
        Some(CommonCommand::Flash { board, skip_build }) => {
            assert_eq!(board.as_deref(), Some("esp32s3"));
            assert!(skip_build);
        }
        _ => panic!("flash command or payload was changed"),
    }
}

#[test]
fn maps_deploy_run_flag_without_invoking_a_provider() {
    assert!(matches!(
        engine_to_common(EngineCommands::Deploy { run: true }),
        Some(CommonCommand::Deploy { run: true })
    ));
}

#[test]
fn maps_ci_generation_unit_command() {
    assert!(matches!(
        engine_to_common(EngineCommands::CiGenerate),
        Some(CommonCommand::CiGenerate)
    ));
}

#[test]
fn maps_verify_unit_command() {
    assert!(matches!(
        engine_to_common(EngineCommands::Verify),
        Some(CommonCommand::Verify)
    ));
}

#[test]
fn maps_start_unit_command() {
    assert!(matches!(
        engine_to_common(EngineCommands::Start),
        Some(CommonCommand::Start)
    ));
}
