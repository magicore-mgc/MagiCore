//! Go lockfile pin source owner wiring — wiring owner nguồn pin lock Go.

#[path = "lock.rs"]
mod implementation;

#[cfg(test)]
pub(super) use implementation::GO_OSV_ECOSYSTEM;
pub(in crate::scanners::govulncheck) use implementation::go_pins_from_mgc_lock;
