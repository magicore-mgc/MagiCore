//! Lock validation wiring — nối validator lockfile dự án.
//! Wiring kiểm tra lock — nối validator lockfile project.

#[path = "lock_validation.rs"]
mod lock_validation;

pub(in crate::dispatch) use lock_validation::validate;
