use super::super::matches;
use crate::dispatch::types::CoreCommand;

#[test]
fn owns_all_four_package_lifecycle_variants_and_rejects_other_cores() {
    let commands = [
        CoreCommand::AddLib {
            packages: Vec::new(),
            dev: false,
            exact: false,
            optional: false,
            peer: false,
            no_save: false,
            global: false,
            compat_runtime: None,
            version: None,
        },
        CoreCommand::RemoveLib {
            packages: Vec::new(),
            compat_runtime: None,
        },
        CoreCommand::ListLib {
            compat_runtime: None,
        },
        CoreCommand::UpdateLib {
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
