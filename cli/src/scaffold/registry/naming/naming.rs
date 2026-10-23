//! Scaffold package-name convention.
//! Quy ước tên package scaffold.

use crate::scaffold::spec::ScaffoldSpec;

/// Package name convention: mgc-create-{core}-{name}.
pub(crate) fn package_name(spec: &ScaffoldSpec) -> String {
    format!("mgc-create-{}-{}", spec.core.as_str(), spec.normalized_name)
}
