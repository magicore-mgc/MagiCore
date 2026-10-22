// registry_client owner — registry_client owner implementation.
// Responsibility boundary — ranh giới trách nhiệm của owner.
use anyhow::Result;
use mgc_oci::client::OciClient;
pub(in crate::commands::model) fn client(
    registry: &str,
    token: Option<String>,
) -> Result<OciClient> {
    let mut c = OciClient::new(registry.to_string(), None)?;
    if let Some(t) = token.filter(|t| !t.is_empty()) {
        c = c.with_token(t);
    }
    Ok(c)
}
