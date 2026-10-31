//! Pipeline templates owner — wiring only.
//! Owner template pipeline — chỉ wiring.

#[path = "templates.rs"]
mod templates;

pub use templates::{
    CIRCLE_TEMPLATE, GITLAB_TEMPLATE, MGC_INSTALLER_SHA, MGC_INSTALLER_SHA256, MGC_RELEASE_TAG,
    WORKFLOW_TEMPLATE, ensure_release_pin, render_ci_template, render_ci_template_with_repo,
};
