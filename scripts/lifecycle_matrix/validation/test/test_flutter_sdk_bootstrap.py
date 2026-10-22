"""Focused LifecycleEnvironmentIsolation test owner.

Owner test hẹp cho LifecycleEnvironmentIsolation.
"""

import subprocess

from .support import (
    Path,
    PureWindowsPath,
    REPO_ROOT,
    bash_compatible_path,
    bootstrap_bash_command,
    os,
    tempfile,
    unittest,
)

class LifecycleEnvironmentIsolation(unittest.TestCase):
    def test_flutter_sdk_dependencies_are_resolved_before_the_lifecycle_matrix_runs(self):
        workflow = (
            REPO_ROOT
            / ".github"
            / "workflows"
            / "lifecycle-matrix.yml"
        ).read_text(encoding="utf-8")

        setup = workflow.index("Setup Flutter (app lane)")
        warm = workflow.index(
            "Resolve Flutter SDK dependencies before MGC process guard", setup
        )
        matrix = workflow.index("Generate lifecycle matrix (real binary, real steps)")
        self.assertLess(setup, warm)
        self.assertLess(warm, matrix)
        self.assertIn("run: bash scripts/bootstrap_flutter_sdk.sh", workflow[warm:matrix])
        job_cache = "PUB_CACHE: ${{ runner.temp }}/flutter-pub-cache"
        self.assertIn(job_cache, workflow[warm:matrix])
        self.assertIn(job_cache, workflow[matrix:])
        job_home = "FLUTTER_SDK_HOME: ${{ runner.temp }}/flutter-sdk-home"
        self.assertIn(job_home, workflow[warm:matrix])
        self.assertIn(job_home, workflow[matrix:])
        self.assertIn("HOME: ${{ runner.temp }}/flutter-sdk-home", workflow[warm:matrix])
        self.assertIn(
            "USERPROFILE: ${{ runner.temp }}/flutter-sdk-home", workflow[warm:matrix]
        )

    def test_windows_spawn_job_resolves_flutter_sdk_dependencies_before_guard_tests(self):
        workflow = (
            REPO_ROOT
            / ".github"
            / "workflows"
            / "ci.yml"
        ).read_text(encoding="utf-8")

        setup = workflow.index("Setup Flutter (for .bat spawn tests)")
        warm = workflow.index(
            "Resolve Flutter SDK dependencies before MGC process guard", setup
        )
        tests = workflow.index("Run Windows spawn tests")
        self.assertLess(setup, warm)
        self.assertLess(warm, tests)
        self.assertIn("run: bash scripts/bootstrap_flutter_sdk.sh", workflow[warm:tests])

    def test_guarded_flutter_workflows_resolve_sdk_tool_dependencies(self):
        workflow_dir = REPO_ROOT / ".github" / "workflows"
        workflows = [
            (
                "release-binary-e2e.yml",
                "Setup Flutter (app lane)",
                "Resolve Flutter SDK dependencies before MGC process guard",
                "Lifecycle app flutter (create → install → test → build)",
            ),
            (
                "delegated-compatibility-matrix.yml",
                "Setup Flutter",
                "Resolve Flutter SDK dependencies before MGC process guard",
                "Create app project",
            ),
            (
                "security.yml",
                "Setup Flutter",
                "Resolve Flutter SDK dependencies before MGC process guard",
                "Install cargo-audit (rust parity lane on this SHA)",
            ),
            (
                "ci.yml",
                "Setup Flutter SDK",
                "Resolve Flutter SDK dependencies before MGC process guard",
                "Run native test-runner security E2E on release binary",
            ),
        ]

        for filename, setup_name, warm_name, guarded_step in workflows:
            with self.subTest(workflow=filename):
                workflow = (workflow_dir / filename).read_text(encoding="utf-8")
                setup = workflow.index(setup_name)
                warm = workflow.index(warm_name, setup)
                guarded = workflow.index(guarded_step, warm)
                self.assertLess(setup, warm)
                self.assertLess(warm, guarded)
                self.assertIn(
                    "run: bash scripts/bootstrap_flutter_sdk.sh", workflow[warm:guarded]
                )

    def test_runtime_e2e_preserves_rust_toolchains_when_isolating_home(self):
        script = (
            REPO_ROOT
            / "cli"
            / "tests"
            / "scripts"
            / "runtime_full_e2e.sh"
        ).read_text(encoding="utf-8")

        isolate_home = script.index('export HOME="$TEMP_BASE/home"')
        original_home = script.index('ORIGINAL_HOME="$HOME"')
        rustup_home = script.index(
            'export RUSTUP_HOME="${RUSTUP_HOME:-${ORIGINAL_HOME}/.rustup}"'
        )
        cargo_home = script.index(
            'export CARGO_HOME="${CARGO_HOME:-${ORIGINAL_HOME}/.cargo}"'
        )
        self.assertLess(original_home, rustup_home)
        self.assertLess(rustup_home, isolate_home)
        self.assertLess(cargo_home, isolate_home)

    def test_flutter_sdk_bootstrap_resolves_dependencies_and_warms_cli(self):
        script = (
            REPO_ROOT / "scripts" / "bootstrap_flutter_sdk.sh"
        ).read_text(encoding="utf-8")

        self.assertIn('executable="dart.exe"', script)
        self.assertIn('executable="dart"', script)
        self.assertIn(
            'pub --suppress-analytics --directory "$sdk_root/packages/flutter_tools" get --example',
            script,
        )
        self.assertIn('local flutter_bin="$sdk_root/bin/flutter"', script)
        self.assertLess(
            script.index('if [[ ! -x "$flutter_bin" ]]'),
            script.index('"$dart_bin" pub --suppress-analytics'),
        )
        self.assertIn('"$flutter_bin" --version', script)
        self.assertNotIn("flutter test --help", script)

    def test_flutter_sdk_bootstrap_invokes_dart_and_cli_for_posix_and_windows_paths(self):
        script = (
            REPO_ROOT / "scripts" / "bootstrap_flutter_sdk.sh"
        )

        with tempfile.TemporaryDirectory(prefix="flutter sdk bootstrap ") as tmp:
            temp_root = Path(tmp)
            sdk_root = temp_root / "fake Flutter SDK"
            bash_sdk_root = bash_compatible_path(sdk_root)
            flutter_tools_path = (
                f"{sdk_root.as_posix()}/packages/flutter_tools"
                if os.name == "nt"
                else f"{bash_sdk_root}/packages/flutter_tools"
            )
            expected_args = [
                "pub",
                "--suppress-analytics",
                "--directory",
                flutter_tools_path,
                "get",
                "--example",
            ]

            dart = sdk_root / "bin" / "cache" / "dart-sdk" / "bin" / (
                "dart.exe" if os.name == "nt" else "dart"
            )
            dart.parent.mkdir(parents=True, exist_ok=True)
            if os.name == "nt":
                # Compile a native fake dart.exe so Windows exercises the complete bootstrap invocation.
                # Biên dịch dart.exe giả để Windows chạy trọn luồng bootstrap.
                fake_dart_source = temp_root / "fake_dart.rs"
                fake_dart_source.write_text(
                    r"""use std::{env, fs::{File, OpenOptions}, io::Write};
fn main() {
    let mut capture = File::create(env::var("CAPTURED_ARGS").unwrap()).unwrap();
    for argument in env::args().skip(1) { writeln!(capture, "{argument}").unwrap(); }
    let mut steps = OpenOptions::new().create(true).append(true).open(env::var("BOOTSTRAP_STEPS").unwrap()).unwrap();
    writeln!(steps, "dart").unwrap();
}
""",
                    encoding="utf-8",
                )
                compiled = subprocess.run(
                    ["rustc", str(fake_dart_source), "--edition=2021", "-o", str(dart)],
                    cwd=temp_root,
                    capture_output=True,
                    text=True,
                    check=False,
                )
                self.assertEqual(
                    compiled.returncode,
                    0,
                    f"stdout={compiled.stdout!r}; stderr={compiled.stderr!r}",
                )
            else:
                dart.write_text(
                    "#!/bin/sh\n"
                    "printf '%s\\n' \"$@\" > \"$CAPTURED_ARGS\"\n"
                    "printf '%s\\n' dart >> \"$BOOTSTRAP_STEPS\"\n",
                    encoding="utf-8",
                )
                dart.chmod(0o755)

            flutter = sdk_root / "bin" / "flutter"
            flutter_capture = temp_root / "flutter-args.txt"
            flutter.write_text(
                "#!/bin/sh\n"
                "printf '%s\\n' \"$@\" > \"$FLUTTER_CAPTURED_ARGS\"\n"
                "printf '%s\\n' flutter >> \"$BOOTSTRAP_STEPS\"\n",
                encoding="utf-8",
            )
            flutter.chmod(0o755)

            capture = temp_root / "args-Linux.txt"
            bootstrap_steps = temp_root / "bootstrap-steps.txt"
            env = os.environ.copy()
            env.update(
                {
                    "FLUTTER_ROOT": bash_sdk_root,
                    "RUNNER_OS": "Windows" if os.name == "nt" else "Linux",
                    "CAPTURED_ARGS": bash_compatible_path(capture),
                    "FLUTTER_CAPTURED_ARGS": bash_compatible_path(flutter_capture),
                    "BOOTSTRAP_STEPS": bash_compatible_path(bootstrap_steps),
                }
            )
            if os.name == "nt":
                self.assertIsNotNone(
                    bootstrap_bash_command(),
                    "Git Bash was not found beside the Git for Windows installation",
                )
                self.assertNotEqual(
                    bootstrap_bash_command(),
                    "bash",
                    "Windows must invoke the selected Git Bash executable",
                )
            result = subprocess.run(
                [bootstrap_bash_command(), bash_compatible_path(script)],
                cwd=temp_root,
                env=env,
                capture_output=True,
                text=True,
                check=False,
            )

            self.assertEqual(
                result.returncode,
                0,
                f"stdout={result.stdout!r}; stderr={result.stderr!r}",
            )
            captured_args = capture.read_text(encoding="utf-8").splitlines()
            if os.name == "nt":
                captured_args[3] = PureWindowsPath(captured_args[3]).as_posix()
            self.assertEqual(captured_args, expected_args)
            self.assertEqual(
                flutter_capture.read_text(encoding="utf-8").splitlines(), ["--version"]
            )
            self.assertEqual(
                bootstrap_steps.read_text(encoding="utf-8").splitlines(),
                ["dart", "flutter"],
            )

            windows_sdk_root = "C:/ci/fake Flutter SDK"
            resolve_windows_dart = (
                'cygpath() { [ "$1" = -u ] && [ "$2" = "C:/ci/fake Flutter SDK" ] || return 2; '
                'printf "%s\\n" "/c/ci/fake Flutter SDK"; }; '
                'source scripts/bootstrap_flutter_sdk.sh; '
                'sdk_root="$(normalize_flutter_sdk_root "$1" "$2")"; '
                'resolve_flutter_sdk_dart_bin "$sdk_root" "$2"'
            )
            resolved = subprocess.run(
                [
                    bootstrap_bash_command(),
                    "-c",
                    resolve_windows_dart,
                    "bootstrap-test",
                    windows_sdk_root,
                    "Windows",
                ],
                cwd=script.parents[1],
                env=env,
                capture_output=True,
                text=True,
                check=False,
            )
            expected_windows_bin = (
                "/c/ci/fake Flutter SDK/bin/cache/dart-sdk/bin/dart.exe"
            )
            self.assertEqual(resolved.returncode, 0, resolved.stderr)
            self.assertEqual(resolved.stdout.strip(), expected_windows_bin)
