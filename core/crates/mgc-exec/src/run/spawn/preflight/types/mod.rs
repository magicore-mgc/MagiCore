//! Prepared invocation type facade.
//! Facade kiểu invocation đã chuẩn bị.

#[path = "types.rs"]
mod implementation;

pub(in crate::run::implementation::spawn::implementation) use implementation::PreparedInvocation;
