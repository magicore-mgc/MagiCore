//! Environment quote edge-case test.
//! (Kiểm thử biên xử lý quote environment.)

use crate::commands::optimizer::env_loader::parse_env_file;
use std::fs;
use tempfile::TempDir;

#[test]
fn trailing_quote_without_an_opening_quote_is_preserved() {
    let temp = TempDir::new().unwrap();
    let env_file = temp.path().join("literal-quote.env");
    fs::write(&env_file, "LITERAL=value\"\n").unwrap();

    let vars = parse_env_file(&env_file).unwrap();
    assert_eq!(vars.get("LITERAL"), Some(&"value\"".to_string()));
}
