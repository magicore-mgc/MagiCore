// Bare install module — wire the mapper and its local diagnostic owner.
// Module install bare — nối mapper với owner diagnostic cục bộ.
mod error;
mod install;

pub(in crate::dispatch) use install::map;
