//! Public PyPI wheel-record entrypoint and compatibility facade.
//! Entry và facade tương thích cho RECORD của wheel PyPI.

use crate::protocols::pypi::PypiProtocol;
use mgc_types::{MgError, MgResult};
use std::path::Path;

/// Preserve the protocol-internal parser contract while the CSV owner keeps
/// its typed error for local handling.
/// Giữ contract parser nội bộ; owner CSV vẫn giữ lỗi typed để xử lý cục bộ.
#[cfg(test)]
pub(in crate::protocols::pypi) fn parse_record_row(
    line: &str,
) -> Result<Vec<String>, &'static str> {
    super::record_csv::parse_record_row(line).map_err(|error| match error {
        super::record_csv::RecordCsvError::ExpectedThreeFields => {
            "expected exactly three CSV fields"
        }
        super::record_csv::RecordCsvError::UnterminatedQuotedField => {
            "unterminated quoted CSV field"
        }
        super::record_csv::RecordCsvError::UnexpectedDataAfterQuotedField => {
            "unexpected data after quoted CSV field"
        }
    })
}

impl PypiProtocol {
    /// Re-verify an installed site tree against the authenticated wheel RECORD.
    /// Xác minh lại cây đã cài bằng RECORD trong wheel đã xác thực.
    pub fn verify_runtime_materialization(
        wheel_bytes: &[u8],
        expected_name: &str,
        expected_version: &str,
        expected_sha256: &str,
        site: &Path,
    ) -> MgResult<()> {
        if expected_sha256.len() != 64
            || !expected_sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return Err(MgError::Integrity(
                "Python runtime lock entry has no valid SHA-256 digest".to_string(),
            ));
        }
        let actual_sha256 = crate::protocols::sha256_hex(wheel_bytes);
        if !actual_sha256.eq_ignore_ascii_case(expected_sha256) {
            return Err(MgError::Integrity(
                "cached Python wheel does not match the mgc.lock SHA-256 digest".to_string(),
            ));
        }

        Self::verify_runtime_materialization_contents(
            wheel_bytes,
            expected_name,
            expected_version,
            site,
        )
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
