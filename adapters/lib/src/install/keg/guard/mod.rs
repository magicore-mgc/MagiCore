//! Keg mutation guard owner wiring (exclusive scope + undo steps).
//! Wiring owner guard thao tác keg (phạm vi độc quyền + bước hoàn tác).

mod guard;

pub use guard::{EntryState, KegGuard, Undo, capture_entry, restore_entry};
