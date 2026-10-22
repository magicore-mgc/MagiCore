//! Scoped environment override owner facade.
//! Facade owner quản lý env override có phạm vi.

#[path = "environment_override.rs"]
mod implementation;

pub(crate) use implementation::{
    EnvironmentOverrideGuard, environment_override_restores_preexisting_and_absent_values,
    lock_environment_access,
};
