//! cargo manifest recognition.
//! Nhận diện manifest cargo.
use super::super::shared::{has_named_dependency, signature_file_exists};
use crate::project::read_regular_project_text;
use std::path::Path;
pub(in crate::project::project::core_detection) fn detect_cargo_cores(
    project_root: &Path,
    has_game_marker: bool,
    has_iot_marker: bool,
) -> anyhow::Result<Vec<String>> {
    let path = project_root.join("Cargo.toml");
    if !signature_file_exists(&path)? {
        return Ok(Vec::new());
    }
    let manifest = read_regular_project_text(&path, "Cargo manifest")?
        .ok_or_else(|| anyhow::anyhow!("Cargo manifest disappeared during core detection"))?;
    let value: toml::Value = toml::from_str(&manifest)?;
    let has_bevy = has_named_dependency(&value, &["bevy"]);
    let has_esp32 = has_named_dependency(
        &value,
        &["esp-idf-hal", "esp-idf-sys", "esp-hal", "esp32-hal"],
    );
    let mut cores = Vec::new();
    if has_bevy {
        cores.push("game".to_string());
    }
    if has_esp32 {
        cores.push("iot".to_string());
    }
    if !has_bevy && !has_esp32 && !has_game_marker && !has_iot_marker {
        cores.push("lib".to_string());
    }
    Ok(cores)
}
