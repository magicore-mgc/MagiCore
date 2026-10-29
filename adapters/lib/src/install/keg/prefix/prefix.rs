//! Prefix links that expose an active keg version at a stable path.
//! Link prefix đưa version keg đang active ra một đường dẫn ổn định.
//!
//! Link targets are relative, so a relocated `MGC_DATA_DIR` tree keeps working
//! and a prefix never depends on where the keg was first created.
//! Đích link là tương đối nên cây `MGC_DATA_DIR` bị dời vẫn dùng được và
//! prefix không phụ thuộc vị trí keg lúc tạo.

use std::path::{Path, PathBuf};

use semver::Version;

use crate::install::keg::error::{KegError, KegResult};
use crate::install::keg::layout::{ACTIVE_LINK, KegLayout};

/// One link that must exist for a version to be exposed.
/// Một link bắt buộc tồn tại để version được phơi ra.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixLink {
    /// Link location.
    /// Vị trí link.
    pub path: PathBuf,
    /// Relative symlink target.
    /// Đích symlink tương đối.
    pub target: PathBuf,
}

/// Build the link set that exposes one keg version.
/// Tạo tập link phơi ra một version keg.
///
/// Two links are produced: the active-version link (`opt/<lib>/current`) and
/// the version prefix (`prefix/<lib>/<version>`), so both `mgc lib` switch and
/// per-version consumption point at the same keg.
/// Hai link được tạo: link version active (`opt/<lib>/current`) và prefix theo
/// version (`prefix/<lib>/<version>`), nên cả switch và tiêu thụ theo version
/// đều trỏ cùng một keg.
///
/// `keg_dir` is the resolved keg path supplied by the caller (already
/// validated by the store), not a path re-composed here.
/// `keg_dir` do caller cấp (store đã xác minh), không tự ghép lại đường dẫn.
pub fn plan_links(
    layout: &KegLayout,
    name: &str,
    version: &Version,
    keg_dir: &Path,
) -> KegResult<Vec<PrefixLink>> {
    let opt_dir = layout.opt_dir(name);
    let current = opt_dir.join(ACTIVE_LINK);
    let version_prefix = layout.version_prefix_dir(name, version);
    Ok(vec![
        PrefixLink {
            target: relative_target(&opt_dir, keg_dir)?,
            path: current,
        },
        PrefixLink {
            target: relative_target(&layout.library_prefix_dir(name), keg_dir)?,
            path: version_prefix,
        },
    ])
}

/// Create the directory structure a link set needs.
/// Tạo cấu trúc thư mục mà tập link cần.
pub fn ensure_link_parents(links: &[PrefixLink]) -> KegResult<()> {
    for link in links {
        let parent = link.path.parent().ok_or_else(|| KegError::UnsafePath {
            path: link.path.display().to_string(),
            reason: "prefix link has no parent directory",
        })?;
        std::fs::create_dir_all(parent)
            .map_err(|error| KegError::io("create prefix directory", parent, error))?;
    }
    Ok(())
}

/// Compute a relative symlink target from a link's parent directory.
/// Tính đích symlink tương đối từ thư mục cha của link.
pub fn relative_target(link_parent: &Path, destination: &Path) -> KegResult<PathBuf> {
    // Relative links can never leave the shared filesystem root, so only the
    // shared prefix length matters; comparing full absolute prefixes would
    // reject valid cross-directory layouts (e.g. `/x` vs `/y`).
    // (Link tương đối không thể vượt root filesystem chung; chỉ độ dài tiền tố
    // chung quan trọng — so toàn tiền tố tuyệt đối sẽ từ chối layout hợp lệ.)
    let from: Vec<_> = link_parent.components().collect();
    let to: Vec<_> = destination.components().collect();
    if from.first() != to.first() {
        return Err(KegError::UnsafePath {
            path: destination.display().to_string(),
            reason: "link and keg are on different filesystem roots",
        });
    }
    let shared = from
        .iter()
        .zip(to.iter())
        .take_while(|(left, right)| left == right)
        .count();
    let mut relative = PathBuf::new();
    for _ in shared..from.len() {
        relative.push("..");
    }
    for component in &to[shared..] {
        relative.push(component);
    }
    if relative.as_os_str().is_empty() {
        Err(KegError::UnsafePath {
            path: destination.display().to_string(),
            reason: "link target resolves to itself",
        })
    } else {
        Ok(relative)
    }
}

