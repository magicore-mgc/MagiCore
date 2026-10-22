use crate::policy::is_mutation_invocation;

// Verb tables: representative mutation probes per ecosystem.
// (Bảng động từ: probe mutation đại diện mỗi ecosystem.)
#[test]
fn mutation_verb_tables() {
    let cases = [
        ("pip", vec!["install", "x"], true),
        ("uv", vec!["sync"], true),
        ("cargo", vec!["add", "x"], true),
        ("cargo", vec!["build", "--locked", "--offline"], false),
        ("cargo", vec!["build"], true),
        ("pub", vec!["get"], true),
        ("composer", vec!["require", "x"], true),
        ("go", vec!["get", "x"], true),
        ("go", vec!["mod", "download"], true),
        ("go", vec!["mod", "verify"], true),
        ("deno", vec!["publish"], true),
        ("npm", vec!["--version"], false),
        ("npm", vec![], false),
        ("node", vec!["server.js"], false),
        ("zig", vec!["run", "probe.zig"], false),
        ("zig", vec!["version"], false),
        ("zig", vec!["fetch", "https://example.invalid/pkg"], true),
        ("zig", vec!["build"], true),
        ("zig", vec!["run", "probe.zig", "--fetch"], true),
    ];
    for (program, args, expected) in cases {
        let owned: Vec<String> = args.iter().map(|a| a.to_string()).collect();
        assert_eq!(
            is_mutation_invocation(program, &owned, &[]),
            expected,
            "program={program} args={args:?}"
        );
    }
}
