//! AI installed-inventory owner wiring.
//! Wiring owner danh sách package đã cài của AI.

mod installed_inventory;

pub(in crate::adapter::implementation) use installed_inventory::list_installed;
