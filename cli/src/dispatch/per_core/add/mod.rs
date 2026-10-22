// Per-core add module — wire the explicit CLI adapter.
// Module add theo core — nối adapter CLI tường minh.
mod add;

pub(in crate::dispatch) use add::map;
