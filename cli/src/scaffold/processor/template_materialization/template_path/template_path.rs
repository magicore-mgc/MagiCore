// Validate template paths before file access — kiểm tra path template trước khi đọc/ghi.
// Template source and target path safety.
use super::*;

/// Validate portable manifest paths and reject existing link/reparse descendants.
/// Chặn đường dẫn thoát root trên mọi OS trước khi đọc source hoặc ghi project.
pub(in crate::scaffold::processor) fn checked_template_path(
    root: &Path,
    relative: &str,
) -> Result<PathBuf> {
    if relative.is_empty()
        || relative.contains(['\\', ':'])
        || relative.chars().any(char::is_control)
        || relative
            .split('/')
            .any(|part| part.is_empty() || part.ends_with(['.', ' ']))
    {
        return Err(crate::scaffold::error::template_materialization_path_invalid(relative));
    }
    let mut current = root.to_path_buf();
    for part in std::iter::once(None).chain(relative.split('/').map(Some)) {
        if let Some(part) = part {
            current.push(part);
        }
        match std::fs::symlink_metadata(&current) {
            Ok(metadata) => {
                let linked = metadata.file_type().is_symlink();
                #[cfg(windows)]
                let linked = {
                    use std::os::windows::fs::MetadataExt;
                    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
                    linked || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
                };
                if linked {
                    return Err(
                        crate::scaffold::error::template_materialization_path_invalid(relative),
                    );
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
    }
    Ok(current)
}
