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
