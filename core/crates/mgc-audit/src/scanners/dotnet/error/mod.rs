//! Wire typed .NET scanner errors and local mapping tests.
//! Nối lỗi typed của scanner .NET và test ánh xạ cục bộ.

mod error;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(super) use self::error::DotnetLockParseError;
