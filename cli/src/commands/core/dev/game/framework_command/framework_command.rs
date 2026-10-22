//! Map Game adapter identity to its existing editor/runtime command.
//! Ánh xạ identity Game adapter sang command editor/runtime hiện tại.

use std::path::Path;

use super::super::error::GameDevError;

pub(in crate::commands::core::dev::game) fn game_dev_command(
    root: &Path,
) -> anyhow::Result<(String, Vec<String>)> {
    let adapter = mgc_game_adapter::adapter_for(root)
        .ok_or_else(|| crate::error::no_framework_detected("game engine", root))?;
    match adapter.engine() {
        "bevy" => Ok((
            "cargo".to_string(),
            vec![
                "run".to_string(),
                "--locked".to_string(),
                "--offline".to_string(),
            ],
        )),
        "godot" => {
            let path = root.to_str().ok_or_else(crate::error::path_not_utf8)?;
            Ok((
                "godot".to_string(),
                vec![
                    "--editor".to_string(),
                    "--path".to_string(),
                    path.to_string(),
                ],
            ))
        }
        // unity: mở editor GUI qua CLI (fail-closed nếu unity không có trong PATH).
        "unity" => {
            let path = root.to_str().ok_or_else(crate::error::path_not_utf8)?;
            Ok((
                "unity".to_string(),
                vec!["-projectPath".to_string(), path.to_string()],
            ))
        }
        // unreal: engine binary đặt tên theo version, không có PATH chuẩn — chỉ dẫn mở editor.
        "unreal" => {
            let uproject = root
                .read_dir()
                .ok()
                .and_then(|mut entries| {
                    entries.find_map(|e| {
                        let name = e.as_ref().ok()?.file_name().to_string_lossy().to_string();
                        name.ends_with(".uproject").then_some(name)
                    })
                })
                .unwrap_or_else(|| "Game.uproject".to_string());
            Err(GameDevError::UnrealEditorGuidance { uproject }.into())
        }
        other => Err(GameDevError::UnsupportedEngine {
            engine: other.to_string(),
        }
        .into()),
    }
}
