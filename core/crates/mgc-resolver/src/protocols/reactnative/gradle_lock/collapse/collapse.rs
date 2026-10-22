//! Collapse same-version configurations and reject conflicting exact pins.
//! Gộp configuration cùng version và từ chối pin chính xác bị xung đột.

use super::super::{GradleLockEntry, error::GradleLockError};

pub(in crate::protocols::reactnative::gradle_lock) fn collapse_entries(
    entries: &[GradleLockEntry],
) -> Result<Vec<(String, String)>, GradleLockError> {
    let mut pins: Vec<(String, String)> = Vec::new();
    for entry in entries {
        let coordinate = entry.coordinate();
        match pins.iter_mut().find(|(name, _)| name == &coordinate) {
            Some((_, version)) if version != &entry.version => {
                return Err(GradleLockError::ConflictingVersions {
                    coordinate,
                    first_version: version.clone(),
                    second_version: entry.version.clone(),
                });
            }
            Some(_) => {}
            None => pins.push((coordinate, entry.version.clone())),
        }
    }
    Ok(pins)
}

#[cfg(test)]
#[path = "test/collapse.rs"]
mod tests;
