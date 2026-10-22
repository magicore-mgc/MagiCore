use anyhow::Result;
use std::path::Path;
use std::path::PathBuf;

/// Create the project build-output path one component at a time, refusing symlinks.
/// Tạo thư mục build từng phần và từ chối symlink trong đường dẫn.
#[cfg(feature = "web")]
pub(in crate::commands::build) fn ensure_go_build_output_dir(root: &Path) -> Result<PathBuf> {
    let canonical_root = root.canonicalize()?;
    let mut current = canonical_root;
    for component in [".magicore", "build", "go"] {
        current.push(component);
        loop {
            match std::fs::symlink_metadata(&current) {
                Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {
                    break;
                }
                Ok(_) => {
                    return Err(crate::error::build_output_path_unsafe(
                        &current,
                        "expected a real directory, found a file or symlink",
                    ));
                }
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    match std::fs::create_dir(&current) {
                        Ok(()) => continue,
                        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                            continue;
                        }
                        Err(error) => return Err(error.into()),
                    }
                }
                Err(error) => return Err(error.into()),
            }
        }
    }
    Ok(current)
}

/// Do not let the compiler follow a planted symlink at the executable path.
/// Không để compiler đi theo symlink có sẵn tại đường dẫn executable.
#[cfg(feature = "web")]
pub(in crate::commands::build::implementation::web_go_backend) fn ensure_go_build_output_file(
    path: &Path,
) -> Result<()> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_file() && !metadata.file_type().is_symlink() => Ok(()),
        Ok(_) => Err(crate::error::build_output_path_unsafe(
            path,
            "expected a regular file or a missing output",
        )),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}
