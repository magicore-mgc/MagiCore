"""Focused LifecycleEnvironmentIsolation test owner.

Owner test hẹp cho LifecycleEnvironmentIsolation.
"""

from .support import (
    lifecycle_environment,
    mgc_add_python_test_build_tools,
    os,
    patch,
    recovery_environment,
    subprocess,
    tempfile,
    unittest,
    _materialize_marker,
)

class LifecycleEnvironmentIsolation(unittest.TestCase):
    def test_lane_environment_preserves_read_only_rustup_toolchain_home(self):
        with tempfile.TemporaryDirectory() as root:
            rustup_home = os.path.join(root, "rustup")
            os.makedirs(rustup_home)
            with patch.dict("os.environ", {"RUSTUP_HOME": rustup_home}):
                env = lifecycle_environment(
                    os.path.join(root, "lane"), os.path.join(root, "project")
                )

            self.assertEqual(env["RUSTUP_HOME"], rustup_home)
            self.assertEqual(env["RUSTUP_AUTO_INSTALL"], "0")

    def test_lane_environment_infers_rustup_home_from_host_home(self):
        with tempfile.TemporaryDirectory() as root:
            host_home = os.path.join(root, "host-home")
            rustup_home = os.path.join(host_home, ".rustup")
            os.makedirs(rustup_home)
            with patch.dict("os.environ", {"HOME": host_home}, clear=False):
                os.environ.pop("RUSTUP_HOME", None)
                env = lifecycle_environment(
                    os.path.join(root, "lane"), os.path.join(root, "project")
                )

            self.assertEqual(env["RUSTUP_HOME"], rustup_home)

    def test_lane_environment_rejects_missing_rustup_home_without_installing(self):
        with tempfile.TemporaryDirectory() as root:
            host_home = os.path.join(root, "host-home")
            os.makedirs(host_home)
            with patch.dict(
                "os.environ",
                {"HOME": host_home, "RUSTUP_HOME": os.path.join(root, "missing")},
                clear=False,
            ):
                env = lifecycle_environment(
                    os.path.join(root, "lane"), os.path.join(root, "project")
                )

            self.assertNotIn("RUSTUP_HOME", env)
            self.assertEqual(env["RUSTUP_AUTO_INSTALL"], "0")

    def test_lane_environment_creates_isolated_tool_temp_directory(self):
        with tempfile.TemporaryDirectory() as root:
            sandbox = os.path.join(root, "lane")
            os.makedirs(sandbox)
            env = lifecycle_environment(sandbox, os.path.join(sandbox, "project"))

            self.assertTrue(os.path.isdir(env["TMPDIR"]))
            self.assertEqual(env["TMP"], env["TMPDIR"])
            self.assertEqual(env["TEMP"], env["TMPDIR"])

    def test_recovery_environment_uses_project_private_store(self):
        with patch.dict(
            "os.environ",
            {
                "MAGICORE_STORE_ROOT": "/host/private/store",
                "MGC_CACHE_DIR": "/host/private/cache",
            },
        ):
            original_environment = os.environ.copy()
            env = recovery_environment("/tmp/lane", "/tmp/lane/project")
            self.assertEqual(os.environ, original_environment)

        self.assertEqual(
            env["HOME"], os.path.join('/tmp/lane/project', '.magicore-recovery', 'home')
        )
        self.assertEqual(
            env["MGC_CACHE_DIR"], os.path.join('/tmp/lane/project', '.magicore-recovery')
        )

    def test_materialization_markers_resolve_inside_lane_environment(self):
        rust_env = {"HOME": "/tmp/lane/.home"}
        self.assertEqual(
            _materialize_marker("/tmp/lane/project", "rust", rust_env),
            (
                "mgc-cargo-store",
                os.path.join('/tmp/lane/.home', '.magicore', 'store', 'cargo'),
            ),
        )

        go_env = {"PATH": "/lane/bin", "GOMODCACHE": "/tmp/lane/.go/pkg/mod"}
        completed = subprocess.CompletedProcess(
            args=["go", "env", "GOMODCACHE"], returncode=0, stdout=go_env["GOMODCACHE"]
        )
        with patch("lifecycle_capability_matrix.shutil.which", return_value="/lane/bin/go"), \
             patch("lifecycle_capability_matrix.subprocess.run", return_value=completed) as run:
            marker = _materialize_marker("/tmp/lane/project", "go", go_env)
        self.assertEqual(marker, ("gomodcache", go_env["GOMODCACHE"]))
        self.assertEqual(run.call_args.kwargs["env"], go_env)

    def test_python_test_build_tools_are_added_through_mgc(self):
        mgc_bin = "/tmp/mgc"
        project = "/tmp/lane/project"
        environment = {"PATH": "/usr/bin:/bin"}
        success = subprocess.CompletedProcess(args=[], returncode=0)
        calls = []

        def fake_run(argv, **kwargs):
            calls.append((argv, kwargs))
            return success

        with patch("lifecycle_capability_matrix.subprocess.run", side_effect=fake_run):
            result, returned_environment = mgc_add_python_test_build_tools(
                mgc_bin, project, environment, timeout_s=30
            )

        self.assertIs(result, success)
        self.assertEqual(
            [call[0] for call in calls],
            [
                [mgc_bin, "add", "pytest@9.1.1"],
                [mgc_bin, "add", "build@1.6.1"],
                [mgc_bin, "add", "setuptools@84.0.0"],
            ],
        )
        self.assertTrue(all(call[1]["cwd"] == project for call in calls))
        self.assertTrue(all(call[1]["env"] == environment for call in calls))
        self.assertIs(returned_environment, environment)
        self.assertEqual(environment, {"PATH": "/usr/bin:/bin"})

    def test_python_tool_add_stops_after_first_mgc_failure(self):
        calls = []
        failure = subprocess.CompletedProcess(args=[], returncode=1, stdout="", stderr="offline")

        def fake_run(argv, **kwargs):
            calls.append(argv)
            return failure

        with patch("lifecycle_capability_matrix.subprocess.run", side_effect=fake_run):
            result, environment = mgc_add_python_test_build_tools(
                "/tmp/mgc", "/tmp/project", {"PATH": "/usr/bin"}, timeout_s=30
            )

        self.assertIs(result, failure)
        self.assertEqual(calls, [["/tmp/mgc", "add", "pytest@9.1.1"]])
        self.assertEqual(environment, {"PATH": "/usr/bin"})
