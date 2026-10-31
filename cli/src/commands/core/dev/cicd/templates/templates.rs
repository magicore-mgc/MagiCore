//! Single placeholder renderer for CI templates — renderer placeholder duy nhất.
//!
//! Every template `ci-generate` writes goes through `render_ci_template` or
//! `render_ci_template_with_repo`, and those delegate to the cicd adapter so a
//! scaffolded pipeline and a regenerated pipeline are the same bytes. The
//! release/installer pins themselves are declared by the adapter, so both lanes
//! read ONE source of truth.
//! Mọi template `ci-generate` ghi đều đi qua `render_ci_template` hoặc
//! `render_ci_template_with_repo`, và chúng ủy quyền cho adapter cicd nên
//! pipeline scaffold và pipeline sinh lại là cùng bytes. Giá trị pin
//! release/installer do adapter khai báo nên cả hai lane đọc MỘT nguồn chân lý.

use anyhow::Result;
use mgc_cicd_adapter::scaffold::{
    ensure_release_pin as adapter_ensure_release_pin, render_ci_template_with_repo as render,
};

/// Render a CI template with the default (generator-time) repository placeholder.
/// Render template CI với placeholder repository mặc định của lúc sinh.
pub(in crate::commands::core::dev::cicd) fn render_ci_template(
    template: &str,
    name: &str,
) -> String {
    render(template, name, "{repo_url}")
}

/// Render a template that also needs the project's repository URL.
/// Render template cần thêm URL repository của project.
pub(in crate::commands::core::dev::cicd) fn render_ci_template_with_repo(
    template: &str,
    name: &str,
    repo_url: &str,
) -> String {
    render(template, name, repo_url)
}

/// Refuse to generate when the release pin is missing (SEC-10).
/// Từ chối sinh khi thiếu pin release (SEC-10).
pub(in crate::commands::core::dev::cicd) fn ensure_release_pin() -> Result<()> {
    Ok(adapter_ensure_release_pin()?)
}
