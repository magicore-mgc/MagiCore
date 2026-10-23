//! Game project scaffolding facade (Q13 template.toml system).
//! Facade scaffold project game (hệ template.toml Q13).

pub mod bevy;
pub mod godot;
pub mod unity;
pub mod unreal;

mod context;
mod project;
mod render;

pub use context::TemplateContext;
pub use project::scaffold_project;
pub use render::render_template;

#[cfg(test)]
#[path = "test/mod_test.rs"]
mod tests;
