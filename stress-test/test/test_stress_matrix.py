"""Unit checks for the cross-framework stress runner.
Unit test cho runner stress nhiều framework.
"""

from __future__ import annotations

import importlib.util
import hashlib
import json
import os
import subprocess
import sys
import tempfile
import tomllib
import unittest
from unittest.mock import patch
from pathlib import Path


SOURCE = Path(__file__).resolve().parents[1] / "src" / "stress_matrix.py"
SPEC = importlib.util.spec_from_file_location("stress_matrix", SOURCE)
assert SPEC is not None and SPEC.loader is not None
stress_matrix = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = stress_matrix
SPEC.loader.exec_module(stress_matrix)


def complete_web_lifecycle_records(binary: Path) -> list[dict[str, object]]:
    """Build the exact schema emitted by the native web E2E helper.
    Tạo schema bằng chứng giống hệt helper E2E web native.
    """
    return [
        {
            "schema_version": 1,
            "core": "web",
            "framework": framework,
            "suite": suite,
            "binary_path": str(binary.resolve()),
            "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
            "category": category,
            "passed": True,
            "returncode": 0,
            "detail": f"{category} succeeded",
        }
        for framework, suite in stress_matrix.WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK.items()
        for category in stress_matrix.FRAMEWORK_RUNTIME_CATEGORIES
    ]


