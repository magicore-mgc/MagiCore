// Write Web scaffold template layers — ghi các template layer Web.
// Web scaffold template output.
use super::*;

const NUXT_STORAGE_IGNORES: &[&str] = &[
    "**/.magicore/cache",
    "**/.magicore/cache/**",
    "**/.magicore/store",
    "**/.magicore/store/**",
    "**/.magicore/exec.log",
];

impl Scaffolder {
    pub(in crate::scaffold::processor) fn write_web_files(
        target: &Path,
        config: &ScaffoldConfig,
        name: &str,
        framework: &str,
    ) -> Result<()> {
        if !config.template_dir.as_os_str().is_empty() {
            Self::copy_online_generated_template(&config.template_dir, target)?;
            Self::ensure_framework_storage_ignores(target, framework)?;
            if Self::backend_without_js_manifest(target, config, framework) {
                return Ok(());
            }
            return Self::ensure_web_test_runner(target, name);
        }

        // No prepared template_dir: either the registry layers or a reused
        // local project must supply the framework source.
        // Không có template_dir chuẩn bị sẵn: registry layer hoặc project local
        // tái dùng phải cung cấp source framework.
        let layers = match Self::resolve_web_template_layers(config) {
            Ok(layers) => layers,
            Err(layer_error) => {
                if let Some(source) =
                    crate::scaffold::local_projects::resolve_offline_source(config, framework)?
                {
                    crate::scaffold::local_projects::reuse_local_project(
                        &source,
                        target,
                        &config.frameworks,
                    )?;
                    Self::ensure_framework_storage_ignores(target, framework)?;
                    return Ok(());
                }
                return Err(layer_error.context(
                    crate::scaffold::error::scaffold_source_unavailable("web", framework),
                ));
            }
        };
        Self::materialize_web_templates(target, config, &layers)?;
        Self::ensure_framework_storage_ignores(target, framework)?;
        if effective_web_mode(config) == "monorepo" {
            return Ok(());
        }
        if Self::backend_without_js_manifest(target, config, framework) {
            Ok(())
        } else {
            Self::ensure_web_test_runner(target, name)
        }
    }

    fn backend_without_js_manifest(
        target: &Path,
        config: &ScaffoldConfig,
        framework: &str,
    ) -> bool {
        effective_web_mode(config) == "backend"
            && infer_backend_language(framework).is_some_and(|language| language != "node")
            && !target.join("package.json").is_file()
    }

    fn ensure_framework_storage_ignores(target: &Path, framework: &str) -> Result<()> {
        if framework != "nuxt" {
            return Ok(());
        }
        let path = target.join(".nuxtignore");
        let mut contents = mgc_config::project::read_regular_project_text(
            &path,
            "Nuxt watcher ignore configuration",
        )?
        .unwrap_or_default();
        for pattern in NUXT_STORAGE_IGNORES {
            if contents.lines().any(|line| line.trim() == *pattern) {
                continue;
            }
            if !contents.is_empty() && !contents.ends_with('\n') {
                contents.push('\n');
            }
            contents.push_str(pattern);
            contents.push('\n');
        }
        Self::write_file(&path, &contents)
    }
}
