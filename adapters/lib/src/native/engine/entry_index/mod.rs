//! Entry Index owner wiring.
//! Nối owner entry index.

mod entry_index;

#[cfg(test)]
mod test;

pub(in crate::native::engine::engine) use entry_index::{
    entry_key, index_entries, resolve_entry_key,
};
