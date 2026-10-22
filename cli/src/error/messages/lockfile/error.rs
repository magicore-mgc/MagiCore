//! Lockfile and dependency lifecycle diagnostics.
//! Lỗi lockfile và vòng đời dependency.

use anyhow::Error;
use anyhow::anyhow;

/// lockfile mới hơn mgc hỗ trợ
pub fn lockfile_newer(version: u32, supported: u32) -> Error {
    anyhow!(
        "mgc.lock version {version} is newer than this version of mgc (supports up to {supported}). \
         Upgrade mgc to read this lockfile."
    )
}

/// dependency id trong lockfile không parse được
pub fn invalid_dep_id(dep: &str, name: &str, version: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!("invalid dependency id '{dep}' in lockfile package '{name}@{version}': {e}")
}

pub fn frozen_lock_missing(cmd: &str) -> Error {
    anyhow!(
        "--frozen: mgc.lock is missing or does not match package.json.\n\
         Run '{cmd}' to generate an up-to-date lockfile."
    )
}

/// Frozen install with a PRESENT but mismatching lockfile — a possible
/// tamper, never silently re-resolved.
pub fn frozen_lock_mismatch(cmd: &str) -> Error {
    anyhow!(
        "--frozen: mgc.lock exists but does not match the manifest (drift or tamper) — refusing to re-resolve.\n\
         Run '{cmd}' without --frozen to regenerate after reviewing the diff."
    )
}

pub fn lock_missing_install() -> Error {
    anyhow!("mgc.lock not found — run 'mgc install' first")
}

// P0-4 (2026-09-15): `mgc why` used to `unimplemented!()` (panic) when it
// reached the v1 lockfile path. A user-reachable panic is never acceptable:
// the command now returns a typed error that explains the lockfile v2
// migration instead of aborting the process.
// (P0-4: `mgc why` từng `unimplemented!()` (panic) khi đi tới nhánh
// lockfile v1. Panic chạm tới được từ user là không chấp nhận được: lệnh
// giờ trả typed error giải thích migration lockfile v2 thay vì abort.)
pub fn why_requires_lockfile_v2() -> Error {
    anyhow!(
        "`mgc why` requires the lockfile v2 graph (dependency-reason lookup is not \
         available in the legacy v1 mgc.lock schema). Re-run `mgc install` to \
         regenerate mgc.lock in the v2 format; until that migration completes, \
         `mgc why` stays unavailable for lockfiles written by the v1 installer."
    )
}

/// `mgc why` target is absent from mgc.lock — nothing can depend on it.
/// (Package hỏi `why` không có trong lock — không gì phụ thuộc nó.)
pub fn why_package_not_in_lock(package: &str) -> Error {
    anyhow!(
        "`mgc why {package}`: '{package}' is not pinned in mgc.lock — nothing in this project can depend on it."
    )
}

pub fn cargo_toml_no_deps() -> Error {
    anyhow!("root Cargo.toml missing [dependencies]")
}

/// `install-lib` got package args: install replays the existing
/// graph/lock, it never adds (adding spawns the provider toolchain and
/// belongs to `add-lib` behind its own gate).
pub fn install_lib_packages_use_add(packages: &[String]) -> Error {
    anyhow!(
        "`install-lib` takes no packages (got {:?}) — install replays the existing graph/lock through the native pipeline; add packages with `mgc add-lib <package>`",
        packages
    )
}

/// `install-app` takes no packages — same split as install-lib.
pub fn install_app_packages_use_add(packages: &[String]) -> Error {
    anyhow!(
        "`install-app` takes no packages (got {:?}) — install replays the existing graph/lock through the native pipeline; add packages with `mgc add-app <package>`",
        packages
    )
}

/// `update` of a package the manifest does not declare — fail loudly
/// instead of adding it silently (add and update stay separate verbs).
pub fn update_unknown_package(name: &str) -> Error {
    anyhow!("cannot update '{name}': not in the project manifest — add it first with `mgc add`")
}

/// `mgc outdated` checked nothing: every registry fetch failed, so any
/// "up to date" verdict would be fabricated.
/// (outdated không check được gì — fail cứng thay vì báo láo.)
pub fn outdated_no_registry_response(failed: String) -> Error {
    anyhow!(
        "could not reach the registry for any dependency ({failed}) — refusing to report 'up to date' on zero evidence"
    )
}

/// --version pinning is not implemented for this lane (its tool call has
/// no verified version semantics) — failed loudly instead of silently
/// dropping the pin.
/// (--version chưa hỗ trợ cho lane này — fail rõ thay vì nuốt version.)
pub fn add_version_unsupported(core: &str, pinned: &str) -> Error {
    anyhow!(
        "`{core}` add does not support `--version {pinned}` yet (no verified version semantics on this lane) — pin the version inside the package spec or omit --version"
    )
}

/// A package already carrying its own version spec combined with
/// `--version` is ambiguous — failed loudly, never merged silently.
pub fn add_version_conflict(package: &str, pinned: &str) -> Error {
    anyhow!(
        "package '{package}' already carries a version spec and `--version {pinned}` was also given — use one or the other"
    )
}

/// `mgc dev` pre-flight: manifest declares dependencies missing from the lock.
/// (Pre-flight `mgc dev`: manifest khai báo dependency không có trong lock.)
pub fn stale_lock_needs_install(detail: &str) -> Error {
    anyhow!(
        "project dependencies changed since mgc.lock was written ({detail}) — run `mgc install` first"
    )
}
