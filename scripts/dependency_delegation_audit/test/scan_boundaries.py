"""ScanBoundaryCoverage tests, kept adjacent to the scanner contract."""

from .support import *

class ScanBoundaryCoverage(unittest.TestCase):
    """Every dependency lane and claimed core-owned tool flow is scanned."""

    def test_complete_cli_command_tree_is_in_dependency_scan_roots(self):
        for root in ("adapters", "cli", "core", "tests", "tools"):
            self.assertIn(
                root,
                gate.SCAN_ROOTS,
                f"the process-boundary gate must scan first-party Rust root {root}",
            )
        repo_root = REPO_ROOT
        scanned = {
            os.path.relpath(path, repo_root).replace(os.sep, "/")
            for path in gate._iter_rust_files()
        }
        self.assertIn("cli/src/main.rs", scanned)
        self.assertIn("cli/src/commands/python_runtime.rs", scanned)
        self.assertIn("cli/tests/native_engine_spawn_audit.rs", scanned)
        self.assertIn("core/crates/mgc-exec/src/run/run.rs", scanned)
        self.assertIn("core/crates/mgc-resolver/src/protocols/swift/registry/registry.rs", scanned)
        self.assertIn("tools/mgc-dist/src/main.rs", scanned)
        python_scanned = {
            os.path.relpath(path, repo_root).replace(os.sep, "/")
            for path in gate._iter_python_files()
        }
        for root in (
            ".",
        ):
            self.assertIn(root, gate.PYTHON_SCAN_ROOTS)
        self.assertIn("scripts/lifecycle_capability_matrix.py", python_scanned)
        self.assertIn("benchmark/scripts/bench_v2.py", python_scanned)
        shell_scanned = {
            os.path.relpath(path, repo_root).replace(os.sep, "/")
            for path in gate._iter_shell_files()
        }
        self.assertIn("CHECK_PROGRESS.sh", shell_scanned)
        self.assertIn("reproduce.sh", shell_scanned)
        self.assertFalse(any("gitnexus" in path.lower() for path in shell_scanned))
        self.assertFalse(any("gitnexus" in path.lower() for path in python_scanned))
        self.assertIn("tools/mgc-mcp/mcp_server.py", python_scanned)
        shell_scanned = {
            os.path.relpath(path, repo_root).replace(os.sep, "/")
            for path in gate._iter_shell_files()
        }
        self.assertIn("scripts/install-from-gh.sh", shell_scanned)
        self.assertIn("cli/tests/scripts/runtime_full_e2e.sh", shell_scanned)

    def test_project_kind_check_is_not_a_spawn(self):
        findings = scan_snippet(
            "cli/src/commands/core/install/clo.rs",
            'fn install() -> Result<()> {\n'
            '    let cloud_kind = "terraform";\n'
            '    if cloud_kind == "terraform" { return Err(err()); }\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(findings, [])

    def test_deployment_tool_is_not_dependency_delegation(self):
        findings = scan_snippet(
            "adapters/cloud/src/deploy/mod.rs",
            'pub async fn deploy() {\n'
            '    mgc_run("terraform", &["apply"], &opts);\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["op_class"], "deploy")
        self.assertEqual(findings[0]["status"], "violation")

__all__ = ['ScanBoundaryCoverage']
