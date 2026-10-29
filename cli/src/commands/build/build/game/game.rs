use super::super::*;
use super::GameBuildError;

/// Game build routes only implemented engines — chỉ chạy engine đã có build contract.
#[cfg(feature = "game")]
pub(in crate::commands::build::implementation) async fn build_game(
    root: &Path,
    allow_output_warnings: bool,
) -> Result<()> {
    let engine = mgc_game_adapter::detect_engine(root);
    match engine {
        Some(mgc_game_adapter::GameEngine::Bevy) => {
            // Load optimizer env for Bevy (Rust) builds
            let runtime = crate::commands::optimizer::runtime_detect::DetectedRuntime::RustLib;
            let optimizer_envs =
                crate::commands::optimizer::env_loader::load_optimizer_env(root, &runtime)?;
            build_rust_with_env(root, optimizer_envs)?;
            // PAR-05: the caller's warning policy applies here exactly as it
            // does for `lib`; hardcoding `false` ignored `--allow-output-warnings`.
            // (PAR-05: chính sách cảnh báo của caller áp ở đây đúng như với
            // `lib`; hardcode `false` bỏ qua `--allow-output-warnings`.)
            crate::commands::build::post_build::run_for_core(
                root,
                "game",
                &root.join("target"),
                allow_output_warnings,
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
