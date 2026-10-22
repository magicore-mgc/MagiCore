"""PowerShellProcessInventory tests, kept adjacent to the scanner contract."""

from .support import *

class PowerShellProcessInventory(unittest.TestCase):
    """PowerShell process boundaries are not hidden from the gate."""

    def test_detects_process_apis_and_native_package_commands(self):
        findings = gate.scan_powershell_text(
            "scripts/setup.ps1",
            "Start-Process npm -ArgumentList 'install'\n"
            "& $PackageManager install\n"
            "cargo fetch\n",
        )
        self.assertEqual(
            [(item["line"], item["api"]) for item in findings],
            [(1, "Start-Process"), (2, "call-operator"), (3, "native-command")],
        )
        self.assertTrue(all(item["review_status"] == "unreviewed" for item in findings))

    def test_ignores_comments_and_quoted_examples(self):
        findings = gate.scan_powershell_text(
            "scripts/example.ps1",
            "# Start-Process npm\n"
            "# saps npm install\n"
            "$example = 'iex cargo fetch'\n"
            "$example = 'cargo fetch'\n",
        )
        self.assertEqual(findings, [])

    def test_finds_quoted_executable_invoked_with_call_operator(self):
        findings = gate.scan_powershell_text(
            "scripts/quoted.ps1",
            "& 'npm' install\n",
        )
        self.assertEqual([(item["api"], item["executable"]) for item in findings], [("call-operator", "npm")])

    def test_source_bound_review_is_invalidated_by_any_file_edit(self):
        path = "scripts/ci/stress/toolchains/godot/install_windows.ps1"
        with open(os.path.join(REPO_ROOT, path), encoding="utf-8") as handle:
            source = handle.read()

        findings = gate.scan_powershell_text(path, source)
        self.assertTrue(findings)
        self.assertTrue(all(item["review_status"] == "reviewed" for item in findings))

        changed = gate.scan_powershell_text(path, source + "\n# changed\n")
        self.assertTrue(changed)
        self.assertTrue(all(item["review_status"] == "unreviewed" for item in changed))

    def test_detects_builtin_process_aliases(self):
        findings = gate.scan_powershell_text(
            "scripts/aliases.ps1",
            "start npm install\n"
            "saps cargo -ArgumentList 'fetch'\n"
            "iex $commandText\n"
            "icm $hostName { go mod download }\n"
            "sajb { pnpm install }\n",
        )
        self.assertEqual(
            [(item["line"], item["api"], item["executable"]) for item in findings],
            [
                (1, "process-alias", "Start-Process"),
                (2, "process-alias", "Start-Process"),
                (3, "process-alias", "Invoke-Expression"),
                (4, "native-command", "go"),
                (4, "process-alias", "Invoke-Command"),
                (5, "native-command", "pnpm"),
                (5, "process-alias", "Start-Job"),
            ],
        )

__all__ = ['PowerShellProcessInventory']
