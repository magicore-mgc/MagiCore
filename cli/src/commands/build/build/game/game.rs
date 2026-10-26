use super::super::*;
use super::GameBuildError;

/// Game build routes only implemented engines — chỉ chạy engine đã có build contract.
#[cfg(feature = "game")]
pub(in crate::commands::build::implementation) async fn build_game(root: &Path) -> Result<()> {
    let engine = mgc_game_adapter::detect_engine(root);
    match engine {
        Some(mgc_game_adapter::GameEngine::Bevy) => {
            // Load optimizer env for Bevy (Rust) builds
            let runtime = crate::commands::optimizer::runtime_detect::DetectedRuntime::RustLib;
            let optimizer_envs =
                crate::commands::optimizer::env_loader::load_optimizer_env(root, &runtime)?;
            build_rust_with_env(root, optimizer_envs)?;
            crate::commands::build::post_build::run_for_core(
                root,
                "game",
                &root.join("target"),
                false,
            )
        }
        Some(mgc_game_adapter::GameEngine::Godot) => Err(GameBuildError::Unsupported {
            target: "game/godot",
            guidance: "configure an export preset and run Godot export (03 §4 P2)",
        }
        .into()),
        Some(mgc_game_adapter::GameEngine::Unity) => Err(GameBuildError::Unsupported {
            target: "game/unity",
            guidance: "Unity batchmode export is not implemented yet (03 §4)",
        }
        .into()),
        Some(mgc_game_adapter::GameEngine::Unreal) => Err(GameBuildError::Unsupported {
            target: "game/unreal",
            guidance: "Unreal build is not implemented yet (03 §4 P2)",
        }
        .into()),
        None => Err(GameBuildError::FrameworkNotDetected {
            root: root.to_path_buf(),
        }
        .into()),
    }
}
