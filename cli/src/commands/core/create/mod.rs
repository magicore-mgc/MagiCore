//! `mgc create-<core>` — router: core detect → file con (v5: LỆNH = folder, CORE = file).
//!
//! T5: provider = registry starter kit (create-mgc-<core>) → fallback local template → wizard.
//! T9a: sau scaffold, tự ghi `.mgc.core` marker tại project folder.
//!
//! Security: `validate_project_name` chặn path traversal/absolute tại CLI
//! boundary — mọi create-<core> đều đi qua đây trước khi chạm filesystem.

mod metadata;
mod validation;

pub(crate) use metadata::scaffold_and_save_metadata;
pub use validation::validate_project_name;

#[cfg(feature = "ai")]
pub mod ai;
#[cfg(feature = "app")]
pub mod app;
#[cfg(feature = "cicd")]
pub mod cicd;
#[cfg(feature = "clo")]
pub mod clo;
#[cfg(feature = "game")]
pub mod game;
#[cfg(feature = "hardware")]
pub mod hardware;
#[cfg(feature = "iot")]
pub mod iot;
#[cfg(feature = "lib")]
pub mod library;
#[cfg(feature = "web")]
pub mod web;

// Router `run(core, ...)` đã bị loại: dispatch live (dispatch/core/create.rs) gọi
// thẳng từng create::<core>::run và ghi .mgc.core marker qua init/wizard path —
// router trung gian này trùng lắp 100% và không còn caller.
// Removed legacy router: live dispatch calls each create subcommand directly.

#[cfg(test)]
#[path = "../../../test/create_security_test.rs"]
mod tests;
