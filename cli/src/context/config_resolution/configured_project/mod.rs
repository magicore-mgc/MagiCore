//! Own loading and identity adoption for projects with `mgc.toml`.
//! Sở hữu việc nạp và nhận identity cho project có `mgc.toml`.

mod configured_project;

pub(super) use configured_project::resolve;
