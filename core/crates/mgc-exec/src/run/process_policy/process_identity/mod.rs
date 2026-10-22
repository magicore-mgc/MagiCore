//! Process identity owner exports.
//! Export owner identity process.

#[path = "process_identity.rs"]
mod implementation;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(in crate::run::implementation) use implementation::{
    is_npm_version_shell_probe, process_basename,
};
