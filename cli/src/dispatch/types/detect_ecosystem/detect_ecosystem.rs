//! Ecosystem detection coordinator — preserve identity and manifest precedence.
//! Điều phối nhận diện ecosystem, giữ nguyên thứ tự identity và manifest.

#[path = "core_hints/mod.rs"]
mod core_hints;
#[path = "explicit_identity/mod.rs"]
mod explicit_identity;
#[path = "lock_validation/mod.rs"]
mod lock_validation;
#[path = "manifest_hints/mod.rs"]
mod manifest_hints;

pub fn detect_ecosystem() -> anyhow::Result<Option<String>> {
    let cwd = std::env::current_dir()?;
    detect_ecosystem_at(&cwd)
}

fn detect_ecosystem_at(cwd: &std::path::Path) -> anyhow::Result<Option<String>> {
    use mgc_config::project::ProjectConfig;

    let root = ProjectConfig::find_project_root(cwd).unwrap_or_else(|| cwd.to_path_buf());

    if let Some(core) = explicit_identity::read(&root)? {
        return Ok(Some(core));
    }

    lock_validation::validate(&root)?;

    let declarations = manifest_hints::read(&root)?;
    if let Some(core) = core_hints::resolve(declarations)? {
        return Ok(Some(core));
    }

    // This shared detector maps one unambiguous native manifest and rejects mixed ecosystems.
    // Detection never writes mgc.toml implicitly.
    ProjectConfig::detect_core(&root)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
