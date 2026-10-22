//! Fail-closed CAS claim context owner.
//! Module chủ quản context claim CAS fail-closed.
mod claims;
mod types;
pub use claims::{cas_claim_context, claim_ctx};
pub use types::CasClaimContext;
