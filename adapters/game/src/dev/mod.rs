//! Development server for game engines facade.
//! Facade dev server cho engine game: Bevy (cargo run), Godot (editor), Unity/Unreal (stub P1).

mod launcher;
mod types;

pub use launcher::start_dev_server;
pub use types::DevCommand;

#[cfg(test)]
#[path = "test/mod_test.rs"]
mod tests;
