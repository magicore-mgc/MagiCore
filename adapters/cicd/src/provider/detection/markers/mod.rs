//! CI/CD marker detector module wiring.
//! Wiring module nhận diện marker CI/CD.

mod markers;

pub(super) use markers::provider_from_markers;
