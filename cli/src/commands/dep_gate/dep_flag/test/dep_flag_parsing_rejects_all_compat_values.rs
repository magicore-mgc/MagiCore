//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn dep_flag_parsing_rejects_all_compat_values() {
    assert!(from_dep_flag(None).is_ok());
    for tool in [
        "uv",
        "cargo",
        "terraform",
        "dotnet",
        "mvn",
        "bun",
        "deno",
        "not-a-tool",
    ] {
        let error = from_dep_flag(Some(tool)).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("disabled for dependency operations")
        );
    }
}
