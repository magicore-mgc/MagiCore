"""MessageMacroExclusion tests, kept adjacent to the scanner contract."""

from .support import *

class MessageMacroExclusion(unittest.TestCase):
    """Bare literals inside message macros are prose, not spawns."""

    def test_tool_name_inside_format_is_not_a_finding(self):
        findings = scan_snippet(
            "adapters/web/src/audit.rs",
            'pub async fn run_audit(project_root: &Path) -> MgResult<AuditReport> {\n'
            '    return Err(MgError::Other(format!(\n'
            '        "rival lockfile(s) detected — run `mgc import {}",\n'
            '        if rival.iter().any(|r| r.contains("deno")) {\n'
            '            "deno"\n'
            '        } else {\n'
            '            "bun"\n'
            '        }\n'
            '    )));\n'
            '}\n',
        )
        tools = [f["tool"] for f in findings]
        self.assertNotIn("deno", tools)
        self.assertNotIn("bun", tools)

    def test_escaped_quote_inside_format_does_not_break_span(self):
        spans = gate._message_spans(
            'format!("say \\"hi\\" {}",\n    "deno"\n)'
        )
        self.assertEqual(len(spans), 1)
        self.assertTrue(gate._inside_spans(spans, 30))

    def test_bare_table_literal_outside_message_still_flagged(self):
        findings = scan_snippet(
            "cli/src/commands/core/install/app.rs",
            'fn tool_command() -> Vec<(&str, Vec<String>)> {\n'
            '    vec![\n'
            '        ("uv", vec!["sync".to_string()]),\n'
            '    ]\n'
            '}\n',
        )
        self.assertTrue(
            any(f["tool"] == "uv" for f in findings),
            "tool-table literals outside messages must stay measured",
        )

    def test_call_shaped_spawn_inside_message_still_flagged(self):
        # A call-shaped spawn can never hide behind a message macro: the
        # call patterns stay active inside message spans by design.
        findings = scan_snippet(
            "adapters/lib/src/install/mod.rs",
            'pub async fn run_install() -> MgResult<()> {\n'
            '    let msg = format!("about to run {}", "cargo");\n'
            '    let _ = mgc_exec::run::run("cargo", &args, &opts)?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertTrue(
            any(f["tool"] == "cargo" for f in findings),
            "call-shaped spawns must match even near message macros",
        )

__all__ = ['MessageMacroExclusion']
