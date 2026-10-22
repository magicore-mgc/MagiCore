//! Shared install execution owner facade and stable result type.
//! Facade owner thực thi install dùng chung và type kết quả ổn định.

#[path = "gateway_entry/mod.rs"]
mod gateway_entry;
#[path = "locked_tail/mod.rs"]
mod locked_tail;
#[path = "preparation/mod.rs"]
mod preparation;
#[path = "types/mod.rs"]
mod types;

pub(crate) use gateway_entry::install_with_adapter;
pub(in crate::commands::core::shared::implementation::implementation) use locked_tail::install_with_adapter_locked;
pub(crate) use preparation::prepare_install_execution;
pub(crate) use types::InstallExecution;
