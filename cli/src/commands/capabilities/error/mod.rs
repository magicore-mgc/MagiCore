//! Capability error wiring — nối lỗi capability.
//! Re-exports the local typed error to the capability owner.
//! Xuất lỗi typed cục bộ cho owner capability.

mod error;

pub(super) use error::CapabilitiesError;
