//! Orders monorepo Web installation and its local helper owners.
//! Sắp thứ tự cài Web monorepo và các owner helper cục bộ.

mod concurrency_policy;
mod link_workspace_packages;
mod monorepo_install;
mod target_installer;
mod workspace_target_install;

pub(super) use monorepo_install::run;
