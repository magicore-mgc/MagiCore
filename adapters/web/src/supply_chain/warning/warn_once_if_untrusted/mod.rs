//! One-time warning owner wiring.
//! Wiring owner cảnh báo một lần.

mod warn_once_if_untrusted;

pub(in crate::supply_chain) use warn_once_if_untrusted::warn_once_if_untrusted;
