#[path = "error.rs"]
mod error;
#[path = "policy.rs"]
mod implementation;

pub(crate) use implementation::reject_external_package_manager_script;
