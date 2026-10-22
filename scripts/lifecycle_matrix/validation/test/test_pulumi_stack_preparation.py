"""Focused LanePreparationFailure test owner.

Owner test hẹp cho LanePreparationFailure.
"""

from .support import (
    Path,
    STATUS_NATIVE,
    STATUS_PLAIN,
    STATUS_UNVERIFIED,
    matrix_module,
    os,
    patch,
    run_lane,
    subprocess,
    unittest,
)

class LanePreparationFailure(unittest.TestCase):
    def test_failed_local_pulumi_stack_setup_blocks_preview_without_cloud_auth(self):
        calls = []

        def fake_run(argv, **kwargs):
            args = argv[1:] if argv[0] == "/tmp/fake-mgc" else argv
            calls.append((args, kwargs.get("env", {})))
            if args[0] == "create-test":
                project = os.path.join(kwargs["cwd"], args[-1])
                os.mkdir(project)
                Path(project, "package.json").write_text("{}", encoding="utf-8")
                Path(project, "Pulumi.yaml").write_text(
                    "name: fixture\nruntime: nodejs\n", encoding="utf-8"
                )
                return subprocess.CompletedProcess(argv, 0, "created", "")
            if args[:2] == ["pulumi", "login"]:
                return subprocess.CompletedProcess(argv, 0, "local login", "")
            if args[:3] == ["pulumi", "stack", "init"]:
                return subprocess.CompletedProcess(argv, 0, "stack initialized", "")
            if args[:3] == ["pulumi", "stack", "select"]:
                return subprocess.CompletedProcess(argv, 1, "", "stack setup failed")
            if args[0] == "install":
                return subprocess.CompletedProcess(argv, 0, "installed", "")
            if args[0] == "test":
                return subprocess.CompletedProcess(argv, 0, "tested", "")
            if args[0] == "optimizer":
                return subprocess.CompletedProcess(argv, 0, "Optimizer skipped", "")
            raise AssertionError(f"Pulumi preview ran without a local stack: {args}")

        lane = {
            "core": "clo",
            "language": "pulumi",
            "scaffold": ["create-test", "fixture"],
            "pre_steps": ["initialize_local_pulumi_stack"],
            "steps": [("install", ["install"]), ("test", ["test"]), ("build", ["build"])],
            "delegated": [],
            "install_owner": "native-engine",
        }

        with patch("lifecycle_capability_matrix.subprocess.run", side_effect=fake_run):
            result = run_lane("/tmp/fake-mgc", lane)

        self.assertEqual(result["dims"]["install"], STATUS_NATIVE, result)
        self.assertEqual(result["dims"]["test"], STATUS_PLAIN, result)
        self.assertEqual(result["dims"]["build"], STATUS_UNVERIFIED)
        pulumi_calls = [call for call in calls if call[0][0] == "pulumi"]
        self.assertEqual(pulumi_calls[0][0][1], "login")
        self.assertEqual(
            [call[0][1:3] for call in pulumi_calls[1:]],
            [["stack", "init"], ["stack", "select"]],
        )
        pulumi_env = pulumi_calls[0][1]
        self.assertTrue(pulumi_env["PULUMI_BACKEND_URL"].startswith("file://"))
        self.assertNotIn("PULUMI_ACCESS_TOKEN", pulumi_env)
        self.assertFalse(any(name.startswith("AWS_") and name != "AWS_EC2_METADATA_DISABLED" for name in pulumi_env))
        self.assertNotIn((['build'], pulumi_env), calls)

    def test_successful_local_pulumi_selects_stack_before_build(self):
        calls = []

        def fake_run(argv, **kwargs):
            args = argv[1:] if argv[0] == "/tmp/fake-mgc" else argv
            calls.append((args, kwargs.get("env", {})))
            if args[0] == "create-test":
                project = os.path.join(kwargs["cwd"], args[-1])
                os.mkdir(project)
                Path(project, "package.json").write_text(
                    '{"name":"fixture"}\n', encoding="utf-8"
                )
                Path(project, "Pulumi.yaml").write_text(
                    "name: fixture\nruntime: nodejs\n", encoding="utf-8"
                )
                return subprocess.CompletedProcess(argv, 0, "created", "")
            if args[:2] == ["pulumi", "login"]:
                return subprocess.CompletedProcess(argv, 0, "local login", "")
            if args[:3] in (
                ["pulumi", "stack", "init"],
                ["pulumi", "stack", "select"],
            ):
                return subprocess.CompletedProcess(argv, 0, "local stack", "")
            if args == ["pulumi", "stack", "get", "--show-name"]:
                return subprocess.CompletedProcess(
                    argv, 0, matrix_module.PULUMI_LIFECYCLE_STACK + "\n", ""
                )
            if args[0] in {"build", "optimizer"}:
                return subprocess.CompletedProcess(argv, 0, "ok", "")
            raise AssertionError(f"Unexpected Pulumi lifecycle command: {args}")

        lane = {
            "core": "clo",
            "language": "pulumi",
            "scaffold": ["create-test", "fixture"],
            "pre_steps": ["initialize_local_pulumi_stack"],
            "steps": [("build", ["build"])],
            "delegated": [],
            "lifecycle_owners": {"build": "plain-delegation"},
            "install_owner": "native-engine",
        }

        with patch("lifecycle_capability_matrix.subprocess.run", side_effect=fake_run):
            result = run_lane("/tmp/fake-mgc", lane)

        self.assertEqual(result["dims"]["build"], STATUS_PLAIN, result)
        pulumi_calls = [call[0] for call in calls if call[0][0] == "pulumi"]
        self.assertEqual(
            pulumi_calls[0][:2],
            ["pulumi", "login"],
        )
        self.assertEqual(
            [call[1:3] for call in pulumi_calls[1:]],
            [
                ["stack", "init"],
                ["stack", "select"],
                ["stack", "get"],
            ],
        )
        self.assertEqual(calls[0][1]["PULUMI_STACK"], matrix_module.PULUMI_LIFECYCLE_STACK)
        build_index = next(index for index, call in enumerate(calls) if call[0] == ["build"])
        select_index = next(
            index for index, call in enumerate(calls)
            if call[0][:3] == ["pulumi", "stack", "select"]
        )
        verify_index = next(
            index for index, call in enumerate(calls)
            if call[0] == ["pulumi", "stack", "get", "--show-name"]
        )
        self.assertLess(select_index, build_index)
        self.assertLess(verify_index, build_index)

    def test_wrong_reported_pulumi_stack_blocks_build(self):
        calls = []

        def fake_run(argv, **kwargs):
            args = argv[1:] if argv[0] == "/tmp/fake-mgc" else argv
            calls.append(args)
            if args[0] == "create-test":
                project = os.path.join(kwargs["cwd"], args[-1])
                os.mkdir(project)
                Path(project, "package.json").write_text("{}\n", encoding="utf-8")
                Path(project, "Pulumi.yaml").write_text(
                    "name: fixture\nruntime: nodejs\n", encoding="utf-8"
                )
                return subprocess.CompletedProcess(argv, 0, "created", "")
            if args[:2] == ["pulumi", "login"]:
                return subprocess.CompletedProcess(argv, 0, "local login", "")
            if args[:3] in (
                ["pulumi", "stack", "init"],
                ["pulumi", "stack", "select"],
            ):
                return subprocess.CompletedProcess(argv, 0, "selected", "")
            if args == ["pulumi", "stack", "get", "--show-name"]:
                return subprocess.CompletedProcess(argv, 0, "unexpected-stack\n", "")
            if args[0] == "optimizer":
                return subprocess.CompletedProcess(argv, 0, "Optimizer skipped", "")
            raise AssertionError(f"Build ran without the expected Pulumi stack: {args}")

        lane = {
            "core": "clo",
            "language": "pulumi",
            "scaffold": ["create-test", "fixture"],
            "pre_steps": ["initialize_local_pulumi_stack"],
            "steps": [("build", ["build"])],
            "delegated": [],
            "install_owner": "native-engine",
        }

        with patch("lifecycle_capability_matrix.subprocess.run", side_effect=fake_run):
            result = run_lane("/tmp/fake-mgc", lane)

        self.assertEqual(result["dims"]["build"], STATUS_UNVERIFIED, result)
        self.assertIn("unexpected-stack", result["detail"]["pulumi_stack_setup_output"])
        self.assertNotIn(["build"], calls)
