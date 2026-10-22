//! Process-monitor data contract facade.
//! Facade hợp đồng dữ liệu process monitor.

#[path = "types.rs"]
mod implementation;

pub(in crate::run::implementation) use implementation::{
    CompatAuditContext, NativeAuditContext, ProcessMonitor,
};
