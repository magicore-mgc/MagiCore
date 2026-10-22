// CoreCommand owner test — protects the route type contract.
// Test owner CoreCommand — giữ nguyên policy cài package theo core.
use super::CoreCommand;

#[test]
fn web_install_retains_all_policy_controls() {
    let command = CoreCommand::InstallWeb {
        packages: vec!["react".into()],
        frozen: true,
        ignore_scripts: false,
        allow_scripts: true,
        prefer_dedupe: true,
        repair: false,
        offline: true,
        compat_runtime: Some("bun".into()),
    };

    match command {
        CoreCommand::InstallWeb {
            packages,
            frozen,
            ignore_scripts,
            allow_scripts,
            prefer_dedupe,
            repair,
            offline,
            compat_runtime,
        } => {
            assert_eq!(packages, vec!["react".to_string()]);
            assert!(frozen && allow_scripts && prefer_dedupe && offline);
            assert!(!ignore_scripts && !repair);
            assert_eq!(compat_runtime.as_deref(), Some("bun"));
        }
        _ => panic!("web install payload changed core route"),
    }
}
