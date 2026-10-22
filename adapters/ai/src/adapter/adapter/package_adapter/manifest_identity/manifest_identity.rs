//! Reports the native Python manifest identity when that lane is selected.
//! Báo cáo identity manifest Python native khi lane đó được chọn.

use crate::adapter::AiAdapter;

pub(in crate::adapter::implementation) fn manifest_identity(
    adapter: &AiAdapter,
) -> Option<mgc_types::ManifestIdentity> {
    adapter
        .python_lane
        .as_ref()
        .map(|_| mgc_types::ManifestIdentity {
            core: "ai".to_string(),
            language: "python".to_string(),
            format: "pyproject.toml".to_string(),
            relpath: "pyproject.toml".to_string(),
        })
}
