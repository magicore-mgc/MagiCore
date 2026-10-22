//! Project/database release-age reader wiring.
//! Wiring bộ đọc tuổi phát hành từ project/database.

mod configured_store_min_age;

pub(in crate::supply_chain) use configured_store_min_age::configured_store_min_age;
