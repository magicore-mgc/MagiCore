//! Optimizer `.env` parsing test.
//! (Kiểm thử parse file `.env` optimizer.)

use crate::commands::optimizer::env_loader::parse_env_file;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_parse_env_file() {
    let temp = TempDir::new().unwrap();
    let env_file = temp.path().join("test.env");

    fs::write(
        &env_file,
        r#"
# Comment line
KEY1=value1
KEY2="quoted value"
KEY3='single quoted'
EMPTY_KEY=

# Another comment
KEY4=value with spaces
"#,
    )
    .unwrap();

    let vars = parse_env_file(&env_file).unwrap();

    assert_eq!(vars.get("KEY1"), Some(&"value1".to_string()));
    assert_eq!(vars.get("KEY2"), Some(&"quoted value".to_string()));
    assert_eq!(vars.get("KEY3"), Some(&"single quoted".to_string()));
    assert_eq!(vars.get("EMPTY_KEY"), Some(&"".to_string()));
    assert_eq!(vars.get("KEY4"), Some(&"value with spaces".to_string()));
}
