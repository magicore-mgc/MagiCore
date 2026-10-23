//! Game project orchestration per engine.
//! Điều phối scaffold project game theo engine.

use crate::engine::GameEngine;
use mgc_types::MgResult;
use std::path::Path;

use super::super::context::TemplateContext;

/// Scaffold new game project
pub async fn scaffold_project(
    engine: GameEngine,
    context: TemplateContext,
    target_dir: &Path,
) -> MgResult<()> {
    std::fs::create_dir_all(target_dir)?;

    match engine {
        GameEngine::Bevy => super::super::bevy::scaffold(context, target_dir).await?,
        GameEngine::Godot => super::super::godot::scaffold(context, target_dir).await?,
        GameEngine::Unity => super::super::unity::scaffold(context, target_dir).await?,
        GameEngine::Unreal => super::super::unreal::scaffold(context, target_dir).await?,
    }

    Ok(())
}
