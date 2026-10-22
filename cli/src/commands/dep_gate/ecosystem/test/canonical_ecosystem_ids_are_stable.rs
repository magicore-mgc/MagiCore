//! Canonical ecosystem vocabulary — vocabulary ecosystem chuẩn.
//! Prevents display names from silently becoming policy identifiers.
//! Ngăn nhãn hiển thị âm thầm trở thành id policy.

use crate::commands::dep_gate::eco;

#[test]
fn canonical_ids_keep_their_stable_values() {
    let ids = [
        eco::JS,
        eco::TS,
        eco::PYTHON,
        eco::RUST,
        eco::GO,
        eco::JAVA,
        eco::DOTNET,
        eco::FLUTTER,
        eco::KOTLIN,
        eco::SWIFT,
        eco::OBJC,
        eco::RN,
        eco::BEVY,
        eco::TERRAFORM,
    ];

    assert_eq!(
        ids,
        [
            "js",
            "ts",
            "python",
            "rust",
            "go",
            "java",
            "dotnet",
            "flutter",
            "kotlin",
            "swift",
            "objc",
            "rn",
            "bevy",
            "terraform",
        ]
    );
}
