use crate::commands::definitions::commands::add::AddCommands;
use clap::Parser;

#[derive(Parser)]
#[command(name = "mgc")]
struct AddCommandTestCli {
    #[command(subcommand)]
    command: AddCommands,
}

#[test]
fn web_add_alias_parses_multiple_packages_and_schema_flags() {
    let parsed = AddCommandTestCli::try_parse_from([
        "mgc",
        "a-web",
        "zod",
        "react",
        "--dev",
        "--exact",
        "--no-install",
        "--version",
        "1.2.3",
    ])
    .unwrap();

    match parsed.command {
        AddCommands::AddWeb {
            packages,
            dev,
            exact,
            no_install,
            version,
            ..
        } => {
            assert_eq!(packages, vec!["zod".to_owned(), "react".to_owned()]);
            assert!(dev);
            assert!(exact);
            assert!(no_install);
            assert_eq!(version.as_deref(), Some("1.2.3"));
        }
        _ => panic!("expected web add variant"),
    }
}
