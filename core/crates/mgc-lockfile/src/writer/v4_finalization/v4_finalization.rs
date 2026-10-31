//! Finalizes a v4 digest and proves canonical serialization round-trips.
//! Hoàn thiện digest v4 và xác nhận serialization canonical round-trip.

use crate::LockfileResult;

use super::error::V4FinalizationError;

/// Finalize a v4 document for disk without taking ownership of publication.
///
/// The digest covers `canonical_toml`, where every collection is emitted in
/// a deterministic total order (packages by name then version, edges by
/// full identity). The written file is a pure function of that payload, so
/// byte-identical files and identical `lockfile_hash` values follow from
/// the same graph regardless of resolution order.
///
/// The round-trip proof is order-insensitive on purpose: this TOML reader
/// normalizes package-row order on parse, so comparing raw vectors would
/// reject the serializer for a difference the file format cannot preserve.
/// Semantic drift in any field (key, edge, artifact, provenance, sources,
/// roots, metadata) still fails closed.
/// (Hoàn thiện document v4 để ghi đĩa. Digest phủ `canonical_toml` — mọi
/// collection theo thứ tự tất định, nên byte và digest không phụ thuộc thứ
/// tự resolve. Phép round-trip cố ý KHÔNG nhạy thứ tự: parser TOML chuẩn
/// hóa thứ tự row package, nên so sánh Vec thô sẽ từ chối serializer vì
/// khác biệt mà định dạng file không giữ được. Lệch ngữ nghĩa ở bất kỳ
/// field nào vẫn fail-closed.)
pub fn finalize_v4_document(doc: &mut crate::canonical::LockfileV4) -> LockfileResult<String> {
    let payload = doc.payload();
    let lockfile_hash = crate::canonical::payload_digest(&payload);
    doc.metadata.lockfile_hash = lockfile_hash;
    let text = crate::canonical::write_v4_document(doc)?;
    let round_trip = crate::canonical::parse_v4_document(&text)?;
    if !crate::writer::same_v4_content(&round_trip, doc) {
        return Err(V4FinalizationError::SerializerDrift.into());
    }
    Ok(text)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
