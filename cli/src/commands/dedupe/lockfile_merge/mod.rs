//! Lockfile merge owner wiring — exports exact-record deduplication to the coordinator.
//! Wiring owner merge lockfile — export khử trùng lặp bản ghi chính xác cho coordinator.
mod error;
mod lockfile_merge;
pub(super) use lockfile_merge::merged_lockfile;
