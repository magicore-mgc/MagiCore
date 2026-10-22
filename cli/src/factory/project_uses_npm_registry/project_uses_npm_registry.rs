use mgc_types::Ecosystem;

/// Check whether this project actually consumes npm registry credentials.
/// Không đọc npmrc cho core/ngôn ngữ ngoài npm chỉ vì CLI dùng context chung.
pub fn project_uses_npm_registry(root: &std::path::Path, ecosystem: &Ecosystem) -> bool {
    let _ = root;
    match ecosystem {
        #[cfg(feature = "web")]
        Ecosystem::Web => {
            // A JS manifest owns npm even in a polyglot project.
            // Manifest native rõ ràng chỉ tránh npm khi project không có package.json.
            root.join("package.json").exists()
                || mgc_lib_adapter::detect_language(root)
                    .is_none_or(|language| language == mgc_lib_adapter::LibLanguage::Ts)
        }
        #[cfg(feature = "lib")]
        Ecosystem::Lib => {
            mgc_lib_adapter::detect_language(root) == Some(mgc_lib_adapter::LibLanguage::Ts)
        }
        _ => false,
    }
}
