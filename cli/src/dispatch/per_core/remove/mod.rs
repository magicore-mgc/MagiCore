// Per-core remove exports — expose the explicit adapter to the coordinator.
// Export remove theo core — cung cấp adapter tường minh cho coordinator.
mod remove;

pub(in crate::dispatch) use remove::map;
