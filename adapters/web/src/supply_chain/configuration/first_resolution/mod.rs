//! first_resolution owner wiring / wiring owner first_resolution.
#[path = "first_resolution.rs"]
mod first_resolution;

pub(in crate::supply_chain) use first_resolution::has_prior_install_decision;
