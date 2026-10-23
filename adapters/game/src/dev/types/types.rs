//! Dev server command description shared by every game engine.
//! Mô tả lệnh dev server dùng chung cho mọi engine game.

/// Dev server command per engine
#[derive(Debug, Clone)]
pub struct DevCommand {
    pub command: String,
    pub args: Vec<String>,
    pub working_dir: String,
}
