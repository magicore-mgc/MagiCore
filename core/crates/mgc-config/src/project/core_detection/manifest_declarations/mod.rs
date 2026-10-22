//! Explicit project-manifest identity wiring.
//! Wiring identity tường minh trong manifest project.
mod cargo;
mod manifest_declarations;
mod package_json;
mod pyproject;
pub(super) use manifest_declarations::detect_manifest_core;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
