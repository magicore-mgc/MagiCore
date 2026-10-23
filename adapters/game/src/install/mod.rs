//! Public Game install facade. Package installation is unsupported until
//! MagiCore owns the selected engine's resolver, lock, fetch, and materializer.
//! Facade install Game công khai; hiện fail-closed tới khi MGC sở hữu đủ engine.

mod dispatch;
mod types;

pub use dispatch::install_dependencies;
pub use types::InstallSummary;

#[cfg(test)]
#[path = "test/mod_test.rs"]
mod tests;
