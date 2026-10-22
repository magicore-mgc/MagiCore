"""Focused LifecycleEnvironmentIsolation test owner.

Owner test hẹp cho LifecycleEnvironmentIsolation.
"""

from .support import (
    LANES,
    Path,
    REPO_ROOT,
    STATUS_FAILED,
    STATUS_NATIVE,
    STATUS_UNVERIFIED,
    matrix_module,
    os,
    patch,
    pulumi_local_environment,
    run_lane,
    subprocess,
    tempfile,
    unittest,
    _lockfile_candidates,
    _materialize_marker,
)

class LifecycleEnvironmentIsolation(unittest.TestCase):
    def test_pulumi_lane_uses_ephemeral_local_backend_and_drops_cloud_credentials(self):
        with tempfile.TemporaryDirectory() as root:
            env = pulumi_local_environment(
                root,
                {
                    "HOME": os.path.join(root, ".home"),
                    "PULUMI_ACCESS_TOKEN": "host-token",
                    "AWS_ACCESS_KEY_ID": "host-access-key",
                    "AWS_SECRET_ACCESS_KEY": "host-secret-key",
                    "AWS_PROFILE": "host-profile",
                    "PATH": "/usr/bin",
                },
            )

        self.assertEqual(env["PULUMI_BACKEND_URL"], Path(root, ".pulumi-backend").as_uri())
        self.assertEqual(env["PULUMI_HOME"], os.path.join(root, ".pulumi-home"))
        self.assertEqual(env["PULUMI_STACK"], matrix_module.PULUMI_LIFECYCLE_STACK)
        self.assertTrue(env["PULUMI_CONFIG_PASSPHRASE"])
        self.assertEqual(env["AWS_EC2_METADATA_DISABLED"], "true")
        for name in ("PULUMI_ACCESS_TOKEN", "AWS_ACCESS_KEY_ID", "AWS_SECRET_ACCESS_KEY", "AWS_PROFILE"):
            self.assertNotIn(name, env)

    def test_pulumi_lane_declares_a_local_stack_setup_and_toolchain_probe(self):
        lane = next(
            lane for lane in LANES
            if lane["core"] == "clo" and lane["language"] == "pulumi"
        )
        self.assertEqual(lane["toolchain_probes"], ["pulumi"])
        self.assertIn("initialize_local_pulumi_stack", lane["pre_steps"])

    def test_pulumi_lane_declares_native_dependency_fixture_and_artifact_probes(self):
        lane = next(
            lane for lane in LANES
            if lane["core"] == "clo" and lane["language"] == "pulumi"
        )

        self.assertIn("mgc_add_real_dependency", lane["pre_steps"])
        self.assertEqual(
            lane["dependency_fixture"],
            {
                "command": "add-clo",
                "package": "@pulumi/pulumi@^3.0.0",
                "manifest_path": "package.json",
                "manifest_name": "@pulumi/pulumi",
                "skip_if_declared": True,
            },
        )
        self.assertEqual(
            _lockfile_candidates("/tmp/lane/project", "pulumi"),
            [("mgc.lock", "mgc-lock"), ("package-lock.json", "json-deps")],
        )
        self.assertEqual(
            _materialize_marker("/tmp/lane/project", "pulumi"),
            ("node_modules", os.path.join("/tmp/lane/project", "node_modules")),
        )

    def test_pulumi_dependency_fixture_runs_through_mgc_before_install(self):
        calls = []

        def fake_run(argv, **kwargs):
            args = argv[1:]
            calls.append(args)
            if args[0] == "create-test":
                project = os.path.join(kwargs["cwd"], args[-1])
                os.mkdir(project)
                Path(project, "Pulumi.yaml").write_text(
                    "name: fixture\nruntime: nodejs\n", encoding="utf-8"
                )
                Path(project, "package.json").write_text(
                    '{"name":"fixture","version":"0.1.0"}',
                    encoding="utf-8",
                )
                return subprocess.CompletedProcess(argv, 0, "created", "")
            if args[0] == "add-clo":
                return subprocess.CompletedProcess(argv, 0, "added", "")
            if args[0] in {"install", "test", "build"}:
                return subprocess.CompletedProcess(argv, 0, args[0], "")
            if args[0] == "optimizer":
                return subprocess.CompletedProcess(
                    argv, 0, "MagiCore Optimizer finished: applied 1/1 configurations", ""
                )
            raise AssertionError(f"unexpected lifecycle command: {args}")

        lane = {
            "core": "clo",
            "language": "pulumi",
            "scaffold": ["create-test", "fixture"],
            "pre_steps": ["mgc_add_real_dependency"],
            "dependency_fixture": {
                "command": "add-clo",
                "package": "@pulumi/pulumi@^3.0.0",
                "manifest_path": "package.json",
                "manifest_name": "@pulumi/pulumi",
                "skip_if_declared": True,
            },
            "steps": [
                ("install", ["install"]),
                ("test", ["test"]),
                ("build", ["build"]),
            ],
            "delegated": [],
            "install_owner": "native-engine",
        }

        with patch("lifecycle_capability_matrix.subprocess.run", side_effect=fake_run), \
             patch("lifecycle_capability_matrix.shutil.which", return_value="/usr/bin/pulumi"):
            result = run_lane("/tmp/fake-mgc", lane)

        add_index = calls.index(["add-clo", "@pulumi/pulumi@^3.0.0"])
        install_index = calls.index(["install"])
        self.assertLess(add_index, install_index)
        self.assertEqual(result["dims"]["add"], STATUS_NATIVE)
        self.assertEqual(result["dims"]["install"], STATUS_NATIVE)

        workflow = (
            REPO_ROOT
            / ".github"
            / "workflows"
            / "lifecycle-matrix.yml"
        ).read_text(encoding="utf-8")
        self.assertIn(
            "pulumi/actions@8e5e406f4007fca908480587cb9893c07090f58d",
            workflow,
        )
        self.assertIn('pulumi-version: "3.268.0"', workflow)

    def test_pulumi_dependency_fixture_does_not_rewrite_existing_manifest_dependency(self):
        calls = []

        def fake_run(argv, **kwargs):
            args = argv[1:]
            calls.append(args)
            if args[0] == "create-test":
                project = os.path.join(kwargs["cwd"], args[-1])
                os.mkdir(project)
                Path(project, "Pulumi.yaml").write_text(
                    "name: fixture\nruntime: nodejs\n", encoding="utf-8"
                )
                Path(project, "package.json").write_text(
                    '{"name":"fixture","dependencies":{"@pulumi/pulumi":"^3.0.0"}}',
                    encoding="utf-8",
                )
                return subprocess.CompletedProcess(argv, 0, "created", "")
            if args[0] in {"install", "test", "build"}:
                return subprocess.CompletedProcess(argv, 0, args[0], "")
            if args[0] == "optimizer":
                return subprocess.CompletedProcess(
                    argv, 0, "MagiCore Optimizer finished: applied 1/1 configurations", ""
                )
            raise AssertionError(f"unexpected lifecycle command: {args}")

        lane = {
            "core": "clo",
            "language": "pulumi",
            "scaffold": ["create-test", "fixture"],
            "pre_steps": ["mgc_add_real_dependency"],
            "dependency_fixture": {
                "command": "add-clo",
                "package": "@pulumi/pulumi@^3.0.0",
                "manifest_path": "package.json",
                "manifest_name": "@pulumi/pulumi",
                "skip_if_declared": True,
            },
            "steps": [
                ("install", ["install"]),
                ("test", ["test"]),
                ("build", ["build"]),
            ],
            "delegated": [],
            "install_owner": "native-engine",
        }

        with patch("lifecycle_capability_matrix.subprocess.run", side_effect=fake_run), \
             patch("lifecycle_capability_matrix.shutil.which", return_value="/usr/bin/pulumi"):
            result = run_lane("/tmp/fake-mgc", lane)

        self.assertNotIn(["add-clo", "@pulumi/pulumi@^3.0.0"], calls)
        self.assertIn(["install"], calls)
        self.assertEqual(result["dims"]["add"], STATUS_UNVERIFIED)
        self.assertEqual(result["dims"]["install"], STATUS_NATIVE)

    def test_unreadable_pulumi_manifest_blocks_lifecycle_instead_of_running_empty(self):
        calls = []

        def fake_run(argv, **kwargs):
            args = argv[1:]
            calls.append(args)
            if args[0] == "create-test":
                project = os.path.join(kwargs["cwd"], args[-1])
                os.mkdir(project)
                Path(project, "Pulumi.yaml").write_text(
                    "name: fixture\nruntime: nodejs\n", encoding="utf-8"
                )
                Path(project, "package.json").write_text("{", encoding="utf-8")
                return subprocess.CompletedProcess(argv, 0, "created", "")
            if args[0] == "optimizer":
                return subprocess.CompletedProcess(
                    argv, 0, "MagiCore Optimizer finished: applied 1/1 configurations", ""
                )
            raise AssertionError(f"lifecycle ran with unreadable dependency manifest: {args}")

        lane = {
            "core": "clo",
            "language": "pulumi",
            "scaffold": ["create-test", "fixture"],
            "pre_steps": ["mgc_add_real_dependency"],
            "dependency_fixture": {
                "command": "add-clo",
                "package": "@pulumi/pulumi@^3.0.0",
                "manifest_path": "package.json",
                "manifest_name": "@pulumi/pulumi",
                "skip_if_declared": True,
            },
            "steps": [
                ("install", ["install"]),
                ("test", ["test"]),
                ("build", ["build"]),
            ],
            "delegated": [],
            "install_owner": "native-engine",
        }

        with patch("lifecycle_capability_matrix.subprocess.run", side_effect=fake_run), \
             patch("lifecycle_capability_matrix.shutil.which", return_value="/usr/bin/pulumi"):
            result = run_lane("/tmp/fake-mgc", lane)

        self.assertEqual(result["dims"]["add"], STATUS_FAILED)
        self.assertEqual(result["dims"]["install"], STATUS_UNVERIFIED)
        self.assertEqual(result["dims"]["test"], STATUS_UNVERIFIED)
        self.assertEqual(result["dims"]["build"], STATUS_UNVERIFIED)
        self.assertNotIn(["install"], calls)
