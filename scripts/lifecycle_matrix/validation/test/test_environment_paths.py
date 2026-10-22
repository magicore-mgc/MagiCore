"""Focused LifecycleEnvironmentIsolation test owner.

Owner test hẹp cho LifecycleEnvironmentIsolation.
"""

from .support import (
    Path,
    bash_compatible_path,
    bootstrap_bash_command,
    matrix_module,
    tempfile,
    unittest,
)

class LifecycleEnvironmentIsolation(unittest.TestCase):
    def test_bootstrap_bash_command_uses_git_for_windows_and_posix_bash(self):
        with tempfile.TemporaryDirectory(prefix="git bash lookup ") as tmp:
            git_root = Path(tmp) / "Program Files" / "Git"
            git_executable = git_root / "cmd" / "git.exe"
            git_executable.parent.mkdir(parents=True)
            git_executable.touch()
            expected_bash = git_root / "bin" / "bash.exe"
            expected_bash.parent.mkdir(parents=True)
            expected_bash.touch()

            self.assertEqual(
                bootstrap_bash_command("nt", git_executable),
                str(expected_bash.resolve()),
            )
            self.assertEqual(bootstrap_bash_command("posix"), "bash")

    def test_bash_compatible_path_maps_windows_drive_to_msys_mount(self):
        self.assertEqual(
            bash_compatible_path(r"D:\a\_temp\fake Flutter SDK", windows=True),
            "/d/a/_temp/fake Flutter SDK",
        )

    def test_bash_compatible_path_preserves_posix_paths(self):
        self.assertEqual(
            bash_compatible_path("/tmp/fake Flutter SDK", windows=False),
            "/tmp/fake Flutter SDK",
        )

    def test_flutter_runtime_steps_reuse_the_warmed_sdk_pub_cache(self):
        lane = {"core": "app", "language": "flutter"}
        lane_env = {"HOME": "/tmp/lane/.home", "PUB_CACHE": "/tmp/lane/.cache/pub"}
        original = dict(lane_env)

        test_env = matrix_module.lifecycle_step_environment(
            lane, "test", lane_env, "/runner/.pub-cache", "/runner/flutter-sdk-home"
        )
        build_env = matrix_module.lifecycle_step_environment(
            lane, "build", lane_env, "/runner/.pub-cache", "/runner/flutter-sdk-home"
        )
        install_env = matrix_module.lifecycle_step_environment(
            lane, "install", lane_env, "/runner/.pub-cache", "/runner/flutter-sdk-home"
        )

        self.assertEqual(test_env["PUB_CACHE"], "/runner/.pub-cache")
        self.assertEqual(build_env["PUB_CACHE"], "/runner/.pub-cache")
        self.assertEqual(test_env["HOME"], "/runner/flutter-sdk-home")
        self.assertEqual(test_env["USERPROFILE"], "/runner/flutter-sdk-home")
        self.assertEqual(build_env["HOME"], "/runner/flutter-sdk-home")
        self.assertEqual(install_env["HOME"], lane_env["HOME"])
        self.assertEqual(install_env["PUB_CACHE"], "/tmp/lane/.cache/pub")
        self.assertEqual(lane_env, original)
        self.assertIsNot(test_env, lane_env)

    def test_non_flutter_runtime_steps_keep_lane_local_pub_cache(self):
        lane = {"core": "web", "language": "ts"}
        lane_env = {"HOME": "/tmp/lane/.home", "PUB_CACHE": "/tmp/lane/.cache/pub"}

        step_env = matrix_module.lifecycle_step_environment(
            lane, "test", lane_env, "/runner/.pub-cache"
        )

        self.assertIs(step_env, lane_env)
        self.assertEqual(step_env["PUB_CACHE"], "/tmp/lane/.cache/pub")

    def test_local_flutter_runtime_keeps_lane_home_and_pub_cache_without_job_override(self):
        lane = {"core": "app", "language": "flutter"}
        lane_env = {"HOME": "/tmp/lane/.home", "PUB_CACHE": "/tmp/lane/.cache/pub"}

        step_env = matrix_module.lifecycle_step_environment(
            lane, "test", lane_env, "/home/runner/.pub-cache"
        )

        self.assertIs(step_env, lane_env)
        self.assertEqual(step_env["HOME"], "/tmp/lane/.home")
        self.assertEqual(step_env["PUB_CACHE"], "/tmp/lane/.cache/pub")
