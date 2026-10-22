"""Focused WindowsMatrixRuntime test owner.

Owner test hẹp cho WindowsMatrixRuntime.
"""

from .support import (
    Path,
    REPO_ROOT,
    matrix_module,
    os,
    patch,
    sys,
    tempfile,
    unittest,
)

class WindowsMatrixRuntime(unittest.TestCase):
    def test_swift_windows_lifecycle_uses_python_310_without_changing_other_lanes(self):
        lane = {"core": "app", "language": "swift"}
        lane_env = {"PATH": f"/host/python312{os.pathsep}/host/tools"}

        with tempfile.TemporaryDirectory() as root:
            swift_python = Path(root) / "python.exe"
            swift_python.touch()
            with patch.dict(
                os.environ,
                {
                    "GITHUB_ACTIONS": "true",
                    "MGC_SWIFT_PYTHON_BIN": str(Path(root)),
                },
            ):
                for step in ("install", "test", "build"):
                    step_env = matrix_module.swift_toolchain_environment(
                        lane, step, lane_env, windows=True
                    )
                    self.assertEqual(
                        step_env["PATH"],
                        f"{Path(root)}{os.pathsep}{lane_env['PATH']}",
                    )
                self.assertIs(
                    matrix_module.swift_toolchain_environment(
                        lane, "create", lane_env, windows=True
                    ),
                    lane_env,
                )

            self.assertEqual(lane_env["PATH"], f"/host/python312{os.pathsep}/host/tools")
            self.assertIs(
                matrix_module.swift_toolchain_environment(
                    {"core": "lib", "language": "python"},
                    "test",
                    lane_env,
                    windows=True,
                ),
                lane_env,
            )

    def test_swift_windows_lifecycle_fails_closed_without_python_310_in_ci(self):
        with patch.dict(
            os.environ,
            {"GITHUB_ACTIONS": "true", "MGC_SWIFT_PYTHON_BIN": ""},
        ):
            with self.assertRaisesRegex(RuntimeError, "requires the Python 3.10 path"):
                matrix_module.swift_toolchain_environment(
                    {"core": "app", "language": "swift"},
                    "test",
                    {"PATH": "/host/python312"},
                    windows=True,
                )

    def test_captured_child_output_replaces_bytes_outside_windows_ansi_codepage(self):
        result = matrix_module.run_text_capture(
            [
                sys.executable,
                "-c",
                "import sys; sys.stdout.buffer.write(bytes([0x90]) + b'ok'); "
                "sys.stderr.buffer.write(bytes([0x90]) + b'err')",
            ],
            capture_output=True,
        )
        self.assertEqual(result.stdout, "\ufffdok")
        self.assertEqual(result.stderr, "\ufffderr")

    def test_gitignore_excludes_python_bytecode_from_matrix_provenance(self):
        ignore_rules = (
            REPO_ROOT / ".gitignore"
        ).read_text(encoding="utf-8")
        self.assertIn("\n__pycache__/\n", ignore_rules)
        self.assertIn("\n*.py[cod]\n", ignore_rules)
