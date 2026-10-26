//! Build command facade — export build command entry points.
//! Facade lệnh build — export entry point các lệnh build.

#[path = "build.rs"]
mod implementation;
pub use implementation::*;
pub(crate) use implementation::{node_bin_args, python_cmd, tool_unavailable};

/// Translate the CLI `--engine` value into a preference.
/// Chuyển giá trị `--engine` của CLI thành một lựa chọn.
pub(crate) fn engine_from_flag(
    value: Option<&str>,
) -> anyhow::Result<crate::commands::build::EnginePreference> {
    use crate::commands::build::EnginePreference;
    match value {
        None | Some("auto") => Ok(EnginePreference::Auto),
        Some("magi") => Ok(EnginePreference::MagiCore),
        Some("framework") => Ok(EnginePreference::Framework),
        Some(other) => {
            anyhow::bail!("unknown --engine value '{other}': expected auto, magi or framework")
        }
    }
}
