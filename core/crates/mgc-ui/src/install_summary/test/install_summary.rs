//! Install-summary tests — kiểm thử summary cài đặt.
//! Assertions target the production row builder.
//! Assertion kiểm tra builder dòng mà production sử dụng.

use super::install_summary::install_summary_lines;

#[test]
fn summary_rows_keep_counts_savings_and_optional_cache_source() {
    let with_source = install_summary_lines(4, 2, 51, "8 MB", Some("shared mgc store"));
    assert!(
        with_source
            .iter()
            .any(|line| line.contains("4 packages installed"))
    );
    assert!(with_source.iter().any(|line| line.contains("2 from cache")));
    assert!(with_source.iter().any(|line| line.contains("51 ms total")));
    assert!(with_source.iter().any(|line| line.contains("8 MB")));
    assert!(
        with_source
            .iter()
            .any(|line| line.contains("shared mgc store"))
    );

    let without_source = install_summary_lines(1, 0, 0, "0 B", None);
    assert!(
        !without_source
            .iter()
            .any(|line| line.contains("cache source:"))
    );
}
