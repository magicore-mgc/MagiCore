// install_execution owner wiring — nối owner install_execution.
// The implementation stays in its same-name module file.
use super::*;
#[path = "install_execution.rs"]
mod implementation;
pub(in crate::commands::core::shared::implementation::implementation) use implementation::install_with_adapter_locked;
pub(crate) use implementation::{
    InstallExecution, install_with_adapter, prepare_install_execution,
};
