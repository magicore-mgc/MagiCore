//! Typed errors for Game dev decisions owned by this core.
//! Lỗi typed cho quyết định dev do Game core sở hữu.

#[derive(Debug, thiserror::Error)]
pub enum GameDevError {
    #[error(
        "unreal dev runs in Unreal Editor (engine binary is named by version, no standard PATH) \
        — open {uproject} in the editor, or install UnrealBuildTool and run the build directly. \
        Run `mgc build` + the editor to run."
    )]
    UnrealEditorGuidance { uproject: String },
    #[error("'mgc dev' has no command for engine '{engine}' — use the engine editor to run it")]
    UnsupportedEngine { engine: String },
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
