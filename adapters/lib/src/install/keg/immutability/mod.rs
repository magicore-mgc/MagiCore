//! Keg immutability owner wiring (seal / verify / thaw / remove).
//! Wiring owner bất biến keg (seal / verify / thaw / remove).

mod immutability;

pub use immutability::{KegFingerprint, remove_keg, seal_keg, thaw_keg, verify_keg};
