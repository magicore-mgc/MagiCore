use super::super::*;

#[test]
fn go_mod_parse_blocks_and_directives() {
    let gomod = parse_go_mod(
        "module example.com/m\n\ngo 1.22\n\nrequire (\n\tgithub.com/pkg/errors v0.9.1 // indirect\n\tgolang.org/x/text v0.14.0\n)\n\nreplace github.com/old/pkg => github.com/new/pkg v1.2.3\n\nexclude github.com/bad/pkg v0.1.0\n",
    )
    .expect("test operation must succeed");
    assert_eq!(gomod.requires.len(), 2);
    assert_eq!(gomod.requires[0].path, "github.com/pkg/errors");
    assert_eq!(gomod.requires[0].version, "0.9.1");
    assert!(gomod.requires[0].indirect);
    assert!(!gomod.requires[1].indirect);
    assert_eq!(gomod.replaces.len(), 1);
    assert_eq!(gomod.replaces[0].old_path, "github.com/old/pkg");
    assert_eq!(gomod.replaces[0].new_path, "github.com/new/pkg");
    assert_eq!(gomod.replaces[0].new_version.as_deref(), Some("1.2.3"));
    assert_eq!(
        gomod.excludes,
        vec![("github.com/bad/pkg".to_string(), "0.1.0".to_string())]
    );
}

#[test]
fn quoted_require_paths_are_unquoted() {
    // gopkg.in/yaml.v3 quotes its module paths — legal go.mod that
    // must not leak quotes into registry URLs (404s).
    // (Path có quote trong go.mod là hợp lệ — phải cắt quote.)
    let pom = "module \"example.com/m\"\n\ngo 1.21\n\nrequire (\n\t\"gopkg.in/check.v1\" v0.0.0-20161208181325-20d25e280405\n)\n";
    let file = parse_go_mod(pom).expect("test operation must succeed");
    assert_eq!(file.requires.len(), 1);
    assert_eq!(file.requires[0].path, "gopkg.in/check.v1");
    assert_eq!(
        file.requires[0].version,
        "0.0.0-20161208181325-20d25e280405"
    );
}

#[test]
fn go_mod_single_line_directives() {
    let gomod = parse_go_mod(
        "module m\nrequire golang.org/x/sys v0.15.0 // indirect\nreplace a/b v1.0.0 => c/d\nexclude e/f v2.0.0\n",
    )
    .expect("test operation must succeed");
    assert_eq!(gomod.requires.len(), 1);
    assert!(gomod.requires[0].indirect);
    assert_eq!(gomod.replaces[0].old_version.as_deref(), Some("1.0.0"));
    assert_eq!(gomod.replaces[0].new_version, None);
    assert_eq!(gomod.excludes.len(), 1);
}
