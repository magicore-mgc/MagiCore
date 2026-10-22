//! Bounded initialization of a missing global store.
//! Khởi tạo có giới hạn global store đang thiếu.

mod remediation;

pub(in crate::commands::doctor) use remediation::{
    RemediationOutcome, RemediationStatus, initialize_missing,
};

#[cfg(test)]
pub(super) use remediation::initialize_missing_at;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
