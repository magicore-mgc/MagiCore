//! Try the framework's own command-line initializer before anything else.
//! Thử initializer CLI của chính framework trước mọi nguồn khác.
//!
//! For game, iot, clo and cicd the framework publishes a scaffolding program.
//! Running it is the product path; a local project of the same framework is the
//! fallback, and failing closed is the last resort. Ordering matters: trying the
//! fallback first would hide a broken initializer behind a stale local project.
//! Với game, iot, clo và cicd, framework phát hành chương trình scaffold. Chạy
//! nó là đường sản phẩm; project local cùng framework là dự phòng, fail-closed
//! là cuối cùng. Thứ tự quan trọng: thử dự phòng trước sẽ che initializer hỏng
//! bằng một project local cũ.
//!
//! Returns `Ok(true)` when it scaffolded the project, `Ok(false)` when the
//! framework has no command-line initializer and the caller should continue.
//! Trả `Ok(true)` khi đã scaffold, `Ok(false)` khi framework không có initializer
//! CLI và caller nên tiếp tục.

use std::path::Path;

use anyhow::Result;

#[cfg(test)]
use crate::scaffold::core_initializers::CoreInitializer;
use crate::scaffold::core_initializers::{RECORDS, run_host_tool};

/// Run the initializer for `ecosystem`/`framework` into `target` when one is
/// registered.
/// Chạy initializer cho `ecosystem`/`framework` vào `target` khi có đăng ký.
pub(crate) fn try_run(ecosystem: &str, framework: &str, target: &Path) -> Result<bool> {
    let Some(record) = RECORDS
        .iter()
        .find(|record| record.ecosystem == ecosystem && record.framework == framework)
    else {
        return Ok(false);
    };
    // Non-Web cores outside Web/Cargo have no local-project fallback that can
    // recreate framework files, so an unusable toolchain is reported here.
    // Core ngoài Web/Cargo không có dự phòng project local tái tạo được file
    // framework, nên toolchain không dùng được báo ngay tại đây.
    // Make the destination absolute before the tool runs: a name-taking
    // scaffolder runs from the parent directory and an empty/relative cwd made
    // the traversal guard fail with an unreadable path error.
    // (Đưa destination về tuyệt đối trước khi chạy tool: scaffolder nhận tên
    // chạy từ thư mục cha, và cwd rỗng/tương đối khiến traversal guard lỗi.)
    let target = std::path::absolute(target)?;
    run_host_tool(record, &target)?;
    Ok(true)
}

/// The registered initializer for a pair, if any.
/// Initializer đã đăng ký cho một cặp, nếu có.
#[cfg(test)]
pub(crate) fn record_for(ecosystem: &str, framework: &str) -> Option<&'static CoreInitializer> {
    RECORDS
        .iter()
        .find(|record| record.ecosystem == ecosystem && record.framework == framework)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
