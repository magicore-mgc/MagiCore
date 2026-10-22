//! Shared monitor-test helper wiring.
//! Wiring helper dùng chung cho test monitor.

mod helpers;
pub(super) use helpers::{
    find_forbidden_descendant_with_program, inspect_forbidden_process_table,
    process_table_with_child, process_table_with_raw_child,
};
