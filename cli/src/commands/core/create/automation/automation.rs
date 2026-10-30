// Automation flags + resolution for `create-<core>` — nhận cờ + phân giải cho `create-<core>`.
// Shared by every per-core create lane so CI can scaffold without a TTY.
// Dùng chung cho mọi lane create theo core để CI scaffold không cần TTY.

use anyhow::Result;
use clap::Args;

/// Non-interactive automation flags for `create-<core>`.
/// Cờ tự động hoá không tương tác cho `create-<core>`.
///
/// `--yes` skips every prompt; `--name` supplies the project directory so the
/// positional PROJECT argument becomes optional in automation.
/// (`--yes` bỏ mọi prompt; `--name` cung cấp thư mục project để tham số PROJECT
/// vị trí trở nên tùy chọn khi tự động hoá.)
#[derive(Debug, Clone, Args, Default)]
pub struct CreateAutomationFlags {
    #[arg(short = 'y', long, help = "Skip prompts, use defaults")]
    pub yes: bool,
    #[arg(long, help = "Project directory name (non-interactive)")]
    pub name: Option<String>,
}

/// Effective, resolved create inputs after applying automation flags.
/// Đầu vào create đã phân giải sau khi áp cờ tự động hoá.
#[derive(Debug)]
pub struct ResolvedAutomation {
    /// Project directory/name to scaffold into.
    /// Tên/thư mục project để scaffold.
    pub project_name: String,
}

/// Resolve the effective project name and interaction mode.
///
/// `--name` wins over the positional PROJECT argument; with neither, an empty
/// name falls through to the wizard only when not `--yes`. When `--yes` is set
/// without a name the call fails closed, because a non-interactive scaffold
/// cannot invent a directory.
/// (`--name` thắng tham số PROJECT vị trí; nếu cả hai trống, tên rỗng chỉ rơi
/// vào wizard khi KHÔNG `--yes`. Khi `--yes` mà thiếu tên thì fail-closed vì
/// scaffold không tương tác không thể tự bịa thư mục.)
pub fn resolve_automation(
    core: &str,
    framework: &str,
    positional_name: &str,
    flags: &CreateAutomationFlags,
) -> Result<ResolvedAutomation> {
    let flag_name = flags
        .name
        .as_deref()
        .map(str::trim)
        .filter(|name| !name.is_empty());
    let positional = positional_name.trim();

    let project_name = match (flag_name, positional.is_empty()) {
        (Some(name), _) => name.to_string(),
        (None, false) => positional.to_string(),
        (None, true) => {
            if flags.yes {
                anyhow::bail!(
                    "create-{core} --yes requires a project name: pass --name <NAME> or a PROJECT argument"
                );
            }
            // No name and no --yes: the wizard owns the interactive fallback.
            // Không tên và không --yes: wizard đảm nhận nhánh tương tác.
            String::new()
        }
    };

    // A `--yes` run with no framework cannot pick a sub-type without a prompt.
    // Reject explicitly instead of hanging on an interactive wizard in CI.
    // Lần chạy `--yes` không có framework không thể chọn sub-type mà không hỏi.
    // Từ chối rõ ràng thay vì treo ở wizard tương tác trong CI.
    if flags.yes && framework.trim().is_empty() {
        anyhow::bail!(
            "create-{core} --yes requires an explicit framework: the sub-type cannot be chosen non-interactively"
        );
    }

    Ok(ResolvedAutomation { project_name })
}

#[cfg(test)]
#[path = "test/automation.rs"]
mod tests;
