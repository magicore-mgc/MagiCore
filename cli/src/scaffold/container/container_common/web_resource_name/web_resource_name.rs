//! Build a stable Web image/resource name within Kubernetes DNS limits.
//! Sinh tên image/resource Web ổn định trong giới hạn DNS Kubernetes.

use crate::scaffold::processor::{Scaffolder, slugify};
use crate::wizard::engine::ScaffoldConfig;

use super::super::super::{KUBERNETES_DNS_LABEL_MAX_BYTES, KUBERNETES_RESOURCE_HASH_HEX_LENGTH};

pub(in crate::scaffold::container) fn web_container_resource_name(
    config: &ScaffoldConfig,
) -> String {
    const PREFIX: &str = "magicore-";
    let display_name = Scaffolder::config_display_name(config);
    let slug = slugify(&display_name);
    if PREFIX.len() + slug.len() <= KUBERNETES_DNS_LABEL_MAX_BYTES {
        return format!("{PREFIX}{slug}");
    }

    let hash = blake3::hash(display_name.as_bytes()).to_hex().to_string();
    let readable_length =
        KUBERNETES_DNS_LABEL_MAX_BYTES - PREFIX.len() - 1 - KUBERNETES_RESOURCE_HASH_HEX_LENGTH;
    format!(
        "{PREFIX}{}-{}",
        &slug[..readable_length],
        &hash[..KUBERNETES_RESOURCE_HASH_HEX_LENGTH]
    )
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
