//! Unreal fallback project files — file project fallback Unreal.

use std::path::Path;

use anyhow::Result;

use super::super::super::write_file;

pub(in crate::scaffold::processors::game) fn files(target: &Path, name: &str) -> Result<()> {
    write_file(
        &target.join(format!("{name}.uproject")),
        &UPROJECT.replace("{}", name),
    )?;
    write_file(
        &target.join("Source/MagiCoreGame/MagiCoreGame.Build.cs"),
        BUILD_SOURCE,
    )?;
    write_file(
        &target.join("Source/MagiCoreGame/MagiCoreGame.cpp"),
        MODULE_SOURCE,
    )?;
    Ok(())
}

const UPROJECT: &str = r#"{
  "FileVersion": 3,
  "EngineAssociation": "",
  "Category": "Games",
  "Description": "{}",
  "Modules": [
    { "Name": "MagiCoreGame", "Type": "Runtime", "LoadingPhase": "Default" }
  ]
}
"#;

const BUILD_SOURCE: &str = r#"using UnrealBuildTool;

public class MagiCoreGame : ModuleRules
{
    public MagiCoreGame(ReadOnlyTargetRules Target) : base(Target)
    {
        PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
    }
}
"#;

const MODULE_SOURCE: &str = r#"#include "CoreMinimal.h"
#include "Modules/ModuleManager.h"

IMPLEMENT_PRIMARY_GAME_MODULE(FDefaultGameModuleImpl, MagiCoreGame, "MagiCoreGame");
"#;
