// Per-core create exports — expose the explicit adapter to the coordinator.
// Export create theo core — cung cấp adapter tường minh cho coordinator.
mod create;

pub(in crate::dispatch) use create::map;
