// Bench mapping — parse the real CLI schema and compare its retained value.
// Mapping bench — parse schema CLI thật và so sánh giá trị được giữ nguyên.
use super::super::bare_common_to_common;
use crate::commands::definitions::commands::{BareCommands, Commands};
use crate::dispatch::types::CommonCommand;
use clap::Parser;

#[test]
fn maps_bench_arguments_without_running_benchmark_or_install() {
    let cli = crate::Cli::try_parse_from(["mgc", "bench", "sample@1.2.3", "--allow-scripts"])
        .unwrap_or_else(|error| panic!("valid bench CLI input was rejected: {error}"));
    let args = match cli.command {
        Some(Commands::Bare(BareCommands::Bench { args })) => args,
        _ => panic!("CLI schema parsed bench into a different command"),
    };
    let expected = format!("{args:?}");

    match bare_common_to_common(BareCommands::Bench { args }) {
        Some(CommonCommand::Bench { args }) => assert_eq!(format!("{args:?}"), expected),
        _ => panic!("bench command or payload was changed"),
    }
}
