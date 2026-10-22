// Diagnostic command mapping — preserve local telemetry and diagnostic flags.
// Mapping lệnh chẩn đoán — giữ telemetry cục bộ và cờ chẩn đoán.
use super::super::bare_common_to_common;
use crate::commands::definitions::commands::bare::BareCommands;
use crate::commands::doctor::DoctorCmd;
use crate::commands::network::NetworkCmd;
use crate::commands::telemetry::TelemetryCmd;
use crate::dispatch::types::CommonCommand;

#[test]
fn maps_telemetry_status_subcommand() {
    assert!(matches!(
        bare_common_to_common(BareCommands::Telemetry {
            cmd: TelemetryCmd::Off,
        }),
        Some(CommonCommand::Telemetry {
            cmd: TelemetryCmd::Off
        })
    ));
}

#[test]
fn maps_network_verification_json_flag() {
    assert!(matches!(
        bare_common_to_common(BareCommands::Network {
            cmd: NetworkCmd::Verify { json: true },
        }),
        Some(CommonCommand::Network {
            cmd: NetworkCmd::Verify { json: true }
        })
    ));
}

#[test]
fn maps_doctor_format_and_repair_flags() {
    assert!(matches!(
        bare_common_to_common(BareCommands::Doctor {
            cmd: DoctorCmd::Check {
                json: true,
                fix: true,
            },
        }),
        Some(CommonCommand::Doctor {
            cmd: DoctorCmd::Check {
                json: true,
                fix: true,
            }
        })
    ));
}
