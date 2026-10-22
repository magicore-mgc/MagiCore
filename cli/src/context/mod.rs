//! Shared CLI project-context exports and owner wiring.
//! Export và kết nối owner ngữ cảnh project dùng chung của CLI.

mod config_resolution;
mod context;

pub use context::ProjectContext;
