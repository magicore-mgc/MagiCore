//! Game scaffold: unity/godot/unreal/bevy templates.

use std::path::Path;

use anyhow::Result;

use super::{slugify, write_file};

pub struct GameProcessor;

impl GameProcessor {
    /// Frameworks generated directly (the `_` arm is the bevy Cargo
    /// template). Unknown ids keep the layer-required error.
    /// (Framework tự sinh trực tiếp — id lạ vẫn fail rõ.)
    pub fn supports(framework: &str) -> bool {
        matches!(framework, "unity" | "godot" | "unreal" | "bevy")
    }

    pub fn files(target: &Path, name: &str, framework: &str) -> Result<()> {
        match framework {
            "unity" => {
                write_file(
                    &target.join("Packages").join("manifest.json"),
                    "{\n  \"dependencies\": {}\n}\n",
                )?;
                write_file(
                    &target.join("Assets").join("Scripts").join("Bootstrap.cs"),
                    "using UnityEngine;\n\npublic class Bootstrap : MonoBehaviour\n{\n    void Start()\n    {\n        Debug.Log(\"MagiCore Unity project ready\");\n    }\n}\n",
                )?;
            }
            "godot" => {
                write_file(
                    &target.join("project.godot"),
                    "[application]\nconfig/name=\"MagiCore Game\"\nrun/main_scene=\"res://Main.tscn\"\n",
                )?;
                write_file(
                    &target.join("Main.tscn"),
                    "[gd_scene load_steps=2 format=3]\n\n[ext_resource type=\"Script\" path=\"res://Main.gd\" id=\"1_main\"]\n\n[node name=\"Main\" type=\"Node2D\"]\nscript = ExtResource(\"1_main\")\n",
                )?;
                write_file(
                    &target.join("Main.gd"),
                    "extends Node2D\n\nfunc _ready() -> void:\n    print(\"MagiCore Godot project ready\")\n",
                )?;
            }
            "unreal" => {
                write_file(
                    &target.join(format!("{name}.uproject")),
                    &format!(
                        "{{\n  \"FileVersion\": 3,\n  \"EngineAssociation\": \"\",\n  \"Category\": \"Games\",\n  \"Description\": \"{}\",\n  \"Modules\": [\n    {{ \"Name\": \"MagiCoreGame\", \"Type\": \"Runtime\", \"LoadingPhase\": \"Default\" }}\n  ]\n}}\n",
                        name
                    ),
                )?;
                write_file(
                    &target.join("Source/MagiCoreGame/MagiCoreGame.Build.cs"),
                    "using UnrealBuildTool;\n\npublic class MagiCoreGame : ModuleRules\n{\n    public MagiCoreGame(ReadOnlyTargetRules Target) : base(Target)\n    {\n        PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;\n    }\n}\n",
                )?;
                write_file(
                    &target.join("Source/MagiCoreGame/MagiCoreGame.cpp"),
                    "#include \"CoreMinimal.h\"\n#include \"Modules/ModuleManager.h\"\n\nIMPLEMENT_PRIMARY_GAME_MODULE(FDefaultGameModuleImpl, MagiCoreGame, \"MagiCoreGame\");\n",
                )?;
            }
            _ => {
                write_file(
                    &target.join("Cargo.toml"),
                    &format!(
                        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nbevy = \"0.19\"\n",
                        slugify(name)
                    ),
                )?;
                write_file(
                    &target.join("src").join("main.rs"),
                    "use bevy::prelude::*;\n\nfn main() {\n    App::new()\n        .add_plugins(DefaultPlugins)\n        .add_systems(Startup, setup)\n        .add_systems(Update, frame_counter)\n        .run();\n}\n\nfn setup(mut commands: Commands) {\n    commands.spawn(Camera2d);\n    commands.spawn(Sprite {\n        color: Color::srgb(0.2, 0.8, 0.4),\n        custom_size: Some(Vec2::new(100.0, 100.0)),\n        ..default()\n    });\n}\n\nfn frame_counter(mut frames: Local<u64>) {\n    *frames += 1;\n    if *frames % 60 == 0 {\n        println!(\"MagiCore bevy game running ({} frames)\", *frames);\n    }\n}\n",
                )?;
            }
        }

        Ok(())
    }
}
