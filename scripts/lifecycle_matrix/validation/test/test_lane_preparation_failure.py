"""Focused LanePreparationFailure test owner.

Owner test hẹp cho LanePreparationFailure.
"""

from .support import (
    STATUS_FAILED,
    STATUS_NATIVE,
    STATUS_UNSUPPORTED,
    STATUS_UNVERIFIED,
    os,
    patch,
    run_lane,
    subprocess,
    unittest,
)

class LanePreparationFailure(unittest.TestCase):
    def test_failed_create_preserves_unsupported_install_without_promoting_native(self):
        lane = {
            "core": "app", "language": "objc",
            "scaffold": ["create-app", "objc", "fixture"],
            "steps": [("install", ["install"])],
            "install_owner": "unsupported",
        }
        for owner, expected in (("unsupported", STATUS_UNSUPPORTED), ("native-engine", STATUS_UNVERIFIED)):
            with self.subTest(owner=owner):
                lane["install_owner"] = owner
                with patch("lifecycle_capability_matrix.subprocess.run", return_value=subprocess.CompletedProcess([], 1, "unsupported scaffold", "")):
                    result = run_lane("/tmp/fake-mgc", lane)
                self.assertEqual(result["dims"]["create"], STATUS_FAILED)
                self.assertEqual(result["dims"]["install"], expected)

    def test_failed_real_dependency_fixture_cannot_be_laundered_by_empty_install(self):
        calls = []

        def fake_run(argv, **kwargs):
            args = argv[1:]
            calls.append(args)
            if args[0] == "create-test":
                os.mkdir(os.path.join(kwargs["cwd"], args[-1]))
                return subprocess.CompletedProcess(argv, 0, "created", "")
            if args[0] == "add-ai":
                return subprocess.CompletedProcess(argv, 1, "", "registry unavailable")
            if args[0] == "optimizer":
                return subprocess.CompletedProcess(argv, 0, "Optimizer skipped", "")
            raise AssertionError(f"blocked lifecycle step was executed: {args}")

        lane = {
            "core": "ai",
            "language": "matrix-test-language",
            "scaffold": ["create-test", "fixture"],
            "pre_steps": ["mgc_add_real_dependency"],
            "dependency_fixture": {
                "command": "add-ai",
                "package": "six@1.17.0",
            },
            "steps": [
                ("install", ["install"]),
                ("test", ["test"]),
                ("build", ["build"]),
            ],
            "delegated": [],
            "install_owner": "native-engine",
        }

        with patch("lifecycle_capability_matrix.subprocess.run", side_effect=fake_run):
            result = run_lane("/tmp/fake-mgc", lane)

        self.assertEqual(result["dims"]["add"], STATUS_FAILED)
        self.assertEqual(result["dims"]["install"], STATUS_UNVERIFIED)
        self.assertEqual(result["dims"]["test"], STATUS_UNVERIFIED)
        self.assertEqual(result["dims"]["build"], STATUS_UNVERIFIED)
        self.assertNotIn(["install"], calls)

    def test_failed_mgc_python_tool_add_blocks_test_and_build(self):
        calls = []

        def fake_run(argv, **kwargs):
            args = argv[1:]
            calls.append(args)
            if args[0] == "create-test":
                os.mkdir(os.path.join(kwargs["cwd"], args[-1]))
                return subprocess.CompletedProcess(argv, 0, "created", "")
            if args[0] == "install":
                return subprocess.CompletedProcess(argv, 0, "installed", "")
            if args[0] == "add":
                return subprocess.CompletedProcess(argv, 1, "", "registry unavailable")
            if args[0] == "optimizer":
                return subprocess.CompletedProcess(argv, 0, "Optimizer skipped", "")
            raise AssertionError(f"step without provisioned tools was run: {args}")

        lane = {
            "core": "lib",
            "language": "matrix-test-language",
            "scaffold": ["create-test", "fixture"],
            "pre_steps": ["mgc_add_python_test_build_tools"],
            "steps": [
                ("install", ["install"]),
                ("test", ["test"]),
                ("build", ["build"]),
            ],
            "delegated": [],
            "install_owner": "native-engine",
        }

        with patch("lifecycle_capability_matrix.subprocess.run", side_effect=fake_run):
            result = run_lane("/tmp/fake-mgc", lane)

        self.assertEqual(result["dims"]["install"], STATUS_NATIVE)
        self.assertEqual(result["dims"]["test"], STATUS_UNVERIFIED)
        self.assertEqual(result["dims"]["build"], STATUS_UNVERIFIED)
        self.assertIn(["install"], calls)
        self.assertNotIn(["test"], calls)
        self.assertNotIn(["build"], calls)
