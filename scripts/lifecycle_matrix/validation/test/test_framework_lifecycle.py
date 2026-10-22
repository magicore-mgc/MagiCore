"""Focused NativeFrameworkLifecycleCoverage test owner.

Owner test hẹp cho NativeFrameworkLifecycleCoverage.
"""

from .support import (
    ALL_DEPENDENCY_OPERATIONS,
    LANES,
    Path,
    REPO_ROOT,
    SCHEMA_VERSION,
    SCRIPTS_ROOT,
    framework_catalog_errors,
    re,
    scaffold_language_matches,
    tempfile,
    unittest,
    validate_lane_registry,
)

class NativeFrameworkLifecycleCoverage(unittest.TestCase):
    def test_framework_qualified_release_scope_uses_new_schema(self):
        self.assertEqual(SCHEMA_VERSION, 7)

    def catalog_record(self, framework, *, core="lib", status="mgc-engine-path", operations=None):
        operation_owners = operations or {
            operation: {"owner": "mgc-native"}
            for operation in ALL_DEPENDENCY_OPERATIONS
        }
        return {
            "core": core,
            "framework": framework,
            "status": status,
            "dependency_ownership": operation_owners,
            "evidence": "test fixture",
        }

    def test_catalog_native_framework_with_complete_lane_and_owners_passes(self):
        source_lane = next(
            lane for lane in LANES
            if lane["core"] == "lib" and lane["language"] == "rust"
        )
        lanes = [{
            "core": "lib",
            "framework_id": "rust",
            "owner_by_operation": source_lane["owner_by_operation"],
        }]
        self.assertEqual(
            framework_catalog_errors(
                [self.catalog_record("rust", operations=self.catalog_owners(source_lane))], lanes
            ),
            [],
        )

    def catalog_owners(self, lane):
        owners = {}
        for operation, matrix_owner in lane["owner_by_operation"].items():
            owner = (
                "mgc-native"
                if matrix_owner in {"mgc", "magicore-shared-cas"}
                else "scaffold-only"
                if matrix_owner == "scaffold-only"
                else "unsupported"
            )
            owners[operation] = {"owner": owner}
        return owners

    def test_native_framework_can_explicitly_leave_an_operation_unsupported(self):
        source_lane = next(
            lane for lane in LANES
            if lane["core"] == "lib" and lane["language"] == "rust"
        )
        owners = self.catalog_owners(source_lane)
        self.assertEqual(owners["gc"]["owner"], "unsupported")
        self.assertEqual(
            framework_catalog_errors(
                [self.catalog_record("rust", operations=owners)],
                [{
                    "core": "lib",
                    "framework_id": "rust",
                    "owner_by_operation": source_lane["owner_by_operation"],
                }],
            ),
            [],
        )

    def test_native_framework_operation_cannot_be_downgraded_from_lane_contract(self):
        source_lane = next(
            lane for lane in LANES
            if lane["core"] == "lib" and lane["language"] == "rust"
        )
        owners = self.catalog_owners(source_lane)
        owners["install"] = {"owner": "unsupported"}
        errors = framework_catalog_errors(
            [self.catalog_record("rust", operations=owners)],
            [{
                "core": "lib",
                "framework_id": "rust",
                "owner_by_operation": source_lane["owner_by_operation"],
            }],
        )
        self.assertTrue(any("operation install differs from lifecycle owner" in error for error in errors))

    def test_catalog_scaffold_only_framework_with_reason_is_an_explicit_scopeout(self):
        row = self.catalog_record("django", status="scaffold-only")
        row["evidence"] = "Framework is scaffolded; lifecycle qualification is not claimed."
        self.assertEqual(framework_catalog_errors([row], []), [])

    def test_catalog_scaffold_only_framework_without_reason_fails_closed(self):
        row = self.catalog_record("django", status="scaffold-only")
        row["evidence"] = "  "
        errors = framework_catalog_errors([row], [])
        self.assertTrue(any("scope-out reason" in error for error in errors))

    def test_catalog_framework_missing_dependency_owner_blocks_promotion(self):
        operations = {
            operation: {"owner": "mgc-native"}
            for operation in ALL_DEPENDENCY_OPERATIONS
        }
        del operations["offline-reinstall"]
        errors = framework_catalog_errors(
            [self.catalog_record("rust", operations=operations)],
            [{
                "core": "lib",
                "framework_id": "rust",
                "owner_by_operation": next(
                    lane["owner_by_operation"] for lane in LANES
                    if lane["core"] == "lib" and lane["language"] == "rust"
                ),
            }],
        )
        self.assertTrue(any("offline-reinstall" in error for error in errors))

    def test_catalog_rejects_duplicate_or_unregistered_framework_identity(self):
        lanes = [{"core": "lib", "framework_id": "rust"}]
        errors = framework_catalog_errors(
            [self.catalog_record("rust"), self.catalog_record("rust")],
            [{"core": "lib", "framework_id": "go"}],
        )
        self.assertTrue(any("duplicate framework catalog" in error for error in errors))
        self.assertTrue(any("not present in framework catalog" in error for error in errors))

    def test_matrix_producer_requires_compiled_framework_and_owner_crosscheck(self):
        source = (
            SCRIPTS_ROOT
            / "lifecycle_matrix/cli/preflight.py"
        ).read_text(encoding="utf-8")
        self.assertIn("dep_gate_rc = validate_dep_gate_consistency(mgc_bin)", source)
        self.assertIn('"framework_catalog": FRAMEWORK_QUALIFICATION_EVIDENCE',
                      (SCRIPTS_ROOT / "lifecycle_matrix/cli/document.py").read_text())
        self.assertIn("or dep_gate_rc != 0", source)

    def test_lane_registry_rejects_duplicate_full_lane_identity(self):
        duplicate = [dict(LANES[0]), dict(LANES[0])]
        with self.assertRaisesRegex(ValueError, "duplicate core/language/framework"):
            validate_lane_registry(duplicate)

    def test_lane_registry_allows_distinct_frameworks_for_same_core_language(self):
        first = dict(LANES[0])
        second = dict(LANES[0], framework_id="rust-macro", framework_ids=["rust-macro"])
        self.assertTrue(validate_lane_registry([first, second]))

    def test_lane_registry_rejects_unknown_required_dimension(self):
        invalid = [dict(LANES[0], required_dims=["not-a-dimension"])]
        with self.assertRaisesRegex(ValueError, "unknown required dimension"):
            validate_lane_registry(invalid)

    def test_lane_registry_rejects_invalid_lifecycle_owner_override(self):
        invalid = [dict(LANES[0], lifecycle_owner_overrides={"dev": "vite"})]
        with self.assertRaisesRegex(ValueError, "invalid lifecycle owner override"):
            validate_lane_registry(invalid)

    def test_lane_registry_rejects_unproven_native_build_test_run_overrides(self):
        for lane in LANES:
            for dimension in ("build", "test", "run"):
                invalid = dict(
                    lane,
                    lifecycle_owner_overrides={dimension: "mgc-native"},
                )
                with self.subTest(
                    core=lane["core"],
                    language=lane["language"],
                    framework=lane.get("framework_id", ""),
                    dimension=dimension,
                ):
                    with self.assertRaisesRegex(
                        ValueError, "native lifecycle owner override lacks evidence"
                    ):
                        validate_lane_registry([invalid])

    def test_lane_registry_rejects_framework_lane_mismatch(self):
        invalid = [dict(LANES[0], framework_id="not-the-declared-id")]
        with self.assertRaisesRegex(ValueError, "framework_id must match"):
            validate_lane_registry(invalid)

    def test_lane_registry_rejects_framework_ids_without_exact_binding(self):
        invalid = [dict(LANES[0], framework_id=None)]
        with self.assertRaisesRegex(ValueError, "requires an exact framework_id"):
            validate_lane_registry(invalid)

    def test_lane_registry_rejects_duplicate_framework_identity(self):
        first = dict(LANES[0])
        second = dict(LANES[1], framework_id=first["framework_id"],
                      framework_ids=list(first["framework_ids"]))
        with self.assertRaisesRegex(ValueError, "duplicate core/framework"):
            validate_lane_registry([first, second])

    def missing_native_lanes(self, source, lanes):
        # Match one Rust struct literal only; matching a bare `}` can span
        # adjacent records because each literal closes with `},`.
        # Khớp đúng một struct literal; bare `}` sẽ nuốt nhiều record liền nhau.
        records = re.findall(r"FrameworkRecord\s*\{(.*?)\n\s*\},", source, re.S)
        native_frameworks = set()
        for record in records:
            if "FrameworkStatus::NativeEngine" not in record:
                continue
            core = re.search(r'core:\s*"([^"]+)"', record)
            framework = re.search(r'framework:\s*"([^"]+)"', record)
            self.assertIsNotNone(core, f"native framework record has no core: {record}")
            self.assertIsNotNone(
                framework, f"native framework record has no id: {record}"
            )
            native_frameworks.add((core.group(1), framework.group(1)))

        covered = {
            (lane["core"], framework)
            for lane in lanes
            for framework in lane.get("framework_ids", ())
        }
        return native_frameworks - covered

    def test_every_native_engine_framework_has_a_lifecycle_lane(self):
        # Registry engine routes need executable lifecycle lanes, not only
        # per-operation declarations. Registry engine route phải có lane.
        root = REPO_ROOT
        source = "\n".join(
            path.read_text(encoding="utf-8")
            for path in sorted((root / "cli/src/commands/framework_records/records").rglob("*.rs"))
            if "test" not in path.parts
        )
        missing = self.missing_native_lanes(source, LANES)
        self.assertEqual(
            missing,
            set(),
            "native engine framework(s) without lifecycle evidence lane: "
            f"{sorted(missing)}",
        )

    def test_negative_control_detects_removed_cloud_framework_lanes(self):
        root = REPO_ROOT
        source = "\n".join(
            path.read_text(encoding="utf-8")
            for path in sorted((root / "cli/src/commands/framework_records/records").rglob("*.rs"))
            if "test" not in path.parts
        )
        without_cloud = [lane for lane in LANES if lane["core"] != "clo"]
        missing = self.missing_native_lanes(source, without_cloud)
        self.assertEqual(missing, {("clo", "cdk"), ("clo", "pulumi")})

    def test_framework_specific_lanes_bind_exactly_one_matching_id(self):
        for lane in LANES:
            framework_id = lane.get("framework_id")
            if framework_id:
                self.assertEqual(
                    lane.get("framework_ids"),
                    [framework_id],
                    f"{lane['core']}/{lane['language']} must bind its exact framework",
                )

    def test_typescript_scaffold_requires_source_and_config_markers(self):
        with tempfile.TemporaryDirectory() as sandbox:
            project = Path(sandbox) / "test-web-ts"
            project.mkdir()
            (project / "index.html").write_text("<main></main>", encoding="utf-8")
            self.assertFalse(
                scaffold_language_matches(
                    sandbox,
                    project.name,
                    "ts",
                    ["index.html", "src/main.ts", "tsconfig.json"],
                )
            )
            (project / "src").mkdir()
            (project / "src/main.ts").write_text("export {}", encoding="utf-8")
            (project / "tsconfig.json").write_text("{}", encoding="utf-8")
            self.assertTrue(
                scaffold_language_matches(
                    sandbox,
                    project.name,
                    "ts",
                    ["index.html", "src/main.ts", "tsconfig.json"],
                )
            )
