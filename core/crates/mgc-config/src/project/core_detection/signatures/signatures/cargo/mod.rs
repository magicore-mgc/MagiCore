//! cargo recognizer wiring.
//! Wiring bộ nhận diện cargo.
mod cargo;
pub(in crate::project::project::core_detection) use cargo::detect_cargo_cores;
