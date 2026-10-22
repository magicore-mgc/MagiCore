use crate::allowlist::*;

#[test]
fn rejects_shell_control_in_simple_script() {
    for script in [
        "node build.js && node post.js",
        "node build.js; rm -rf dist",
        "echo hi > out.txt",
        "node $(which build)",
        "node `which build`",
        "node build.js\nnode post.js",
    ] {
        let err = parse_simple_script(script).unwrap_err();
        assert!(
            err.to_string().contains("unsupported shell control"),
            "unexpected error for {script}: {err}"
        );
    }
}
