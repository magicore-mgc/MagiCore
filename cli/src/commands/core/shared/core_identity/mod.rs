// core_identity owner wiring — nối owner core_identity.
// The implementation stays in its same-name module file.
use super::*;
#[path = "core_identity.rs"]
mod implementation;
pub(crate) use implementation::{
    core_project_root, ensure_project_core_identity, find_project_root,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