/// Replace `path` with a symlink to `target`, atomically for the final entry.
/// Thay `path` bằng symlink tới `target`, atomic cho entry cuối.
pub fn place_link(path: &Path, target: &Path) -> KegResult<()> {
    let parent = path.parent().ok_or_else(|| KegError::UnsafePath {
        path: path.display().to_string(),
        reason: "prefix link has no parent directory",
    })?;
    std::fs::create_dir_all(parent)
        .map_err(|error| KegError::io("create prefix directory", parent, error))?;
    let staging = parent.join(format!(
        ".link-{}.{}",
        std::process::id(),
        path.file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default()
    ));
    if std::fs::symlink_metadata(&staging).is_ok() {
        std::fs::remove_file(&staging)
            .map_err(|error| KegError::io("remove stale link staging", &staging, error))?;
    }
    // The staged entry has no extension, so pick the link kind from the
    // destination rather than from the staging name (Windows needs this).
    // (Entry staging không có đuôi, nên chọn loại link theo đích, không theo
    // tên staging — Windows cần điều này.)
    create_link_to(target, &staging)?;
    // Rename both directions, exactly like a fully featured `ln -sf`: a
    // regular file at the destination is replaced instead of failing.
    // (Rename hai chiều như `ln -sf`: file thật ở đích bị thay, không lỗi.)
    match mgc_platform::atomic_replace::atomic_replace_file(&staging, path) {
        Ok(()) => Ok(()),
        Err(_) if is_regular_file(path) => {
            mgc_platform::atomic_replace::atomic_replace_file(path, &staging)
                .map_err(|error| KegError::io("replace prefix link", path, error))
        }
        Err(error) => Err(KegError::io("replace prefix link", path, error)),
    }
}

fn is_regular_file(path: &Path) -> bool {
    std::fs::symlink_metadata(path)
        .map(|metadata| metadata.file_type().is_file())
        .unwrap_or(false)
}

/// Create a link to `target` choosing file/dir kind from the target.
/// Tạo link tới `target`, chọn loại file/dir theo đích.
pub fn create_link_to(target: &Path, link: &Path) -> KegResult<()> {
    #[cfg(windows)]
    {
        if target.is_dir() {
            std::os::windows::fs::symlink_dir(target, link)
        } else {
            std::os::windows::fs::symlink_file(target, link)
        }
        .map_err(|error| KegError::io("create prefix symlink", link, error))
    }
    #[cfg(not(windows))]
    {
        create_symlink(target, link)
    }
}

/// Create a symlink without following an existing destination.
/// Tạo symlink và không đi theo đích đã tồn tại.
pub fn create_symlink(target: &Path, link: &Path) -> KegResult<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(target, link)
            .map_err(|error| KegError::io("create prefix symlink", link, error))
    }
    #[cfg(windows)]
    {
        if target.is_dir() {
            std::os::windows::fs::symlink_dir(target, link)
        } else {
            std::os::windows::fs::symlink_file(target, link)
        }
        .map_err(|error| KegError::io("create prefix symlink", link, error))
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = (target, link);
        Err(KegError::Layout(
            "symlinks are not supported on this platform".to_string(),
        ))
    }
}

/// Read a link's raw target.
/// Đọc đích thô của một link.
pub fn link_target(link: &Path) -> KegResult<Option<PathBuf>> {
    match std::fs::symlink_metadata(link) {
        Ok(metadata) if metadata.file_type().is_symlink() => std::fs::read_link(link)
            .map(Some)
            .map_err(|error| KegError::io("read prefix symlink", link, error)),
        Ok(_) => Ok(None),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(KegError::io("inspect prefix link", link, error)),
    }
}

/// Remove a link if present; refuses to remove a real directory silently.
/// Xoá link nếu có; từ chối âm thầm xoá thư mục thật.
pub fn remove_link(link: &Path) -> KegResult<()> {
    match std::fs::symlink_metadata(link) {
        Ok(metadata) if metadata.file_type().is_symlink() => std::fs::remove_file(link)
            .map_err(|error| KegError::io("remove prefix symlink", link, error)),
        Ok(_) => Err(KegError::UnsafePath {
            path: link.display().to_string(),
            reason: "prefix entry is not a managed symlink",
        }),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(KegError::io("inspect prefix link", link, error)),
    }
}
