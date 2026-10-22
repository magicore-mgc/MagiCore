use super::error;
use crate::commands::template::{
    TemplatePublishArgs, layer_rel, put_manifest, registry_package_name, select_registry,
    template_layer_has_contract, template_registry_url, upload_tarball, validate_template_identity,
};
use anyhow::Result;
use mgc_config::npmrc::NpmRc;
use mgc_publish::auth::resolve_auth;
use std::path::Path;
use tempfile;

/// Publish kernel template layer `templates/{core}/{name}` thành package
/// `mgc-create-<core>-<name>` trên registry (mgc-pack tarball + npm PUT).
pub(in crate::commands::template) async fn publish(args: TemplatePublishArgs) -> Result<()> {
    validate_template_identity(&args.core, &args.name)?;
    let rel = layer_rel(&args.core, &args.name);
    let layer = crate::scaffold::template_root::TemplateRoot::resolve(&rel)
        .path()
        .to_path_buf();
    if !template_layer_has_contract(&layer)? {
        return Err(error::template_layer_missing_manifest(&rel));
    }

    let version = args.version.unwrap_or_else(|| "0.1.0".to_string());
    semver::Version::parse(&version).map_err(|_| error::template_metadata_invalid())?;
    let name = registry_package_name(&args.core, &args.name);
    let registry = template_registry_url(&select_registry(args.registry.as_deref())?)?.to_string();

    let temp_dir = tempfile::tempdir()?;
    let tarball_path = temp_dir
        .path()
        .join(format!("{}.tgz", name.replace('/', "-")));
    let pack_result = mgc_pack::tarball::pack(&layer, &tarball_path, &format!("{name}-{version}"))?;

    let npmrc = NpmRc::load(Path::new(".")).unwrap_or_default();
    let auth = resolve_auth(&npmrc, &registry, None, None).unwrap_or_default();
    if auth.is_empty() {
        let host = url::Url::parse(&registry)
            .map(|u| u.host_str().unwrap_or_default().to_string())
            .unwrap_or_default();
        if host.contains("npmjs") || host.contains("npm") {
            return Err(error::template_publish_auth_required());
        }
    }

    upload_tarball(&registry, &auth, &name, &version, &tarball_path).await?;

    let manifest = serde_json::json!({
        "name": name,
        "version": version,
        "description": format!("MagiCore kernel template {}/{}", args.core, args.name),
        "main": "template.toml",
    });
    put_manifest(&registry, &auth, &name, &version, &manifest, &pack_result).await?;
    println!(
        "Template '{}' published → {}/{}/{}",
        name,
        registry.trim_end_matches('/'),
        name,
        version
    );
    Ok(())
}
