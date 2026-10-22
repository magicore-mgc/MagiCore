//! Validate that a monitored child has a non-empty native argv snapshot.
//! Xác thực snapshot argv native của child được monitor không rỗng.

use super::super::ProcessPolicyError;
use std::ffi::OsString;

pub(in crate::run::implementation) fn validate_monitored_child_command_line(
    image_name: &str,
    command: &[OsString],
    pid: u32,
) -> std::result::Result<(), ProcessPolicyError> {
    if command.is_empty() {
        return Err(ProcessPolicyError::MissingChildCommandLine {
            image_name: image_name.to_owned(),
            process_id: pid,
        });
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
