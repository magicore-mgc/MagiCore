//! Keep the CLI environment contract for signature audit.
//! Giữ hợp đồng biến môi trường CLI cho audit chữ ký.

#[test]
fn signature_audit_reads_registry_from_mgc_registry_env() {
    use clap::CommandFactory as _;

    let command = crate::Cli::command();
    let audit = command
        .get_subcommands()
        .find(|subcommand| subcommand.get_name() == "audit")
        .expect("audit subcommand");
    let signatures = audit
        .get_subcommands()
        .find(|subcommand| subcommand.get_name() == "signatures")
        .expect("signatures subcommand");
    let registry = signatures
        .get_arguments()
        .find(|argument| argument.get_id() == "registry")
        .expect("registry argument");
    assert_eq!(
        registry.get_env().and_then(|value| value.to_str()),
        Some("MGC_REGISTRY")
    );
}
