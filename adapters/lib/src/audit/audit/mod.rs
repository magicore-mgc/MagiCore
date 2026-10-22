//! Wiring for Lib audit language dispatch.
//! Wiring cho dispatcher audit theo ngôn ngữ của Lib.

mod audit;
pub(crate) use audit::run_audit;
