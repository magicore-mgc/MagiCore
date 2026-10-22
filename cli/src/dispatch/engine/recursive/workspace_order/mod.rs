// Workspace-order wiring — expose dependency-first graph ordering to the coordinator.
// Wiring thứ tự workspace — cung cấp sắp thứ tự graph dependency-first cho coordinator.
#[cfg(test)]
mod test;
mod workspace_order;

pub(super) use workspace_order::order_workspaces;
