// Coordinate the Web create command — điều phối lệnh tạo project Web.
// Web create command orchestration.
use super::*;

use crate::commands::core::web::implementation::project_context::project_root::install_hint_command;

pub async fn run_create_with_options(
    framework: &str,
    project_name: &str,
    flags: Option<ScaffoldFlags>,
) -> Result<()> {
    let mut flags = flags.unwrap_or_default();

    if let Some(preset_name) = &flags.preset.clone() {
        apply_preset(preset_name, &mut flags);
    }

    // Phase 4: Parse scaffold spec sớm với typo detection
    use crate::scaffold::spec::{CoreKind, parse_scaffold_spec};
    let spec = parse_scaffold_spec(CoreKind::Web, framework)
        .map_err(|e| anyhow::anyhow!("Invalid framework specification '{}': {}", framework, e))?;

    apply_framework_alias_defaults(&spec.name, &mut flags);

    // Use normalized name cho resolve
    let fe_framework = resolve_framework(Some(&spec.name), &flags)?;
    enforce_framework_language_defaults(&fe_framework, &mut flags);
    validate_flags(&flags, &fe_framework)?;

    info(&format!(
        "Creating new web project '{}' with {}",
        project_name, fe_framework
    ));

    let mut config = build_web_config(&fe_framework, project_name, &flags)?;

    // A split fullstack combo is two halves, and its combined name has no leaf
    // layer of its own unless that layer was published. Compose it from the
    // halves instead of dead-ending at "no scaffold source": the frontend half
    // runs its framework initializer, the server half is written into
    // `server/`. Only an explicit `--from-local` bypasses composition, because
    // the user named the source.
    // (Combo fullstack tách rời là hai nửa, tên ghép không có leaf layer riêng
    // trừ khi layer đó được publish. Ghép từ hai nửa thay vì chết ở "no
    // scaffold source": nửa frontend chạy initializer của framework, nửa server
    // ghi vào `server/`. Chỉ `--from-local` tường minh bỏ qua ghép vì user đã
    // chỉ định nguồn.)
    if config.sub_type == "fullstack" && config.from_local.is_none() && is_combo(&spec.name) {
        save_composed_combo(&spec, project_name, config.frameworks.clone(), &flags).await?;
        return Ok(());
    }

    // Official generators own source creation; MGC owns package resolution and storage.
    // Generator chính thức tạo source; MGC quản lý dependency, lockfile và shared store.
    let frontend = framework_request_from_spec(&fe_framework, &spec.requested_ref);
    let scaffold_framework = match config.sub_type.as_str() {
        "frontend" | "fullstack" => config.frameworks.first(),
        "backend" => config.frameworks.last(),
        _ => None,
    };
    let initializer = scaffold_framework.and_then(|framework| {
        let mode = super::Mode::parse(&config.sub_type)?;
        super::lookup(mode, framework)
    });
    let mut generator_workspace = None;
    let provenance_layers;

    if let Some(record) = initializer {
        // The initializer is the only path that runs framework-published code.
        // Validate its record before a single byte is downloaded.
        // Initializer là đường duy nhất chạy code do framework phát hành. Kiểm
        // record trước khi tải một byte nào.
        if let Err(refusal) = super::check(&record) {
            anyhow::bail!("refusing initializer: {}", refusal.message());
        }
        info(&format!(
            "Creating from the official '{}' initializer through MagiCore",
            record.package
        ));
        let (workspace, source) = run_create_initializer(&mut config, project_name, record).await?;
        generator_workspace = Some(workspace);
        provenance_layers = vec![source];
    } else {
        // Registry layer is the default source; verify it before scaffolding.
        // Registry layer là nguồn mặc định; xác minh trước khi scaffold.
        let rels = required_web_layers_for_config(&config, &frontend);
        let mut report = crate::scaffold::resolver::MissingLayersReport::new();
        for rel in &rels {
            match crate::commands::template::ensure_layer(rel).await {
                Ok(status) if status.is_available() => {}
                Ok(status) => report.add_required_failure(rel, format!("{status:?}")),
                Err(error) => report.add_required_failure(rel, error),
            }
        }
        if let Some(source) = crate::scaffold::local_projects::resolve_offline_source(
            &config,
            scaffold_framework
                .map(String::as_str)
                .unwrap_or(&frontend.normalized),
        )? {
            // Registry could not supply the layers: the only permitted offline
            // source is a project already on this machine.
            // Registry không cung cấp được layer: nguồn offline duy nhất được
            // phép là project đã có trên máy.
            let workspace = tempfile::Builder::new()
                .prefix("mgc-local-web-source-")
                .tempdir()?;
            let reused = crate::scaffold::local_projects::reuse_local_project(
                &source,
                workspace.path(),
                &config.frameworks,
            )?;
            config.template_dir = workspace.path().to_path_buf();
            generator_workspace = Some(workspace);
            provenance_layers = vec![format!("local-project:{}", reused.display())];
        } else {
            if report.has_required_missing() {
                return Err(anyhow::anyhow!(
                    "{}\n{}",
                    report.format_error("web", &frontend.normalized),
                    crate::scaffold::error::scaffold_source_unavailable(
                        "web",
                        scaffold_framework
                            .map(String::as_str)
                            .unwrap_or(&frontend.normalized),
                    )
                ));
            }
            provenance_layers = rels;
        }
    }

    let project_dir = crate::scaffold::Scaffolder::scaffold(&config)?;
    // The generated source path is ephemeral and must not be written into mgc.toml.
    // Đường dẫn nguồn chỉ tồn tại tạm; không ghi nó vào mgc.toml của người dùng.
    config.template_dir = std::path::PathBuf::new();

    let proj_config = mgc_config::project::ProjectConfig::from_scaffold(
        crate::scaffold::Scaffolder::display_name(&project_dir),
        "web",
        &config.sub_type,
        config.frameworks.clone(),
        config.template_dir.to_string_lossy(),
        config.features.clone(),
    );
    proj_config.save(&project_dir)?;

    // Write scaffold provenance (R10 - supply chain tracking)
    let provenance = crate::scaffold::provenance::ScaffoldProvenance::new(
        spec.name.clone(),
        "web".to_string(),
        match &spec.requested_ref {
            crate::scaffold::spec::ScaffoldRef::DistTag(t) => t.clone(),
            crate::scaffold::spec::ScaffoldRef::Version(v) => v.clone(),
            _ => "default".to_string(),
        },
        None, // Registry URL would come from ensure_layer results
        provenance_layers,
    );
    if let Err(e) = provenance.write(&project_dir) {
        mgc_ui::warning(&format!("Failed to write provenance: {}", e));
    }

    let be_name = detect_backend_framework(&flags);
    let backend = be_name
        .as_deref()
        .map(parse_framework_request)
        .or_else(|| fullstack_backend_framework(&frontend.normalized).map(parse_framework_request));
    enrich_web_project_manifest(&project_dir, &frontend, backend.as_ref(), &flags).await?;

    info(&format!("Project '{}' created!", project_dir.display()));
    info(&format!(
        "  cd {} && {}",
        project_name,
        install_hint_command()
    ));

    drop(generator_workspace);

    Ok(())
}

