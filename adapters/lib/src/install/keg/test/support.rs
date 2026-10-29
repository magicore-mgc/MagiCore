//! Shared fixtures for keg/prefix tests.
//! Fixture dùng chung cho test keg/prefix.

// Shared fixture prelude: not every test file uses every helper.
// Prelude fixture dùng chung: không phải file test nào cũng dùng hết helper.
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use crate::install::keg::error::KegResult;
use crate::install::keg::keg::KegStore;
use crate::install::keg::layout::{KegLayout, validate_version};

/// A temporary keg layout with an explicit data root (no env mutation).
/// Layout keg tạm với data root chỉ định (không sửa env).
pub(super) struct Fixture {
    _temp: tempfile::TempDir,
    pub(super) layout: KegLayout,
    pub(super) store: KegStore,
}

impl Fixture {
    pub(super) fn new() -> Self {
        let temp = tempfile::tempdir().expect("create keg test fixture");
        let layout = KegLayout::with_data_root(temp.path().join("data"));
        let store = KegStore::new(layout.clone());
        Self {
            _temp: temp,
            layout,
            store,
        }
    }

    /// Data root of the fixture (used to assert relocation-independent links).
    /// Root dữ liệu của fixture (kiểm link không phụ thuộc vị trí).
    pub(super) fn data_root(&self) -> &Path {
        &self.layout.data_root
    }

    /// Install `version` with a fixed, verifiable file tree.
    /// Cài `version` với cây file cố định, kiểm được.
    pub(super) fn install(&self, name: &str, version: &str) -> PathBuf {
        self.store
            .create_keg(name, version, |staging| write_tree(staging, name, version))
            .expect("create keg");
        self.layout.keg_dir(
            name,
            &validate_version(version).expect("valid fixture version"),
        )
    }
}

/// Write a small deterministic tree into a staging keg.
/// Ghi cây nhỏ, tất định vào keg staging.
pub(super) fn write_tree(root: &Path, name: &str, version: &str) -> KegResult<()> {
    std::fs::create_dir_all(root.join("shared"))?;
    std::fs::create_dir_all(root.join("include"))?;
    std::fs::write(
        root.join("shared").join(format!("{name}.txt")),
        format!("{name} {version}\n"),
    )?;
    std::fs::write(root.join("include").join("header.h"), b"/* header */\n")?;
    std::fs::write(root.join("README"), format!("README {version}"))?;
    // Relative symlink inside the keg must survive sealing.
    // Symlink tương đối trong keg phải sống qua seal.
    crate::install::keg::prefix::create_symlink(
        Path::new("shared").join(format!("{name}.txt")).as_path(),
        &root.join("current.txt"),
    )?;
    // Fixture teardown may need to remove a sealed tree.
    // (Teardown fixture có thể cần xoá cây đã seal.)
    Ok(())
}

/// Permission bits of a path (0 on non-Unix).
/// Bit quyền của một đường dẫn (0 trên non-Unix).
pub(super) fn mode_of(path: &Path) -> u32 {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::symlink_metadata(path)
            .map(|metadata| metadata.permissions().mode() & 0o777)
            .unwrap_or(0)
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        0
    }
}

/// Make a path writable from inside the fixture teardown.
/// Đặt lại quyền ghi trong teardown fixture.
pub(super) fn make_writable(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if std::fs::symlink_metadata(path)
            .map(|metadata| metadata.file_type().is_symlink())
            .unwrap_or(false)
        {
            return;
        }
        if let Ok(metadata) = std::fs::metadata(path) {
            let mode = metadata.permissions().mode() | 0o200;
            let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode));
        }
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                make_writable(&entry.path());
            }
        }
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
}

/// Thaw every keg under a fixture before the temp dir is dropped.
/// Rã băng mọi keg trong fixture trước khi temp dir bị xoá.
pub(super) fn thaw_all(fixture: &Fixture, name: &str) {
    let library_root = fixture.layout.cellar_root().join(name);
    let Ok(entries) = std::fs::read_dir(&library_root) else {
        return;
    };
    for entry in entries.flatten() {
        make_writable(&entry.path());
    }
    make_writable(&fixture.layout.lib_root());
}
