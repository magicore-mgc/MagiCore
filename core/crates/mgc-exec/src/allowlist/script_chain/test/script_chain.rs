#[test]
fn and_chain_preserves_quoted_and_escaped_arguments() {
    use crate::allowlist::parse_script_chain;
    for script in [
        r#"vite build --base "dist&&preview" && tsc --noEmit"#,
        r"vite build --base dist\&\&preview && tsc --noEmit",
    ] {
        let commands = parse_script_chain(script).unwrap();
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].args.last().unwrap(), "dist&&preview");
        assert_eq!(commands[1].program, "tsc");
    }
    for script in [
        "vite &&",
        "&& vite",
        "vite || tsc",
        "vite; tsc",
        "vite && 'tsc",
        "vite && tsc\\",
    ] {
        assert!(
            parse_script_chain(script).is_err(),
            "invalid chain accepted: {script}"
        );
    }
}
