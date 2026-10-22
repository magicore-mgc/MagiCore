//! OCI namespace selection and trusted publisher binding request.
//! Chọn namespace OCI và gửi request trusted publisher binding.

use super::super::registry_url::{RegistryUrlError, admin_client, binding_url};
use super::error::TrustedBindingError;

/// Apply a core namespace only to OCI bindings; preserve existing npm/PyPI names.
/// Chỉ thêm namespace core cho binding OCI; giữ nguyên tên npm/PyPI hiện hữu.
pub(in crate::commands::registry) fn effective_trusted_binding_package(
    protocol: &str,
    core: Option<&str>,
    package: &str,
) -> anyhow::Result<String> {
    if protocol == "oci"
        && let Some(core) = core
    {
        return mgc_registry_server::trusted::core_scoped_oci_repository(core, package)
            .map_err(|_| anyhow::anyhow!("invalid core-scoped OCI repository"));
    }
    Ok(package.to_owned())
}

pub(in crate::commands::registry) async fn trust_publisher(
    package: &str,
    repository: &str,
    protocol: &str,
    registry: &str,
    admin_token: Option<String>,
) -> Result<(), TrustedBindingError> {
    let admin_token = admin_token
        .filter(|token| !token.is_empty() && token.bytes().all(|byte| byte.is_ascii_graphic()))
        .ok_or(TrustedBindingError::AdminTokenRequired)?;
    let url = binding_url(registry).map_err(map_registry_url_error)?;
    let client = admin_client(&url)?;
    let response = client
        .put(url)
        .bearer_auth(admin_token)
        .json(&serde_json::json!({
            "protocol": protocol,
            "package": package,
            "repository": repository,
        }))
        .send()
        .await
        .map_err(|error| TrustedBindingError::RequestFailed(error.without_url()))?;
    if !response.status().is_success() {
        return Err(TrustedBindingError::BindingFailed(
            response.status().as_u16(),
        ));
    }
    println!("Trusted {protocol} publisher bound: {package} ← {repository}");
    Ok(())
}

fn map_registry_url_error(error: RegistryUrlError) -> TrustedBindingError {
    match error {
        RegistryUrlError::RequiresHttps => TrustedBindingError::RequiresHttps,
        error @ (RegistryUrlError::ClientBuild(_) | RegistryUrlError::InvalidAdminTimeout) => {
            TrustedBindingError::ClientPolicy(error)
        }
        RegistryUrlError::InvalidRegistry | RegistryUrlError::InvalidPath => {
            TrustedBindingError::InvalidRegistry
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
