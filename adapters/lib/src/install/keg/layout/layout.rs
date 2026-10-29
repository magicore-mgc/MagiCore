//! Keg/prefix filesystem layout for the lib core (Homebrew-style transfer layer).
//! Layout filesystem keg/prefix cho core lib (lớp luân chuyển kiểu Homebrew).
//!
//! ```text
//! <data-root>/lib/
//!   Cellar/<library>/<version>/      immutable keg (write-protected, sealed)
//!   opt/<library>/                   "active version" directory:
//!                                    Cellar/<lib>/<ver> + link -> active keg
//!   prefix/<library>/<version>/      linked prefix for one version
//! ```
//!
//! Directories derive from `GlobalPaths::root`, so `MGC_DATA_DIR` relocates the
//! whole layout and no path literal lives in logic (RULE §12).
//! Thư mục dẫn xuất từ `GlobalPaths::root`, nên `MGC_DATA_DIR` dời toàn bộ
//! layout và không có literal đường dẫn trong logic.

use std::path::{Path, PathBuf};

use mgc_platform::paths::GlobalPaths;
use semver::Version;

use crate::install::keg::error::{KegError, KegResult};

/// Directory name holding all immutable kegs.
/// Tên thư mục chứa mọi keg bất biến.
pub const CELLAR_DIR: &str = "Cellar";
/// Directory name holding active-version pointers and prefixes.
/// Tên thư mục chứa con trỏ version active và prefix.
pub const OPT_DIR: &str = "opt";
/// Directory name holding linked prefixes.
/// Tên thư mục chứa prefix đã link.
pub const PREFIX_DIR: &str = "prefix";
/// Link name inside `opt/<library>/` pointing at the active keg.
/// Tên link trong `opt/<library>/` trỏ tới keg active.
pub const ACTIVE_LINK: &str = "current";
/// Per-version symlink prefix inside a linked prefix directory.
/// Tiền tố symlink theo version trong thư mục prefix.
pub const PREFIX_ENTRY_PREFIX: &str = ".";

/// Resolved locations for the lib keg transfer layer.
/// Các vị trí đã resolve của lớp keg core lib.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KegLayout {
    /// MagiCore data root (`MGC_DATA_DIR` or the platform default).
    /// Root dữ liệu MagiCore.
    pub data_root: PathBuf,
}

impl KegLayout {
    /// Resolve from the user-global MagiCore data paths.
    /// Resolve từ đường dẫn dữ liệu MagiCore toàn user.
    pub fn resolve() -> KegResult<Self> {
        let globals = GlobalPaths::new()
            .map_err(|error| KegError::Layout(format!("resolve global paths: {error}")))?;
        Ok(Self {
            data_root: globals.root,
        })
    }

    /// Build a layout rooted at an explicit data directory (tests, sandboxes).
    /// Tạo layout với root dữ liệu chỉ định (test, sandbox).
    pub fn with_data_root(data_root: impl Into<PathBuf>) -> Self {
        Self {
            data_root: data_root.into(),
        }
    }

    /// Root directory of the lib keg transfer layer.
    /// Root của lớp keg core lib.
    pub fn lib_root(&self) -> PathBuf {
        self.data_root.join("lib")
    }

    /// Cellar directory holding every immutable keg.
    /// Thư mục Cellar chứa mọi keg bất biến.
    pub fn cellar_root(&self) -> PathBuf {
        self.lib_root().join(CELLAR_DIR)
    }

    /// `opt` directory holding active-version directories.
    /// Thư mục `opt` chứa thư mục version active.
    pub fn opt_root(&self) -> PathBuf {
        self.lib_root().join(OPT_DIR)
    }

    /// Linked-prefix root.
    /// Root prefix đã link.
    pub fn prefix_root(&self) -> PathBuf {
        self.lib_root().join(PREFIX_DIR)
    }

    /// Keg directory for one library version.
    /// Thư mục keg cho một version của library.
    /// Directory holding executable shims for keg-installed binaries.
    /// Thư mục chứa shim executable cho binary cài qua keg.
    ///
    /// A path, not a state: callers create it only when they actually install a
    /// binary, so a package-only install leaves no empty directory behind.
    /// (Đây là đường dẫn, không phải trạng thái: caller chỉ tạo khi thật sự cài
    /// binary, nên lần cài package thuần không để lại thư mục rỗng.)
    pub fn bin_dir(&self) -> PathBuf {
        self.lib_root().join("bin")
    }

    pub fn keg_dir(&self, name: &str, version: &Version) -> PathBuf {
        self.cellar_root().join(name).join(version.to_string())
    }

    /// Active-version directory for one library.
    /// Thư mục version active của một library.
    pub fn opt_dir(&self, name: &str) -> PathBuf {
        self.opt_root().join(name)
    }

    /// Library prefix directory (contains every linked version).
    /// Thư mục prefix của library (chứa mọi version đã link).
    pub fn library_prefix_dir(&self, name: &str) -> PathBuf {
        self.prefix_root().join(name)
    }

    /// Linked prefix directory for one version.
    /// Thư mục prefix đã link cho một version.
    pub fn version_prefix_dir(&self, name: &str, version: &Version) -> PathBuf {
        self.library_prefix_dir(name).join(version.to_string())
    }
}

/// Validate a library name used as a single path component.
/// Kiểm tra tên library dùng làm một thành phần đường dẫn.
pub fn validate_library_name(name: &str) -> KegResult<()> {
    if name.is_empty() {
        return Err(KegError::UnsafePath {
            path: name.to_string(),
            reason: "library name is empty",
        });
    }
    if name.len() > 128 {
        return Err(KegError::UnsafePath {
            path: name.to_string(),
            reason: "library name exceeds 128 bytes",
        });
    }
    if name == "." || name == ".." {
        return Err(KegError::UnsafePath {
            path: name.to_string(),
            reason: "library name is a relative traversal",
        });
    }
    let safe = name.chars().all(|character| {
        character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_' | '+' | '@')
    });
    if !safe || name.contains("..") {
        return Err(KegError::UnsafePath {
            path: name.to_string(),
            reason: "library name has characters outside [A-Za-z0-9._+@-]",
        });
    }
    Ok(())
}

/// Parse a version that must be a plain semantic version.
/// Parse version và bắt buộc là semver thuần.
pub fn validate_version(version: &str) -> KegResult<Version> {
    let parsed =
        Version::parse(version).map_err(|_| KegError::InvalidVersion(version.to_string()))?;
    if !parsed.build.is_empty() {
        return Err(KegError::InvalidVersion(version.to_string()));
    }
    Ok(parsed)
}

/// Reject a relative path that escapes its base directory.
/// Từ chối đường dẫn tương đối vượt ra ngoài thư mục gốc.
pub fn validate_relative(relative: &Path) -> KegResult<()> {
    if relative.as_os_str().is_empty() || relative.is_absolute() {
        return Err(KegError::UnsafePath {
            path: relative.display().to_string(),
            reason: "path must be relative and non-empty",
        });
    }
    for component in relative.components() {
        match component {
            std::path::Component::Normal(part) if !part.is_empty() => {}
            _ => {
                return Err(KegError::UnsafePath {
                    path: relative.display().to_string(),
                    reason: "path contains '..' / root / prefix components",
                });
            }
        }
    }
    Ok(())
}
