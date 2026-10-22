//! Local value returned by the process signal planner.
//! Value cục bộ do planner process signal trả về.

#[cfg(unix)]
#[derive(Debug, Eq, PartialEq)]
pub(in crate::run::implementation::termination) struct ProcessSignalPlan {
    pub(in crate::run::implementation::termination) process_groups: std::collections::HashSet<u32>,
    pub(in crate::run::implementation::termination) signal_root_directly: bool,
}
