//! java recognizer wiring.
//! Wiring bộ nhận diện java.
mod java;
pub(in crate::project::project::core_detection) use java::detect_jvm_cores;
