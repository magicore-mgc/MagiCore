//! Nearest test modules for the lib keg/prefix transfer layer.
//! Các module test gần lớp keg/prefix của core lib.

#[path = "active_pointer.rs"]
mod active_pointer;
#[path = "guard_rollback.rs"]
mod guard_rollback;
#[path = "immutability.rs"]
mod immutability;
#[path = "install_switch.rs"]
mod install_switch;
#[path = "keg_store.rs"]
mod keg_store;
#[path = "layout_paths.rs"]
mod layout_paths;
#[path = "lib_facade.rs"]
mod lib_facade;
#[path = "prefix_links.rs"]
mod prefix_links;
mod support;
#[path = "switch_fail_closed.rs"]
mod switch_fail_closed;
