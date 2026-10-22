// Bare list module — wire the adapter and its local diagnostic owner.
// Module list bare — nối adapter với owner diagnostic cục bộ.
mod error;
mod list;

pub(in crate::dispatch) use list::map;
