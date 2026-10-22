//! Track provider blocks and preserve checksum failure order.
//! Theo dõi provider block và giữ thứ tự lỗi thiếu checksum.

use super::super::super::error::TerraformLockError;
use super::super::super::types::TerraformLockState;

pub(crate) fn start_if_header(
    trimmed: &str,
    state: &mut TerraformLockState,
) -> Result<bool, TerraformLockError> {
    let Some(rest) = trimmed.strip_prefix("provider \"") else {
        return Ok(false);
    };
    let Some(close) = rest.find('"') else {
        return Err(TerraformLockError::MalformedProviderHeader {
            header: trimmed.to_string(),
        });
    };

    record_current_without_checksum(state);
    state.current_provider = Some(rest[..close].to_string());
    state.current_has_checksum = false;
    state.providers += 1;
    Ok(true)
}

pub(crate) fn close_if_block_end(trimmed: &str, state: &mut TerraformLockState) {
    if trimmed == "}" {
        record_current_without_checksum(state);
    }
}

pub(crate) fn finish_at_eof(state: &mut TerraformLockState) {
    record_current_without_checksum(state);
}

fn record_current_without_checksum(state: &mut TerraformLockState) {
    if let Some(name) = state.current_provider.take()
        && !state.current_has_checksum
    {
        state
            .failures
            .push(format!("provider '{name}' has no checksums"));
    }
}

#[cfg(test)]
#[path = "test/provider_state.rs"]
mod tests;
