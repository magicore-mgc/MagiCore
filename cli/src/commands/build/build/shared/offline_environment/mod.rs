#[path = "offline_environment.rs"]
mod implementation;

pub(in crate::commands::build::implementation) use implementation::go_offline_env;
