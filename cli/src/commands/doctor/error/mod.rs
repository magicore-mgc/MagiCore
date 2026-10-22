//! Local errors produced while rendering doctor output.
//! Lỗi cục bộ phát sinh khi render output doctor.

mod error;

pub(super) use error::DoctorError;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
