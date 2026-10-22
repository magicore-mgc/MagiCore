//! Godot fallback project files — file project fallback Godot.

use std::path::Path;

use anyhow::Result;

use super::super::super::write_file;

pub(in crate::scaffold::processors::game) fn files(target: &Path) -> Result<()> {
    write_file(&target.join("project.godot"), PROJECT_CONFIG)?;
    write_file(&target.join("Main.tscn"), MAIN_SCENE)?;
    write_file(&target.join("Main.gd"), MAIN_SCRIPT)?;
    Ok(())
}

const PROJECT_CONFIG: &str = r#"[application]
config/name="MagiCore Game"
run/main_scene="res://Main.tscn"
"#;

const MAIN_SCENE: &str = r#"[gd_scene load_steps=2 format=3]

[ext_resource type="Script" path="res://Main.gd" id="1_main"]

[node name="Main" type="Node2D"]
script = ExtResource("1_main")
"#;

const MAIN_SCRIPT: &str = r#"extends Node2D

func _ready() -> void:
    print("MagiCore Godot project ready")
"#;
