//! Project-name resolution: interactive prompt plus CI-friendly defaults.
//! Giải quyết tên dự án: hỏi tương tác và mặc định thân thiện CI.

use anyhow::Result;

pub(in crate::commands::init) fn ask_project_name() -> Result<String> {
    // Interactive input is user-controlled too — same traversal gate. After
    // MAX_NAME_RETRIES invalid attempts the wizard FAILS: we never invent a
    // project name the user did not ask for (fail-closed).
    // Input tương tác cũng từ user — cùng gate traversal. Hết số lần retry
    // thì wizard FAIL — không bao giờ tự sinh tên project user không yêu
    // cầu (fail-closed).
    const MAX_NAME_RETRIES: u32 = 3;
    for attempt in 1..=MAX_NAME_RETRIES {
        let name = mgc_ui::prompt::input(&format!(
            "Project name (attempt {attempt}/{}):",
            MAX_NAME_RETRIES
        ))
        .unwrap_or_default();
        if crate::commands::core::create::validate_project_name(&name).is_ok() {
            return Ok(name);
        }
        mgc_ui::warning(
            "Invalid project name: use a single path segment (no '/', '\\', '..', \
             absolute paths). Try again.",
        );
    }
    Err(crate::error::invalid_project_name_retries_exhausted(
        MAX_NAME_RETRIES,
    ))
}

/// Resolve a project name non-interactively for CI.
/// Giải tên dự án theo cách không tương tác cho CI.
///
/// Precedence: an explicit `--name` wins and is validated exactly like the
/// wizard but FAILS FAST (no re-prompt) so a bad flag surfaces a clear error.
/// With only `--yes`, the name is the current directory basename (a
/// deterministic, non-interactive default) and is validated the same way.
/// Thứ tự ưu tiên: `--name` thắng và được xác thực như wizard nhưng FAIL FAST
/// (không hỏi lại) để flag sai trả lỗi rõ ràng. Chỉ có `--yes` thì tên lấy từ
/// basename thư mục hiện tại (mặc định tất định, không tương tác) và cũng được
/// xác thực tương tự.
pub(in crate::commands::init) fn resolve_non_interactive_name(
    name: Option<String>,
    yes: bool,
) -> Result<Option<String>> {
    let candidate = match name {
        // Explicit flag: validate and fail fast, never re-prompt.
        // Cờ tường minh: xác thực và fail fast, không hỏi lại.
        Some(explicit) => explicit,
        None if yes => {
            // `--yes` without `--name`: derive from cwd basename.
            // `--yes` không có `--name`: lấy từ basename thư mục hiện tại.
            let cwd = std::env::current_dir().map_err(|e| crate::error::cwd_deleted(&e))?;
            match cwd.file_name().and_then(|segment| segment.to_str()) {
                Some(basename) if !basename.is_empty() => basename.to_string(),
                _ => {
                    return Err(crate::error::invalid_project_name(
                        &cwd.display().to_string(),
                    ));
                }
            }
        }
        None => return Ok(None),
    };
    crate::commands::core::create::validate_project_name(&candidate)?;
    Ok(Some(candidate))
}