class StressMatrixTests(unittest.TestCase):
    """Guard fail-closed inventory and scaffold assertions.
    Khóa kiểm tra fail-closed cho inventory và scaffold.
    """

    def test_scaffold_requires_identity_and_real_payload(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            project = Path(temporary) / "sample"
            project.mkdir()
            (project / ".mgc.core").write_text("web\n", encoding="utf-8")
            (project / "mgc.toml").write_text(
                'name = "sample"\necosystem = "web"\nframeworks = ["react-vite"]\n',
                encoding="utf-8",
            )
            self.assertIn("no non-empty", stress_matrix.validate_scaffold(project, "web", "react-vite"))
            (project / "package.json").write_text('{"name":"sample"}\n', encoding="utf-8")
            self.assertIn("no non-empty", stress_matrix.validate_scaffold(project, "web", "react-vite"))
            (project / "src").mkdir()
            (project / "src" / "main.ts").write_text("console.log('ok');\n", encoding="utf-8")
            self.assertIsNone(stress_matrix.validate_scaffold(project, "web", "react-vite"))

    def test_scaffold_rejects_wrong_core_and_framework(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            project = Path(temporary) / "sample"
            project.mkdir()
            (project / ".mgc.core").write_text("lib\n", encoding="utf-8")
            (project / "mgc.toml").write_text(
                'name = "sample"\necosystem = "lib"\nframeworks = ["rust"]\n',
                encoding="utf-8",
            )
            (project / "Cargo.toml").write_text("[package]\nname='sample'\n", encoding="utf-8")
            self.assertIn(".mgc.core", stress_matrix.validate_scaffold(project, "web", "rust"))
            self.assertIn("framework identity", stress_matrix.validate_scaffold(project, "lib", "go"))

    def test_web_alias_identity_maps_only_to_a_known_scaffold(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            project = Path(temporary) / "sample"
            project.mkdir()
            (project / ".mgc.core").write_text("web\n", encoding="utf-8")
            (project / "mgc.toml").write_text(
                'ecosystem = "web"\nmode = "backend"\nframeworks = ["node", "express"]\n',
                encoding="utf-8",
            )
            (project / "src/server.js").parent.mkdir()
            (project / "src/server.js").write_text("console.log('ready')\n", encoding="utf-8")
            self.assertIsNone(stress_matrix.validate_scaffold(project, "web", "backend"))
            self.assertIn(
                "framework identity",
                stress_matrix.validate_scaffold(project, "web", "custom"),
            )

    def test_iot_board_alias_uses_the_iot_board_field(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            project = Path(temporary) / "sample"
            project.mkdir()
            (project / ".mgc.core").write_text("iot\n", encoding="utf-8")
            (project / "mgc.toml").write_text(
                'ecosystem = "iot"\nframeworks = ["esp32-rust"]\n\n'
                '[iot]\nframework = "esp32-rust"\nboard = "esp32c3"\n',
                encoding="utf-8",
            )
            (project / "src").mkdir()
            (project / "src/main.rs").write_text("fn main() {}\n", encoding="utf-8")
            self.assertIsNone(stress_matrix.validate_scaffold(project, "iot", "esp32c3"))
            self.assertIn(
                "framework identity",
                stress_matrix.validate_scaffold(project, "iot", "esp32s3"),
            )

    def test_scaffold_language_inventory_uses_source_files_and_skips_managed_payloads(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            project = Path(temporary)
            (project / "src").mkdir()
            (project / "src/main.ts").write_text("export {}\n", encoding="utf-8")
            (project / "src/main.rs").write_text("fn main() {}\n", encoding="utf-8")
            (project / ".magicore").mkdir()
            (project / ".magicore/hidden.py").write_text("print('hidden')\n", encoding="utf-8")
            (project / "node_modules/pkg").mkdir(parents=True)
            (project / "node_modules/pkg/index.js").write_text("module.exports = {}\n", encoding="utf-8")

            languages, suffixes = stress_matrix.scaffold_language_files(project, "web")

        self.assertEqual(languages, {"Rust": 1, "TypeScript": 1})
        self.assertEqual(suffixes, {".rs": 1, ".ts": 1})
        self.assertEqual(
            set(stress_matrix.SOURCE_LANGUAGE_BY_SUFFIX),
            stress_matrix.SOURCE_SUFFIXES,
        )

        with tempfile.TemporaryDirectory() as temporary:
            cicd = Path(temporary) / "cicd"
            workflow = cicd / ".github/workflows/ci.yml"
            workflow.parent.mkdir(parents=True)
            workflow.write_text("name: CI\n", encoding="utf-8")
            hardware = Path(temporary) / "hardware"
            hardware.mkdir()
            (hardware / "bench.json").write_text('{"kind":"bench"}\n', encoding="utf-8")
            cicd_languages, _ = stress_matrix.scaffold_language_files(cicd, "cicd")
            hardware_languages, _ = stress_matrix.scaffold_language_files(hardware, "hardware")

        self.assertEqual(cicd_languages, {"CI workflow YAML": 1})
        self.assertEqual(hardware_languages, {"JSON hardware descriptor": 1})

    def test_isolated_environment_redirects_language_caches_and_temp(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            with patch.dict(
                os.environ,
                {"MGC_STRESS_LIFECYCLE_EVIDENCE": str(root / "private-sidecar.jsonl")},
            ):
                env = stress_matrix.isolated_environment(root)
            self.assertNotIn("MGC_STRESS_LIFECYCLE_EVIDENCE", env)
            for key in (
                "HOME",
                "MGC_DATA_DIR",
                "MGC_CACHE_DIR",
                "TMPDIR",
                "TEMP",
                "TMP",
                "XDG_CACHE_HOME",
                "CARGO_HOME",
                "CARGO_TARGET_DIR",
                "GOCACHE",
                "GOMODCACHE",
                "DOTNET_CLI_HOME",
                "NUGET_PACKAGES",
            ):
                with self.subTest(key=key):
                    self.assertTrue(Path(env[key]).is_relative_to(root))
            expected_rustup_home = os.environ.get("RUSTUP_HOME") or str(
                Path(os.environ.get("HOME") or Path.home()).expanduser() / ".rustup"
            )
            self.assertEqual(Path(env["RUSTUP_HOME"]), Path(expected_rustup_home))

    def test_language_report_summary_does_not_claim_runtime_support(self) -> None:
        evidence = json.dumps(
            {
                "languages_or_source_formats": ["Rust", "TypeScript"],
                "source_files_by_language_or_format": {"Rust": 2, "TypeScript": 1},
                "source_suffix_counts": {".rs": 2, ".ts": 1},
                "runtime_verified": False,
            },
            sort_keys=True,
        )
        summary = stress_matrix.source_language_inventory_summary(
            [
                stress_matrix.Result(
                    "framework-language-scaffold",
                    "web/vanilla",
                    True,
                    evidence,
                )
            ]
        )

        self.assertFalse(summary["runtime_verified"])
        self.assertEqual(summary["frameworks_with_inventory"], 1)
        self.assertEqual(summary["by_language_or_format"]["Rust"]["source_files"], 2)
        self.assertEqual(summary["frameworks"][0]["subject"], "web/vanilla")

    def test_capability_inventory_preserves_all_declared_operation_owners(self) -> None:
        ownership = {
            operation: {"owner": "unsupported"}
            for operation in stress_matrix.DEPENDENCY_OPERATIONS
        }
        ownership["install"] = {"owner": "mgc-native", "requires": "package.json"}
        payload = {
            "version": "1.1.0-rc.9",
            "cores": [
                {
                    "core": core,
                    "framework_qualification": [
                        {
                            "framework": f"{core}-sample",
                            "status": "scaffold-only",
                            "dependency_ownership": ownership,
                        }
                    ],
                }
                for core in stress_matrix.CORE_NAMES
            ],
        }
        completed = subprocess.CompletedProcess(
            ["mgc", "capabilities", "--json"], 0, json.dumps(payload)
        )

        with patch.object(stress_matrix, "run_command", return_value=completed):
            version, rows = stress_matrix.capability_rows(Path("mgc"), Path("."), {})

        self.assertEqual(version, "1.1.0-rc.9")
        self.assertEqual(len(rows), len(stress_matrix.CORE_NAMES))
        self.assertEqual(
            rows[0]["dependency_ownership"]["install"],
            {"owner": "mgc-native", "requires": "package.json"},
        )
        self.assertEqual(
            set(rows[0]["dependency_ownership"]),
            set(stress_matrix.DEPENDENCY_OPERATIONS),
        )

    def test_capability_inventory_rejects_incomplete_or_malformed_owners(self) -> None:
        def payload_with_ownership(ownership):
            return {
                "version": "1.1.0-rc.9",
                "cores": [
                    {
                        "core": core,
                        "framework_qualification": [
                            {
                                "framework": f"{core}-sample",
                                "status": "scaffold-only",
                                "dependency_ownership": ownership,
                            }
                        ],
                    }
                    for core in stress_matrix.CORE_NAMES
                ],
            }

        valid_owners = {
            operation: {"owner": "unsupported"}
            for operation in stress_matrix.DEPENDENCY_OPERATIONS
        }
        incomplete = dict(valid_owners)
        incomplete.pop("gc")
        malformed = dict(valid_owners)
        malformed["install"] = {"owner": []}

        for ownership, expected in (
            (incomplete, "missing gc"),
            (malformed, "invalid owner"),
        ):
            with self.subTest(expected=expected):
                completed = subprocess.CompletedProcess(
                    ["mgc", "capabilities", "--json"],
                    0,
                    json.dumps(payload_with_ownership(ownership)),
                )
                with patch.object(stress_matrix, "run_command", return_value=completed):
                    with self.assertRaisesRegex(RuntimeError, expected):
                        stress_matrix.capability_rows(Path("mgc"), Path("."), {})

        completed = subprocess.CompletedProcess(
            ["mgc", "capabilities", "--json"], 0, "[]"
        )
        with patch.object(stress_matrix, "run_command", return_value=completed):
            with self.assertRaisesRegex(RuntimeError, "root must be an object"):
                stress_matrix.capability_rows(Path("mgc"), Path("."), {})

    def test_capability_inventory_rejects_unexpected_core_even_without_frameworks(self) -> None:
        payload = {
            "version": "1.1.0-rc.9",
            "cores": [
                {
                    "core": core,
                    "framework_qualification": [
                        {
                            "framework": f"{core}-sample",
                            "status": "scaffold-only",
                            "dependency_ownership": {
                                operation: {"owner": "unsupported"}
                                for operation in stress_matrix.DEPENDENCY_OPERATIONS
                            },
                        }
                    ],
                }
                for core in stress_matrix.CORE_NAMES
            ]
            + [{"core": "new-core", "framework_qualification": []}],
        }
        completed = subprocess.CompletedProcess(
            ["mgc", "capabilities", "--json"], 0, json.dumps(payload)
        )

        with patch.object(stress_matrix, "run_command", return_value=completed):
            with self.assertRaisesRegex(RuntimeError, "unsupported core.*new-core"):
                stress_matrix.capability_rows(Path("mgc"), Path("."), {})

    def test_framework_inventory_baseline_matches_all_rows_and_owners(self) -> None:
        baseline = json.loads(
            stress_matrix.DEFAULT_INVENTORY_BASELINE.read_text(encoding="utf-8")
        )
        rows = [
            {
                "core": item["core"],
                "framework": item["framework"],
                "status": item["status"],
                "dependency_ownership": {
                    operation: {"owner": owner}
                    for operation, owner in item["dependency_ownership"].items()
                },
            }
            for item in baseline["frameworks"]
        ]

        matched = stress_matrix.check_framework_inventory(
            rows, stress_matrix.DEFAULT_INVENTORY_BASELINE
        )

        self.assertEqual(matched, 86)

    def test_framework_inventory_baseline_rejects_silent_scope_or_owner_drift(self) -> None:
        original = json.loads(
            stress_matrix.DEFAULT_INVENTORY_BASELINE.read_text(encoding="utf-8")
        )
        rows = [
            {
                "core": item["core"],
                "framework": item["framework"],
                "status": item["status"],
                "dependency_ownership": {
                    operation: {"owner": owner}
                    for operation, owner in item["dependency_ownership"].items()
                },
            }
            for item in original["frameworks"]
        ]

        def change_owner(target_rows):
            row = next(
                item for item in target_rows
                if item["dependency_ownership"]["add"]["owner"] != "unsupported"
            )
            previous = row["dependency_ownership"]["add"]["owner"]
            row["dependency_ownership"]["add"]["owner"] = next(
                owner for owner in stress_matrix.DEPENDENCY_OWNERS
                if owner != previous
            )

        mutations = (
            (lambda target_rows: target_rows.pop(), "missing"),
            (
                lambda target_rows: target_rows.append(
                    {
                        **target_rows[0],
                        "framework": "unexpected-new-framework",
                    }
                ),
                "unexpected",
            ),
            (
                lambda target_rows: target_rows[0].update(status="mgc-engine-path"),
                "qualification status",
            ),
            (change_owner, "add owner"),
        )
        for mutate, expected_error in mutations:
            with self.subTest(expected_error=expected_error):
                changed_rows = json.loads(json.dumps(rows))
                mutate(changed_rows)
                with self.assertRaisesRegex(RuntimeError, expected_error):
                    stress_matrix.check_framework_inventory(
                        changed_rows, stress_matrix.DEFAULT_INVENTORY_BASELINE
                    )

    def test_framework_inventory_baseline_rejects_malformed_rows_and_metadata(self) -> None:
        original = json.loads(
            stress_matrix.DEFAULT_INVENTORY_BASELINE.read_text(encoding="utf-8")
        )
        rows = [
            {
                "core": item["core"],
                "framework": item["framework"],
                "status": item["status"],
                "dependency_ownership": {
                    operation: {"owner": owner}
                    for operation, owner in item["dependency_ownership"].items()
                },
            }
            for item in original["frameworks"]
        ]

        def append_malformed_row(data):
            data["frameworks"].append(None)
            data["captured_from"]["framework_count"] += 1

        mutations = (
            (append_malformed_row, "malformed row"),
            (lambda data: data["captured_from"].update(framework_count=1), "metadata"),
            (
                lambda data: data["captured_from"].update(
                    framework_count=float(data["captured_from"]["framework_count"])
                ),
                "metadata",
            ),
            (lambda data: data.update(schema_version=True), "unsupported schema"),
        )
        for mutate, expected_error in mutations:
            with self.subTest(expected_error=expected_error), tempfile.TemporaryDirectory() as temporary:
                changed = json.loads(json.dumps(original))
                mutate(changed)
                manifest = Path(temporary) / "inventory.json"
                manifest.write_text(json.dumps(changed), encoding="utf-8")
                with self.assertRaisesRegex(RuntimeError, expected_error):
                    stress_matrix.check_framework_inventory(rows, manifest)

    def test_hardware_payload_requires_a_valid_matching_descriptor(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            project = Path(temporary)
            (project / "bench.json").write_text(
                '{"name":"demo","kind":"optimizer"}\n', encoding="utf-8"
            )
            self.assertFalse(stress_matrix.has_payload(project, "hardware"))
            (project / "bench.json").write_text(
                '{"name":"demo","kind":"bench"}\n', encoding="utf-8"
            )
            self.assertTrue(stress_matrix.has_payload(project, "hardware"))

    def test_project_slug_is_a_single_portable_segment(self) -> None:
        slug = stress_matrix.project_slug("web", "React + Vite")
        self.assertEqual(slug, "mgc-web-react-vite")
        self.assertNotIn("/", slug)
        self.assertNotIn("\\", slug)

    def test_help_parser_reads_commands_and_ignores_the_builtin_help_alias(self) -> None:
        output = """Usage: mgc cache [COMMAND]

Commands:
  status      Inspect cache status
  clean       Remove cache entries
  help        Print this message

Options:
  -h, --help  Print help
"""
        self.assertEqual(
            stress_matrix.parse_help_commands(output), ["status", "clean", "help"]
        )

    def test_cli_contract_walks_nested_help_paths_and_checks_invalid_options(self) -> None:
        calls: list[list[str]] = []
        help_outputs = {
            ("--help",): "Usage: mgc [COMMAND]\n\nCommands:\n  alpha  Alpha command\n  beta   Beta command\n  help   Help command\n",
            ("help",): "Usage: mgc [COMMAND]\n\nCommands:\n  alpha  Alpha command\n",
            ("alpha", "--help"): "Usage: mgc alpha [COMMAND]\n\nCommands:\n  beta  Nested command\n",
            ("beta", "--help"): "Usage: mgc beta\n",
            ("alpha", "beta", "--help"): "Usage: mgc alpha beta\n",
        }

        def fake_run(argv, **kwargs):
            command = tuple(str(part) for part in argv[1:])
            calls.append(list(command))
            if command == ("--definitely-invalid-mgc-option",) or command[-1:] == (
                "--definitely-invalid-mgc-option",
            ):
                return subprocess.CompletedProcess(
                    argv,
                    2,
                    "error: unexpected argument '--definitely-invalid-mgc-option' found\n"
                    "Usage: mgc [OPTIONS]\n",
                )
            return subprocess.CompletedProcess(
                argv, 0 if command in help_outputs else 2,
                help_outputs.get(command, "unexpected command path"),
            )

        with patch.object(stress_matrix, "run_command", side_effect=fake_run):
            results = stress_matrix.run_cli_contracts(
                Path("mgc"), Path("."), {}
            )

        self.assertTrue(all(result.passed for result in results), results)
        self.assertIn(["alpha", "beta", "--help"], calls)
        self.assertIn(["help"], calls)
        self.assertIn(["--definitely-invalid-mgc-option"], calls)
        self.assertIn(["alpha", "--definitely-invalid-mgc-option"], calls)
        self.assertIn(["alpha", "beta", "--definitely-invalid-mgc-option"], calls)
        self.assertEqual(
            sum(result.category == "cli-command-help" for result in results), 4
        )
        self.assertEqual(
            sum(result.category == "security-cli-command" for result in results), 4
        )

    def test_workflow_runs_all_real_native_web_lifecycle_targets_on_three_os(self) -> None:
        repository = SOURCE.parents[2]
        workflow = (repository / ".github/workflows/stress-matrix.yml").read_text(
            encoding="utf-8"
        )
        expected_frontend_targets = {
            "web_fw_fe_angular",
            "web_fw_fe_astro",
            "web_fw_fe_nextjs",
            "web_fw_fe_react_vite",
            "web_fw_fe_remix",
            "web_fw_fe_sveltekit",
            "web_fw_fe_vue_vite",
        }

        self.assertIn("os: [ubuntu-24.04, macos-latest, windows-latest]", workflow)
        self.assertIn('MGC_PYTHON_VERSION: "3.12"', workflow)
        self.assertIn('python-version: "3.12"', workflow)
        self.assertEqual(workflow.count('".github/workflows/release.yml"'), 2)
        self.assertIn("actions/setup-go@b7ad1dad31e06c5925ef5d2fc7ad053ef454303e", workflow)
        self.assertIn("actions/setup-java@de7274f081f381c8f8158605e0321c36c376e2e6", workflow)
        self.assertIn('java-version: "25.0.4"', workflow)
        self.assertIn(
            "Set up Kotlin compiler for Kotlin runtime probes (Linux/macOS)", workflow
        )
        self.assertIn(
            "Set up Kotlin compiler for Kotlin runtime probes (Windows)", workflow
        )
        self.assertIn(
            "7cc140e76daf416a0424a5557f99afd7ca89ebd463ea1101261e3e01c33e75cd",
            workflow,
        )
        self.assertIn(
            "shivammathur/setup-php@f3e473d116dcccaddc5834248c87452386958240",
            workflow,
        )
        self.assertIn('php-version: "8.5"', workflow)
        self.assertIn("tools: none", workflow)
        self.assertIn("actions/setup-dotnet@a98b56852c35b8e3190ac28c8c2271da59106c68", workflow)
        self.assertIn('dotnet-version: "10.0.401"', workflow)
        self.assertIn("subosito/flutter-action@44ac965b96f18d999802d4b807e3256d5a3f9fa1", workflow)
        self.assertIn("swift-actions/setup-swift@7ca6abe6b3b0e8b5421b88be48feee39cbf52c6a", workflow)
        self.assertIn('swift-version: "6.2.1"', workflow)
        self.assertIn("Set up Swift toolchain (Windows)", workflow)
        self.assertIn("Set up Zig for Zig runtime probes", workflow)
        self.assertIn('version: "0.17.0"', workflow)
        self.assertIn("mlugg/setup-zig@d1434d08867e3ee9daa34448df10607b98908d29", workflow)
        self.assertIn("Set up Terraform for HCL syntax probes", workflow)
        self.assertIn("Set up Godot for GDScript runtime probes (Linux/macOS)", workflow)
        self.assertIn("Set up Godot for GDScript runtime probes (Windows)", workflow)
        self.assertIn("Exposing the Windows GCC toolchain", workflow)
        self.assertIn("9aa00f7a605200940bce3027a567b782f49bd8e940dd06ae9e987bd65aee1b1467edd56ed84fcdcbdd44354bf613bdbb4e5d2913e925850368e150c59ed54c65", workflow)
        self.assertIn("38aa16e5bba2083941fc5b3e54be0089bd4cc35e32415f5b9fd9a8a6a7b9818255d44532ea8ef94b5aef56c4b407c2d634fa4f657e4ebe681ebbf59b7bac69ca", workflow)
        self.assertIn("83decd58fdf67b9d657958a1ae6bf1929c20785315a81effe245874cdc57acb709bf868e00778a96984338c1b29dafdb453c6847747694621c6ecf5da2259993", workflow)
        self.assertIn("71fc43d92ea09907be5d416d2405a6a9c2d1ceaed633f5e175c0af26e8c4b365", workflow)
        self.assertIn("8362e7284b38a1194884963deed83481696d468b42dab88052775f4280383584", workflow)
        self.assertIn("5ef8e19091106b1921af26db5bcee3cd84a475eae2fd190fc02b1049b320d042", workflow)
        self.assertIn("e75a97238f2dc43ab90fca1af554c18f995b16a4b06a4ec27bcdbab52c1d2dc8", workflow)
        preflight = workflow.split(
            "- name: Verify language probe toolchains", maxsplit=1
        )[1].split("- name:", maxsplit=1)[0]
        preflight_start = workflow.index("- name: Verify language probe toolchains")
        for setup_step in (
            "Set up Kotlin compiler for Kotlin runtime probes (Linux/macOS)",
            "Set up Kotlin compiler for Kotlin runtime probes (Windows)",
            "Set up PHP for PHP runtime probes",
        ):
            with self.subTest(setup_step=setup_step):
                self.assertLess(workflow.index(setup_step), preflight_start)
        common_toolchains = preflight.split("runtime_tools=(", maxsplit=1)[1].split(")", maxsplit=1)[0]
        for executable in (
            "bash", "node", "python", "go", "dart", "rustc", "cargo", "swift",
            "java", "dotnet", "php", "kotlinc", "terraform", "godot", "zig",
        ):
            with self.subTest(executable=executable):
                self.assertIn(executable, common_toolchains)
        self.assertIn("macOS)", preflight)
        self.assertIn("runtime_tools+=(clang clang++)", preflight)
        self.assertIn("Linux|Windows)", preflight)
        self.assertIn("runtime_tools+=(gcc g++)", preflight)
        self.assertNotIn("clang", common_toolchains)
        self.assertNotIn("gcc", common_toolchains)
        inventory_step = workflow.split(
            "Create every advertised framework and run CLI dimensions", maxsplit=1
        )[1].split("Upload stress evidence", maxsplit=1)[0]
        self.assertIn("--require-lockfile-evidence", inventory_step)
        self.assertIn("--require-full-evidence", inventory_step)
        self.assertIn("--require-language-runtime-evidence", inventory_step)
        self.assertIn("--require-universal-support", inventory_step)
        self.assertNotIn("require_language_runtime_evidence:", workflow)
        self.assertNotIn("require_universal_support:", workflow)
        self.assertIn("workflow_call:", workflow)
        self.assertGreater(
            workflow.index("Create every advertised framework and run CLI dimensions"),
            workflow.index("Verify the V4 add-install-update-remove CLI lifecycle"),
        )
        frontend_step = workflow.split(
            "Run all dedicated frontend lifecycle E2Es", maxsplit=1
        )[1].split("timeout-minutes: 90", maxsplit=1)[0]
        self.assertIn("shell: bash", frontend_step)
        self.assertIn("cargo test --locked -p mgc --test \"${target}\"", frontend_step)
        for target in sorted(expected_frontend_targets):
            with self.subTest(target=target):
                self.assertIn(target, frontend_step)
        self.assertIn("--test web_fw_native_matrix", workflow)
        self.assertIn("Run FastAPI native backend install and dev lifecycle E2E", workflow)
        self.assertIn(
            "cargo test --locked --no-default-features --features web -p mgc --test web_fw_fastapi",
            workflow,
        )
        self.assertIn("Verify Python runtime paths in the single-core AI distribution", workflow)
        self.assertIn("--no-default-features --features ai", workflow)
        self.assertIn("MGC_STRESS_LIFECYCLE_EVIDENCE", workflow)
        self.assertIn("mgc-framework-lifecycle.jsonl", workflow)
        self.assertIn("Clear native web lifecycle evidence sidecar", workflow)
        upload = workflow.split("- name: Upload stress evidence", maxsplit=1)[1]
        self.assertIn("${{ runner.temp }}/mgc-framework-lifecycle.jsonl", upload)

    def test_publication_requires_the_reusable_universal_support_evidence_gate(self) -> None:
        repository = SOURCE.parents[2]
        release_workflow = (repository / ".github/workflows/release.yml").read_text(
            encoding="utf-8"
        )
        gate = release_workflow.split(
            "  release-universal-support-evidence-gate:", maxsplit=1
        )[1].split("\n  publish-github-release:", maxsplit=1)[0]
        publish = release_workflow.split(
            "  publish-github-release:", maxsplit=1
        )[1].split("\n  verify-public-download:", maxsplit=1)[0]

        self.assertIn("uses: ./.github/workflows/stress-matrix.yml", gate)
        publish_needs = publish.split("needs:", maxsplit=1)[1].split("\n", maxsplit=1)[0]
        self.assertIn("release-universal-support-evidence-gate", publish_needs)
        self.assertIn("needs.release-universal-support-evidence-gate.result == 'success'", publish)

        stress_workflow = (repository / ".github/workflows/stress-matrix.yml").read_text(
            encoding="utf-8"
        )
        matrix_step = stress_workflow.split(
            "Create every advertised framework and run CLI dimensions", maxsplit=1
        )[1].split("Upload stress evidence", maxsplit=1)[0]
        self.assertIn("--require-universal-support", matrix_step)
        self.assertIn("--require-language-runtime-evidence", matrix_step)

    def test_single_core_web_feature_includes_native_backend_runtime_without_lib_cli(self) -> None:
        repository = SOURCE.parents[2]
        cli_manifest = tomllib.loads((repository / "cli/Cargo.toml").read_text(encoding="utf-8"))
        package_manifest = tomllib.loads(
            (repository / "packaging/packages/magicore-web.toml").read_text(encoding="utf-8")
        )
        features = cli_manifest["features"]

        self.assertIn("web-backend-runtime", features["web"])
        self.assertIn("native-python-runtime", features["web-backend-runtime"])
        self.assertIn("mgc-lib-adapter", features["native-python-runtime"])
        self.assertIn("native-python-runtime", features["ai"])
        self.assertEqual(package_manifest["features"], ["web"])
        self.assertNotIn("lib", package_manifest["features"])

    def test_ci_lifecycle_and_strict_gate_use_the_same_frozen_binary(self) -> None:
        workflow = (SOURCE.parents[2] / ".github/workflows/stress-matrix.yml").read_text()
        self.assertIn("shutil.copy2(source, destination)", workflow)
        self.assertIn("MGC_STRESS_CLI_SNAPSHOT={destination}", workflow)
        for name in ("Verify the consolidated 11 native web framework lifecycle E2Es",
                     "Run all dedicated frontend lifecycle E2Es, even after a lane fails"):
            block = workflow.split("- name: " + name, 1)[1].split("\n      - name:", 1)[0]
            self.assertIn("MGC_TEST_CLI_BINARY: ${{ env.MGC_STRESS_CLI_SNAPSHOT }}", block)
        block = workflow.split("- name: Create every advertised framework and run CLI dimensions", 1)[1]
        self.assertIn('--binary "${MGC_STRESS_CLI_SNAPSHOT}"', block)
        for name in ("Run FastAPI native backend install and dev lifecycle E2E",
                     "Verify Python runtime paths in the single-core AI distribution",
                     "Verify App-only Flutter offline replay under V3 and V4 locks"):
            block = workflow.split("- name: " + name, 1)[1].split("\n      - name:", 1)[0]
            self.assertNotIn("MGC_TEST_CLI_BINARY:", block)

    def test_native_web_dev_ports_are_unique_rule_permutations(self) -> None:
        import re
        source = (SOURCE.parents[2] / "cli/tests/common/mod.rs").read_text(encoding="utf-8")
        block = source.split("pub fn native_web_dev_port", 1)[1].split("/// Run a real", 1)[0]
        ports = re.findall(r'"[^"\n]+" => (\d+)', block)
        self.assertGreaterEqual(len(ports), 18)
        self.assertEqual(len(ports), len(set(ports)))
        self.assertNotIn("4315", ports, "registry port must remain reserved in seeded CI")
        for port in ports:
            self.assertEqual(sorted(port), sorted("4315"), port)

    def test_macos_native_framework_probe_raises_file_limit_before_spawn(self) -> None:
        workflow = (SOURCE.parents[2] / ".github/workflows/stress-matrix.yml").read_text(encoding="utf-8")
        self.assertIn('MGC_STRESS_MIN_OPEN_FILES: "4096"', workflow)
        for step in (
            "Verify the consolidated 11 native web framework lifecycle E2Es",
            "Run all dedicated frontend lifecycle E2Es, even after a lane fails",
        ):
            with self.subTest(step=step):
                block = workflow.split("- name: " + step, 1)[1].split("\n      - name:", 1)[0]
                self.assertIn("shell: bash", block)
                self.assertIn('"${RUNNER_OS}" == "macOS"', block)
                self.assertIn('current_limit < MGC_STRESS_MIN_OPEN_FILES', block)
                self.assertIn('ulimit -Sn "${MGC_STRESS_MIN_OPEN_FILES}"', block)
                self.assertLess(block.index("ulimit -Sn"), block.index("cargo test"))
                self.assertIn("-- --test-threads=1", block)

    def test_ci_runs_sdk_qualified_unix_go_cold_cache_regression(self) -> None:
        workflow = (SOURCE.parents[2] / ".github/workflows/stress-matrix.yml").read_text()
        block = workflow.split("- name: Verify Go readonly execution with cold compiler caches", 1)[1].split("\n      - name:", 1)[0]
        self.assertNotIn("if: runner.os", block)
        self.assertIn("shutil.which('go')", block)
        self.assertIn("print(pathlib.Path(executable).resolve())", block)
        self.assertIn("MGC_E2E_GO_BINARY", block)
        self.assertIn("--test test_run", block)
        self.assertIn("real_go_readonly_run_survives_cold_compiler_processes", block)
        self.assertIn("--test test_protocols_go", block)
        self.assertIn("materialized_go_module_imports_with_real_sdk_without_rewriting_hash", block)
        self.assertIn("-- --ignored --test-threads=1", block)

    def test_ci_requires_real_dart_import_in_app_only_distribution(self) -> None:
        workflow = (SOURCE.parents[2] / ".github/workflows/stress-matrix.yml").read_text()
        block = workflow.split("- name: Verify App-only offline packages import with real Dart", 1)[1].split("\n      - name:", 1)[0]
        self.assertIn("MGC_E2E_DART_BINARY", block)
        self.assertIn("if not dart.is_file():", block)
        self.assertIn("--no-default-features --features app", block)
        self.assertIn("flutter_offline_replay_imports_package_with_real_dart", block)
        self.assertIn("-- --ignored --test-threads=1", block)
        self.assertNotIn("MGC_TEST_CLI_BINARY", block)
        source = (SOURCE.parents[2] / "cli/tests/app_native_install_e2e.rs").read_text(encoding="utf-8")
        self.assertIn('std::env::var("MGC_E2E_DART_BINARY").expect', source)
        self.assertIn('import \'package:meta/meta.dart\'', source)
        self.assertRegex(source, r'assert_eq!\(\s*code,\s*Some\(0\),\s*"Dart package import failed')

    def test_web_lifecycle_sidecar_rejects_replaced_binary_and_missing_digest(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            binary = Path(temporary) / "mgc"
            binary.write_bytes(b"original CLI")
            records = complete_web_lifecycle_records(binary)
            sidecar = Path(temporary) / "evidence.jsonl"
            for change in ("replace", "missing"):
                with self.subTest(change=change):
                    binary.write_bytes(b"replacement CLI" if change == "replace" else b"original CLI")
                    candidate = [dict(row) for row in records]
                    if change == "missing":
                        for row in candidate:
                            row.pop("binary_sha256", None)
                    sidecar.write_text("".join(json.dumps(row) + "\n" for row in candidate))
                    result = stress_matrix.read_web_lifecycle_evidence(sidecar, binary)
                    self.assertTrue(any(not row.passed for row in result))

    def test_web_lifecycle_sidecar_imports_complete_binary_bound_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            binary = root / "mgc"
            binary.write_bytes(b"test binary")
            sidecar = root / "lifecycle.jsonl"
            records = complete_web_lifecycle_records(binary)
            sidecar.write_text(
                "".join(json.dumps(record, sort_keys=True) + "\n" for record in records),
                encoding="utf-8",
            )

            imported = stress_matrix.read_web_lifecycle_evidence(sidecar, binary)
            operations = [
                result
                for result in imported
                if result.category in stress_matrix.FRAMEWORK_RUNTIME_CATEGORIES
            ]
            source = next(
                result
                for result in imported
                if result.category == "framework-lifecycle-evidence-source"
            )

            self.assertEqual(len(records), 90)
            self.assertEqual(len(operations), 90)
            self.assertTrue(all(result.passed for result in operations))
            hono = [result for result in operations if result.subject == "web/hono"]
            self.assertEqual(
                {result.category for result in hono},
                set(stress_matrix.FRAMEWORK_RUNTIME_CATEGORIES),
            )
            self.assertTrue(source.passed, source.detail)
            summary = stress_matrix.framework_lifecycle_evidence_summary(imported)
            self.assertEqual(summary["status"], "complete")
            self.assertEqual(summary["record_count"], 90)
            self.assertEqual(len(summary["verified_frameworks"]), 18)

    def test_web_lifecycle_sidecar_rejects_wrong_binary_malformed_unknown_and_duplicate_rows(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            binary = root / "mgc"
            other_binary = root / "other-mgc"
            binary.write_bytes(b"current binary")
            other_binary.write_bytes(b"other binary")
            original = complete_web_lifecycle_records(binary)
            wrong_binary = json.loads(json.dumps(original))
            wrong_binary[0]["binary_path"] = str(other_binary.resolve())
            unknown_suite = json.loads(json.dumps(original))
            unknown_suite[0]["suite"] = "unrecognized-suite"
            duplicate = json.loads(json.dumps(original))
            duplicate.append(json.loads(json.dumps(duplicate[0])))

            cases = (
                ("wrong-binary", wrong_binary, None, "different CLI binary"),
                ("unknown-suite", unknown_suite, None, "suite does not match"),
                ("duplicate", duplicate, None, "duplicate framework-cli-install evidence"),
                ("malformed-json", original, "{not-json}\n", "malformed JSON"),
            )
            for name, records, extra, expected_error in cases:
                with self.subTest(name=name):
                    sidecar = root / f"{name}.jsonl"
                    contents = "".join(
                        json.dumps(record, sort_keys=True) + "\n" for record in records
                    )
                    if extra is not None:
                        contents += extra
                    sidecar.write_text(contents, encoding="utf-8")
                    imported = stress_matrix.read_web_lifecycle_evidence(sidecar, binary)
                    source = next(
                        result
                        for result in imported
                        if result.category == "framework-lifecycle-evidence-source"
                    )
                    self.assertFalse(source.passed)
                    self.assertIn(expected_error, source.detail)
                    if name == "duplicate":
                        duplicate_operation = next(
                            result
                            for result in imported
                            if result.subject == "web/nuxt"
                            and result.category == "framework-cli-install"
                        )
                        self.assertFalse(duplicate_operation.passed)

    def test_web_lifecycle_sidecar_is_optional_for_local_diagnostic_runs(self) -> None:
        self.assertEqual(stress_matrix.read_web_lifecycle_evidence(None, Path("mgc")), [])

    def test_web_lifecycle_sidecar_rejects_files_over_the_bounded_read_limit(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            binary = root / "mgc"
            binary.write_bytes(b"test binary")
            sidecar = root / "oversized.jsonl"
            sidecar.write_bytes(b" " * (stress_matrix.MAX_WEB_LIFECYCLE_EVIDENCE_BYTES + 1))

            imported = stress_matrix.read_web_lifecycle_evidence(sidecar, binary)
            source = next(
                result
                for result in imported
                if result.category == "framework-lifecycle-evidence-source"
            )

            self.assertFalse(source.passed)
            self.assertIn("exceeds the 1 MiB evidence limit", source.detail)
            self.assertIsNone(
                stress_matrix.framework_lifecycle_evidence_summary(imported)["sha256"]
            )

    def test_stress_workflow_requires_live_osv_for_audit_end_to_end(self) -> None:
        repository = SOURCE.parents[2]
        workflow = (repository / ".github/workflows/stress-matrix.yml").read_text(
            encoding="utf-8"
        )
        audit_step = workflow.split(
            "- name: Run optimizer and audit end-to-end suites", maxsplit=1
        )[1].split("- name:", maxsplit=1)[0]
        self.assertIn("MGC_E2E_AUDIT_NETWORK: required", audit_step)
        self.assertIn("--test audit_cli_e2e", audit_step)

    def test_cli_contract_does_not_mistake_command_failure_for_parser_rejection(self) -> None:
        def fake_run(argv, **kwargs):
            command = tuple(str(part) for part in argv[1:])
            if command == ("--help",):
                return subprocess.CompletedProcess(
                    argv, 0, "Usage: mgc [COMMAND]\n\nCommands:\n  alpha  Alpha\n"
                )
            if command == ("alpha", "--help"):
                return subprocess.CompletedProcess(argv, 0, "Usage: mgc alpha\n")
            if command == ("--definitely-invalid-mgc-option",):
                return subprocess.CompletedProcess(
                    argv, 2, "error: unexpected argument '--definitely-invalid-mgc-option'\n"
                )
            if command == ("alpha", "--definitely-invalid-mgc-option"):
                return subprocess.CompletedProcess(
                    argv, 1, "runtime error while handling --definitely-invalid-mgc-option"
                )
            return subprocess.CompletedProcess(argv, 2, "unexpected invocation")

        with patch.object(stress_matrix, "run_command", side_effect=fake_run):
            results = stress_matrix.run_cli_contracts(Path("mgc"), Path("."), {})

        alpha_result = next(
            result for result in results
            if result.category == "security-cli-command" and result.subject.startswith("mgc alpha ")
        )
        self.assertFalse(alpha_result.passed)

    def test_cli_contract_records_root_timeout_and_invalid_option_spawn_error(self) -> None:
        def failing_run(argv, **kwargs):
            command = [str(part) for part in argv[1:]]
            if command == ["--help"]:
                raise subprocess.TimeoutExpired(argv, timeout=10)
            raise OSError("binary could not be spawned")

        with patch.object(stress_matrix, "run_command", side_effect=failing_run):
            results = stress_matrix.run_cli_contracts(
                Path("missing-mgc"), Path("."), {}
            )

        self.assertEqual(
            [(result.category, result.subject) for result in results],
            [
                ("cli-help-inventory", "mgc --help"),
                ("security-cli", "unknown global option is rejected"),
            ],
        )
        self.assertTrue(all(not result.passed for result in results))

    def test_identity_parser_accepts_multiline_framework_arrays(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            config = Path(temporary) / "mgc.toml"
            config.write_text(
                'ecosystem = "web"\nframeworks = [\n  "react-vite",\n  "nextjs"\n]\n\n[execution]\nmode = "native"\n',
                encoding="utf-8",
            )
            self.assertEqual(
                stress_matrix.read_project_identity(config),
                {"ecosystem": "web", "frameworks": ["react-vite", "nextjs"]},
            )

    def test_identity_parser_accepts_toml_literal_strings(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            config = Path(temporary) / "mgc.toml"
            config.write_text(
                "ecosystem = 'web'\nframeworks = ['actix-web', 'react-vite']\n",
                encoding="utf-8",
            )
            self.assertEqual(
                stress_matrix.read_project_identity(config),
                {"ecosystem": "web", "frameworks": ["actix-web", "react-vite"]},
            )

    def test_upsert_script_entries_adds_and_replaces_without_duplicate_keys(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            config = Path(temporary) / "mgc.toml"
            config.write_text(
                'ecosystem = "web"\n\n[scripts]\nbuild = "old"\n\n[lock]\nschema = "3"\n',
                encoding="utf-8",
            )
            stress_matrix.upsert_script_entries(
                config, {"test": "python probe.py", "build": "python build.py"}
            )
            text = config.read_text(encoding="utf-8")
            self.assertEqual(text.count("build ="), 1)
            self.assertIn('build = "python build.py"', text)
            self.assertIn('test = "python probe.py"', text)
            self.assertIn('[lock]\nschema = "3"', text)

    def test_set_lock_schema_creates_and_updates_only_one_lock_table(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            config = Path(temporary) / "mgc.toml"
            config.write_text('ecosystem = "web"\n\n[lock]\npolicy = "warn"\n', encoding="utf-8")

            stress_matrix.set_lock_schema(config, "3")
            stress_matrix.set_lock_schema(config, "v4")

            text = config.read_text(encoding="utf-8")
            self.assertEqual(text.count("[lock]"), 1)
            self.assertEqual(text.count("schema ="), 1)
            self.assertIn('schema = "v4"', text)
            self.assertIn('policy = "warn"', text)

    def test_set_lock_schema_rejects_ambiguous_or_inline_config(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            config = Path(temporary) / "mgc.toml"
            config.write_text('[lock]\nschema = "3"\n\n[lock]\nschema = "v4"\n', encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "more than one"):
                stress_matrix.set_lock_schema(config, "v4")

            config.write_text('lock = { schema = "3" }\n', encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "inline lock"):
                stress_matrix.set_lock_schema(config, "v4")

    def test_framework_script_contract_runs_both_cli_paths(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            project = Path(temporary)
            (project / "mgc.toml").write_text(
                'ecosystem = "web"\nframeworks = ["vanilla"]\n',
                encoding="utf-8",
            )
            calls: list[list[str]] = []

            def fake_run(argv, **kwargs):
                calls.append([str(part) for part in argv])
                marker = (
                    "MGC_STRESS_SCRIPT:mgc_stress_probe"
                    if "mgc_stress_probe" in argv
                    else "MGC_STRESS_SCRIPT:test"
                )
                return subprocess.CompletedProcess(argv, 0, marker)

            with patch.object(stress_matrix, "run_command", side_effect=fake_run):
                results = stress_matrix.run_framework_script_contracts(
                    Path("mgc"), "web", "vanilla", project, {}
                )

            self.assertTrue(all(result.passed for result in results), results)
            self.assertEqual(
                [result.category for result in results],
                ["mgc-run-script-orchestration", "mgc-test-script-orchestration"],
            )
            self.assertTrue(all("orchestration only" in result.detail for result in results))
            self.assertEqual(
                calls,
                [
                    ["mgc", "run", "--core", "web", "mgc_stress_probe"],
                    ["mgc", "test", "--core", "web"],
                ],
            )
            self.assertIn("[scripts]", (project / "mgc.toml").read_text(encoding="utf-8"))

    def test_go_probe_passes_owned_cache_and_offline_policy_in_script(self) -> None:
        with tempfile.TemporaryDirectory(prefix="mgc go probe ") as temporary:
            root = Path(temporary)
            project = root / "project"
            project.mkdir()
            (project / "mgc.toml").write_text('name = "probe"\necosystem = "lib"\n')
            env = {"MGC_DATA_DIR": str(root / "data"), "PATH": "test-path"}
            with patch.object(stress_matrix.shutil, "which", return_value="go"), patch.object(
                stress_matrix, "run_command", return_value=subprocess.CompletedProcess([], 0, "MGC_STRESS_LANGUAGE_RUNTIME:Go\n")
            ):
                result = stress_matrix._run_framework_language_probe_for_language(
                    Path("mgc"), "lib", "go", project, env, "Go"
                )
            self.assertTrue(result.passed, result.detail)
            probe = root / "data/language-probes/lib/go/go"
            config = tomllib.loads((probe / "mgc.toml").read_text())
            script = config["scripts"]["mgc_language_runtime_probe_1"]
            self.assertIn("GOCACHE=", script)
            self.assertIn(str(probe / ".mgc-go-build-cache"), script)
            self.assertIn("GOPROXY=off GOSUMDB=off GOTOOLCHAIN=local", script)
            self.assertIn("go run -mod=readonly mgc_stress_language.go", script)
            self.assertTrue((probe / ".mgc-go-build-cache").is_dir())

    def test_go_probe_filename_is_not_ignored_by_the_go_toolchain(self) -> None:
        profile = stress_matrix.language_runtime_profile("Go")
        self.assertIsNotNone(profile)
        filename = Path(profile[1]).name
        self.assertFalse(filename.startswith((".", "_")))
        self.assertEqual(profile[3], (f"GOPROXY=off GOSUMDB=off GOTOOLCHAIN=local go run -mod=readonly {filename}",))

    def test_language_runtime_profiles_use_native_interpreters_without_package_managers(self) -> None:
        c_compiler = "clang" if stress_matrix.sys.platform == "darwin" else "gcc"
        cpp_compiler = "clang++" if stress_matrix.sys.platform == "darwin" else "g++"
        c_output = ".mgc-stress-language-c.exe" if stress_matrix.os.name == "nt" else ".mgc-stress-language-c"
        cpp_output = ".mgc-stress-language-cpp.exe" if stress_matrix.os.name == "nt" else ".mgc-stress-language-cpp"
        expected = {
            "TypeScript": ("node --experimental-strip-types .mgc-stress-language.ts",),
            "JavaScript": ("node .mgc-stress-language.mjs",),
            "Python": (f"{'python' if stress_matrix.os.name == 'nt' else 'python3'} .mgc-stress-language.py",),
            "Go": ("GOPROXY=off GOSUMDB=off GOTOOLCHAIN=local go run -mod=readonly mgc_stress_language.go",),
            "Dart": ("dart .mgc-stress-language.dart",),
            "Rust": ("cargo run --quiet --locked --offline --manifest-path .mgc-stress-runtime-rust/Cargo.toml",),
            "Swift": ((
                "swift -module-cache-path .mgc-swift-module-cache "
                "-sdk-module-cache-path .mgc-swift-sdk-module-cache "
                ".mgc-stress-language.swift"
            ),),
            "C": (f"{c_compiler} .mgc-stress-language.c -o {c_output}", f"./{c_output}"),
            "C++": (f"{cpp_compiler} .mgc-stress-language.cpp -o {cpp_output}", f"./{cpp_output}"),
            "C#": ("dotnet exec .mgc-stress-dotnet/bin/Debug/net10.0/MgcStressProbe.dll",),
            "Java": ((
                "java -XX:-UsePerfData -Djava.io.tmpdir=.mgc-java-tmp "
                "MgcStressLanguageProbe.java"
            ),),
            "Kotlin": (
                "kotlinc .mgc-stress-language.kt -include-runtime -d .mgc-stress-language-kotlin.jar",
                "java -XX:-UsePerfData -Djava.io.tmpdir=.mgc-java-tmp -jar .mgc-stress-language-kotlin.jar",
            ),
            "PHP": ("php -n .mgc-stress-language.php",),
            "Shell": ("bash .mgc-stress-language.sh",),
            "GDScript": ((
                "godot --headless --path .mgc-stress-godot "
                "--script probe.gd --quit"
            ),),
            "Zig": ("zig run .mgc-stress-language.zig",),
            "Terraform HCL": ("terraform fmt -check .mgc-stress-language.tf",),
        }
        for language, command in expected.items():
            with self.subTest(language=language):
                profile = stress_matrix.language_runtime_profile(language)
                self.assertIsNotNone(profile)
                self.assertEqual(profile[3], command)
                if language != "Rust":
                    self.assertIn(f"MGC_STRESS_LANGUAGE_RUNTIME:{language}", profile[2])
        if stress_matrix.sys.platform == "darwin":
            for language in ("Objective-C", "Objective-C++"):
                with self.subTest(language=language):
                    self.assertIsNotNone(stress_matrix.language_runtime_profile(language))

    def test_rust_runtime_probe_runs_through_mgc_with_an_offline_dependency_free_crate(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            project = Path(temporary)
            (project / "mgc.toml").write_text(
                'ecosystem = "lib"\nframeworks = ["rust"]\n', encoding="utf-8"
            )
            (project / "src").mkdir()
            (project / "src/main.rs").write_text("fn main() {}\n", encoding="utf-8")
            calls: list[list[str]] = []

            def fake_run(argv, **kwargs):
                calls.append([str(part) for part in argv])
                probe_project = Path(kwargs["cwd"])
                self.assertNotEqual(probe_project, project)
                manifest = probe_project / ".mgc-stress-runtime-rust/Cargo.toml"
                source = probe_project / ".mgc-stress-runtime-rust/src/main.rs"
                self.assertIn("[package]", manifest.read_text(encoding="utf-8"))
                self.assertNotIn("[dependencies]", manifest.read_text(encoding="utf-8"))
                self.assertIn("MGC_STRESS_LANGUAGE_RUNTIME:Rust", source.read_text(encoding="utf-8"))
                self.assertIn(
                    'name = "mgc-stress-runtime-probe"',
                    (manifest.parent / "Cargo.lock").read_text(encoding="utf-8"),
                )
                return subprocess.CompletedProcess(
                    argv, 0, "MGC_STRESS_LANGUAGE_RUNTIME:Rust\n"
                )

            with (
                patch.object(stress_matrix.shutil, "which", return_value="/usr/bin/cargo"),
                patch.object(stress_matrix, "run_command", side_effect=fake_run),
            ):
                results = stress_matrix.run_framework_language_runtime_probe(
                    Path("mgc"), "lib", "rust", project, {"PATH": "/usr/bin"}
                )

            self.assertEqual(len(results), 1)
            result = results[0]
            self.assertTrue(result.passed, result.detail)
            self.assertEqual(result.evidence_status, "verified")
            self.assertNotIn(
                "mgc_language_runtime_probe",
                (project / "mgc.toml").read_text(encoding="utf-8"),
            )
            self.assertEqual(
                calls,
                [["mgc", "run", "--core", "lib", "mgc_language_runtime_probe_1"]],
            )

    def test_compiled_language_probe_splits_compile_and_execution_into_mgc_runs(self) -> None:
        compiler = "clang" if stress_matrix.sys.platform == "darwin" else "gcc"
        output = ".mgc-stress-language-c.exe" if stress_matrix.os.name == "nt" else ".mgc-stress-language-c"
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            project = root / "projects/lib/c"
            project.mkdir(parents=True)
            (project / "mgc.toml").write_text(
                'ecosystem = "lib"\nframeworks = ["c"]\n', encoding="utf-8"
            )
            (project / "main.c").write_text("int main(void) { return 0; }\n", encoding="utf-8")
            env = stress_matrix.isolated_environment(root)
            calls: list[list[str]] = []

            def fake_run(argv, **kwargs):
                calls.append([str(part) for part in argv])
                probe_project = Path(kwargs["cwd"])
                config = (probe_project / "mgc.toml").read_text(encoding="utf-8")
                self.assertIn(
                    f'mgc_language_runtime_probe_1 = "TMPDIR=',
                    config,
                )
                self.assertIn(f" {compiler} .mgc-stress-language.c -o {output}", config)
                if calls[-1][-1].endswith("_1"):
                    return subprocess.CompletedProcess(argv, 0, "compiled\n")
                self.assertIn(
                    f'mgc_language_runtime_probe_2 = "./{output}"', config
                )
                return subprocess.CompletedProcess(
                    argv, 0, "MGC_STRESS_LANGUAGE_RUNTIME:C\n"
                )

            with (
                patch.object(stress_matrix.shutil, "which", return_value=f"/usr/bin/{compiler}"),
                patch.object(stress_matrix, "run_command", side_effect=fake_run),
            ):
                results = stress_matrix.run_framework_language_runtime_probe(
                    Path("mgc"), "lib", "c", project, env
                )

            self.assertEqual(len(results), 1)
            self.assertTrue(results[0].passed, results[0].detail)
            self.assertEqual(
                calls,
                [
                    ["mgc", "run", "--core", "lib", "mgc_language_runtime_probe_1"],
                    ["mgc", "run", "--core", "lib", "mgc_language_runtime_probe_2"],
                ],
            )

    def test_dotnet_probe_builds_offline_before_running_only_the_compiled_assembly(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            project = root / "projects/web/dotnet"
            project.mkdir(parents=True)
            (project / "mgc.toml").write_text(
                'ecosystem = "web"\nframeworks = ["dotnet-minimal"]\n',
                encoding="utf-8",
            )
            (project / "Program.cs").write_text("Console.WriteLine(\"original\");\n", encoding="utf-8")
            env = stress_matrix.isolated_environment(root)
            calls: list[list[str]] = []

            def fake_run(argv, **kwargs):
                command = [str(part) for part in argv]
                calls.append(command)
                probe_project = Path(kwargs["cwd"])
                if command[:2] == ["dotnet", "--list-sdks"]:
                    return subprocess.CompletedProcess(argv, 0, "10.0.401 [/sdk]\n")
                if command[:2] == ["dotnet", "restore"]:
                    config = (probe_project / "NuGet.Config").read_text(encoding="utf-8")
                    self.assertIn("<clear />", config)
                    return subprocess.CompletedProcess(argv, 0, "restored offline\n")
                if command[:2] == ["dotnet", "build"]:
                    self.assertIn("--no-restore", command)
                    return subprocess.CompletedProcess(argv, 0, "built offline\n")
                self.assertEqual(command[-1], "mgc_language_runtime_probe_1")
                manifest = (probe_project / "mgc.toml").read_text(encoding="utf-8")
                self.assertIn(
                    'mgc_language_runtime_probe_1 = "dotnet exec .mgc-stress-dotnet/bin/Debug/net10.0/MgcStressProbe.dll"',
                    manifest,
                )
                self.assertNotIn("dotnet run", manifest)
                return subprocess.CompletedProcess(
                    argv, 0, "MGC_STRESS_LANGUAGE_RUNTIME:C#\n"
                )

            with (
                patch.object(stress_matrix.shutil, "which", return_value="/usr/bin/dotnet"),
                patch.object(stress_matrix, "run_command", side_effect=fake_run),
            ):
                results = stress_matrix.run_framework_language_runtime_probe(
                    Path("mgc"), "web", "dotnet-minimal", project, env
                )

            self.assertEqual(len(results), 1)
            self.assertTrue(results[0].passed, results[0].detail)
            self.assertEqual(
                [call[1] for call in calls], ["--list-sdks", "restore", "build", "run"]
            )

    def test_java_launcher_without_a_runtime_is_unverified_not_a_probe_failure(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            project = root / "projects/web/java"
            project.mkdir(parents=True)
            (project / "mgc.toml").write_text(
                'ecosystem = "web"\nframeworks = ["java"]\n', encoding="utf-8"
            )
            (project / "Main.java").write_text("class Main {}\n", encoding="utf-8")
            env = stress_matrix.isolated_environment(root)
            calls: list[list[str]] = []

            def fake_run(argv, **kwargs):
                calls.append([str(part) for part in argv])
                return subprocess.CompletedProcess(
                    argv, 1, "Unable to locate a Java Runtime.\n"
                )

            with (
                patch.object(stress_matrix.shutil, "which", return_value="/usr/bin/java"),
                patch.object(stress_matrix, "run_command", side_effect=fake_run),
            ):
                results = stress_matrix.run_framework_language_runtime_probe(
                    Path("mgc"), "web", "java", project, env
                )

            self.assertEqual(len(results), 1)
            self.assertTrue(results[0].passed)
            self.assertEqual(results[0].evidence_status, "unverified")
            self.assertIn("no usable Java runtime", results[0].detail)
            self.assertEqual(calls, [["java", "-version"]])

    def test_swift_runtime_probe_routes_compiler_caches_into_isolated_mgc_data(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            project = root / "projects/app/swift"
            project.mkdir(parents=True)
            (project / "mgc.toml").write_text(
                'ecosystem = "app"\n[app]\nlanguage = "swift"\n',
                encoding="utf-8",
            )
            (project / "main.swift").write_text('print("ready")\n', encoding="utf-8")
            env = stress_matrix.isolated_environment(root)
            captured: dict[str, str] = {}

            def fake_run(argv, **kwargs):
                captured.update(kwargs["env"])
                probe_project = Path(kwargs["cwd"])
                self.assertNotEqual(probe_project, project)
                script = (probe_project / "mgc.toml").read_text(encoding="utf-8")
                self.assertIn("-module-cache-path .mgc-swift-module-cache", script)
                self.assertIn("-sdk-module-cache-path .mgc-swift-sdk-module-cache", script)
                self.assertTrue((probe_project / ".mgc-swift-module-cache").is_dir())
                self.assertTrue((probe_project / ".mgc-swift-sdk-module-cache").is_dir())
                return subprocess.CompletedProcess(
                    argv, 0, "MGC_STRESS_LANGUAGE_RUNTIME:Swift\n"
                )

            with (
                patch.object(stress_matrix.shutil, "which", return_value="/usr/bin/swift"),
                patch.object(stress_matrix, "run_command", side_effect=fake_run),
            ):
                results = stress_matrix.run_framework_language_runtime_probe(
                    Path("mgc"), "app", "swift", project, env
                )

            self.assertEqual(len(results), 1)
            result = results[0]
            self.assertTrue(result.passed, result.detail)
            self.assertEqual(result.evidence_status, "verified")
            for key in ("CLANG_MODULE_CACHE_PATH", "SWIFT_MODULECACHE_PATH"):
                with self.subTest(key=key):
                    self.assertTrue(Path(captured[key]).is_relative_to(Path(env["MGC_DATA_DIR"])))

    def test_python_runtime_probe_uses_an_empty_isolated_manifest(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            project = root / "projects/web/fastapi"
            project.mkdir(parents=True)
            (project / "mgc.toml").write_text(
                'ecosystem = "web"\nmode = "backend"\nframeworks = ["python"]\n',
                encoding="utf-8",
            )
            (project / ".mgc.core").write_text("web\n", encoding="utf-8")
            manifest = project / "pyproject.toml"
            manifest.write_text(
                '[project]\nname="fastapi-app"\ndependencies=["fastapi>=1"]\n',
                encoding="utf-8",
            )
            (project / "main.py").write_text("print('project source stays untouched')\n", encoding="utf-8")
            env = stress_matrix.isolated_environment(root)

            def fake_run(argv, **kwargs):
                probe_project = Path(kwargs["cwd"])
                probe_manifest = (probe_project / "pyproject.toml").read_text(encoding="utf-8")
                self.assertIn("dependencies = []", probe_manifest)
                self.assertIn("fastapi>=1", manifest.read_text(encoding="utf-8"))
                return subprocess.CompletedProcess(
                    argv, 0, "MGC_STRESS_LANGUAGE_RUNTIME:Python\n"
                )

            with (
                patch.object(stress_matrix.shutil, "which", return_value="/usr/bin/python3"),
                patch.object(stress_matrix, "run_command", side_effect=fake_run),
            ):
                results = stress_matrix.run_framework_language_runtime_probe(
                    Path("mgc"), "web", "fastapi", project, env
                )

            self.assertEqual(len(results), 1)
            result = results[0]
            self.assertTrue(result.passed, result.detail)
            self.assertEqual(result.evidence_status, "verified")

    def test_unprofiled_language_runtime_stays_unverified_and_does_not_spawn(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            project = Path(temporary)
            (project / "mgc.toml").write_text(
                'ecosystem = "web"\nframeworks = ["spring-boot"]\n',
                encoding="utf-8",
            )
            with patch.object(stress_matrix, "run_command") as run_command:
                result = stress_matrix._run_framework_language_probe_for_language(
                    Path("mgc"),
                    "web",
                    "spring-boot",
                    project,
                    {"PATH": "/usr/bin"},
                    "Unknown DSL",
                )

            self.assertTrue(result.passed)
            self.assertEqual(result.evidence_status, "unverified")
            self.assertEqual(result.subject, "web/spring-boot/Unknown DSL")
            self.assertIn("no standalone runtime probe", result.detail)
            run_command.assert_not_called()

    def test_hcl_probe_runs_terraform_format_parser_without_fetching_providers(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            project = root / "projects/web/terraform"
            project.mkdir(parents=True)
            (project / "mgc.toml").write_text(
                'ecosystem = "web"\nframeworks = ["terraform"]\n',
                encoding="utf-8",
            )
            (project / "main.tf").write_text(
                'output "source" { value = "ignored inventory file" }\n',
                encoding="utf-8",
            )
            env = stress_matrix.isolated_environment(root)

            def fake_run(argv, **kwargs):
                probe_project = Path(kwargs["cwd"])
                source = probe_project / ".mgc-stress-language.tf"
                self.assertTrue(source.is_file())
                self.assertIn('output "mgc_stress_language_runtime"', source.read_text())
                script = (probe_project / "mgc.toml").read_text(encoding="utf-8")
                self.assertIn(
                    'terraform fmt -check .mgc-stress-language.tf', script
                )
                self.assertEqual(
                    argv[-1], "mgc_language_runtime_probe_1"
                )
                return subprocess.CompletedProcess(argv, 0, "")

            with (
                patch.object(stress_matrix.shutil, "which", return_value="/usr/bin/terraform"),
                patch.object(stress_matrix, "run_command", side_effect=fake_run),
            ):
                results = stress_matrix.run_framework_language_runtime_probe(
                    Path("mgc"), "web", "terraform", project, env
                )

            self.assertEqual(len(results), 1)
            self.assertTrue(results[0].passed, results[0].detail)
            self.assertEqual(results[0].evidence_status, "verified")
            self.assertIn("no provider", results[0].detail)

    def test_language_runtime_gate_requires_verified_probes_and_ignores_platform_na(self) -> None:
        results = [
            stress_matrix.Result(
                "framework-language-runtime-probe",
                "lib/zig/Zig",
                True,
                "compiled and ran",
                evidence_status="verified",
            ),
            stress_matrix.Result(
                "framework-language-runtime-probe",
                "lib/swift/Objective-C",
                True,
                "not available on this OS",
                evidence_status="not_applicable",
            ),
            stress_matrix.Result(
                "framework-language-runtime-probe",
                "lib/zig/Unknown DSL",
                True,
                "probe not configured",
                evidence_status="unverified",
            ),
        ]

        self.assertEqual(
            stress_matrix.language_runtime_evidence_gaps(results), [results[2]]
        )

    def test_language_runtime_probe_runs_every_detected_language_in_separate_fixtures(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            project = root / "projects/web/mixed"
            project.mkdir(parents=True)
            (project / "mgc.toml").write_text(
                'ecosystem = "web"\nframeworks = ["mixed"]\n',
                encoding="utf-8",
            )
            (project / "main.ts").write_text('console.log("project stays untouched")\n', encoding="utf-8")
            (project / "main.py").write_text('print("project stays untouched")\n', encoding="utf-8")
            env = stress_matrix.isolated_environment(root)
            invocations: list[Path] = []

            def fake_run(argv, **kwargs):
                probe_project = Path(kwargs["cwd"])
                invocations.append(probe_project)
                language = {
                    "typescript": "TypeScript",
                    "python": "Python",
                }[probe_project.name]
                marker = f"MGC_STRESS_LANGUAGE_RUNTIME:{language}"
                command = (probe_project / "mgc.toml").read_text(encoding="utf-8")
                self.assertIn("mgc_language_runtime_probe", command)
                self.assertNotEqual(probe_project, project)
                return subprocess.CompletedProcess(argv, 0, marker + "\n")

            with (
                patch.object(stress_matrix.shutil, "which", return_value="/usr/bin/runtime"),
                patch.object(stress_matrix, "run_command", side_effect=fake_run),
            ):
                results = stress_matrix.run_framework_language_runtime_probe(
                    Path("mgc"), "web", "mixed", project, env
                )

            self.assertEqual([result.subject for result in results], [
                "web/mixed/TypeScript",
                "web/mixed/Python",
            ])
            self.assertEqual([result.evidence_status for result in results], ["verified", "verified"])
            self.assertEqual(len(set(invocations)), 2)
            self.assertTrue(all(path.is_relative_to(Path(env["MGC_DATA_DIR"])) for path in invocations))

    def test_framework_script_contract_clears_declared_python_dependencies(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            project = Path(temporary)
            (project / "mgc.toml").write_text(
                'ecosystem = "web"\nframeworks = ["fastapi"]\n',
                encoding="utf-8",
            )
            pyproject = project / "pyproject.toml"
            pyproject.write_text(
                '[project]\nname = "sample"\nversion = "0.1.0"\n'
                'dependencies = ["network-only-package>=1"]\n',
                encoding="utf-8",
            )

            def fake_run(argv, **kwargs):
                self.assertIn("dependencies = []", pyproject.read_text(encoding="utf-8"))
                marker = (
                    "MGC_STRESS_SCRIPT:mgc_stress_probe"
                    if "mgc_stress_probe" in argv
                    else "MGC_STRESS_SCRIPT:test"
                )
                return subprocess.CompletedProcess(argv, 0, marker)

            with patch.object(stress_matrix, "run_command", side_effect=fake_run):
                results = stress_matrix.run_framework_script_contracts(
                    Path("mgc"), "web", "fastapi", project, {}
                )

            self.assertTrue(all(result.passed for result in results), results)
            self.assertIn("requires-python", pyproject.read_text(encoding="utf-8"))

    def test_quality_dimensions_run_before_lifecycle_fixture_clears_dependencies(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            project = root / "project"
            project.mkdir()
            package_json = project / "package.json"
            package_json.write_text(
                '{"name":"sample","dependencies":{"react":"19.0.0"}}\n',
                encoding="utf-8",
            )
            events: list[tuple[str, dict[str, str], Path]] = []

            def fake_run(argv, **kwargs):
                cwd = Path(kwargs["cwd"])
                fixture_package = cwd / "package.json"
                dependencies = json.loads(fixture_package.read_text(encoding="utf-8"))["dependencies"]
                command = str(argv[1])
                events.append((command, dependencies, cwd))
                if command == "optimizer":
                    (cwd / "optimizer-output.json").write_text("{}", encoding="utf-8")
                    output = "optimizer completed"
                elif command == "cache":
                    self.assertFalse((cwd / "optimizer-output.json").exists())
                    output = "cache status completed"
                elif command == "audit":
                    output = json.dumps(
                        {
                            "scanner_status": "available",
                            "packages_audited": 1,
                            "vulnerability_count": 0,
                        }
                    )
                else:
                    output = f"{command} completed"
                return subprocess.CompletedProcess(argv, 0, output)

            def clear_dependencies(*args, **kwargs):
                package = json.loads(package_json.read_text(encoding="utf-8"))
                package["dependencies"] = {}
                package_json.write_text(json.dumps(package), encoding="utf-8")
                events.append(("lifecycle-scripts", package["dependencies"]))
                return []

            row = {
                "core": "web",
                "framework": "vanilla",
                "dependency_ownership": {"install": {"owner": "unsupported"}},
            }
            with (
                patch.object(stress_matrix, "run_command", side_effect=fake_run),
                patch.object(stress_matrix, "run_framework_script_contracts", side_effect=clear_dependencies),
                patch.object(stress_matrix, "run_native_install_contract", return_value=(None, False, None)),
                patch.object(
                    stress_matrix,
                    "run_store_scope_contract",
                    return_value=[
                        stress_matrix.Result("shared-store-scope", "web/vanilla", True, "checked"),
                        stress_matrix.Result("shared-store-runtime", "web/vanilla", True, "checked"),
                    ],
                ),
                patch.object(stress_matrix, "run_lockfile_contracts", return_value=[]),
                patch.object(stress_matrix, "run_shared_store_check", return_value=[]),
            ):
                results = stress_matrix.run_project_dimensions(
                    Path("mgc"),
                    [("web", "vanilla", project)],
                    [row],
                    root,
                    {},
                )

            self.assertEqual(
                [event[0] for event in events],
                ["optimizer", "cache", "audit", "lifecycle-scripts"],
            )
            self.assertTrue(all(event[1] == {"react": "19.0.0"} for event in events[:3]))
            self.assertEqual(len({event[2] for event in events[:3]}), 3)
            self.assertTrue(all(event[2] != project for event in events[:3]))
            self.assertEqual(json.loads(package_json.read_text(encoding="utf-8"))["dependencies"], {})
            self.assertFalse((project / "optimizer-output.json").exists())
            self.assertEqual(
                [result.evidence_status for result in results[:3]],
                ["verified", "verified", "verified"],
            )

    def test_quality_audit_probe_runs_after_native_empty_install(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            project = root / "project"
            project.mkdir()
            (project / "package.json").write_text(
                '{"name":"sample","dependencies":{"react":"19.0.0"}}\n',
                encoding="utf-8",
            )
            (project / "mgc.toml").write_text(
                'ecosystem = "web"\nframeworks = ["vanilla"]\n', encoding="utf-8"
            )
            calls: list[tuple[str, Path]] = []

            def fake_run(argv, *, cwd, env, timeout_seconds, input_text=""):
                command = argv[1]
                calls.append((command, cwd))
                if command == "optimizer":
                    return subprocess.CompletedProcess(argv, 0, "optimized")
                if command == "cache":
                    return subprocess.CompletedProcess(argv, 0, "cache ready")
                if command == "audit":
                    self.assertTrue((cwd / "mgc.lock").is_file())
                    return subprocess.CompletedProcess(
                        argv,
                        0,
                        '{"scanner_status":"available","packages_audited":0,'
                        '"vulnerability_count":0}',
                    )
                self.fail(f"unexpected command: {argv}")

            def fake_install(_binary, _row, install_project, _env):
                calls.append(("install", install_project))
                (install_project / "package.json").write_text(
                    '{"name":"sample","dependencies":{}}\n', encoding="utf-8"
                )
                (install_project / "mgc.lock").write_text("empty graph\n", encoding="utf-8")
                return stress_matrix.Result(
                    "mgc-install-empty-graph", "web/vanilla", True, "installed"
                ), True, "package.json"

            row = {
                "core": "web",
                "framework": "vanilla",
                "dependency_ownership": {"install": {"owner": "mgc-native"}},
            }
            with (
                patch.object(stress_matrix, "run_command", side_effect=fake_run),
                patch.object(stress_matrix, "run_native_install_contract", side_effect=fake_install),
                patch.object(stress_matrix, "run_framework_script_contracts", return_value=[]),
                patch.object(
                    stress_matrix,
                    "run_store_scope_contract",
                    return_value=[
                        stress_matrix.Result("shared-store-scope", "web/vanilla", True, "checked"),
                        stress_matrix.Result("shared-store-runtime", "web/vanilla", True, "checked"),
                    ],
                ),
                patch.object(stress_matrix, "run_lockfile_contracts", return_value=[]),
                patch.object(stress_matrix, "run_shared_store_check", return_value=[]),
            ):
                results = stress_matrix.run_project_dimensions(
                    Path("mgc"), [("web", "vanilla", project)], [row], root, {}
                )

            self.assertEqual([name for name, _cwd in calls], ["install", "optimizer", "cache", "audit"])
            self.assertTrue(all(cwd != project for _name, cwd in calls[1:]))
            audit = next(result for result in results if result.category == "security-audit")
            self.assertTrue(audit.passed)
            self.assertEqual(audit.evidence_status, "not_applicable")

    def test_native_install_contract_executes_only_mgc_owned_lane(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            project = Path(temporary)
            (project / "mgc.toml").write_text('ecosystem = "web"\nframeworks = ["vanilla"]\n', encoding="utf-8")
            (project / "package.json").write_text(
                '{"name":"sample","dependencies":{"one":"1.0.0"}}\n',
                encoding="utf-8",
            )
            row = {
                "core": "web",
                "framework": "vanilla",
                "dependency_ownership": {"install": {"owner": "mgc-native"}},
            }
            completed = subprocess.CompletedProcess(
                ["mgc", "install"], 0, "All dependencies installed\n"
            )
            with patch.object(stress_matrix, "run_command", return_value=completed) as run:
                result, passed, manifest_name = stress_matrix.run_native_install_contract(
                    Path("mgc"), row, project, {}
                )

            self.assertTrue(passed)
            self.assertIsNotNone(result)
            self.assertTrue(result.passed)
            self.assertEqual(run.call_args.args[0], ["mgc", "install-web"])
            manifest = json.loads((project / "package.json").read_text(encoding="utf-8"))
            self.assertEqual(manifest["dependencies"], {})
            self.assertIn('schema = "3"', (project / "mgc.toml").read_text(encoding="utf-8"))
            self.assertEqual(manifest_name, "package.json")

    def test_native_install_contract_does_not_execute_unsupported_owner(self) -> None:
        row = {
            "core": "app",
            "framework": "react-native",
            "dependency_ownership": {"install": {"owner": "unsupported"}},
        }
        with patch.object(stress_matrix, "run_command", side_effect=AssertionError("must not spawn")):
            result, passed, manifest = stress_matrix.run_native_install_contract(
                Path("mgc"), row, Path("."), {}
            )
        self.assertFalse(passed)
        self.assertIsNotNone(result)
        self.assertEqual(result.evidence_status, "unsupported")
        self.assertIsNone(manifest)

    def test_empty_app_install_without_lock_fails_lockfile_contract(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            project = Path(temporary)
            (project / "mgc.toml").write_text(
                'ecosystem = "app"\nframeworks = ["flutter"]\n', encoding="utf-8"
            )
            (project / "pubspec.yaml").write_text(
                "name: sample\ndependencies: {}\n",
                encoding="utf-8",
            )
            row = {
                "core": "app",
                "framework": "flutter",
                "dependency_ownership": {"install": {"owner": "mgc-native"}},
            }
            completed = subprocess.CompletedProcess(["mgc", "install-app"], 0, "installed")
            with patch.object(stress_matrix, "run_command", return_value=completed):
                results = stress_matrix.run_lockfile_contracts(
                    Path("mgc"),
                    row,
                    project,
                    {},
                    install_succeeded=True,
                )

            self.assertEqual([result.evidence_status for result in results], ["verified"] * 2)
            self.assertTrue(all(not result.passed for result in results))
            self.assertIn("did not produce a version 3 mgc.lock", results[0].detail)
            self.assertIn("lock version is None", results[1].detail)

    def test_store_doctor_unsupported_core_is_reported_without_claiming_health(self) -> None:
        row = {
            "core": "app",
            "framework": "maui",
            "dependency_ownership": {
                "install": {"owner": "unsupported"},
                "store": {"owner": "unsupported"},
            },
        }
        with patch.object(stress_matrix, "run_command") as run:
            results = stress_matrix.run_store_scope_contract(
                Path("mgc"), row, Path("."), {}, install_succeeded=True
            )
        run.assert_not_called()
        self.assertEqual([result.category for result in results], ["shared-store-scope", "shared-store-runtime"])
        self.assertTrue(all(result.passed for result in results))
        self.assertTrue(all(result.evidence_status == "unsupported" for result in results))

    def test_store_doctor_verifies_native_route_and_runtime_scope_separately(self) -> None:
        row = {
            "core": "app",
            "framework": "flutter",
            "dependency_ownership": {
                "install": {"owner": "mgc-native"},
                "store": {"owner": "mgc-native"},
            },
        }
        payload = {
            "core": "app",
            "framework": "flutter",
            "scope": "managed-root-path-safety-only",
            "integrity_checked": False,
            "healthy": True,
            "roots": [{"name": "pub", "safe": True, "exists": True}],
        }
        completed = subprocess.CompletedProcess(
            ["mgc", "store", "doctor"], 0, json.dumps(payload)
        )

        def fake_run(argv, **kwargs):
            self.assertEqual(
                argv[1:],
                [
                    "store",
                    "doctor",
                    "--core",
                    "app",
                    "--framework",
                    "flutter",
                    "--json",
                ],
            )
            return completed

        with patch.object(stress_matrix, "run_command", side_effect=fake_run):
            results = stress_matrix.run_store_scope_contract(
                Path("mgc"), row, Path("."), {}, install_succeeded=True
            )
        self.assertEqual([result.category for result in results], ["shared-store-scope", "shared-store-runtime"])
        self.assertTrue(results[0].passed, results[0])
        self.assertEqual(results[0].evidence_status, "verified")
        self.assertTrue(results[1].passed, results[1])
        self.assertEqual(results[1].evidence_status, "unverified")
        self.assertIn("no package-bearing install", results[1].detail)

    def test_global_store_empty_graph_without_initialized_root_is_unverified(self) -> None:
        row = {
            "core": "ai",
            "framework": "python-agent",
            "dependency_ownership": {
                "install": {"owner": "mgc-native"},
                "store": {"owner": "mgc-native"},
            },
        }
        payload = {
            "core": "ai",
            "framework": "python-agent",
            "scope": "managed-root-path-safety-only",
            "integrity_checked": False,
            "healthy": True,
            "roots": [
                {"name": "content-cas", "safe": True, "exists": False},
                {"name": "pypi", "safe": True, "exists": False},
            ],
        }
        completed = subprocess.CompletedProcess(
            ["mgc", "store", "doctor"], 0, json.dumps(payload)
        )

        with patch.object(stress_matrix, "run_command", return_value=completed):
            results = stress_matrix.run_store_scope_contract(
                Path("mgc"), row, Path("."), {}, install_succeeded=True
            )

        self.assertTrue(results[0].passed)
        self.assertEqual(results[0].evidence_status, "verified")
        self.assertTrue(results[1].passed)
        self.assertEqual(results[1].evidence_status, "unverified")
        self.assertIn("shared-store runtime remains unverified", results[1].detail)

    def test_global_store_unsafe_root_is_verified_unhealthy(self) -> None:
        row = {
            "core": "app",
            "framework": "flutter",
            "dependency_ownership": {
                "install": {"owner": "mgc-native"},
                "store": {"owner": "mgc-native"},
            },
        }
        payload = {
            "core": "app",
            "framework": "flutter",
            "scope": "managed-root-path-safety-only",
            "integrity_checked": False,
            "healthy": False,
            "roots": [
                {"name": "content-cas", "safe": True, "exists": False},
                {"name": "pub", "safe": False, "exists": True},
            ],
        }
        completed = subprocess.CompletedProcess(
            ["mgc", "store", "doctor"], 1, json.dumps(payload)
        )

        with patch.object(stress_matrix, "run_command", return_value=completed):
            results = stress_matrix.run_store_scope_contract(
                Path("mgc"), row, Path("."), {}, install_succeeded=True
            )

        self.assertTrue(results[0].passed)
        self.assertFalse(results[1].passed)
        self.assertEqual(results[1].evidence_status, "verified")

    def test_full_evidence_gate_only_requires_optimizer_and_security_audits(self) -> None:
        results = [
            stress_matrix.Result(
                "optimizer", "app/flutter", True, "no runtime adapter", evidence_status="unverified"
            ),
            stress_matrix.Result(
                "security-audit-nonempty", "app/flutter", True, "no package graph", evidence_status="unverified"
            ),
            stress_matrix.Result(
                "shared-store-runtime", "app/flutter", True, "no non-empty store proof", evidence_status="unverified"
            ),
            stress_matrix.Result(
                "framework-cli-dev", "app/flutter", True, "runner unavailable", evidence_status="unsupported"
            ),
        ]

        gaps = stress_matrix.full_evidence_gaps(results)

        self.assertEqual(
            [(result.category, result.subject) for result in gaps],
            [
                ("optimizer", "app/flutter"),
                ("security-audit-nonempty", "app/flutter"),
            ],
        )

    def test_lockfile_contract_replays_v3_and_v4_with_byte_stability(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            project = Path(temporary)
            config = project / "mgc.toml"
            config.write_text('ecosystem = "web"\nframeworks = ["vanilla"]\n[lock]\nschema = "3"\n', encoding="utf-8")
            lock = project / "mgc.lock"
            # V3 serializes the schema as a quoted string; V4's canonical
            # document may use an integer. Both forms are valid evidence.
            lock.write_text('version = "3"\n', encoding="utf-8")
            row = {
                "core": "web",
                "framework": "vanilla",
                "dependency_ownership": {"install": {"owner": "mgc-native"}},
            }
            calls: list[list[str]] = []

            def fake_run(argv, **kwargs):
                calls.append([str(part) for part in argv])
                if "--frozen" not in argv:
                    if 'schema = "v4"' in config.read_text(encoding="utf-8"):
                        lock.write_text('version = "4.1"\n', encoding="utf-8")
                    return subprocess.CompletedProcess(argv, 0, "All dependencies installed\n")
                return subprocess.CompletedProcess(argv, 0, "All dependencies installed\n")

            with patch.object(stress_matrix, "run_command", side_effect=fake_run):
                results = stress_matrix.run_lockfile_contracts(
                    Path("mgc"), row, project, {}, install_succeeded=True
                )

            self.assertEqual(
                [result.category for result in results],
                ["mgc-lock-v3-empty-graph", "mgc-lock-v4-empty-graph"],
            )
            self.assertTrue(all(result.passed and result.evidence_status == "verified" for result in results), results)
            self.assertEqual(calls, [
                ["mgc", "install-web", "--frozen"],
                ["mgc", "install-web"],
                ["mgc", "install-web", "--frozen"],
            ])
            self.assertEqual(stress_matrix.lockfile_version(lock), 4)

    def test_lockfile_contract_marks_unwired_v4_as_unsupported(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            project = Path(temporary)
            (project / "mgc.toml").write_text('ecosystem = "hardware"\nframeworks = ["benchmark"]\n', encoding="utf-8")
            (project / "mgc.lock").write_text('version = "3"\n', encoding="utf-8")
            row = {
                "core": "hardware",
                "framework": "benchmark",
                "dependency_ownership": {"install": {"owner": "mgc-native"}},
            }
            with patch.object(
                stress_matrix,
                "run_command",
                return_value=subprocess.CompletedProcess(["mgc", "install"], 0, "installed"),
            ):
                results = stress_matrix.run_lockfile_contracts(
                    Path("mgc"), row, project, {}, install_succeeded=True
                )
            self.assertEqual(results[0].evidence_status, "verified")
            self.assertEqual(results[1].evidence_status, "unsupported")
            self.assertIn("no native V4 install writer", results[1].detail)

    def test_lockfile_version_accepts_quoted_and_integer_schema_values(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            lock = Path(temporary) / "mgc.lock"
            for text, expected in (
                ('version = "3"\n', 3),
                ("version = '3'\n", 3),
                ('version = "4.1"\n', 4),
                ("version = '4.1'\n", 4),
                ("version = 4\n", 4),
                ("version = 4.1\n", None),
                ("version = \"not-a-version\"\n", None),
            ):
                lock.write_text(text, encoding="utf-8")
                self.assertEqual(stress_matrix.lockfile_version(lock), expected, text)

    def test_lockfile_evidence_gate_checks_every_framework_row(self) -> None:
        native_owners = {
            operation: {"owner": "mgc-native"}
            for operation in stress_matrix.DEPENDENCY_OPERATIONS
        }
        unsupported_owners = {
            operation: {"owner": "unsupported"}
            for operation in stress_matrix.DEPENDENCY_OPERATIONS
        }
        rows = [
            {"core": "web", "framework": "vanilla", "status": "mgc-engine-path", "dependency_ownership": native_owners},
            {"core": "cicd", "framework": "github-actions", "status": "scaffold-only", "dependency_ownership": unsupported_owners},
        ]
        results = [
            stress_matrix.Result("mgc-lock-v3-empty-graph", "web/vanilla", True, "V3 stable"),
            stress_matrix.Result("mgc-lock-v4-empty-graph", "web/vanilla", True, "V4 stable"),
            stress_matrix.Result("mgc-lock-v3-empty-graph", "cicd/github-actions", True, "not owned", evidence_status="unsupported"),
            stress_matrix.Result("mgc-lock-v4-empty-graph", "cicd/github-actions", True, "not owned", evidence_status="unsupported"),
        ]
        self.assertEqual(stress_matrix.lockfile_evidence_gaps(results, rows), [])
        self.assertEqual(len(stress_matrix.lockfile_evidence_gaps(results[:2], rows)), 2)

    def test_lockfile_evidence_gate_requires_lock_after_empty_native_app_install(self) -> None:
        native_owners = {
            operation: {"owner": "mgc-native"}
            for operation in stress_matrix.DEPENDENCY_OPERATIONS
        }
        app_row = {
            "core": "app",
            "framework": "flutter",
            "status": "mgc-engine-path",
            "dependency_ownership": native_owners,
        }
        results = [
            stress_matrix.Result(
                "mgc-lock-v3-empty-graph",
                "app/flutter",
                True,
                "not applicable: successful empty pubspec.yaml dependency set does not create mgc.lock",
                evidence_status="not_applicable",
            ),
            stress_matrix.Result(
                "mgc-lock-v4-empty-graph",
                "app/flutter",
                True,
                "not applicable: successful empty pubspec.yaml dependency set does not create mgc.lock",
                evidence_status="not_applicable",
            ),
        ]
        self.assertEqual(
            stress_matrix.lockfile_evidence_gaps(results, [app_row]),
            [
                "mgc-lock-v3-empty-graph:app/flutter expected one of ['verified'], got not_applicable (passed=True)",
                "mgc-lock-v4-empty-graph:app/flutter expected one of ['verified'], got not_applicable (passed=True)",
            ],
        )

        verified_results = [
            stress_matrix.Result(result.category, result.subject, True, "empty lock replayed")
            for result in results
        ]
        self.assertEqual(stress_matrix.lockfile_evidence_gaps(verified_results, [app_row]), [])

        weak_results = [
            stress_matrix.Result(
                result.category,
                result.subject,
                True,
                "not applicable: no lockfile",
                evidence_status="not_applicable",
            )
            for result in results
        ]
        self.assertEqual(len(stress_matrix.lockfile_evidence_gaps(weak_results, [app_row])), 2)

        web_row = {
            **app_row,
            "core": "web",
            "framework": "vanilla",
        }
        web_results = [
            stress_matrix.Result(
                result.category,
                "web/vanilla",
                result.passed,
                result.detail,
                evidence_status=result.evidence_status,
            )
            for result in results
        ]
        self.assertEqual(len(stress_matrix.lockfile_evidence_gaps(web_results, [web_row])), 2)

    def test_cross_core_writer_routes_cannot_be_downgraded_to_unsupported(self) -> None:
        for core, framework in (
            ("ai", "python-agent"), ("game", "bevy"),
            ("iot", "esp32-rust"), ("clo", "cdk"), ("clo", "pulumi"),
        ):
            with self.subTest(core=core, framework=framework):
                subject = f"{core}/{framework}"
                row = {"core": core, "framework": framework,
                       "dependency_ownership": {"install": {"owner": "mgc-native"}}}
                results = [
                    stress_matrix.Result("mgc-lock-v3-empty-graph", subject, True, "stable"),
                    stress_matrix.Result("mgc-lock-v4-empty-graph", subject, True, "unwired",
                                         evidence_status="unsupported"),
                ]
                self.assertEqual(len(stress_matrix.lockfile_evidence_gaps(results, [row])), 1)

    def test_report_records_failures_without_coercing_them_to_pass(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "evidence.json"
            rows = [
                {
                    "core": "web",
                    "framework": "vanilla",
                    "status": "mgc-engine-path",
                    "dependency_ownership": {
                        operation: {"owner": "mgc-native"}
                        for operation in stress_matrix.DEPENDENCY_OPERATIONS
                    },
                }
            ]
            results = [
                stress_matrix.Result("cli", "bad command", False, "exit 2", 2),
                stress_matrix.Result("optimizer", "unprobed", True, "no runtime",
                                     evidence_status="unverified"),
            ]
            stress_matrix.write_report(path, "1.1.0-rc.9", results, rows)
            report = json.loads(path.read_text(encoding="utf-8"))
            self.assertEqual(report["schema_version"], 7)
            self.assertEqual(report["passed"], 1)
            self.assertEqual(report["failed"], 1)
            self.assertEqual(report["verified_evidence"], 0)
            self.assertEqual(report["unverified_evidence"], 1)
            self.assertFalse(report["results"][0]["passed"])
            evidence = report["evidence_contract"]
            self.assertFalse(evidence["framework_runtime_verified"])
            self.assertIn("synthetic Python sentinel", evidence["run_test"])
            self.assertIn("empty graphs", evidence["native_install"])
            self.assertIn("empty-graph", evidence["lockfile"])
            self.assertEqual(report["universal_support_gate"]["status"], "not-evaluated")
            self.assertEqual(report["framework_lifecycle_evidence"]["status"], "not-provided")
            self.assertFalse(report["source_language_inventory"]["runtime_verified"])
            self.assertIn("not runtime proof", report["source_language_inventory"]["evidence_scope"])
            declared = report["dependency_ownership_declaration"]
            self.assertFalse(declared["runtime_verified"])
            self.assertIn("declaration-only", declared["evidence_scope"])
            self.assertEqual(
                declared["by_core_operation"]["web"]["install"]["mgc-native"], 1
            )
            self.assertEqual(declared["frameworks"][0]["framework"], "vanilla")

    def test_universal_support_gate_requires_runtime_evidence_for_every_dimension(self) -> None:
        subject = "web/vanilla"
        row = {
            "core": "web",
            "framework": "vanilla",
            "status": "mgc-engine-path",
            "dependency_ownership": {
                operation: {"owner": "mgc-native"}
                for operation in stress_matrix.DEPENDENCY_OPERATIONS
            },
        }
        results = [
            stress_matrix.Result(category, subject, True, "verified runtime evidence")
            for category in stress_matrix.UNIVERSAL_FRAMEWORK_CATEGORIES
            if category != "framework-language-scaffold"
        ]
        results.extend(
            [
                stress_matrix.Result(
                    "framework-language-scaffold",
                    subject,
                    True,
                    json.dumps({"languages_or_source_formats": ["Rust"]}),
                ),
                stress_matrix.Result(
                    "framework-language-runtime-probe",
                    f"{subject}/Rust",
                    True,
                    "verified runtime evidence",
                ),
            ]
        )
        results.extend(
            stress_matrix.Result(category, "global", True, "verified global evidence")
            for category in stress_matrix.UNIVERSAL_GLOBAL_CATEGORIES
        )

        self.assertEqual(stress_matrix.universal_support_gaps(results, [row]), [])
        self.assertTrue(stress_matrix.framework_runtime_verified(results, [row]))

        incomplete = [
            result
            for result in results
            if (result.category, result.subject)
            not in {("framework-cli-dev", subject), ("cache-runtime", subject)}
        ]
        incomplete.append(
            stress_matrix.Result(
                "cache-runtime",
                subject,
                True,
                "cache status only",
                evidence_status="not_applicable",
            )
        )
        row["dependency_ownership"]["update"]["owner"] = "unsupported"
        gaps = stress_matrix.universal_support_gaps(incomplete, [row])

        self.assertIn(
            "framework-cli-dev:web/vanilla expected exactly one runtime result, found 0",
            gaps,
        )
        self.assertIn("cache-runtime:web/vanilla is not_applicable (passed=True)", gaps)
        self.assertIn("dependency-owner:web/vanilla:update is unsupported", gaps)
        self.assertFalse(stress_matrix.framework_runtime_verified(incomplete, [row]))

    def test_universal_gate_requires_per_framework_language_runtime_evidence(self) -> None:
        subject = "lib/rust"
        row = {
            "core": "lib",
            "framework": "rust",
            "status": "scaffold-only",
            "dependency_ownership": {
                operation: {"owner": "mgc-native"}
                for operation in stress_matrix.DEPENDENCY_OPERATIONS
            },
        }
        results = [
            stress_matrix.Result(category, subject, True, "verified")
            for category in stress_matrix.UNIVERSAL_FRAMEWORK_CATEGORIES
            if category != "framework-language-scaffold"
        ]
        results.append(
            stress_matrix.Result(
                "framework-language-scaffold",
                subject,
                True,
                json.dumps({"languages_or_source_formats": ["Rust"]}),
            )
        )
        results.extend(
            stress_matrix.Result(category, "global", True, "verified")
            for category in stress_matrix.UNIVERSAL_GLOBAL_CATEGORIES
        )

        gaps = stress_matrix.universal_support_gaps(results, [row])

        self.assertEqual(
            gaps,
            ["framework-language-runtime-probe:lib/rust/Rust expected exactly one result, found 0"],
        )

    def test_language_gate_checks_each_standalone_language_and_leaves_framework_formats_to_lifecycle(self) -> None:
        row = {"core": "web", "framework": "sample"}
        subject = "web/sample"
        results = [
            stress_matrix.Result(
                "framework-language-scaffold",
                subject,
                True,
                json.dumps({"languages_or_source_formats": ["Rust", "Python", "CSS"]}),
            ),
            stress_matrix.Result(
                "framework-language-runtime-probe",
                f"{subject}/Rust",
                True,
                "runtime started",
            ),
            stress_matrix.Result(
                "framework-language-runtime-probe",
                f"{subject}/Python",
                True,
                "runtime started",
            ),
            stress_matrix.Result(
                "framework-language-runtime-probe",
                f"{subject}/CSS",
                True,
                "CSS is checked by the framework lifecycle",
                evidence_status="not_applicable",
            ),
        ]

        self.assertEqual(stress_matrix.framework_language_runtime_gaps(results, [row]), [])

        results[1] = stress_matrix.Result(
            "framework-language-runtime-probe",
            f"{subject}/Rust",
            True,
            "toolchain missing",
            evidence_status="unverified",
        )
        del results[2]
        self.assertEqual(
            stress_matrix.framework_language_runtime_gaps(results, [row]),
            [
                "framework-language-runtime-probe:web/sample/Python expected exactly one result, found 0",
                "framework-language-runtime-probe:web/sample/Rust is unverified (passed=True)",
            ],
        )

    def test_language_inventory_labels_are_all_runtime_or_framework_scoped(self) -> None:
        emitted_labels = set(stress_matrix.SOURCE_LANGUAGE_BY_SUFFIX.values()) | {
            "CI workflow YAML",
            "JSON hardware descriptor",
        }
        classified_labels = set(stress_matrix.RUNTIME_PROBE_LANGUAGE_ORDER) | set(
            stress_matrix.NON_STANDALONE_RUNTIME_FORMATS
        )

        self.assertEqual(emitted_labels - classified_labels, set())

    def test_language_gate_rejects_duplicate_or_malformed_scaffold_evidence(self) -> None:
        row = {"core": "lib", "framework": "rust"}
        subject = "lib/rust"
        malformed = stress_matrix.Result(
            "framework-language-scaffold", subject, True, "not-json"
        )
        self.assertEqual(
            stress_matrix.framework_language_runtime_gaps([malformed], [row]),
            ["framework-language-scaffold:lib/rust has malformed language inventory"],
        )

        inventory = stress_matrix.Result(
            "framework-language-scaffold",
            subject,
            True,
            json.dumps({"languages_or_source_formats": ["Rust"]}),
        )
        self.assertEqual(
            stress_matrix.framework_language_runtime_gaps([inventory, inventory], [row]),
            ["framework-language-scaffold:lib/rust expected exactly one result, found 2"],
        )

    def test_source_provenance_binds_report_to_commit_and_binary_bytes(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            binary = root / "mgc"
            binary_bytes = b"MagiCore test binary bytes"
            binary.write_bytes(binary_bytes)
            commit = "a" * 40
            git_results = [
                subprocess.CompletedProcess(["git", "rev-parse", "HEAD"], 0, f"{commit}\n", ""),
                subprocess.CompletedProcess(["git", "status"], 0, "", ""),
            ]
            with patch.object(stress_matrix.subprocess, "run", side_effect=git_results) as run:
                provenance = stress_matrix.collect_source_provenance(binary, root)

            self.assertEqual(provenance["git_commit"], commit)
            self.assertTrue(provenance["working_tree_clean"])
            self.assertEqual(
                provenance["binary_sha256"],
                hashlib.sha256(binary_bytes).hexdigest(),
            )
            self.assertEqual(run.call_count, 2)

    def test_dependency_ownership_summary_counts_declared_owners_per_operation(self) -> None:
        owners = {
            operation: {"owner": "mgc-native" if operation == "install" else "unsupported"}
            for operation in stress_matrix.DEPENDENCY_OPERATIONS
        }
        rows = [
            {
                "core": "web",
                "framework": "react-vite",
                "status": "scaffold-only",
                "dependency_ownership": owners,
            },
            {
                "core": "web",
                "framework": "vanilla",
                "status": "mgc-engine-path",
                "dependency_ownership": owners,
            },
        ]

        summary = stress_matrix.dependency_ownership_summary(rows)

        self.assertEqual(summary["by_core_operation"]["web"]["install"]["mgc-native"], 2)
        self.assertEqual(summary["by_core_operation"]["web"]["add"]["unsupported"], 2)
        self.assertEqual(
            [row["framework"] for row in summary["frameworks"]],
            ["react-vite", "vanilla"],
        )
        self.assertFalse(summary["runtime_verified"])

    def test_optimizer_without_a_registered_runtime_is_unverified(self) -> None:
        completed = subprocess.CompletedProcess(
            ["mgc", "optimizer"],
            1,
            "Error: No runtime detected for `cicd` core — nothing was optimized (not a success).",
        )
        passed, evidence_status, detail = stress_matrix.assess_dimension("optimizer", completed)
        self.assertTrue(passed)
        self.assertEqual(evidence_status, "unverified")
        self.assertIn("no optimizer runtime", detail)

    def test_optimizer_without_runtime_is_not_applicable_when_lane_claims_no_support(self) -> None:
        completed = subprocess.CompletedProcess(
            ["mgc", "optimizer"],
            1,
            "Error: No runtime detected for `cicd` core — nothing was optimized (not a success).",
        )
        passed, evidence_status, detail = stress_matrix.assess_dimension(
            "optimizer", completed, optimizer_no_runtime_is_not_applicable=True
        )
        self.assertTrue(passed)
        self.assertEqual(evidence_status, "not_applicable")
        self.assertIn("no optimizer capability is claimed", detail)

    def test_empty_installed_graph_is_not_an_audit_scan_claim(self) -> None:
        completed = subprocess.CompletedProcess(
            ["mgc", "audit"],
            0,
            '{"scanner_status":"available","packages_audited":0,"vulnerability_count":0}',
        )
        passed, evidence_status, detail = stress_matrix.assess_dimension(
            "security-audit", completed, empty_graph_installed=True
        )
        self.assertTrue(passed)
        self.assertEqual(evidence_status, "not_applicable")
        self.assertIn("non-empty audit behavior is covered by audit E2E", detail)

    def test_missing_external_lock_is_not_applicable_to_native_empty_graph_matrix(self) -> None:
        completed = subprocess.CompletedProcess(
            ["mgc", "audit"],
            2,
            '{"scanner_status":"partial","packages_audited":0,"vulnerability_count":0,'
            '"skipped_reasons":["lockfile is missing"]}\nError: scanner unavailable in strict mode',
        )
        passed, evidence_status, detail = stress_matrix.assess_dimension(
            "security-audit", completed, native_install_owned=False
        )
        self.assertTrue(passed)
        self.assertEqual(evidence_status, "not_applicable")
        self.assertIn("no MGC-owned resolved dependency graph", detail)

    def test_unowned_cargo_feature_audit_gap_is_not_a_native_lane_claim(self) -> None:
        completed = subprocess.CompletedProcess(
            ["mgc", "audit"],
            1,
            "iot adapter audit failed: lib core does not support native Cargo dependency resolution",
        )
        passed, evidence_status, detail = stress_matrix.assess_dimension(
            "security-audit", completed, native_install_owned=False
        )
        self.assertTrue(passed)
        self.assertEqual(evidence_status, "not_applicable")
        self.assertIn("no MGC-owned non-empty Cargo graph", detail)

    def test_empty_graph_scope_does_not_hide_security_findings(self) -> None:
        completed = subprocess.CompletedProcess(
            ["mgc", "audit"],
            1,
            '{"scanner_status":"available","packages_audited":1,"vulnerability_count":1}',
        )
        passed, evidence_status, detail = stress_matrix.assess_dimension(
            "security-audit", completed, empty_graph_installed=True
        )
        self.assertFalse(passed)
        self.assertEqual(evidence_status, "failed")
        self.assertIn("finding", detail)

    def test_security_audit_refusal_is_not_scanner_evidence(self) -> None:
        completed = subprocess.CompletedProcess(
            ["mgc", "audit"],
            2,
            '{"scanner_status":"partial","packages_audited":0,"vulnerability_count":0,'
            '"skipped_reasons":["lockfile is missing"]}\nError: scanner unavailable in strict mode',
        )
        passed, evidence_status, detail = stress_matrix.assess_dimension("security-audit", completed)
        self.assertTrue(passed)
        self.assertEqual(evidence_status, "unverified")
        self.assertIn("strict audit refused", detail)

    def test_audit_scanner_failure_with_manifest_count_is_unverified(self) -> None:
        completed = subprocess.CompletedProcess(
            ["mgc", "audit"],
            2,
            '{"scanner_status":"failed","packages_audited":1,"vulnerability_count":0,'
            '"skipped_reasons":["no supported resolved lockfile"]}',
        )
        passed, evidence_status, detail = stress_matrix.assess_dimension("security-audit", completed)
        self.assertTrue(passed)
        self.assertEqual(evidence_status, "unverified")
        self.assertIn("strict audit refused", detail)

    def test_audit_native_resolver_gap_is_unverified(self) -> None:
        completed = subprocess.CompletedProcess(
            ["mgc", "audit"],
            1,
            "iot adapter audit failed: lib core does not support native Cargo dependency resolution",
        )
        passed, evidence_status, detail = stress_matrix.assess_dimension("security-audit", completed)
        self.assertTrue(passed)
        self.assertEqual(evidence_status, "unverified")
        self.assertIn("feature semantics", detail)

    def test_json_with_trailing_cli_error_is_still_parsed(self) -> None:
        payload = stress_matrix._parse_json_object(
            'WARNING: audit is strict\n{"scanner_status":"partial","packages_audited":0}'
            '\nError: scanner unavailable in strict mode'
        )
        self.assertEqual(payload, {"scanner_status": "partial", "packages_audited": 0})

    def test_security_finding_remains_a_failure(self) -> None:
        completed = subprocess.CompletedProcess(
            ["mgc", "audit"],
            1,
            '{"scanner_status":"available","packages_audited":1,"vulnerability_count":1}',
        )
        passed, evidence_status, detail = stress_matrix.assess_dimension("security-audit", completed)
        self.assertFalse(passed)
        self.assertEqual(evidence_status, "failed")
        self.assertIn("finding", detail)


if __name__ == "__main__":
    unittest.main()
