#[path = "chain/mod.rs"]
mod chain;
#[path = "framework_script.rs"]
mod implementation;
#[path = "policy/mod.rs"]
mod policy;
#[path = "script/mod.rs"]
mod script;
#[path = "types/mod.rs"]
mod types;

#[cfg(test)]
pub(crate) use chain::map_framework_build_chain;
pub(crate) use implementation::run_framework_build_if_supported;
#[cfg(test)]
pub(crate) use policy::reject_external_package_manager_script;
#[cfg(test)]
pub(crate) use script::map_framework_build_script;
