//! Register scoped environment restoration coverage.
//! Đăng ký test khôi phục environment scoped.

#[path = "environment_override.rs"]
mod environment_override;

pub(crate) use environment_override::environment_override_restores_preexisting_and_absent_values;
