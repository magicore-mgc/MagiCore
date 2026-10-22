//! Wait-loop coordinator implementation export.
//! Export coordinator của vòng chờ.

#[path = "wait_loop.rs"]
mod implementation;

pub(in crate::run::implementation) use implementation::wait_with_timeout;
