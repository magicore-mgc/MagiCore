//! High-level OCI model publication.
//! Publish model OCI ở mức cao.

use anyhow::Result;
use std::path::PathBuf;

use crate::client::OciClient;
use crate::manifest::{OciDescriptor, OciManifest};

impl OciClient {
    /// Push model with config + layers (convenience method for AI models)
    pub async fn push_model(
        &self,
        repo: &str,
        tag: &str,
        config: &crate::manifest::OciImageConfig,
        layers: &[(PathBuf, String)], // (file_path, media_type)
    ) -> Result<String> {
        // Push layers
        let mut layer_descriptors = Vec::new();
        for (path, media_type) in layers {
            let data = std::fs::read(path)?;
            let digest = self.push_blob(repo, &data).await?;
            let size = data.len() as i64;
            let mut desc = OciDescriptor::new(media_type.to_string(), size, digest);
            // Standard OCI image spec: title annotation allows oras/docker to
            // pull the artifact layer as a named file (without a title it is skipped).
            if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                desc = desc.with_annotations(
                    [(
                        "org.opencontainers.image.title".to_string(),
                        name.to_string(),
                    )]
                    .into(),
                );
            }
            layer_descriptors.push(desc);
        }

        // Create config
        let config_data = serde_json::to_vec(config)?;
        let config_digest = self.push_blob(repo, &config_data).await?;
        let config_desc = OciDescriptor::new(
            "application/vnd.oci.image.config.v1+json".to_string(),
            config_data.len() as i64,
            config_digest,
        );

        // Create manifest
        let manifest = OciManifest::new(config_desc, layer_descriptors);

        // Push manifest
        let tag = tag.to_string();
        // Push manifest with repo and tag
        self.push_manifest(repo, &tag, &manifest).await?;

        Ok(tag)
    }
}
