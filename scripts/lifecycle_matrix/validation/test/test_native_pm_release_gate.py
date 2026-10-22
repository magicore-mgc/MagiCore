"""Focused NativePackageManagerVerdict test owner.

Owner test hẹp cho NativePackageManagerVerdict.
"""

from .support import (
    LANES,
    PLATFORM_EVIDENCE_RELEASE_SCOPE,
    REPO_ROOT,
    native_pm_claim_scope,
    release_scope_out_reason,
    unittest,
)

class NativePackageManagerVerdict(unittest.TestCase):
    def expanded_workflow(self, name):
        # Inspect only literal scripts invoked by a workflow step.
        # Đọc đúng script được step gọi sau khi inline code đã được tách owner.
        workflow = (REPO_ROOT / ".github/workflows" / name).read_text(encoding="utf-8")
        import re
        def expand(match):
            line = match.group(0)
            script = re.search(r"(?:[A-Za-z0-9_.-]+/)*scripts/[A-Za-z0-9_./-]+\.(?:ps1|sh|py)", line)
            if script is None:
                return line
            target = REPO_ROOT / script.group(0)
            self.assertTrue(target.is_file(), str(target))
            return line + "\n" + "\n".join("          " + part for part in target.read_text().splitlines())
        return re.sub(r"^        run: .+$", expand, workflow, flags=re.MULTILINE)

    def test_release_scopeouts_are_named_and_do_not_remove_lanes_from_inventory(self):
        expected = {
            ("lib", "java", "java"),
            ("lib", "dotnet", "dotnet"),
            ("app", "swift", ""),
            ("app", "objc", ""),
            ("app", "react-native", ""),
            ("clo", "terraform", ""),
            ("game", "rust", ""),
            ("hardware", "benchmark", ""),
            ("iot", "rust", ""),
            ("cicd", "github-actions", ""),
        }
        self.assertTrue(expected.issubset(PLATFORM_EVIDENCE_RELEASE_SCOPE))
        self.assertTrue(all(release_scope_out_reason(*key) for key in expected))
        evidence_only = {
            (lane["core"], lane["language"], lane.get("framework_id", ""))
            for lane in LANES
            if lane.get("evidence_only")
        }
        self.assertEqual(expected, evidence_only)
        self.assertEqual(
            expected & native_pm_claim_scope(LANES),
            {
                ("lib", "java", "java"),
                ("lib", "dotnet", "dotnet"),
                ("app", "swift", ""),
                ("game", "rust", ""),
                ("iot", "rust", ""),
            },
        )
        self.assertIsNone(release_scope_out_reason("web", "javascript"))

    def test_dotnet_lifecycle_is_scoped_out_until_native_build_state_exists(self):
        lane = next(
            lane for lane in LANES
            if (lane["core"], lane["language"], lane["framework_id"])
            == ("lib", "dotnet", "dotnet")
        )
        reason = release_scope_out_reason("lib", "dotnet", "dotnet")

        self.assertTrue(lane["evidence_only"])
        self.assertEqual([step for step, _ in lane["steps"]], ["install"])
        self.assertEqual(
            lane["lifecycle_owner_overrides"],
            {"test": "unsupported", "build": "unsupported"},
        )
        self.assertIn("MSBuild restore state", reason)
        self.assertIn("00-index §5.1", reason)

    def test_lifecycle_workflow_uses_shared_native_claim_gate(self):
        workflow = (REPO_ROOT / ".github/workflows/lifecycle-matrix.yml").read_text()
        self.assertIn("scripts/lifecycle_matrix/validation/release_gate/main.py", workflow)
        self.assertGreaterEqual(workflow.count('- "scripts/**"'), 2)
        entry = (REPO_ROOT / "scripts/lifecycle_matrix/validation/release_gate/main.py").read_text()
        for call in ("prepare_gate(data)", "validate_lanes(state)", "validate_scope(state)",
                     "if state.failures:", "raise SystemExit(1)"):
            self.assertIn(call, entry)


    def test_lifecycle_workflow_loads_dimensions_before_owner_consistency_check(self):
        source = (REPO_ROOT / "scripts/lifecycle_matrix/validation/release_gate/lane_dimensions.py").read_text()
        self.assertLess(source.index('dimensions = lane.get("dimensions", {})'),
                        source.index("lifecycle_status_owner_matches("))
        self.assertIn('if not isinstance(dimensions, dict):', source)


    def test_lifecycle_workflow_enforces_matrix_on_every_operating_system(self):
        workflow = self.expanded_workflow("lifecycle-matrix.yml")
        self.assertIn("os: [ubuntu-24.04, macos-latest, windows-latest]", workflow)
        self.assertIn(
            "swift-actions/setup-swift@364295d9c23900ce04d4e5cc708387921b4e50f9 # v3.0.0-beta.1",
            workflow,
        )
        self.assertIn("if: matrix.os == 'ubuntu-24.04'", workflow)
        self.assertIn('swift-version: "6.4.1"', workflow)
        self.assertIn("id: swift_python_310", workflow)
        self.assertIn("${{ steps.swift_python_310.outputs.python-path }}", workflow)
        self.assertIn("- name: Preserve Swift Python 3.10 runtime (Windows)", workflow)
        preserved_python = workflow.split(
            "- name: Preserve Swift Python 3.10 runtime (Windows)", 1
        )[1].split("- name:", 1)[0]
        self.assertIn("$PSNativeCommandUseErrorActionPreference = $false", preserved_python)
        self.assertIn("robocopy.exe $sourceBin $pythonBin /E", preserved_python)
        self.assertIn("if ($LASTEXITCODE -gt 7)", preserved_python)
        self.assertIn('Join-Path $pythonBin "python.exe"', preserved_python)
        self.assertIn("$preservedPython --version", preserved_python)
        self.assertIn("MGC_SWIFT_PYTHON_BIN=$pythonBin", preserved_python)
        self.assertLess(
            workflow.index("- name: Preserve Swift Python 3.10 runtime (Windows)"),
            workflow.index("- name: Setup Swift toolchain (Windows)"),
        )
        release_workflow = self.expanded_workflow("release-binary-e2e.yml")
        self.assertIn("os: [ubuntu-24.04, macos-latest, windows-2022]", release_workflow)
        self.assertIn(
            "swift-actions/setup-swift@364295d9c23900ce04d4e5cc708387921b4e50f9 # v3.0.0-beta.1",
            release_workflow,
        )
        self.assertIn("if: matrix.os == 'ubuntu-24.04'", release_workflow)
        self.assertIn('swift-version: "6.4.1"', release_workflow)
        self.assertIn("id: swift_python_310", release_workflow)
        self.assertIn("${{ steps.swift_python_310.outputs.python-path }}", release_workflow)
        swift_windows_setup = release_workflow.split(
            "- name: Setup Swift toolchain (Windows)", 1
        )[1].split("- name: Lifecycle app swift", 1)[0]
        self.assertIn(
            "https://download.swift.org/swift-6.4.1-release/windows10/swift-6.4.1-RELEASE/swift-6.4.1-RELEASE-windows10.exe",
            swift_windows_setup,
        )
        self.assertIn(
            "9382E14919EEAE5E7B5DBFE0575452FE2154178E089A595C8B44D03960D91D59",
            swift_windows_setup,
        )
        self.assertIn("Get-AuthenticodeSignature", swift_windows_setup)
        self.assertNotIn("winget install", swift_windows_setup)
        for current_workflow in (workflow, release_workflow):
            windows_setup = current_workflow.split(
                "- name: Setup Swift toolchain (Windows)", 1
            )[1].split("\n      - name:", 1)[0]
            self.assertIn(
                "[Environment]::GetEnvironmentVariable('SDKROOT', 'User')",
                windows_setup,
            )
            self.assertIn(
                "Test-Path -LiteralPath $sdkRoot -PathType Container",
                windows_setup,
            )
            self.assertIn(
                'Add-Content -Path $env:GITHUB_ENV -Value "SDKROOT=$sdkRoot"',
                windows_setup,
            )
            self.assertIn(
                'Join-Path (Split-Path -Parent $swiftRoot) "Runtimes\\$swiftRuntimeVersion\\usr\\bin"',
                windows_setup,
            )
            self.assertIn(
                'Join-Path $swiftRuntimeBin "swiftCore.dll"',
                windows_setup,
            )
            self.assertIn(
                'Add-Content -Path $env:GITHUB_PATH -Value $swiftRuntimeBin',
                windows_setup,
            )
        for current_workflow in (workflow, release_workflow):
            key_import = current_workflow.index(
                "- name: Import Swift.org signing keys (Linux)"
            )
            swift_setup = current_workflow.index(
                "- name: Setup Swift toolchain (Ubuntu/macOS)"
            )
            self.assertLess(key_import, swift_setup)
            self.assertIn("https://www.swift.org/keys/all-keys.asc", current_workflow)
            self.assertIn("E813C892820A6FA13755B268F167DF1ACF9CE069", current_workflow)
            self.assertIn("52BB7E3DE28A71BE22EC05FFEF80A866B47A981F", current_workflow)
            self.assertNotIn("skip-verify-signature: true", current_workflow)
        self.assertIn("${{ matrix.python_cmd }} scripts/lifecycle_matrix/validation/release_gate/main.py", workflow)
        self.assertNotIn("shell: ${{ matrix.", workflow)
        self.assertNotIn("if: matrix.os != 'windows-latest'", workflow)
        self.assertNotIn("windows-evidence-promotion:", workflow)