/// Compose one split combo offline, then write the metadata MagiCore owns.
/// Ghép một combo tách rời offline, rồi ghi metadata do MagiCore sở hữu.
///
/// The combo keeps its combined name in `mgc.toml` (`react-express`) with the
/// fullstack mode, so later `mgc install`/`mgc dev` see one project with a
/// `server/` half — the shape `detect_project_mode` already recognises.
/// Combo giữ tên ghép trong `mgc.toml` (`react-express`) với mode fullstack, để
/// `mgc install`/`mgc dev` sau đó thấy một project có nửa `server/` — đúng dạng
/// `detect_project_mode` đã nhận.
async fn save_composed_combo(
    spec: &crate::scaffold::spec::ScaffoldSpec,
    project_name: &str,
    frameworks: Vec<String>,
    flags: &ScaffoldFlags,
) -> Result<()> {
    let combo = compose_offline(&spec.name, project_name, flags).await?;
    let project_dir = &combo.project_dir;

    let proj_config = mgc_config::project::ProjectConfig::from_scaffold(
        crate::scaffold::Scaffolder::display_name(project_dir),
        "web",
        "fullstack",
        frameworks,
        String::new(),
        web_features(flags),
    );
    proj_config.save(project_dir)?;

    // Write scaffold provenance (R10 - supply chain tracking)
    let provenance = crate::scaffold::provenance::ScaffoldProvenance::new(
        spec.name.clone(),
        "web".to_string(),
        match &spec.requested_ref {
            crate::scaffold::spec::ScaffoldRef::DistTag(tag) => tag.clone(),
            crate::scaffold::spec::ScaffoldRef::Version(version) => version.clone(),
            _ => "default".to_string(),
        },
        None,
        combo.provenance,
    );
    if let Err(error) = provenance.write(project_dir) {
        mgc_ui::warning(&format!("Failed to write provenance: {}", error));
    }

    // Same manifest seeding as the single-framework lane: the frontend seed
    // rewrites the root manifest, the backend seed fills the server's.
    // (Seed manifest giống lane một framework: seed frontend ghi lại manifest
    // gốc, seed backend điền manifest của server.)
    let frontend = parse_framework_request(&combo.frontend_framework);
    let backend = fullstack_backend_framework(&spec.name).map(parse_framework_request);
    enrich_web_project_manifest(project_dir, &frontend, backend.as_ref(), flags).await?;

    info(&format!("Project '{}' created!", project_dir.display()));
    info(&format!(
        "  cd {} && {}",
        project_name,
        install_hint_command()
    ));
    Ok(())
}
