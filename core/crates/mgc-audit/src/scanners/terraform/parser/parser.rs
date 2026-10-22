//! Iterate lock lines and coordinate existing scanner rules.
//! Duyệt các dòng lock và điều phối rule scanner hiện tại.

use super::super::error::TerraformLockError;
use super::super::report;
use super::super::types::TerraformLockState;
use super::checksum_presence;
use super::provider_state;
use mgc_types::adapter::AuditReport;

pub(crate) fn parse(raw: &str) -> Result<AuditReport, TerraformLockError> {
    let mut state = TerraformLockState::default();

    for line in raw.lines() {
        let trimmed = line.trim();
        if provider_state::start_if_header(trimmed, &mut state)? {
            continue;
        }
        checksum_presence::observe(trimmed, &mut state);
        provider_state::close_if_block_end(trimmed, &mut state);
    }
    provider_state::finish_at_eof(&mut state);

    Ok(report::project(state))
}

#[cfg(test)]
#[path = "test/parser.rs"]
mod tests;
