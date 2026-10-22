//! Wires shared launch context, runtime commands and framework launch owners.
//! Nối launch context, lệnh runtime và owner framework.

mod framework_launch;
mod package_launch;
mod runtime_commands;
mod types;

pub(in crate::commands::core::web::implementation) use package_launch::build_dev_launch;
