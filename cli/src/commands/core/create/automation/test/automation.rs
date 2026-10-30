// Automation resolution tests — verify non-interactive create inputs.
// Test phân giải tự động hoá — kiểm tra đầu vào create không tương tác.
use super::*;

fn flags(yes: bool, name: Option<&str>) -> CreateAutomationFlags {
    CreateAutomationFlags {
        yes,
        name: name.map(str::to_string),
    }
}

/// `--name` wins over the positional argument.
/// `--name` thắng tham số vị trí.
#[test]
fn name_flag_overrides_positional() {
    let resolved = resolve_automation(
        "game",
        "bevy",
        "positional-name",
        &flags(true, Some("flag-name")),
    )
    .expect("resolved automation");
    assert_eq!(resolved.project_name, "flag-name");
}

/// The positional name is used when `--name` is absent.
/// Dùng tên vị trí khi không có `--name`.
#[test]
fn positional_name_is_used_when_name_flag_absent() {
    let resolved = resolve_automation("game", "bevy", "positional-name", &flags(false, None))
        .expect("resolved");
    assert_eq!(resolved.project_name, "positional-name");
}

/// Without any name and without `--yes` the wizard owns the interactive fallback.
/// Không tên và không `--yes` thì wizard đảm nhận nhánh tương tác.
#[test]
fn empty_name_without_yes_defers_to_wizard() {
    let resolved = resolve_automation("game", "bevy", "", &flags(false, None)).expect("resolved");
    assert!(resolved.project_name.is_empty());
}

/// `--yes` with no name fails closed: a non-interactive run cannot invent a directory.
/// `--yes` không tên thì fail-closed: chạy không tương tác không thể tự bịa thư mục.
#[test]
fn yes_without_any_name_fails_closed() {
    let error = resolve_automation("game", "bevy", "", &flags(true, None))
        .expect_err("must reject missing name");
    assert!(error.to_string().contains("--yes requires a project name"));
}

/// `--yes` with no framework fails closed on the ambiguous sub-type.
/// `--yes` không framework thì fail-closed vì sub-type mơ hồ.
#[test]
fn yes_without_framework_fails_closed() {
    let error = resolve_automation("game", "", "ci-game", &flags(true, Some("ci-game")))
        .expect_err("must reject ambiguous framework");
    assert!(error.to_string().contains("requires an explicit framework"));
}

/// `--name` alone still resolves a concrete project name (interactive mode kept).
/// Chỉ `--name` vẫn phân giải ra tên project cụ thể (giữ chế độ tương tác).
#[test]
fn name_without_yes_still_resolves_name() {
    let resolved =
        resolve_automation("game", "bevy", "", &flags(false, Some("named"))).expect("resolved");
    assert_eq!(resolved.project_name, "named");
}
