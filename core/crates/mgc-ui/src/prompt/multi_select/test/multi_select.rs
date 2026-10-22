//! Multi-selection prompt tests — kiểm thử prompt chọn nhiều.
//! Checks use the same label builder passed to dialoguer in production.
//! Kiểm tra dùng builder label mà production truyền cho dialoguer.

use super::multi_select::{multi_select, styled_options};

#[test]
fn styled_options_keep_checkbox_and_choice_text() {
    let options = styled_options(&["alpha"]);
    assert_eq!(options.len(), 1);
    assert!(options[0].contains("☐"));
    assert!(options[0].contains("alpha"));
}

#[test]
fn multi_select_keeps_its_anyhow_index_list_result() {
    let _select: fn(&str, &[&str]) -> anyhow::Result<Vec<usize>> = multi_select;
}
