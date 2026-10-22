//! Self-validating record owner wiring.
//! Wiring owner record tự kiểm tra.

mod record;
pub(in crate::cas::compiled_cache) use record::{CompiledRecord, VerifyOutcome};
