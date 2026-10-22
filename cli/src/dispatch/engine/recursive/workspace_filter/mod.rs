// Workspace-filter wiring — expose package and path selection to the coordinator.
// Wiring lọc workspace — cung cấp bộ chọn package và path cho coordinator.
#[cfg(test)]
mod test;
mod workspace_filter;

pub(super) use workspace_filter::filter_workspaces;
