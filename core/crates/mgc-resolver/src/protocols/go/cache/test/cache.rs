use super::super::*;

#[test]
fn module_path_escape() {
    assert_eq!(
        escape_module_path("github.com/BurntSushi/toml"),
        "github.com/!burnt!sushi/toml"
    );
    assert_eq!(escape_module_path("golang.org/x/text"), "golang.org/x/text");
}
