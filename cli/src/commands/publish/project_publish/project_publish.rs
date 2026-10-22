use super::super::{
    dist_tag::set_dist_tag,
    git_checks::git_checks,
    lifecycle::{publish_lifecycle_decision, run_lifecycle},
    npm_publish::publish_put,
    npm_tarball_upload::upload_tarball_client,
    project_publish::{commit_project_version, print_publish_summary},
    publish::{NPM_PROTOCOL, PublishArgs},
    registry_selection::{select_registry, verify_token},
    trusted_session::trusted_publish_session,
};
use anyhow::{Result, anyhow};
use mgc_config::{npmrc::NpmRc, project::ProjectConfig};
use mgc_pack::ignore::select_files;
use mgc_pack::manifest::{dep_fields, sanitize};
use mgc_pack::tarball::pack;
use mgc_publish::auth::resolve_auth;
use semver::Version;
use std::fs;
use std::path::Path;

pub(in crate::commands) async fn publish_project(
    args: &PublishArgs,
    project_root: &Path,
) -> Result<()> {
    // 1. Git checks
    if !args.no_git_checks {
        git_checks(args.publish_branch.as_deref(), project_root)?;
    }

    // 2. Load project config
    let mut project = ProjectConfig::load(project_root)?
        .ok_or_else(|| anyhow!("mgc.toml does not exist — run mgc init"))?;

    // 3. Version bump (Q4)
    let bump = match (args.patch, args.minor, args.major) {
        (true, false, false) => "patch",
        (false, true, false) => "minor",
        (false, false, true) => "major",
        _ => "",
    };
    let new_version = if !bump.is_empty() {
        let mut v = Version::parse(&project.version)?;
        match bump {
            "patch" => v.patch += 1,
            "minor" => {
                v.minor += 1;
                v.patch = 0;
            }
            "major" => {
                v.major += 1;
                v.minor = 0;
                v.patch = 0;
            }
            _ => {}
        }
        let new_v = v.to_string();
        project.version = new_v.clone();
        new_v
    } else {
        project.version.clone()
    };

    // 4. Read package.json (web/lib ts)
    let pkg_json_path = project_root.join("package.json");
    let mut pkg_json: serde_json::Value = if pkg_json_path.exists() {
        serde_json::from_str(&fs::read_to_string(&pkg_json_path)?)?
    } else {
        serde_json::json!({})
    };

    // Update package.json name/version
    if pkg_json.get("name").is_none() {
        pkg_json["name"] = serde_json::Value::String(project.name.clone());
    }
    pkg_json["version"] = serde_json::Value::String(new_version.clone());

    // 5. Lifecycle scripts
    if publish_lifecycle_decision(&pkg_json, args.ignore_scripts, args.allow_scripts)? {
        run_lifecycle(&pkg_json, "prepublishOnly", project_root)?;
        run_lifecycle(&pkg_json, "prepublish", project_root)?;
        run_lifecycle(&pkg_json, "prepare", project_root)?;
    }

    // 6. Manifest sanitize (exportable)
    let sanitized = sanitize(pkg_json.clone(), &project.name, &new_version)?;
    let dep_fields_map = dep_fields(&sanitized.manifest);
    // Tên publish = package.json name (scoped) — mgc.toml name có thể thiếu scope
    let publish_name = pkg_json
        .get("name")
        .and_then(serde_json::Value::as_str)
        .filter(|n| !n.is_empty())
        .unwrap_or(&project.name)
        .to_string();

    // 7. Files selection
    let files = select_files(project_root)?;

    // 8. mgc-pack: build tarball (filename dùng unscoped — scoped name chứa '/' làm tempdir join fail)
    let entry_prefix = format!("{}-{}", project.name, new_version);
    let filename = format!(
        "{}-{}.tgz",
        project.name.replace(['/', '@'], "-"),
        new_version
    );
    let temp_dir = tempfile::tempdir()?;
    let tarball_path = temp_dir.path().join(filename);
    let pack_result = pack(project_root, &tarball_path, &entry_prefix)?;

    // 9. Registry select (01 §2)
    let registry_url = select_registry(&pkg_json, project_root, &project, args)?;

    // 10. Auth resolution
    let npmrc = NpmRc::load(project_root)?;
    let config_reg = project.registries.iter().find(|r| r.url == registry_url);
    let mut trusted_session = None;
    let auth = if args.trusted && !args.dry_run {
        let session = trusted_publish_session(
            &registry_url,
            &publish_name,
            NPM_PROTOCOL,
            args.trusted_audience.as_deref(),
        )
        .await?;
        let auth = mgc_publish::auth::Auth {
            token: Some(session.token.clone()),
            ..Default::default()
        };
        trusted_session = Some(session);
        auth
    } else if args.trusted {
        mgc_publish::auth::Auth::default()
    } else {
        resolve_auth(&npmrc, &registry_url, config_reg, args.token.as_deref())?
    };

    // 11. Pre-flight whoami (npmjs only)
    if registry_url.contains("registry.npmjs.org") && !args.dry_run && !args.trusted {
        verify_token(&registry_url, &auth).await?;
    }

    // 12. Publish PUT
    if !args.dry_run {
        upload_tarball_client(
            &registry_url,
            &auth,
            &publish_name,
            &new_version,
            &tarball_path,
            trusted_session.as_mut(),
        )
        .await?;
        publish_put(
            &registry_url,
            &auth,
            &publish_name,
            &new_version,
            &sanitized.manifest,
            &dep_fields_map,
            &pack_result,
            args,
            trusted_session.as_mut(),
        )
        .await?;
    }

    // 13. Dist-tags
    if !args.dry_run && !args.trusted && args.tag.as_deref() != Some("latest") {
        set_dist_tag(
            &registry_url,
            &auth,
            &publish_name,
            &new_version,
            args.tag.as_deref().unwrap_or("latest"),
        )
        .await?;
    }

    // 14. Output summary
    print_publish_summary(args, &project, new_version, &pack_result, files.len())?;

    // Save updated versions after the publish summary, under the shared writer lock.
    commit_project_version(project_root, &project, &pkg_json_path, &pkg_json)?;

    Ok(())
}
