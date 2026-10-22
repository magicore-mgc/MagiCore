// Bare add module — wire its adapter and local diagnostic owner.
// Module add bare — nối adapter với owner diagnostic cục bộ.
mod add;
mod error;

pub(in crate::dispatch) use add::map;
