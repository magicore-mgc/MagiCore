//! Cargo feature activation owner wiring.
//! Wiring owner kích hoạt feature Cargo.

mod activation;
mod error;

pub(super) use activation::activate_crate_features;
pub(super) use error::{FeatureActivationError, FeatureActivationResult};
