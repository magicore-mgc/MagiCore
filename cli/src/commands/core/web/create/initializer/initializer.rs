// Run the selected online Web initializer — chạy initializer Web online đã chọn.
// Online Web project initialization.
use super::*;

pub(in crate::commands::core::web::implementation) async fn run_create_initializer(
    config: &mut crate::wizard::engine::ScaffoldConfig,
    project_name: &str,
    generator: super::super::InitializerRecord,
) -> Result<(tempfile::TempDir, String)> {
    crate::commands::core::create::validate_project_name(project_name)?;
    let target = Path::new(project_name);
    if target.try_exists()? {
        return Err(crate::error::dir_already_exists(target));
    }

    let workspace = tempfile::Builder::new()
        .prefix("mgc-online-scaffold-")
        .tempdir()?;
    let generated_project = workspace.path().join(project_name);

    // An HTTP-archive initializer delegates to the framework's own generator
    // service; nothing local runs and no package manager is involved.
    // Initializer dạng archive HTTP giao cho dịch vụ sinh project của chính
    // framework; không chạy gì local và không dính package manager nào.
    if let super::super::Runtime::HttpArchive {
        endpoint,
        params,
        name_param,
        unwrap_single_root,
    } = generator.runtime
    {
        super::run_http_archive(
            endpoint,
            params,
            name_param,
            project_name,
            &generated_project,
        )
        .await?;
        if unwrap_single_root {
            hoist_single_root(&generated_project)?;
        }
        let root = workspace.path().canonicalize()?;
        let resolved = generated_project.canonicalize()?;
        if !resolved.starts_with(&root) {
            return Err(crate::error::web_generator_output_unsafe(
                &generated_project,
            ));
        }
        config.template_dir = resolved;
        return Ok((
            workspace,
            format!("http:{}#generator={}", endpoint, generator.identity),
        ));
    }

    let package_request = format!("{}@{}", generator.package, generator.version.selector());
    let generator_args = generator
        .args
        .iter()
        .map(|argument| match argument {
            super::super::ArgSlot::Project => project_name.to_string(),
            super::super::ArgSlot::Literal(value) => (*value).to_string(),
        })
        .collect();
    let report = crate::commands::package_runtime::run_registry_package_in(
        package_request.clone(),
        Some(generator.binary.to_string()),
        false,
        generator_args,
        workspace.path(),
        crate::commands::package_runtime::PackageRunPurpose::CreateInitializer,
    )
    .await
    .with_context(|| {
        format!(
            "create the '{}' project with '{}' through the MagiCore create initializer",
            project_name, package_request
        )
    })?;

    let workspace_root = workspace.path().canonicalize()?;
    let generated_metadata = std::fs::symlink_metadata(&generated_project)
        .map_err(|_| crate::error::web_generator_output_invalid(&generated_project))?;
    if generated_metadata.file_type().is_symlink() || !generated_metadata.is_dir() {
        return Err(crate::error::web_generator_output_unsafe(
            &generated_project,
        ));
    }
    let generated_project = generated_project.canonicalize()?;
    if !generated_project.starts_with(&workspace_root) {
        return Err(crate::error::web_generator_output_unsafe(
            &generated_project,
        ));
    }
    let package_json = generated_project.join("package.json");
    let package_metadata = std::fs::symlink_metadata(&package_json)
        .map_err(|_| crate::error::web_generator_output_invalid(&generated_project))?;
    if package_metadata.file_type().is_symlink() || !package_metadata.is_file() {
        return Err(crate::error::web_generator_output_unsafe(&package_json));
    }

    config.template_dir = generated_project;
    Ok((
        workspace,
        format!(
            "npm:{}@{}#generator={}",
            report.package, report.version, generator.identity
        ),
    ))
}

/// Move the contents of a single wrapper directory up one level.
///
/// `code.quarkus.io` returns `<name>/pom.xml`, so the scaffold would otherwise
/// nest the real project one directory deep.
/// Chuyển nội dung của thư mục bọc duy nhất lên một cấp. `code.quarkus.io`
/// trả `<name>/pom.xml`, nếu không sẽ lồng project thật xuống một cấp.
fn hoist_single_root(root: &Path) -> Result<()> {
    let entries: Vec<_> = std::fs::read_dir(root)?
        .filter_map(|entry| entry.ok())
        .collect();
    let [only] = entries.as_slice() else {
        return Ok(());
    };
    if !only.path().is_dir() {
        return Ok(());
    }
    let inner = only.path();
    let staging = root.with_extension("hoist");
    std::fs::create_dir_all(&staging)?;
    for child in std::fs::read_dir(&inner)? {
        let child = child?;
        std::fs::rename(child.path(), staging.join(child.file_name()))?;
    }
    std::fs::remove_dir_all(&inner)?;
    for child in std::fs::read_dir(&staging)? {
        let child = child?;
        std::fs::rename(child.path(), root.join(child.file_name()))?;
    }
    std::fs::remove_dir_all(&staging)?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
