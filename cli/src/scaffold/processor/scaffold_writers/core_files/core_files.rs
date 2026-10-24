// Write non-Web core scaffold files — ghi file scaffold cho core không phải Web.
// Core-specific scaffold file generation.
use super::*;

impl Scaffolder {
    /// Add the toolchain MagiCore owns on top of reused framework source.
    /// Bổ sung toolchain MagiCore sở hữu lên trên source framework tái dùng.
    fn apply_owned_toolchain(
        config: &ScaffoldConfig,
        framework: &str,
        target: &Path,
    ) -> Result<()> {
        if matches!(
            (config.core.as_str(), framework),
            ("ai", _) | ("lib", "python")
        ) {
            crate::scaffold::processor::ensure_python_dev_group(target)?;
        }
        Ok(())
    }

    pub(in crate::scaffold::processor) fn write_core_files(
        target: &Path,
        config: &ScaffoldConfig,
    ) -> Result<()> {
        // Same as write_common_files: name must come from the config because
        // the target path during staging has a random temp name.
        // Giống write_common_files: tên phải lấy từ config vì target trong
        // staging nguyên tử mang tên temp random.
        let name = Self::config_display_name(config);
        let framework = Self::framework(config);

        if config.core == "web" {
            // Web output comes from a resolved template layer or a reused local
            // project; both arrive through template_dir. MagiCore writes none of
            // the framework source itself.
            // Output Web đến từ template layer đã resolve hoặc project local
            // tái dùng; cả hai đi qua template_dir. MagiCore không tự ghi
            // source framework.
            return Self::write_web_files(target, config, &name, &framework);
        }

        let context = CoreTemplateContext::try_new(config, &name, &framework)?;

        // Hardware packages (`optimizer`, `bench`) are MagiCore-published
        // artifacts, not third-party framework source: their marker files are
        // written here because MagiCore owns them end to end.
        // Package Hardware (`optimizer`, `bench`) là artifact do MagiCore phát
        // hành, không phải source framework bên thứ ba: marker của chúng được
        // ghi ở đây vì MagiCore sở hữu toàn bộ.
        if config.core == "hardware" {
            return crate::scaffold::processor::write_hardware_marker(target, &name, &framework);
        }

        // An explicit `--from-local` overrides the registry: the user named the
        // source, so MagiCore must not silently fetch a different one.
        // `--from-local` tường minh thắng registry: user đã chỉ định nguồn nên
        // MagiCore không được lặng lẽ tải nguồn khác.
        if config.from_local.is_some() {
            let source =
                crate::scaffold::local_projects::resolve_offline_source(config, &framework)?
                    .ok_or_else(|| {
                        crate::scaffold::error::scaffold_source_unavailable(
                            &config.core,
                            &framework,
                        )
                    })?;
            crate::scaffold::local_projects::reuse_local_project(
                &source,
                target,
                &config.frameworks,
            )?;
            Self::apply_owned_toolchain(config, &framework, target)?;
            return Ok(());
        }

        // Registry layer next — the default source when the network is up.
        // Registry layer tiếp theo — nguồn mặc định khi có mạng.
        let layer = Self::core_template_layer(&config.core, &framework);
        if Self::layer_has_contract(&layer) {
            return Self::materialize_core_template(target, &layer, &context);
        }

        // No registry layer: the only permitted offline source is a project
        // already on this machine. Never synthesize framework source here.
        // Không có registry layer: nguồn offline duy nhất được phép là project
        // đã có trên máy. Không bao giờ tự sinh source framework.
        if let Some(source) =
            crate::scaffold::local_projects::resolve_offline_source(config, &framework)?
        {
            crate::scaffold::local_projects::reuse_local_project(
                &source,
                target,
                &config.frameworks,
            )?;
            Self::apply_owned_toolchain(config, &framework, target)?;
            // MagiCore still owns the metadata written after this call
            // (mgc.toml, lockfile scaffold); the copied project keeps only
            // framework source.
            // MagiCore vẫn ghi metadata sau lời gọi này (mgc.toml, lockfile
            // scaffold); project copy chỉ giữ source framework.
            return Ok(());
        }

        Err(crate::scaffold::error::scaffold_source_unavailable(
            &config.core,
            &framework,
        ))
    }
}
