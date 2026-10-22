"""ShellProcessInventory test slice 4; method bodies retained verbatim."""

from .support import *

class ShellProcessInventoryPart4(unittest.TestCase):
    def test_extensionless_absolute_script_command_is_not_skipped(self):
        findings = gate.scan_shell_text("scripts/runner.sh", "/tmp/hook\n")
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["executable"], "<dynamic-shell-command>")
        self.assertEqual(findings[0]["review_status"], "unreviewed")

    def test_source_anchored_alias_requires_a_scanned_script_target(self):
        source = (
            'SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"\n'
            '"$SCRIPT_DIR/hook"\n'
            '"$SCRIPT_DIR/hook.SH"\n'
        )
        findings = gate.scan_shell_text(
            "scripts/runner.sh", source, {"scripts/known.sh"}
        )
        self.assertEqual(
            [item["executable"] for item in findings],
            ["<dynamic-shell-command>", "<dynamic-shell-command>"],
        )
        self.assertTrue(all(item["review_status"] == "unreviewed" for item in findings))

    def test_quoted_heredoc_text_does_not_redefine_trap_callback(self):
        source = (
            "cleanup() {\n"
            "  :\n"
            "}\n"
            "trap cleanup EXIT\n"
            "cat <<'EOF'\n"
            "cleanup() { ./evil; }\n"
            "EOF\n"
        )
        findings = gate.scan_shell_text("scripts/trap.sh", source)
        self.assertTrue(
            any(item["executable"] == "shell-trap-static" for item in findings)
        )
        self.assertFalse(
            any(item["executable"] == "<dynamic-shell-command>" for item in findings)
        )

    def test_unset_trap_function_invalidates_static_callback_review(self):
        source = (
            "cleanup() {\n"
            "  :\n"
            "}\n"
            "trap cleanup EXIT\n"
            "unset -f cleanup\n"
        )
        findings = gate.scan_shell_text("scripts/trap.sh", source)
        trap_findings = [item for item in findings if item["executable"] == "shell-trap-static"]
        self.assertEqual(len(trap_findings), 0)
        self.assertTrue(any(item["executable"] == "<dynamic-shell-command>" for item in findings))

        redefined = gate.scan_shell_text(
            "scripts/trap.sh",
            "cleanup() {\n  :\n}\n"
            "trap cleanup EXIT\n"
            "cleanup() {\n  ./later-hook\n}\n",
        )
        self.assertFalse(
            any(item["executable"] == "shell-trap-static" for item in redefined)
        )

    def test_trailing_shell_continuation_is_unparsed_not_clean(self):
        findings = gate.scan_shell_text("scripts/broken.sh", "cargo fetch \\\n")
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["executable"], "<unparsed-shell>")


__all__ = ['ShellProcessInventoryPart4']
