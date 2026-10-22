//! Embedded Web package-engine calls — lời gọi package engine Web nhúng.

mod web_delegation;

pub(super) use web_delegation::{
    arm_age_gate_for, prepare_add, set_dedupe_pref, set_existing_versions, supports_age_gate_for,
};
