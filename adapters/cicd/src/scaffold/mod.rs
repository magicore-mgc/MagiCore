//! Offline CI/CD scaffolding and the released pipeline templates.
//! Scaffold CI/CD offline và các template pipeline đã phát hành.
//!
//! Two owners live here on purpose. `templates` holds the exact bytes
//! `mgc ci-generate` writes, and `project` is the offline `create-cicd`
//! generator. Keeping both behind one adapter module means the pipeline that a
//! fresh project starts with and the pipeline regenerated later cannot drift.
//! Hai owner ở đây là có chủ đích. `templates` giữ đúng bytes `mgc ci-generate`
//! ghi, còn `project` là generator offline của `create-cicd`. Gộp cùng một
//! module adapter để pipeline của project mới và pipeline sinh lại sau đó không
//! thể lệch nhau.

mod layout;
mod project;
mod templates;

pub use layout::{GENERATED_PROVIDERS, PipelineFile, can_generate, generation_plan};
pub use project::scaffold_project;
pub use templates::{
    CIRCLE_TEMPLATE, GITLAB_TEMPLATE, MGC_INSTALLER_SHA, MGC_INSTALLER_SHA256, MGC_RELEASE_TAG,
    WORKFLOW_TEMPLATE, ensure_release_pin, render_ci_template, render_ci_template_with_repo,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
