//! CI template resolution — resolve template CI.
//!
//! The three providers an offline `create-cicd` can scaffold (github-actions,
//! gitlab, circleci) have their bytes in the cicd adapter, so `mgc ci-generate`
//! and the scaffold emit the SAME pipeline from ONE generator. Providers that
//! only `ci-generate` emits stay here.
//! (Ba provider mà `create-cicd` offline scaffold được có bytes nằm ở adapter
//! cicd, nên `ci-generate` và scaffold phát ra CÙNG pipeline từ MỘT generator.
//! Provider chỉ `ci-generate` phát ra thì ở lại đây.)

mod argocd;
mod azure_pipelines;
mod bitbucket_pipelines;
mod jenkins;
mod templates;

pub(super) use argocd::argocd::{ARGOCD_APPLICATION_TEMPLATE, ARGOCD_PROJECT_TEMPLATE};
pub(super) use azure_pipelines::AZURE_TEMPLATE;
pub(super) use bitbucket_pipelines::BITBUCKET_TEMPLATE;
pub(super) use jenkins::JENKINS_TEMPLATE;
/// Re-export the adapter-owned templates so both lanes share one byte source.
/// Xuất lại template do adapter sở hữu để hai lane dùng chung một nguồn bytes.
pub(super) use mgc_cicd_adapter::scaffold::{CIRCLE_TEMPLATE, GITLAB_TEMPLATE, WORKFLOW_TEMPLATE};
#[cfg(test)]
pub(super) use mgc_cicd_adapter::scaffold::{
    MGC_INSTALLER_SHA, MGC_INSTALLER_SHA256, MGC_RELEASE_TAG,
};
pub(super) use templates::{ensure_release_pin, render_ci_template, render_ci_template_with_repo};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
