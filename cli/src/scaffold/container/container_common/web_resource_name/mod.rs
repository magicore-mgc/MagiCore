//! Web resource-name owner wiring — nối implementation cùng tên.
//! Wiring owner tên resource Web — nối implementation cùng tên.

#[path = "web_resource_name.rs"]
mod web_resource_name;

pub(in crate::scaffold::container) use web_resource_name::web_container_resource_name;
