// Per-core update exports — expose the explicit adapter to its coordinator.
// Export update theo core — cung cấp adapter tường minh cho coordinator.
mod update;

pub(in crate::dispatch) use update::map;
