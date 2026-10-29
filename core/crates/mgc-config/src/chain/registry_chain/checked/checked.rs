//! Checked and credential-scope-preserving registry resolution.
//! Resolve registry có kiểm lỗi và giữ nguyên credential scope.

use crate::registry::Registry;

use super::super::builder::registry_chain_with_npmrc;
use super::super::scoped_registry::ScopedRegistry;

/// Build a validated chain and resolve missing credentials by URL scope.
/// Không nuốt lỗi npmrc hoặc scope mâu thuẫn; token env/config tường minh được ưu tiên.
pub fn registry_chain_checked(
    project_root: Option<&std::path::Path>,
    project_config: Option<&crate::project::ProjectConfig>,
) -> anyhow::Result<Vec<Registry>> {
    Ok(registry_chain_scoped_checked(project_root, project_config)?
        .into_iter()
        .map(|entry| entry.registry)
        .collect())
}

/// Resolve explicit origin credentials and npmrc path credentials without flattening scope.
/// Token env/config giữ contract origin; npmrc giữ scope cụ thể đã chọn.
pub fn registry_chain_scoped_checked(
    project_root: Option<&std::path::Path>,
    project_config: Option<&crate::project::ProjectConfig>,
) -> anyhow::Result<Vec<ScopedRegistry>> {
    let npmrc = project_root
        .map(crate::npmrc::NpmRc::load_for_registry)
        .transpose()?;
    let chain = registry_chain_with_npmrc(project_config, npmrc.as_ref(), false);
    let validation = crate::npmrc::NpmRc::default();
    let mut scoped = Vec::with_capacity(chain.len());
    for mut registry in chain {
        validation.credential_for_url_checked(&registry.url)?;
        let credential_scope = if registry.token.is_some() {
            let mut origin = url::Url::parse(&registry.url)?;
            origin.set_path("/");
            origin.set_query(None);
            origin.set_fragment(None);
            Some(origin)
        } else if let Some(credential) = npmrc
            .as_ref()
            .map(|rc| rc.credential_for_url_checked(&registry.url))
            .transpose()?
            .flatten()
        {
            registry.token = Some(credential.token.clone());
            Some(credential.scope)
        } else {
            None
        };
        scoped.push(ScopedRegistry {
            registry,
            credential_scope,
        });
    }
    Ok(scoped)
}
