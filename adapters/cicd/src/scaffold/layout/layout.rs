//! Where each provider's pipeline file lands inside a project tree.
//! Vị trí file pipeline của mỗi provider trong cây project.
//!
//! The path and the rendered bytes come from one place so `create-cicd` and
//! `mgc ci-generate` cannot disagree about a file name or create a second YAML
//! document by hand.
//! Đường dẫn và bytes render lấy từ một chỗ, nên `create-cicd` và
//! `mgc ci-generate` không thể lệch tên file hay sinh YAML thứ hai bằng tay.

use std::path::PathBuf;

use mgc_types::MgResult;

use super::super::templates::{
    CIRCLE_TEMPLATE, GITLAB_TEMPLATE, WORKFLOW_TEMPLATE, render_ci_template,
};
use crate::provider::CicdProvider;

/// One generated pipeline file: path relative to the project root, contents,
/// whether its parent directory must be created, and the success label.
/// (Một file pipeline: đường dẫn tương đối, nội dung, có cần tạo thư mục cha
/// không, và nhãn thành công.)
#[derive(Debug, Clone)]
pub struct PipelineFile {
    /// Path relative to the project root — đường dẫn tương đối so với gốc.
    pub relative_path: PathBuf,
    /// Fully rendered file contents — nội dung file đã render đầy đủ.
    pub contents: String,
    /// Whether writing requires `create_dir_all` on the parent.
    /// Có cần `create_dir_all` thư mục cha khi ghi hay không.
    pub create_parent: bool,
    /// Success message printed after the write.
    /// Thông điệp thành công in sau khi ghi.
    pub success_label: &'static str,
}

/// Providers for which a complete, byte-pinned pipeline can be generated.
/// Provider mà generator sinh được pipeline hoàn chỉnh, ghim theo byte.
///
/// Only providers whose template is released here are listed: advertising a
/// name that has no template is exactly the hollow scaffold claim PAR-02
/// removed.
/// (Chỉ liệt kê provider thật sự có template ở đây: quảng cáo tên không có
/// template chính là claim scaffold rỗng mà PAR-02 đã loại bỏ.)
pub const GENERATED_PROVIDERS: &[&str] = &["github-actions", "gitlab", "circleci"];

/// Whether `provider` has a pipeline template this build can emit.
/// Provider có template pipeline mà build này sinh được hay không.
pub fn can_generate(provider: CicdProvider) -> bool {
    GENERATED_PROVIDERS.contains(&provider.as_str())
}

/// The pipeline files for `provider`, rendered with `name` as project name.
/// Các file pipeline cho `provider`, render với `name` làm tên project.
pub fn generation_plan(provider: CicdProvider, name: &str) -> MgResult<Vec<PipelineFile>> {
    let plan = match provider {
        CicdProvider::GithubActions => vec![PipelineFile {
            relative_path: PathBuf::from(".github").join("workflows").join("ci.yml"),
            contents: render_ci_template(WORKFLOW_TEMPLATE, name),
            create_parent: true,
            success_label: "CI workflow generated",
        }],
        CicdProvider::Gitlab => vec![PipelineFile {
            relative_path: PathBuf::from(".gitlab-ci.yml"),
            contents: render_ci_template(GITLAB_TEMPLATE, ""),
            create_parent: false,
            success_label: "GitLab CI generated",
        }],
        CicdProvider::CircleCi => vec![PipelineFile {
            relative_path: PathBuf::from(".circleci").join("config.yml"),
            contents: render_ci_template(CIRCLE_TEMPLATE, ""),
            create_parent: true,
            success_label: "CircleCI config generated",
        }],
        // A provider the registry knows but this build has no template for is
        // refused here instead of writing an empty project.
        // (Provider registry biết nhưng build này không có template bị từ chối
        // ở đây thay vì ghi một project rỗng.)
        other => {
            return Err(mgc_types::MgError::Unsupported {
                core: "cicd",
                capability: "scaffold",
                guidance: format!(
                    "no offline pipeline template for provider '{}' — this build generates: {}; \
                     run `mgc ci-generate` after adding a provider template",
                    other.as_str(),
                    GENERATED_PROVIDERS.join(" | ")
                ),
            });
        }
    };
    Ok(plan)
}
