#[cfg(test)]
use super::record_validation::normalize_tool_target;
#[cfg(test)]
use super::shim::render_windows_shim_contents;
use super::{install::install, list::list, remove::remove, update::update};
use crate::commands::definitions::ToolCmd;
use anyhow::Result;

pub fn run(command: ToolCmd) -> Result<()> {
    match command {
        ToolCmd::Install {
            package,
            bin,
            allow_scripts,
        } => install(&package, bin.as_deref(), allow_scripts),
        ToolCmd::Update {
            binary,
            version,
            allow_scripts,
        } => update(&binary, version.as_deref(), allow_scripts),
        ToolCmd::Remove { binary } => remove(&binary),
        ToolCmd::List => list(),
    }
}

#[cfg(test)]
#[path = "../test/tool.rs"]
mod tests;
