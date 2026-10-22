//! Interactive project-name validation and bounded retries.
//! Nhận và xác thực tên dự án với số lần thử giới hạn.

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
