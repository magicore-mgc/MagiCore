// Flattened public CLI command schema — giữ tên lệnh CLI ở cùng cấp.
use clap::Subcommand;

use super::{
    AddCommands, BareCommands, CommonCommands, CreateCommands, EngineCommands, InstallCommands,
    ListCommands, RemoveCommands, UpdateCommands,
};

#[derive(Subcommand, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Commands {
    #[command(flatten)]
    Common(CommonCommands),
    #[command(flatten)]
    Engine(EngineCommands),
    #[command(flatten)]
    Bare(BareCommands),
    #[command(flatten)]
    Create(CreateCommands),
    #[command(flatten)]
    Install(InstallCommands),
    #[command(flatten)]
    Add(AddCommands),
    #[command(flatten)]
    Remove(RemoveCommands),
    #[command(flatten)]
    List(ListCommands),
    #[command(flatten)]
    Update(UpdateCommands),
}
