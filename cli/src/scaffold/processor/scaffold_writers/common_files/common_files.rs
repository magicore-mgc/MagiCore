// Write shared files and safe file bytes — ghi file chung và nội dung file an toàn.
// Common generated files and write primitives.
use super::*;

impl Scaffolder {
    pub(in crate::scaffold::processor) fn write_common_files(
        target: &Path,
        config: &ScaffoldConfig,
    ) -> Result<()> {
        // Web projects come from a generator or a reused project, so the
        // framework owns its own .gitignore/README — MagiCore only fills gaps.
        // Project Web đến từ generator hoặc project tái dùng, nên framework tự
        // có .gitignore/README — MagiCore chỉ điền chỗ còn thiếu.
        let only_if_missing = config.core == "web";

        // Derive the display name from the config, NOT from the target path —
        // during atomic staging the target is a random temp dir name.
        // Lấy tên hiển thị từ config, KHÔNG từ target path — trong staging
        // nguyên tử, target là temp dir với tên random.
        let name = Self::config_display_name(config);
        let framework = Self::framework(config);

        if !(only_if_missing && target.join(".gitignore").exists()) {
            Self::write_file(
                &target.join(".gitignore"),
                &common_gitignore(&config.core, &framework),
            )?;
        }
        if only_if_missing && target.join("README.md").exists() {
            return Ok(());
        }
        Self::write_file(
            &target.join("README.md"),
            &common_readme(&name, &config.core, &framework, &config.features),
        )?;

        Ok(())
    }
    /// C9 — multi-platform: shared Kotlin (KMP) + android/ios (swift+objc)/react-native/flutter.
    pub(in crate::scaffold) fn write_file(path: &Path, content: &str) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }
    pub(in crate::scaffold::processor) fn write_bytes(path: &Path, content: &[u8]) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }
}
