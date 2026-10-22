//! Checks legacy and v4 dispatch plus missing/unknown schema handling.
//! Kiểm tra dispatch legacy/v4 và xử lý schema thiếu/không hỗ trợ.

use super::super::{LockDocument, parse_document, parse_lockfile};
use crate::LockfileError;

const LEGACY_V3: &str = "version = \"3\"\npackage = []\n\n[metadata]\ngenerated_at = \"now\"\ngenerator = \"mgc\"\nlockfile_hash = \"\"\n";

#[test]
fn legacy_parser_accepts_v3_and_dispatcher_preserves_legacy_variant() {
    assert_eq!(
        parse_lockfile(LEGACY_V3)
            .expect("test operation must succeed")
            .version,
        crate::LOCKFILE_SCHEMA_VERSION
    );
    assert!(matches!(
        parse_document(LEGACY_V3).expect("test operation must succeed"),
        LockDocument::Legacy(_)
    ));
}

#[test]
fn document_parser_rejects_missing_and_unknown_versions() {
    assert!(matches!(
        parse_document("package = []\n"),
        Err(LockfileError::ParseError(message)) if message.contains("missing lockfile version")
    ));
    assert!(matches!(
        parse_document("version = \"99\"\n"),
        Err(LockfileError::ParseError(message)) if message.contains("unsupported lockfile version: 99")
    ));
}
