// Nested service command mapping — preserve store, trust and hook payloads.
// Mapping service lồng nhau — giữ payload store, trust và hook.
use super::super::bare_common_to_common;
use crate::commands::definitions::commands::bare::BareCommands;
use crate::commands::hooks::HooksCmd;
use crate::commands::store::StoreCmd;
use crate::commands::trust::TrustCmd;
use crate::dispatch::types::CommonCommand;

#[test]
fn maps_store_health_check_options() {
    match bare_common_to_common(BareCommands::Store {
        cmd: StoreCmd::Doctor {
            repair: true,
            core: "app".to_owned(),
            framework: Some("flutter".to_owned()),
            json: true,
        },
    }) {
        Some(CommonCommand::Store {
            cmd:
                StoreCmd::Doctor {
                    repair,
                    core,
                    framework,
                    json,
                },
        }) => {
            assert!(repair);
            assert_eq!(core, "app");
            assert_eq!(framework.as_deref(), Some("flutter"));
            assert!(json);
        }
        _ => panic!("store command or payload was changed"),
    }
}

#[test]
fn maps_trust_anchor_rotation_options() {
    match bare_common_to_common(BareCommands::Trust {
        cmd: TrustCmd::Anchor {
            re_attest: true,
            rotate: false,
            key_id: Some("key-7".to_owned()),
        },
    }) {
        Some(CommonCommand::Trust {
            cmd:
                TrustCmd::Anchor {
                    re_attest,
                    rotate,
                    key_id,
                },
        }) => {
            assert!(re_attest);
            assert!(!rotate);
            assert_eq!(key_id.as_deref(), Some("key-7"));
        }
        _ => panic!("trust command or payload was changed"),
    }
}

#[test]
fn maps_hook_event_name() {
    match bare_common_to_common(BareCommands::Hooks {
        cmd: HooksCmd::Run {
            event: "pre-install".to_owned(),
        },
    }) {
        Some(CommonCommand::Hooks {
            cmd: HooksCmd::Run { event },
        }) => assert_eq!(event, "pre-install"),
        _ => panic!("hooks command or payload was changed"),
    }
}
