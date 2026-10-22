//! Patch engine facade and ownership wiring.
//! Facade và kết nối các owner của patch engine.

mod error;
mod hunk_application;
mod hunk_parser;
mod integrity;
mod patches;
mod patches_dir;
mod path_validation;
pub(super) mod types;

pub use integrity::verify_patch_integrity;
pub use patches::apply_patch;
pub use patches_dir::get_patches_dir;
