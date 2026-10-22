use super::super::matches;
use crate::dispatch::types::CoreCommand;

#[test]
fn owns_all_four_package_lifecycle_variants_and_rejects_other_cores() {
    let commands = [
        CoreCommand::AddHardware {
            packages: Vec::new(),
            compat_runtime: None,
            version: None,
        },
        CoreCommand::RemoveHardware {
            packages: Vec::new(),
            compat_runtime: None,
        },
        CoreCommand::ListHardware {
            compat_runtime: None,
        },
        CoreCommand::UpdateHardware {
            packages: Vec::new(),
            install: false,
            compat_runtime: None,
        },
    ];

    assert!(commands.iter().all(matches));
    assert!(!matches(&CoreCommand::ListGame {
        compat_runtime: None
    }));
}
