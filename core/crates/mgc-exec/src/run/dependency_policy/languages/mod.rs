//! Per-ecosystem dependency-resolution rules.
//! Rule resolve dependency riêng theo từng ecosystem.

#[path = "cargo/mod.rs"]
mod cargo;
#[path = "dotnet/mod.rs"]
mod dotnet;
#[path = "flutter/mod.rs"]
mod flutter;
#[path = "go/mod.rs"]
mod go;
#[path = "java/mod.rs"]
mod java;
#[path = "python/mod.rs"]
mod python;
#[path = "rustc/mod.rs"]
mod rustc;
#[path = "swift/mod.rs"]
mod swift;
#[path = "zig/mod.rs"]
mod zig;

#[cfg(test)]
pub(in crate::run) use go::is_go_toolchain_version_probe;

mod dispatch;

pub(in crate::run) use dispatch::denial_reason;
