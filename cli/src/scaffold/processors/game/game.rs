//! Stable GameProcessor API and framework dispatch — API ổn định và dispatch framework.

use anyhow::Result;
use std::path::Path;

use super::{bevy, godot, unity, unreal};

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
            "unity" => unity::files(target),
            "godot" => godot::files(target),
            "unreal" => unreal::files(target, name),
            _ => bevy::files(target, name),
        }
    }
}
