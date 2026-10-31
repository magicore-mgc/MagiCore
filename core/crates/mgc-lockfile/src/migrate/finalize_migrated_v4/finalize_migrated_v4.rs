//! Finalizes and round-trip checks a lossless v4 lock document.
//! Hoàn thiện và kiểm tra round-trip lock v4 không hao hụt.

use super::super::lossy_migration::LossyMigration;
use super::super::v3_to_v4::migrate_v3_to_v4;
use crate::Lockfile;

/// Migrate a merged v3 lock to a finalized v4 document text, refusing
/// lossy migrations fail-closed (shared by every adapter write path so the
/// gate cannot drift between lanes).
/// (Migrate lock v3 đã merge thành text v4 hoàn thiện, từ chối hao hụt
/// fail-closed (dùng chung mọi đường ghi adapter để gate không lệch giữa
/// các lane).)
pub fn finalize_migrated_v4(
    legacy: Lockfile,
    generator: &str,
    generated_at: &str,
) -> Result<String, LossyMigration> {
    let (mut v4, warnings) = migrate_v3_to_v4(legacy).map_err(|error| LossyMigration {
        warnings: vec![error.to_string()],
    })?;
    if !warnings.is_empty() {
        return Err(LossyMigration { warnings });
    }
    v4.metadata.generator = generator.to_string();
    v4.metadata.generated_at = generated_at.to_string();
    v4.metadata.lockfile_hash = crate::canonical::payload_digest(&v4.payload());
    let text = crate::canonical::write_v4_document(&v4).map_err(|_| LossyMigration {
        warnings: vec!["v4 serialization failed".to_string()],
    })?;
    let round_trip = crate::canonical::parse_v4_document(&text).map_err(|_| LossyMigration {
        warnings: vec!["v4 round-trip parse failed".to_string()],
    })?;
    // The reader re-sorts set-like collections, so compare content, not the
    // `Vec` order the writer chose. Any lost value still fails closed.
    // (Reader sắp lại collection dạng tập nên so nội dung, không so thứ
    // tự Vec mà writer chọn. Giá trị bị mất vẫn fail-closed.)
    if !crate::writer::same_v4_content(&round_trip, &v4) {
        return Err(LossyMigration {
            warnings: vec!["v4 serializer drift".to_string()],
        });
    }
    Ok(text)
}
