//! Detect the current scanner's checksum-array presence shape.
//! Nhận diện shape checksum-array hiện tại của scanner.

use super::super::super::types::TerraformLockState;

pub(crate) fn observe(trimmed: &str, state: &mut TerraformLockState) {
    if (trimmed.starts_with("hashes") || trimmed.starts_with("checksums")) && trimmed.contains('[')
    {
        state.current_has_checksum = true;
    }
}

#[cfg(test)]
#[path = "test/checksum_presence.rs"]
mod tests;
