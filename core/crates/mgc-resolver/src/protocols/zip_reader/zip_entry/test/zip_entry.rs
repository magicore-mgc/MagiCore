//! ZIP entry DTO tests — test DTO entry ZIP.

use super::ZipEntry;

#[test]
fn zip_entry_retains_name_and_payload() {
    let entry = ZipEntry {
        name: "pkg/file.txt".to_string(),
        data: b"payload".to_vec(),
            unix_mode: None,
    };
    assert_eq!(entry.name, "pkg/file.txt");
    assert_eq!(entry.data, b"payload");
}
