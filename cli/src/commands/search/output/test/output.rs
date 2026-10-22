//! Pin JSON and terminal presentation output.
//! Cố định output JSON và terminal.

use super::super::{SearchTextLine, emit_text_lines, render_json};
use crate::commands::search::types::SearchResult;

fn sample_result(description: &str) -> SearchResult {
    SearchResult {
        name: "pkg".to_string(),
        version: "1.0.0".to_string(),
        description: description.to_string(),
        score: "0.500".to_string(),
        core_label: "[Core: Web / Node]".to_string(),
    }
}

#[test]
fn preserves_pretty_json_field_order_and_values() {
    let json = render_json(1, vec![sample_result("description")]).unwrap();

    assert_eq!(
        json,
        concat!(
            "{\n",
            "  \"total\": 1,\n",
            "  \"results\": [\n",
            "    {\n",
            "      \"name\": \"pkg\",\n",
            "      \"version\": \"1.0.0\",\n",
            "      \"description\": \"description\",\n",
            "      \"score\": \"0.500\",\n",
            "      \"core_label\": \"[Core: Web / Node]\"\n",
            "    }\n",
            "  ]\n",
            "}"
        )
    );
}

#[test]
fn preserves_text_lines_and_pagination_order() {
    let mut rendered = Vec::new();
    emit_text_lines(
        45,
        20,
        &[sample_result("description")],
        Some(2),
        Some("web"),
        |line| {
            rendered.push(match line {
                SearchTextLine::Information(message) => Some(message),
                SearchTextLine::Blank => None,
            });
        },
    );

    assert_eq!(
        rendered,
        vec![
            Some("Found 1 result(s) (total: 45) [filter: --core web]:".to_string()),
            None,
            Some("  [Core: Web / Node] pkg@1.0.0".to_string()),
            Some("    description".to_string()),
            Some("Page 2 of ~3".to_string()),
        ]
    );
}
