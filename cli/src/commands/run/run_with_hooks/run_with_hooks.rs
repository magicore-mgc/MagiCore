//! Pre/main/post script lifecycle.
//! Vòng đời script pre/main/post.

use super::super::{resolve_task_script, run_task_script};
use anyhow::Result;
use mgc_ui::warning;
use std::path::Path;

/// Pre/post lifecycle orchestration (Spec rev4 §16.1 — decided):
/// `pre<script>` runs first and aborts the chain on failure; `post<script>`
/// always runs once; a main failure is reported even when post also fails
/// (the hook failure is still warned).
/// (Điều phối lifecycle pre/post (Spec rev4 §16.1 — đã chốt): `pre<script>`
/// chạy trước, hỏng thì dừng chuỗi; `post<script>` luôn chạy một lần; lỗi
/// main được báo kể cả khi post cũng hỏng (lỗi hook vẫn cảnh báo).)
pub(crate) fn run_with_hooks(
    project_root: &Path,
    script: &str,
    args: &[String],
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
    operation: &str,
) -> Result<()> {
    let pre = format!("pre{script}");
    if resolve_task_script(project_root, &pre)?.is_some() {
        run_task_script(project_root, &pre, &[], compat, compat_runner, operation)?;
    }
    let main_result = run_task_script(project_root, script, args, compat, compat_runner, operation);
    let post = format!("post{script}");
    if resolve_task_script(project_root, &post)?.is_some() {
        let post_result =
            run_task_script(project_root, &post, &[], compat, compat_runner, operation);
        if let Err(post_err) = post_result {
            // Main failure wins the report; the hook failure is still loud.
            // A post failure after main success IS the result.
            // (Lỗi main thắng khi báo; lỗi hook vẫn ồn ào. Post hỏng sau
            // main thành công CHÍNH là kết quả.)
            warning(&format!("post hook '{post}' failed: {post_err:#}"));
            if main_result.is_ok() {
                return Err(post_err);
            }
        }
    }
    main_result
}
