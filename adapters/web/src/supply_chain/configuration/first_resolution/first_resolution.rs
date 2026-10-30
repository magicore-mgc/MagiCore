//! Decide whether this process resolves a package it has never installed.
//! Quyết định process này có resolve package chưa từng được cài hay không.

/// A project with no lockfile has no previous decision about its packages:
/// the Web scope is the first release decision for that package set. A
/// project that already carries `mgc.lock` has a decision and must stay
/// installable after the default quarantine tightens — FIX-05 default is
/// fail-closed for NEW packages, not a lock-invalidating rewrite.
/// (Project chưa có lockfile nghĩa là chưa từng quyết định về tập package —
/// scope Web chính là quyết định đầu tiên. Project đã có `mgc.lock` thì đã
/// quyết định và phải cài được sau khi siết quarantine mặc định.)
///
/// A released binary cannot prove that a package it resolved moments ago is
/// older than the quarantine, and checkout order will not tell it: lying here
/// would let a typosquat published minutes ago install through a project that
/// never had a lock. Missing lock = first resolution = default gate armed.
/// (Binary phát hành không thể chứng minh package vừa resolve đủ tuổi; nói
/// dối ở đây sẽ để typosquat vừa publish cài qua project chưa có lock.)
pub(in crate::supply_chain) fn has_prior_install_decision(cwd: &std::path::Path) -> bool {
    cwd.join("mgc.lock").is_file()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
