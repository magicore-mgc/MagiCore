// Go scanner local error contract tests.
// Kiểm thử hợp đồng lỗi cục bộ của scanner Go.

use super::GovulncheckError;
use mgc_types::MgError;

#[test]
fn typed_errors_keep_legacy_messages_and_mapping() {
    let cases = [
        (
            GovulncheckError::InvalidJson("line 1 column 2".to_string()),
            "invalid govulncheck JSON: line 1 column 2",
        ),
        (
            GovulncheckError::FindingWithoutOsvId,
            "govulncheck finding without an OSV id — stream contract violation",
        ),
        (
            GovulncheckError::UnknownOsvId("GO-2026-0001".to_string()),
            "govulncheck finding references OSV 'GO-2026-0001' that never appeared in the stream",
        ),
        (
            GovulncheckError::InvalidPackageName("invalid path".to_string()),
            "govulncheck module path 'invalid path' cannot be represented as a package name",
        ),
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
        let MgError::Other(message) = MgError::from(error) else {
            panic!("Go scanner parse errors must retain the Other variant");
        };
        assert_eq!(message, expected);
    }
}
