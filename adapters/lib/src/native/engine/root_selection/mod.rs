//! Root Selection owner wiring.
//! Nối owner root selection.

mod root_selection;

#[cfg(test)]
mod test;

pub(in crate::native::engine::engine) use root_selection::{
    registry_roots, resolve_root_ids, selected_roots,
};
