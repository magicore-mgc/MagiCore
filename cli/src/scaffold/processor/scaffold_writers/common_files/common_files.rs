// Write shared files and safe file bytes — ghi file chung và nội dung file an toàn.
// Common generated files and write primitives.
use super::*;

impl Scaffolder {
    pub(in crate::scaffold::processor) fn write_common_files(
        target: &Path,
        config: &ScaffoldConfig,
    ) -> Result<()> {
        // Web projects arrive with the framework's own `.gitignore`/README from
        // `create-vite`/`create-next-app`, so MagiCore must not REPLACE it — the
        // framework's list is more complete for its own build. But replacing is
        // not the only risk: skipping the file entirely left the MagiCore entries
        // missing, and a `create-web` project then tracked `.mgc-optimizer/`,
        // `.magicore/store` and `.magicore/exec.log`.
        //
        // So: merge instead of replace-or-skip. A framework `.gitignore` keeps
        // every line it already had, and MagiCore appends only the entries it is
        // missing.
        // (Project Web có `.gitignore`/README riêng từ `create-vite`… nên MagiCore
        // không được GHI ĐÈ — nhưng bỏ qua hoàn toàn thì mất entry của MagiCore và
        // project track `.mgc-optimizer/`, `.magicore/store`… Vậy: HỢP NHẤT.)
        let only_if_missing = config.core == "web";

        // Derive the display name from the config, NOT from the target path —
        // during atomic staging the target is a random temp dir name.
        // Lấy tên hiển thị từ config, KHÔNG từ target path — trong staging
        // nguyên tử, target là temp dir với tên random.
        let name = Self::config_display_name(config);
        let framework = Self::framework(config);

        // Non-web cores own the whole `.gitignore`, so it is written here. Web
        // projects get the merge in `merge_gitignore_after_framework`, which runs
        // after the framework's own file exists.
        // (Core không phải web sở hữu trọn `.gitignore`, ghi ở đây. Project web
        // được hợp nhất ở `merge_gitignore_after_framework`, chạy sau khi file của
        // framework tồn tại.)
        if !only_if_missing {
            Self::write_file(
                &target.join(".gitignore"),
                &common_gitignore(&config.core, &framework),
            )?;
        }
        // Web: write MagiCore's README now; a framework that ships its own
        // overwrites it later, which is the desired precedence. Starting Spring
        // Boot ships none, so skipping the write left that project with no
        // README at all (`web_fw_spring_boot` caught it).
        //
        // This is the one place the "framework wins" rule needs a fallback rather
        // than a merge: a README has no additive content, so "framework or ours"
        // is the whole decision.
        // (Web: ghi README MagiCore ngay; framework có README riêng sẽ đè sau —
        // đúng thứ tự ưu tiên. Spring Boot không có README nên bỏ ghi khiến project
        // trống README. Đây là chỗ duy nhất quy tắc "framework thắng" cần fallback
        // thay vì merge: README không có nội dung cộng dồn.)
        Self::write_file(
            &target.join("README.md"),
            &common_readme(&name, &config.core, &framework, &config.features),
        )?;

        Ok(())
    }
    /// C9 — multi-platform: shared Kotlin (KMP) + android/ios (swift+objc)/react-native/flutter.
    /// Merge MagiCore's ignore entries into a Web project's `.gitignore`.
    ///
    /// Called after the framework's files are written. A framework that ships no
    /// `.gitignore` gets MagiCore's full list; one that does keeps its own lines
    /// and receives only the entries it lacks.
    /// (Gọi sau khi file framework đã ghi. Framework không có `.gitignore` nhận
    /// trọn danh sách; framework có thì giữ dòng của nó và chỉ nhận entry còn thiếu.)
    pub(in crate::scaffold::processor) fn merge_gitignore_after_framework(
        target: &Path,
        config: &ScaffoldConfig,
    ) -> Result<()> {
        if config.core != "web" {
            return Ok(());
        }
        let framework = Self::framework(config);
        let generated = common_gitignore(&config.core, &framework);
        let path = target.join(".gitignore");
        let body = Self::merge_gitignore(&path, &generated)?;
        Self::write_file(&path, &body)
    }

    /// Merge MagiCore's ignore entries into an existing `.gitignore`.
    ///
    /// The framework's file is the base: every line it has is preserved verbatim
    /// (including comments, negations and ordering). Only entries the file lacks
    /// are appended, under a marker so a reader can see where they came from.
    ///
    /// An unreadable existing file is an error rather than a silent replace: the
    /// framework's list may contain `!` negations whose loss would change what the
    /// repository tracks.
    /// (Hợp nhất entry MagiCore vào `.gitignore` có sẵn: giữ nguyên mọi dòng của
    /// framework, chỉ thêm entry còn thiếu dưới một marker. File không đọc được là
    /// lỗi, không ghi đè âm thầm — mất `!` negation sẽ đổi thứ repo track.)
    pub(in crate::scaffold::processor) fn merge_gitignore(
        existing_path: &Path,
        generated: &str,
    ) -> Result<String> {
        let existing = match std::fs::read_to_string(existing_path) {
            Ok(body) => body,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(error) => {
                return Err(anyhow::anyhow!(
                    "cannot read existing .gitignore at {}: {error}",
                    existing_path.display()
                ));
            }
        };
        if existing.is_empty() {
            return Ok(generated.to_string());
        }

        let present: std::collections::BTreeSet<&str> = existing.lines().map(str::trim).collect();
        let missing: Vec<&str> = generated
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !present.contains(line))
            .collect();
        if missing.is_empty() {
            return Ok(existing);
        }

        let mut body = existing;
        if !body.ends_with('\n') {
            body.push('\n');
        }
        body.push_str("\n# MagiCore runtime state (added by mgc)\n");
        for entry in missing {
            body.push_str(entry);
            body.push('\n');
        }
        Ok(body)
    }

    /// Test-only entry point for the merge primitive.
    /// Điểm vào chỉ cho test của hàm hợp nhất.
    #[cfg(test)]
    pub(in crate::scaffold::processor) fn merge_gitignore_for_test(
        existing_path: &Path,
        generated: &str,
    ) -> Result<String> {
        Self::merge_gitignore(existing_path, generated)
    }

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
