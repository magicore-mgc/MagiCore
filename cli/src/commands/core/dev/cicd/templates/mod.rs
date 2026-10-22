mod circleci;
mod github_actions;
mod gitlab;
mod templates;

pub(super) use circleci::CIRCLE_TEMPLATE;
pub(super) use github_actions::WORKFLOW_TEMPLATE;
pub(super) use gitlab::GITLAB_TEMPLATE;
pub(super) use templates::render_ci_template;
#[cfg(test)]
pub(super) use templates::{MGC_INSTALLER_SHA, MGC_INSTALLER_SHA256, MGC_RELEASE_TAG};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
