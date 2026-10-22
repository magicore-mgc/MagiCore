//! Typed failures from execution timeout environment configuration.
//! Lỗi typed từ cấu hình timeout môi trường của lệnh thực thi.

use std::{fmt, num::ParseIntError};

/// Invalid timeout configuration, kept local to the execution-timeout owner.
/// Cấu hình timeout sai, chỉ thuộc owner execution-timeout.
#[derive(Debug)]
pub(in crate::run::implementation) enum TimeoutError {
    InvalidUnsignedSeconds(ParseIntError),
    ZeroSeconds,
    NonUnicodeValue,
}

impl fmt::Display for TimeoutError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUnsignedSeconds(_) => formatter
                .write_str("MGC_EXEC_TIMEOUT_SECS must be an unsigned whole number of seconds"),
            Self::ZeroSeconds => {
                formatter.write_str("MGC_EXEC_TIMEOUT_SECS must be greater than zero")
            }
            Self::NonUnicodeValue => {
                formatter.write_str("MGC_EXEC_TIMEOUT_SECS must contain valid Unicode text")
            }
        }
    }
}

impl std::error::Error for TimeoutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidUnsignedSeconds(source) => Some(source),
            Self::ZeroSeconds | Self::NonUnicodeValue => None,
        }
    }
}
