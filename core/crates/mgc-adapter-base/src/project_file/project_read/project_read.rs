//! Read a project-relative regular file through an opened directory handle.
//! Đọc file thường qua directory handle đã mở và xác thực.

use super::super::{
    error::{error, error_with_source},
    project_path::project_parent_dir,
};
use mgc_types::MgError;
use std::path::Path;

#[cfg(unix)]
pub(in crate::project_file) fn read_project_regular_text_dirfd(
    project_root: &Path,
    relative_path: &Path,
    display_name: &str,
) -> Result<String, MgError> {
    use rustix::fs::{Mode, OFlags, openat};
    use std::io::Read;
    use std::os::fd::AsFd;

    let (parent, leaf) = project_parent_dir(project_root, relative_path)?;
    let opened = openat(
        parent.as_fd(),
        &leaf,
        OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::NONBLOCK | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .map_err(|source| {
        error_with_source(source, |e| {
            format!("open {display_name} without following links: {e}")
        })
    })?;
    let mut file = std::fs::File::from(opened);
    if !file
        .metadata()
        .map_err(|source| {
            error_with_source(source, |e| format!("inspect opened {display_name}: {e}"))
        })?
        .is_file()
    {
        return Err(error(format!("{display_name} is not a regular file")));
    }
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|source| error_with_source(source, |e| format!("read {display_name}: {e}")))?;
    Ok(content)
}

#[cfg(test)]
#[path = "test/project_read.rs"]
mod tests;
