"""PythonProcessInventory test slice 3; method bodies retained verbatim."""

from .support import *

class PythonProcessInventoryPart3(unittest.TestCase):
    def test_import_aliases_and_os_shell_calls_are_reported(self):
        findings = gate.scan_python_text(
            "scripts/example.py",
            "import subprocess as sp\n"
            "from os import system as shell\n"
            "def run():\n"
            "    sp.run(['cargo', 'fetch'])\n"
            "    shell('pip install demo')\n",
        )
        self.assertEqual(
            [(item["line"], item["api"], item["executable"]) for item in findings],
            [
                (4, "subprocess.run", "cargo"),
                (5, "os.system", "<shell-command>"),
            ],
        )
        self.assertTrue(all(item["review_status"] == "unreviewed" for item in findings))

    def test_subprocess_shell_command_is_not_guessed_as_executable(self):
        findings = gate.scan_python_text(
            "scripts/example.py",
            "import subprocess\n"
            "subprocess.run('go test ./...', shell=True)\n",
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["executable"], "<shell-command>")

    def test_comments_and_non_execution_strings_are_ignored(self):
        findings = gate.scan_python_text(
            "scripts/example.py",
            "# subprocess.run(['cargo', 'fetch'])\n"
            "message = \"os.system('pip install demo')\"\n",
        )
        self.assertEqual(findings, [])

    def test_dynamic_process_target_is_not_dropped(self):
        findings = gate.scan_python_text(
            "tools/tool.py",
            "import subprocess\n"
            "def invoke(argv):\n"
            "    subprocess.Popen(argv)\n",
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["executable"], "<dynamic>")
        self.assertEqual(findings[0]["api"], "subprocess.Popen")

    def test_python_process_review_matches_exact_call_ast(self):
        approved = gate.scan_python_text(
            "scripts/audit/audit_capability_matrix.py",
            "import subprocess\n"
            "subprocess.run(\n"
            '    ["git", "status", "--porcelain", "--untracked-files=all"],\n'
            "    cwd=str(root), capture_output=True, text=True, check=False,\n"
            ")\n",
        )
        reviewed = gate.review_python_process_calls(approved, expected_files=set())
        self.assertEqual(len(reviewed), 1)
        self.assertEqual(reviewed[0]["review_status"], "reviewed")
        self.assertIn("fixed git arguments", reviewed[0]["review_reason"])

        changed = gate.scan_python_text(
            "scripts/audit/audit_capability_matrix.py",
            "import subprocess\n"
            "subprocess.run(\n"
            '    ["git", "status", "--porcelain", "--ignored", "--untracked-files=all"],\n'
            "    cwd=str(root), capture_output=True, text=True, check=False,\n"
            ")\n",
        )
        unreviewed = gate.review_python_process_calls(changed, expected_files=set())
        self.assertEqual(unreviewed[0]["review_status"], "unreviewed")
        self.assertIsNone(unreviewed[0]["review_reason"])

    def test_removed_python_process_call_leaves_a_stale_review_blocker(self):
        stale = gate.review_python_process_calls(
            [], expected_files={"scripts/audit/audit_capability_matrix.py"}
        )
        self.assertEqual(len(stale), 3)
        self.assertTrue(all(item["review_status"] == "stale-review" for item in stale))

    def test_invalid_python_is_reported_as_unparsed_not_clean(self):
        findings = gate.scan_python_text("scripts/broken.py", "def broken(:\n")
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["executable"], "<unparsed-python>")
        self.assertIsNotNone(findings[0]["parse_error"])

    def test_process_module_star_import_is_unreviewed(self):
        findings = gate.scan_python_text(
            "scripts/star.py",
            "from subprocess import *\n"
            "def invoke():\n"
            "    Popen(['cargo', 'fetch'])\n",
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["api"], "<wildcard-import:subprocess>")
        self.assertEqual(findings[0]["executable"], "<dynamic>")


__all__ = ['PythonProcessInventoryPart3']
