// Per-core install module — wire the CLI adapter and its local tests.
// Module install theo core — nối adapter CLI và test cục bộ.
mod install;

pub(in crate::dispatch) use install::map;
