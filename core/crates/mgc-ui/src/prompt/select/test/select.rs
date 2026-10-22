//! Single-selection prompt tests — kiểm thử prompt chọn một.
//! These checks cover the production option builder and API signature.
//! Các kiểm tra này bao phủ builder production và chữ ký API.

use super::select::{select, styled_options};

#[test]
fn styled_options_keep_each_choice_text() {
    let options = styled_options(&["alpha", "beta"]);
    assert_eq!(options.len(), 2);
    assert!(options[0].contains("alpha"));
    assert!(options[1].contains("beta"));
}

#[test]
fn select_keeps_its_anyhow_index_result() {
    let _select: fn(&str, &[&str]) -> anyhow::Result<usize> = select;
}
