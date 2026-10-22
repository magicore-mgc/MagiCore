//! Split-language inventory — danh sách split-language.
//! Locks the identifiers used to project capability rows.
//! Khóa các id dùng để chiếu hàng capability.

use crate::commands::dep_gate::SPLIT_LANGUAGES;

#[test]
fn inventory_keeps_all_supported_matrix_identifiers_without_duplicates() {
    let expected = [
        "ts",
        "js",
        "rust",
        "python",
        "go",
        "java",
        "dotnet",
        "flutter",
        "swift",
        "kotlin",
        "objc",
        "rn",
        "javascript",
        "bevy",
        "esp32-rust",
        "platformio",
        "zephyr",
        "terraform",
    ];

    assert_eq!(SPLIT_LANGUAGES, expected.as_slice());
    let mut unique = SPLIT_LANGUAGES.to_vec();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), SPLIT_LANGUAGES.len());
}
