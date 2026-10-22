//! Guarded PATH owner wiring.
//! Wiring owner PATH được bảo vệ.

mod guarded_path;

pub(in crate::run) use guarded_path::{guarded_path_env, is_path_env_key};
