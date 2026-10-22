// Go module requirement parser integration test.
// Kiểm thử integration parser go.mod.

/// go.mod require parsing: block + single-line requires become Go pins;
/// toolchain/replace/exclude lines are skipped honestly (recorded, never
/// silently dropped into a clean count).
/// Parse require go.mod: block + đơn dòng thành ghim Go; dòng
/// toolchain/replace/exclude được skip có ghi nhận.
#[test]
fn read_go_mod_requires_parses_pins_and_skips_honestly() {
    let raw = "module example.com/x\n\ngo 1.21\n\ntoolchain go1.21.0\n\nrequire (\n\tgolang.org/x/text v0.14.0\n\tgithub.com/foo/bar v1.2.3 // indirect\n)\n\nrequire github.com/baz v2.0.0\n\nreplace github.com/foo/bar => ../local\n\nexclude golang.org/x/text v0.13.0\n";
    let (pins, skipped) = mgc_audit::scanners::read_go_mod_requires(raw);
    assert_eq!(pins.len(), 3);
    assert_eq!(pins[0].name, "golang.org/x/text");
    assert_eq!(pins[0].version, "v0.14.0");
    assert!(pins.iter().all(|p| p.ecosystem == "Go"));
    assert!(
        skipped
            .iter()
            .any(|s| s.contains("replace") || s.contains("exclude") || s.contains("toolchain")),
        "non-require directives must be recorded as skipped, got: {skipped:?}"
    );
}
