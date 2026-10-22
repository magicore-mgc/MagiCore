//! Workflow text scan routes and rule owners.
//! Route quét văn bản workflow và các owner rule.

mod permissions;
mod pull_request_target;
mod uses_reference;
mod workflow_text;

pub use workflow_text::scan_workflow_text;
