//! Patch apply engine — parse unified diff, apply to vstore (16 §5)
//! (Apply engine nội bộ — KHÔNG gọi binary `patch`, cross-platform, chạy offline)

use super::{
    error::PatchCoordinatorError,
    hunk_application::apply_hunks,
    hunk_parser::{parse_file_header, parse_hunk},
    path_validation::safe_vstore_target,
};
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Apply a unified diff patch to a vstore directory.
/// Returns the list of modified files. Trả về danh sách file đã sửa.
pub fn apply_patch(vstore_root: &Path, patch_path: &Path) -> Result<Vec<PathBuf>> {
    let content = fs::read_to_string(patch_path)
        .with_context(|| format!("read patch file {}", patch_path.display()))?;
    let mut staged = Vec::new();
    let mut lines = content.lines().map(|s| s.to_string()).peekable();

    while lines.peek().is_some() {
        // Parse file header: --- a/file and +++ b/file
        let old_file = parse_file_header(&mut lines, "---")?;
        let new_file = parse_file_header(&mut lines, "+++")?;

        if old_file != new_file {
            return Err(PatchCoordinatorError::FileMismatch {
                old: old_file,
                new: new_file,
            }
            .into());
        }

        let target = safe_vstore_target(vstore_root, &old_file)?;
        if !target.exists() {
            return Err(PatchCoordinatorError::TargetNotFound(target.display().to_string()).into());
        }

        let mut hunks = Vec::new();
        while let Some(line) = lines.peek() {
            if line.starts_with("---") {
                break;
            }
            if !line.starts_with("@@") {
                return Err(PatchCoordinatorError::ExpectedHunkHeader(line.clone()).into());
            }
            let Some(header) = lines.next() else {
                return Err(PatchCoordinatorError::HunkStreamEnded.into());
            };
            hunks.push(parse_hunk(header, &mut lines)?);
        }
        let original = fs::read_to_string(&target)?;
        let updated = apply_hunks(&original, hunks)?;
        staged.push((target, updated));
    }

    // Parse every file and validate every hunk before the first write. This
    // prevents a malformed later file from leaving earlier files modified.
    // (Đọc/kiểm tra mọi file trước lần ghi đầu tiên.)
    for (target, updated) in &staged {
        fs::write(target, updated)?;
    }
    Ok(staged.into_iter().map(|(target, _)| target).collect())
}
