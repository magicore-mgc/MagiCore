//! Warning gate wiring / Wiring cổng cảnh báo.
//! Provides one process-wide warning gate shared by enforcement policies.

mod warning;

pub(in crate::attestation) use warning::warn_once;
