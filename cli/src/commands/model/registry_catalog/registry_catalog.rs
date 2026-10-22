// registry_catalog owner — registry_catalog owner implementation.
// Responsibility boundary — ranh giới trách nhiệm của owner.
use super::super::registry_client::client;
use super::RegistryCatalogError;
use anyhow::{Context, Result};
pub(in crate::commands::model) async fn list(registry: &str, token: Option<String>) -> Result<()> {
    let c = client(registry, token)?;
    let repos = c
        .list_repositories()
        .await
        .with_context(|| RegistryCatalogError::ListingFailed)?;
    for repo in repos {
        let tags = c.list_tags(&repo).await.unwrap_or_default();
        println!("{repo}: {}", tags.join(", "));
    }
    Ok(())
}
