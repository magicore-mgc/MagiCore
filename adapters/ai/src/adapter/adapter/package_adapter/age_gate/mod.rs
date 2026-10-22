//! AI package-age gate owner wiring.
//! Wiring owner cổng tuổi package AI.

mod age_gate;

pub(in crate::adapter::implementation) use age_gate::{arm_age_gate_for, supports_age_gate_for};
