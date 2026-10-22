// Per-core list module — wire the explicit CLI adapter.
// Module list theo core — nối adapter CLI tường minh.
mod list;

pub(in crate::dispatch) use list::map;
