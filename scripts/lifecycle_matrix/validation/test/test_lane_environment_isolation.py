"""Focused LifecycleEnvironmentIsolation test owner.

Owner test hẹp cho LifecycleEnvironmentIsolation.
"""

from .support import (
    lifecycle_environment,
    matrix_module,
    os,
    patch,
    unittest,
)

class LifecycleEnvironmentIsolation(unittest.TestCase):
    def test_flutter_sdk_cache_uses_the_explicit_job_local_path(self):
        cache = matrix_module.flutter_sdk_pub_cache_directory(
            {"PUB_CACHE": "/runner/_temp/flutter-pub-cache", "HOME": "/home/runner"}
        )

        self.assertEqual(cache, os.path.abspath("/runner/_temp/flutter-pub-cache"))

    def test_flutter_sdk_cache_resolves_relative_override_from_the_host(self):
        cache = matrix_module.flutter_sdk_pub_cache_directory(
            {"PUB_CACHE": "relative-flutter-pub-cache"}
        )

        self.assertTrue(os.path.isabs(cache))
        self.assertEqual(os.path.basename(cache), "relative-flutter-pub-cache")

    def test_flutter_sdk_cache_uses_the_windows_default_when_unconfigured(self):
        cache = matrix_module.flutter_sdk_pub_cache_directory(
            {"LOCALAPPDATA": "C:/Users/runner/AppData/Local"}
        )

        self.assertEqual(cache, "")

    def test_flutter_sdk_cache_uses_home_default_on_posix_when_unconfigured(self):
        cache = matrix_module.flutter_sdk_pub_cache_directory(
            {"HOME": "/home/runner"}
        )

        self.assertEqual(cache, "")

    def test_flutter_sdk_pub_cache_without_override_stays_below_job_home(self):
        cache = matrix_module.flutter_sdk_pub_cache_directory(
            {"FLUTTER_SDK_HOME": "/runner/flutter-sdk-home"}
        )

        self.assertEqual(
            cache,
            os.path.join(os.path.abspath("/runner/flutter-sdk-home"), ".pub-cache"),
        )

    def test_flutter_sdk_home_uses_only_explicit_job_local_override(self):
        home = matrix_module.flutter_sdk_home_directory(
            {"FLUTTER_SDK_HOME": "relative/flutter-sdk-home", "HOME": "/home/runner"}
        )

        self.assertTrue(os.path.isabs(home))
        self.assertEqual(os.path.basename(home), "flutter-sdk-home")

    def test_flutter_sdk_home_does_not_fall_back_to_the_host_home(self):
        home = matrix_module.flutter_sdk_home_directory({"HOME": "/home/runner"})

        self.assertEqual(home, "")

    def test_lane_environment_overrides_host_store_and_project_cache(self):
        with patch.dict(
            "os.environ",
            {
                "MAGICORE_STORE_ROOT": "/host/private/mgc-store",
                "MGC_CACHE_DIR": "/host/private/mgc-cache",
            },
        ):
            original_environment = os.environ.copy()
            env = lifecycle_environment(
                "/tmp/mgc-lane-sandbox", os.path.join("/tmp/mgc-lane-sandbox", "project")
            )
            self.assertEqual(os.environ, original_environment)

        self.assertEqual(
            env["MGC_CACHE_DIR"],
            os.path.join("/tmp/mgc-lane-sandbox", "project", ".magicore"),
        )
        self.assertEqual(env["HOME"], os.path.join("/tmp/mgc-lane-sandbox", ".home"))
        self.assertEqual(
            env["MAGICORE_STORE_ROOT"],
            os.path.join("/tmp/mgc-lane-sandbox", ".home", ".magicore", "store"),
        )
        self.assertEqual(
            env["CARGO_HOME"], os.path.join("/tmp/mgc-lane-sandbox", ".cargo-home")
        )
        self.assertEqual(
            env["PIP_CACHE_DIR"], os.path.join("/tmp/mgc-lane-sandbox", ".cache", "pip")
        )
        self.assertEqual(
            env["npm_config_cache"], os.path.join("/tmp/mgc-lane-sandbox", ".cache", "npm")
        )
        self.assertEqual(env["GOMODCACHE"], os.path.join("/tmp/mgc-lane-sandbox", ".go", "pkg", "mod"))
        self.assertEqual(env["GOCACHE"], os.path.join("/tmp/mgc-lane-sandbox", ".cache", "go-build"))
        self.assertEqual(
            env["DOTNET_CLI_HOME"], os.path.join("/tmp/mgc-lane-sandbox", ".home", ".dotnet")
        )
        self.assertEqual(
            env["TMPDIR"], os.path.join("/tmp/mgc-lane-sandbox", ".tmp")
        )
        self.assertEqual(
            env["DENO_DIR"], os.path.join("/tmp/mgc-lane-sandbox", ".cache", "deno")
        )
