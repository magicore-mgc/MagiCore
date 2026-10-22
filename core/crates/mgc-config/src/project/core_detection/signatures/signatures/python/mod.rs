//! Python manifest recognizer wiring.
//! Wiring nhận diện manifest Python.
mod python;
pub(in crate::project::project::core_detection) use python::detect_python_core;
