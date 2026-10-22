// CommonCommand owner test — protects the route type contract.
// Test owner CommonCommand — giữ nguyên payload của lệnh dùng chung.
use super::CommonCommand;

#[test]
fn dlx_retains_package_binary_script_policy_and_arguments() {
    let command = CommonCommand::Dlx {
        package: "@openai/codex@0.162.0".into(),
        bin: Some("codex".into()),
        allow_scripts: true,
        args: vec!["--version".into()],
    };

    match command {
        CommonCommand::Dlx {
            package,
            bin,
            allow_scripts,
            args,
        } => {
            assert_eq!(package, "@openai/codex@0.162.0");
            assert_eq!(bin.as_deref(), Some("codex"));
            assert!(allow_scripts);
            assert_eq!(args, vec!["--version".to_string()]);
        }
        _ => panic!("DLX payload changed command route"),
    }
}
