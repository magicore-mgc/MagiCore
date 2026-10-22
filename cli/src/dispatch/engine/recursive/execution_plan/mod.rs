// Execution-plan wiring — validate and resolve all workspace targets up front.
// Wiring execution plan — xác minh và resolve mọi workspace target từ đầu.
mod execution_plan;
#[cfg(test)]
mod test;

pub(super) use execution_plan::{WorkspaceExecutionTarget, plan_workspaces};
