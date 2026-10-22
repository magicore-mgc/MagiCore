//! CI/CD provider detection module wiring.
//! Wiring module nhận diện provider CI/CD.

mod detection;
mod explicit_config;
mod markers;
mod provider_name;

pub use detection::detect_provider;
