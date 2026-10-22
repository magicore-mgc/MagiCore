// Cache command mapping — retain action, target and safety flags.
// Mapping lệnh cache — giữ action, target và cờ an toàn.
use super::super::engine_to_common;
use crate::commands::definitions::commands::engine::EngineCommands;
use crate::dispatch::types::CommonCommand;

#[test]
fn maps_cache_confirmation_and_dry_run_options() {
    match engine_to_common(EngineCommands::Cache {
        action: "prune".to_owned(),
        target: "shared".to_owned(),
        yes: true,
        dry_run: true,
    }) {
        Some(CommonCommand::Cache {
            action,
            target,
            yes,
            dry_run,
        }) => {
            assert_eq!(action, "prune");
            assert_eq!(target, "shared");
            assert!(yes);
            assert!(dry_run);
        }
        _ => panic!("cache command or payload was changed"),
    }
}
