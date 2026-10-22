//! Project security age-gate policy hooks.
//! Hook chính sách age-gate bảo mật theo project.

mod age_gate;

pub(super) use age_gate::{arm_age_gate_for, supports_age_gate_for};
