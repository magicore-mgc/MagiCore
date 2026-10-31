//! Wires canonical v4 finalization.
//! Nối hoàn thiện canonical v4.

mod error;
mod v4_finalization;

pub use self::v4_finalization::{deterministic_generated_at, finalize_v4_document};
