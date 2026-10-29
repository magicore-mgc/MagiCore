//! `mgc lib` facade over the keg/prefix transfer layer.
//! Facade `mgc lib` trên lớp keg/prefix.
//!
//! One entry point per CLI verb (`list`, `switch`, `rollback`, `install`) keeps
//! CLI code free of keg details and gives tests a stable surface. Every function
//! returns typed results; none reports success for work it did not perform.
//! Mỗi lệnh CLI một hàm vào (`list`, `switch`, `rollback`, `install`) để code
//! CLI không phải biết chi tiết keg. Mọi hàm trả kết quả typed; không bao giờ
//! báo thành công cho việc chưa làm.

use std::path::Path;

use semver::Version;

use crate::install::keg::active::{ActiveState, pointer_path, read_active};
use crate::install::keg::error::{KegError, KegResult};
use crate::install::keg::install::{InstallOutcome, install_and_activate};
use crate::install::keg::keg::KegStore;
use crate::install::keg::switch::{SwitchOutcome, switch_active};

/// One row of `mgc lib list`.
/// Một dòng của `mgc lib list`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KegListing {
    /// Library name.
    /// Tên library.
    pub name: String,
    /// Every installed version, oldest first.
    /// Mọi version đã cài, cũ nhất trước.
    pub versions: Vec<Version>,
    /// Active version, if any.
    /// Version active, nếu có.
    pub active: Option<Version>,
    /// Version active before the last switch, if any.
    /// Version active trước lần switch gần nhất, nếu có.
    pub previous: Option<Version>,
}

/// `mgc lib list <name>`: kegs, active version and previous version.
/// `mgc lib list <name>`: danh sách keg cùng version active và trước đó.
pub fn list_kegs(store: &KegStore, name: &str) -> KegResult<KegListing> {
    let records = store.list_kegs(name)?;
    let versions: Vec<Version> = records
        .iter()
        .filter_map(|record| Version::parse(&record.version).ok())
        .collect();
    let state = read_active(store.layout(), name)?;
    let active = state
        .as_ref()
        .and_then(|state| Version::parse(&state.version).ok());
    if let Some(active) = &active
        && !versions.contains(active)
    {
        {
            return Err(KegError::InvalidActive {
                name: name.to_string(),
                path: pointer_path(store.layout(), name).display().to_string(),
                reason: format!("active version {active} has no keg"),
            });
        }
    }
    let previous = state
        .as_ref()
        .and_then(|state| state.previous.as_ref())
        .and_then(|previous| Version::parse(previous).ok());
    Ok(KegListing {
        name: name.to_string(),
        versions,
        active,
        previous,
    })
}

/// `mgc lib switch <name>@<version>`: activate an installed version.
/// `mgc lib switch <name>@<version>`: kích hoạt một version đã cài.
pub fn lib_switch(store: &KegStore, name: &str, version: &str) -> KegResult<SwitchOutcome> {
    switch_active(store, name, version)
}

/// `mgc lib switch <name>@<version>` with a single `name@version` argument.
/// `mgc lib switch <name>@<version>` với một đối số `name@version`.
pub fn lib_switch_spec(store: &KegStore, spec: &str) -> KegResult<SwitchOutcome> {
    let (name, version) = split_spec(spec)?;
    switch_active(store, name, version)
}

/// `mgc lib rollback <name>`: re-activate the previous version.
/// `mgc lib rollback <name>`: kích hoạt lại version trước đó.
pub fn lib_rollback(store: &KegStore, name: &str) -> KegResult<SwitchOutcome> {
    crate::install::keg::switch::rollback_active(store, name)
}

/// `mgc lib install`: materialize a version into an immutable keg, then activate.
/// `mgc lib install`: materialize version vào keg bất biến rồi kích hoạt.
pub fn lib_install<F>(
    store: &KegStore,
    name: &str,
    version: &str,
    materialize: F,
) -> KegResult<InstallOutcome>
where
    F: FnOnce(&Path) -> KegResult<()>,
{
    install_and_activate(store, name, version, materialize)
}

