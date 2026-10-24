//! End-to-end checks that container flags are honored by the shipped CLI.
//! Kiểm E2E rằng flag container được CLI phát hành thực thi thật.

#[path = "common/mod.rs"]
pub(crate) mod common;

#[path = "container_scaffold_e2e/container_scaffold_e2e.rs"]
mod container_scaffold_e2e;
