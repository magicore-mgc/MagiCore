//! Finalizes a v4 digest and proves canonical serialization round-trips.
//! Hoàn thiện digest v4 và xác nhận serialization canonical round-trip.

use crate::LockfileResult;

use super::error::V4FinalizationError;

/// Finalize a v4 document for disk without taking ownership of publication.
/// Hoàn thiện document v4 để ghi đĩa mà không chiếm quyền publish.
pub fn finalize_v4_document(doc: &mut crate::canonical::LockfileV4) -> LockfileResult<String> {
    doc.metadata.lockfile_hash = crate::canonical::payload_digest(&doc.payload());
    let text = crate::canonical::write_v4_document(doc)?;
    let round_trip = crate::canonical::parse_v4_document(&text)?;
    if round_trip != *doc {
        return Err(V4FinalizationError::SerializerDrift.into());
    }
    Ok(text)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
