//! Detect a Cloud project from trusted markers — nhận diện project Cloud từ marker hợp lệ.

use std::path::Path;

use super::super::CloudType;

pub fn detect_type(root: &Path) -> Option<CloudType> {
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Ok(Some(content)) =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
        && let Ok(v) = toml::from_str::<toml::Value>(&content)
        && let Some(t) = v
            .get("cloud")
            .and_then(|c| c.get("type"))
            .and_then(|t| t.as_str())
    {
        return match t {
            "cdk" | "cdk-typescript" => Some(CloudType::Cdk),
            "pulumi" => Some(CloudType::Pulumi),
            "terraform" | "terraform-gcp" => Some(CloudType::Terraform),
            "cloudflare" => Some(CloudType::Cloudflare),
            _ => None,
        };
    }
    if is_regular_manifest(&root.join("wrangler.toml")) {
        return Some(CloudType::Cloudflare);
    }
    if is_regular_manifest(&root.join("Pulumi.yaml")) {
        return Some(CloudType::Pulumi);
    }
    if has_tf_files(root) {
        return Some(CloudType::Terraform);
    }
    if let Ok(Some(content)) = mgc_config::project::read_regular_project_text(
        &root.join("package.json"),
        "package manifest",
    ) && let Ok(v) = serde_json::from_str::<serde_json::Value>(&content)
    {
        let has_cdk = v
            .get("dependencies")
            .and_then(|d| d.as_object())
            .map(|deps| deps.keys().any(|k| k.starts_with("aws-cdk") || k == "cdk"))
            .unwrap_or(false);
        if has_cdk {
            return Some(CloudType::Cdk);
        }
    }
    None
}

fn has_tf_files(root: &Path) -> bool {
    let Ok(entries) = std::fs::read_dir(root) else {
        return false;
    };
    entries.flatten().any(|entry| {
        let path = entry.path();
        path.extension().is_some_and(|ext| ext == "tf")
            && std::fs::symlink_metadata(path).is_ok_and(|metadata| metadata.file_type().is_file())
    })
}

/// Ignore symlinked and special-file framework markers during core selection.
/// Bỏ qua marker framework là symlink hoặc file đặc biệt khi chọn core.
fn is_regular_manifest(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|metadata| metadata.file_type().is_file())
}
