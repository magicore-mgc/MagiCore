//! Wire game engine detection from config and filesystem markers.
//! Nối nhận diện game engine từ config và marker filesystem.

mod detect_engine;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

pub use detect_engine::detect_engine;
