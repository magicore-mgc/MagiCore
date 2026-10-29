//! Packagist (Composer) resolver module wiring and stable exports.
//! Nối module resolver Packagist (Composer) và giữ đường export ổn định.

use super::ResolvedEntry;
use mgc_types::MgResult;

mod error;
mod metadata_client;
mod package_catalog;
mod registry;
mod version_constraints;

#[path = "composer.rs"]
mod composer_implementation;

pub use composer_implementation::ComposerProtocol;

impl ComposerProtocol {
    /// Resolve the exact version of a development dependency.
    /// Composer `require-dev` edges are recorded as `dev:<vendor>/<package>`
    /// in [`ResolvedEntry::deps`]; the workspace decides whether to install
    /// them, so this helper resolves one of those edges explicitly without
    /// joining the production graph.
    /// Resolve version chính xác của một dev dependency. Cạnh `require-dev`
    /// của Composer được ghi dạng `dev:<vendor>/<package>` trong
    /// [`ResolvedEntry::deps`]; workspace quyết định có cài hay không nên
    /// helper này resolve tường minh, không nhập vào graph production.
    pub async fn resolve_development_dependency(
        &self,
        name: &str,
        version: &str,
    ) -> MgResult<ResolvedEntry> {
        self.resolve_exact(name, version).await
    }
}

#[cfg(test)]
mod test;
