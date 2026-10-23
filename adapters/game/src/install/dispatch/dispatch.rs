//! Game install dispatch: fail closed until MagiCore owns the engine toolchain.
//! Dispatch install Game: fail-closed tới khi MGC sở hữu toolchain engine.

use crate::engine::GameEngine;
use mgc_types::{MgError, MgResult};
use std::path::Path;

use super::super::types::InstallSummary;

/// Install dependencies for a game project.
/// Install dependency cho project game.
pub async fn install_dependencies(
    engine: GameEngine,
    _project_root: &Path,
) -> MgResult<InstallSummary> {
    Err(MgError::Unsupported {
        core: "game",
        capability: "install",
        guidance: format!(
            "MagiCore does not own dependency installation for the {engine:?} engine; no provider package manager was invoked"
        ),
    })
}
