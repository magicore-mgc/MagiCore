// Atomic scaffold lifecycle and target claims — vòng đời scaffold nguyên tử và claim target.
// Atomic staging, target claims, and final publish.
use super::*;

impl Scaffolder {
    pub fn scaffold(config: &ScaffoldConfig) -> Result<PathBuf> {
        let target = Self::target_dir(config);
        let claim = Self::claim_target(&target)?;

        // Atomic scaffold: build in a hidden temp dir on the SAME filesystem,
        // rename into place only when fully written. A crash mid-write can
        // never leave a partial project visible at the target path.
        // Scaffold nguyên tử: ghi trong temp dir ẩn CÙNG filesystem, chỉ
        // rename sang target khi hoàn tất — crash giữa chừng không để lại
        // project partial ở vị trí cuối.
        let parent = target.parent().map(Path::to_path_buf).unwrap_or_default();
        let staging = tempfile::tempdir_in(&parent).map_err(|e| {
            let _ = std::fs::remove_file(&claim);
            crate::scaffold::error::scaffold_staging_failed(&target, anyhow::anyhow!(e))
        })?;
        let staging_path = staging.path().to_path_buf();

        let write_result = (|| -> Result<()> {
            std::fs::create_dir_all(&staging_path)?;
            Self::write_common_files(&staging_path, config)?;
            Self::write_core_files(&staging_path, config)?;
            // The framework's `.gitignore` only exists after `write_core_files`,
            // so the merge has to run after it. Running it inside
            // `write_common_files` (which is called first) saw no file and wrote
            // MagiCore's list from scratch, then the framework overwrote it — the
            // project ended with zero MagiCore entries.
            // (`.gitignore` của framework chỉ tồn tại SAU `write_core_files`, nên
            // hợp nhất phải chạy sau. Chạy trong `write_common_files` (gọi trước)
            // không thấy file, ghi đè xong framework lại ghi đè tiếp — project
            // không có entry MagiCore nào.)
            Self::merge_gitignore_after_framework(&staging_path, config)?;
            crate::scaffold::container::generate_assets(&staging_path, config)?;
            Ok(())
        })();

        if let Err(err) = write_result {
            // Fail atomically — staging dir is removed by TempDir drop and
            // the claim slot is released; the target was never touched.
            // Fail nguyên tử — staging dọn bởi TempDir drop, claim được giải
            // phóng; target chưa từng bị chạm nên không có phần dư.
            let _ = std::fs::remove_file(&claim);
            return Err(crate::scaffold::error::scaffold_staging_failed(
                &target, err,
            ));
        }

        // Rename is atomic within one filesystem. The claim slot guarantees
        // no other process renamed a directory over our target meanwhile.
        // Rename nguyên tử trong cùng filesystem. Claim-slot bảo đảm không
        // process nào khác rename directory đè lên target trong lúc đó.
        if let Err(e) = std::fs::rename(&staging_path, &target) {
            let _ = std::fs::remove_file(&claim);
            return Err(crate::scaffold::error::scaffold_staging_failed(
                &target,
                anyhow::anyhow!(e),
            ));
        }
        // Keep the TempDir handle from cleaning the now-renamed path, then
        // release the claim so future creates for this name are possible.
        // Giữ TempDir không dọn path vừa rename, rồi giải phóng claim để
        // lần create sau cho tên này vẫn khả dụng.
        std::mem::forget(staging);
        let _ = std::fs::remove_file(&claim);

        mgc_ui::success(&format!(
            "Created {} project: {}",
            config.core,
            target.display()
        ));
        Ok(target)
    }
    /// Hidden claim-slot path next to the target (same parent, dot-prefixed).
    /// Đường dẫn claim-slot ẩn cạnh target (cùng parent, tiền tố dot).
    pub(in crate::scaffold::processor) fn claim_slot_path(target: &Path) -> PathBuf {
        let name = target
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "project".to_string());
        let claim_name = format!(".mgc-create-{name}.lock");
        match target.parent() {
            Some(parent) => parent.join(claim_name),
            None => PathBuf::from(claim_name),
        }
    }
    /// Atomically claim a target name, then check it while holding the claim.
    /// Claim tên target nguyên tử, rồi kiểm tra khi vẫn đang giữ claim.
    pub(in crate::scaffold::processor) fn claim_target(target: &Path) -> Result<PathBuf> {
        // The exclusive claim must precede the target check: checking first lets
        // a loser acquire the claim after the winner publishes and releases it.
        // Phải lấy claim độc quyền trước khi kiểm tra target: nếu kiểm tra trước,
        // process thua có thể lấy claim sau khi process thắng đã tạo target và nhả khóa.
        let claim = Self::claim_slot_path(target);
        let claim_file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&claim)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    crate::scaffold::error::create_claim_conflict(&claim)
                } else {
                    crate::scaffold::error::scaffold_staging_failed(target, anyhow::anyhow!(e))
                }
            })?;
        // Release the handle early; the claim is guarded by its file's existence.
        // Nhả handle sớm; claim được giữ bằng sự tồn tại của file khóa.
        drop(claim_file);

        let target_exists = match target.try_exists() {
            Ok(exists) => exists,
            Err(error) => {
                let _ = std::fs::remove_file(&claim);
                return Err(crate::scaffold::error::scaffold_staging_failed(
                    target,
                    anyhow::anyhow!(error),
                ));
            }
        };
        if target_exists {
            let conflict = crate::error::dir_already_exists(target);
            if let Err(cleanup_error) = std::fs::remove_file(&claim) {
                return Err(crate::scaffold::error::scaffold_staging_failed(
                    target,
                    anyhow::anyhow!(
                        "{conflict}; failed to release claim slot '{}': {cleanup_error}",
                        claim.display()
                    ),
                ));
            }
            return Err(conflict);
        }

        Ok(claim)
    }
    pub fn display_name(project_dir: &Path) -> String {
        project_dir
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "project".to_string())
    }
    /// Display name derived from the configured project name rather than
    /// the (possibly staging temp) target path.
    /// Tên hiển thị lấy từ project name trong config, không phải từ target
    /// path (có thể là temp dir của staging nguyên tử).
    pub(in crate::scaffold) fn config_display_name(config: &ScaffoldConfig) -> String {
        Path::new(&config.project_name)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "project".to_string())
    }
    pub(in crate::scaffold::processor) fn target_dir(config: &ScaffoldConfig) -> PathBuf {
        PathBuf::from(&config.project_name)
    }
}
