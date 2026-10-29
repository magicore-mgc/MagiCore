//! Active-version pointer owner wiring.
//! Wiring owner con trỏ version active.

mod active;

pub use active::{ACTIVE_POINTER_FILE, ActiveState, pointer_path, read_active, write_active};
