//! Nearest-owner test wiring — wiring test tại owner gần nhất.
//! Test files stay grouped by the behavior they verify.
//! File test được nhóm theo hành vi cần xác minh.

mod game_and_iot_list_are_blocked_without_installed_state_evidence;
mod list_is_not_native_when_the_core_cannot_read_its_dependency_manifest;
mod offline_reinstall_is_not_inherited_from_online_install;
mod store_gc_is_not_inherited_from_an_install_owner;
