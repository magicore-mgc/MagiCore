//! Finding constructors preserve severity, category and optional file context.
//! Constructor Finding giữ severity, category và ngữ cảnh file tùy chọn.

use super::*;

#[test]
fn test_finding_builders() {
    let f = Finding::critical("pickle", "Dangerous import");
    assert_eq!(f.severity, Severity::Critical);
    assert_eq!(f.category, "pickle");

    let f2 = Finding::high("weights", "Large file").with_file("model.bin");
    assert_eq!(f2.file_path, Some("model.bin".to_string()));
}
