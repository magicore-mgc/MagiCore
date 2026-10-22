//! Wiring facade for RECORD member checks.
//! Facade wiring cho kiểm tra member theo RECORD.

#[path = "record_verification.rs"]
mod implementation;

pub(super) use implementation::verify_record_members;
