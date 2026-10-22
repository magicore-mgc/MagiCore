// mutation_gateway owner wiring — nối owner mutation_gateway.
// The implementation stays in its same-name module file.
use super::*;
#[path = "mutation_gateway.rs"]
mod implementation;
pub(crate) use implementation::{begin_dependency_mutation, writer_lock_timeout};
pub(in crate::commands::core::shared::implementation::implementation) use implementation::{
    ensure_native_manifest_owner, ensure_native_update_owner, install_command_for_adapter,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
