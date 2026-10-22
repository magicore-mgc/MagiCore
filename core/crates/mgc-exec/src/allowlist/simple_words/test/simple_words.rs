use super::super::split_simple_words;

#[test]
fn lexer_keeps_quoted_words_and_unescapes_space_delimiters() {
    assert_eq!(
        split_simple_words(r#"node "script with space.js" path\ with\ spaces"#).unwrap(),
        vec![
            "node".to_string(),
            "script with space.js".to_string(),
            "path with spaces".to_string()
        ]
    );
    assert!(split_simple_words("node 'unfinished").is_err());
    assert!(split_simple_words("node trailing\\").is_err());
}