/// `mgc lib install <binary>`: materialize into a keg, activate it, and expose
/// the executable on the keg bin path (P-6).
/// `mgc lib install <binary>`: materialize vào keg, kích hoạt, và phơi
/// executable trên bin path của keg (P-6).
///
/// Installing into the keg store without a link would leave the version active
/// in `opt/` while nothing executable was reachable, which is the gap
/// `install-lib` had: it sealed the artifact and never exposed it. The link is
/// created after activation succeeds, so a failed install never leaves a shim
/// pointing at a missing keg.
/// (Cài vào keg mà không tạo link sẽ để version active trong `opt/` trong khi
/// không có gì chạy được — đúng lỗ hổng `install-lib` gặp: seal artifact mà
/// không phơi ra. Link được tạo sau khi kích hoạt thành công, nên lần cài hỏng
/// không bao giờ để shim trỏ tới keg không tồn tại.)
pub fn install_binary<F>(
    store: &KegStore,
    name: &str,
    version: &str,
    binary: &str,
    materialize: F,
) -> KegResult<InstallOutcome>
where
    F: FnOnce(&Path) -> KegResult<()>,
{
    let outcome = install_and_activate(store, name, version, materialize)?;
    let keg_dir = store.keg_dir(&outcome.keg.name, &outcome.keg.version)?;
    let bin_dir = store.layout().bin_dir();
    std::fs::create_dir_all(&bin_dir).map_err(|error| KegError::Io {
        operation: "create keg bin directory",
        path: bin_dir.clone(),
        source: error,
    })?;
    let shim = bin_dir.join(binary);
    // Re-pointing an existing shim is the normal upgrade path, so the old link
    // is replaced only after the new target exists.
    // (Trỏ lại shim sẵn có là đường nâng cấp bình thường, nên link cũ chỉ bị
    // thay sau khi target mới tồn tại.)
    if shim.exists() || shim.symlink_metadata().is_ok() {
        std::fs::remove_file(&shim).map_err(|error| KegError::Io {
            operation: "replace keg binary shim",
            path: shim.clone(),
            source: error,
        })?;
    }
    let target = keg_dir.join(binary);
    if !target.is_file() {
        return Err(KegError::Io {
            operation: "link keg binary",
            path: target,
            source: std::io::Error::other("the keg does not contain the requested binary"),
        });
    }
    create_keg_symlink(&target, &shim)?;
    Ok(outcome)
}

/// Create the shim link, using a copy on platforms without symlink support.
/// Tạo link shim, dùng bản copy trên nền tảng không hỗ trợ symlink.
fn create_keg_symlink(target: &Path, shim: &Path) -> KegResult<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(target, shim).map_err(|error| KegError::Io {
            operation: "symlink keg binary",
            path: shim.to_path_buf(),
            source: error,
        })
    }
    #[cfg(not(unix))]
    {
        std::fs::copy(target, shim).map_err(|error| KegError::Io {
            operation: "copy keg binary",
            path: shim.to_path_buf(),
            source: error,
        })
    }
}

/// Active state for one library.
/// Trạng thái active của một library.
pub fn lib_active(store: &KegStore, name: &str) -> KegResult<Option<ActiveState>> {
    read_active(store.layout(), name)
}

/// Split `name@version`; the last `@` separates the version.
/// Tách `name@version`; `@` cuối cùng phân tách version.
pub fn split_spec(spec: &str) -> KegResult<(&str, &str)> {
    match spec.rsplit_once('@') {
        Some((name, version)) if !name.is_empty() && !version.is_empty() => Ok((name, version)),
        _ => Err(KegError::UnsafePath {
            path: spec.to_string(),
            reason: "expected '<library>@<version>'",
        }),
    }
}
