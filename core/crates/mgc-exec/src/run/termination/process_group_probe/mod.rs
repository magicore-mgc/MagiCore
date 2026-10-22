//! Private owner for Unix process/group identity and liveness probes.
//! Owner riêng cho probe identity và liveness process/group Unix.

#[path = "process_group_probe.rs"]
mod implementation;

pub(in crate::run::implementation) use implementation::{
    process_group_id, process_group_is_live, process_groups_have_descendants,
};
