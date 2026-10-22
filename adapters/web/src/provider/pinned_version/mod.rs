//! pinned_version owner wiring.
//! Wiring owner policy cho version được pin.
mod pinned_version;
pub use pinned_version::check_pinned_version;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
