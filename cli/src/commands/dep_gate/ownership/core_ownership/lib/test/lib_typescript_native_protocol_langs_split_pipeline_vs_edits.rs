//! Existing Lib lifecycle ownership test — test lifecycle Lib hiện có.
//! Its assertion set runs in the original order through focused helpers.
//! Chạy toàn bộ assertion theo thứ tự gốc qua helper theo chủ đề.

#[path = "lib_native_mutation_assertions.rs"]
mod mutation_assertions;
#[path = "lib_protocol_pipeline_assertions.rs"]
mod protocol_assertions;
#[path = "lib_tool_argument_assertions.rs"]
mod tool_argument_assertions;

#[test]
fn lib_typescript_native_protocol_langs_split_pipeline_vs_edits() {
    protocol_assertions::assert_protocol_pipeline();
    mutation_assertions::assert_native_mutations();
    tool_argument_assertions::assert_tool_arguments_do_not_authorize_compat();
}
