//! GitHub Actions scanner owners and stable public routes.
//! Các owner scanner GitHub Actions và các route công khai ổn định.

mod gh_actions;
mod report;
mod types;
mod workflow_text;

pub use gh_actions::audit_github_actions;
pub use types::WorkflowFinding;
pub use workflow_text::scan_workflow_text;
