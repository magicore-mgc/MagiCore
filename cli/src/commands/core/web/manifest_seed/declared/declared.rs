//! Detect user/generator-declared package constraints.
//! Phát hiện constraint package do generator/user khai báo.

use serde_json::{Map, Value};

/// Preserve declared generator/user constraints before resolving defaults.
/// Giữ constraint của generator/user thay vì tự nâng toolchain ngoài ý muốn.
pub(crate) fn has_declared_package(root: &Map<String, Value>, package: &str) -> bool {
    [
        "dependencies",
        "devDependencies",
        "optionalDependencies",
        "peerDependencies",
    ]
    .iter()
    .any(|section| {
        root.get(*section)
            .and_then(Value::as_object)
            .and_then(|packages| packages.get(package))
            .and_then(Value::as_str)
            .is_some_and(|version| !version.is_empty())
    })
}
