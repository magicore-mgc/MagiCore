// Managed environment for native Web backend installs — môi trường cài backend native qua MGC.
// Native Web backend install environment.
use super::*;

pub(in crate::commands::core::web::implementation) fn native_install_env(
    project_root: &Path,
    program: &str,
) -> Result<Vec<(String, String)>> {
    let mut env = Vec::new();
    // Hermetic HOME: clean-env spawns wipe $HOME, but some tools
    // hard-require it (composer refuses to run without HOME or
    // COMPOSER_HOME). Point it at project-scoped cache so cache writes
    // stay inside .magicore instead of the user's real home.
    // (HOME cách ly: tool đòi $HOME cũng chỉ thấy cache của project.)
    let cache_home = project_root.join(".magicore").join("cache").join("home");
    std::fs::create_dir_all(&cache_home)?;
    env.push(("HOME".to_string(), cache_home.display().to_string()));
    if program == "composer" {
        let composer_home = project_root
            .join(".magicore")
            .join("cache")
            .join("composer");
        let composer_cache = composer_home.join("cache");
        std::fs::create_dir_all(&composer_cache)?;
        env.push((
            "COMPOSER_HOME".to_string(),
            composer_home.display().to_string(),
        ));
        env.push((
            "COMPOSER_CACHE_DIR".to_string(),
            composer_cache.display().to_string(),
        ));
    }
    if program == "go" {
        let paths = mgc_platform::paths::GlobalPaths::new()?;
        let mod_cache = paths.store.join("go");
        let build_cache = paths.cache.join("go").join("build");
        std::fs::create_dir_all(&mod_cache)?;
        std::fs::create_dir_all(&build_cache)?;
        env.extend(go_native_runtime_env(&mod_cache, &build_cache));
    }
    if program == "cargo" {
        env.push(("CARGO_NET_OFFLINE".to_string(), "true".to_string()));
    }
    Ok(env)
}
/// Keep Go dev execution on the same verified module cache used by native install.
/// Buộc `mgc dev` dùng module cache đã cài/xác minh bởi MGC và chặn tải ngoài.
pub(in crate::commands::core::web::implementation) fn go_native_runtime_env(
    mod_cache: &Path,
    build_cache: &Path,
) -> Vec<(String, String)> {
    vec![
        ("GOMODCACHE".to_string(), mod_cache.display().to_string()),
        ("GOCACHE".to_string(), build_cache.display().to_string()),
        ("GOPROXY".to_string(), "off".to_string()),
        ("GOSUMDB".to_string(), "off".to_string()),
        ("GOTOOLCHAIN".to_string(), "local".to_string()),
    ]
}
