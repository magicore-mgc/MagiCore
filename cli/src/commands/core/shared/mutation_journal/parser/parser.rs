//! Parse and validate the versioned mutation journal.
//! Parse và xác thực journal mutation có version.

use std::path::{Path, PathBuf};

use super::super::{MUTATION_JOURNAL_SCHEMA, MutationJournal};
use super::error::MutationJournalParseError;

pub(in crate::commands::core::shared::implementation::implementation::mutation_journal::implementation) fn parse_mutation_journal(
    raw: &str,
    journal_path: &Path,
) -> Result<MutationJournal, MutationJournalParseError> {
    // Check the version before field parsing so operators see schema mismatch first.
    // Kiểm tra version trước khi parse field để báo rõ schema không tương thích.
    let value: serde_json::Value =
        serde_json::from_str(raw).map_err(|source| MutationJournalParseError::Malformed {
            path: PathBuf::from(journal_path),
            source,
        })?;
    let version = value.get("v").and_then(|value| value.as_u64());
    if version != Some(u64::from(MUTATION_JOURNAL_SCHEMA)) {
        return Err(MutationJournalParseError::UnsupportedSchema {
            version: version
                .map(|version| version.to_string())
                .unwrap_or_else(|| "missing".to_string()),
            expected: MUTATION_JOURNAL_SCHEMA,
            path: PathBuf::from(journal_path),
        });
    }
    serde_json::from_value(value).map_err(|source| MutationJournalParseError::InvalidFields {
        path: PathBuf::from(journal_path),
        source,
    })
}
