"""Focused PlatformGreenCounter test owner.

Owner test hẹp cho PlatformGreenCounter.
"""

from .support import (
    LANES,
    Path,
    PlatformGreenMatrixFixture,
    SCHEMA_VERSION,
    STATUS_FAILED,
    STATUS_NATIVE,
    STATUS_PLAIN,
    STATUS_UNSUPPORTED,
    STATUS_UNVERIFIED,
    evidence_commit_error,
    json,
    lifecycle_owner_for,
    lifecycle_pass_status,
    lifecycle_status_for_owner,
    patch,
    platform_evidence_errors,
    record_platform_green,
    subprocess,
    tempfile,
    unittest,
)

class PlatformGreenCounter(PlatformGreenMatrixFixture, unittest.TestCase):
    def test_dirty_or_unknown_source_tree_cannot_record_platform_green(self):
        for value in (False, None):
            with self.subTest(working_tree_clean=value):
                matrix = self.green_matrix()
                if value is None:
                    del matrix["working_tree_clean"]
                else:
                    matrix["working_tree_clean"] = value
                errors = platform_evidence_errors(
                    matrix, "a" * 40, current_tree_clean=True
                )
                self.assertTrue(any("working tree" in error for error in errors))

    def test_stack_overflow_lane_cannot_increment_green_counter(self):
        matrix = self.green_matrix()
        matrix["lanes"][0]["dimensions"]["create"] = STATUS_FAILED
        matrix["lanes"][0]["verdict"] = "partial"
        errors = platform_evidence_errors(
            matrix, "a" * 40, current_tree_clean=True
        )
        self.assertTrue(any("create" in error for error in errors))

    def test_missing_release_lane_cannot_increment_green_counter(self):
        matrix = self.green_matrix()
        matrix["lanes"].pop()
        self.assertTrue(
            platform_evidence_errors(
                matrix, "a" * 40, current_tree_clean=True
            )
        )

    def test_missing_full_framework_inventory_cannot_increment_green_counter(self):
        matrix = self.green_matrix()
        del matrix["framework_catalog"]
        errors = platform_evidence_errors(
            matrix, "a" * 40, current_tree_clean=True
        )
        self.assertTrue(any("framework catalog" in error for error in errors))

    def test_framework_id_is_part_of_release_evidence_identity(self):
        matrix = self.green_matrix()
        cdk = next(
            lane for lane in matrix["lanes"]
            if lane["core"] == "clo" and lane.get("framework_id") == "cdk"
        )
        cdk["framework_id"] = ""
        errors = platform_evidence_errors(
            matrix, "a" * 40, current_tree_clean=True
        )
        self.assertTrue(any("outside global v1.2 scope" in error for error in errors))
        self.assertTrue(any("clo/cdk" in error and "missing" in error for error in errors))

    def test_lifecycle_owner_must_match_the_source_contract(self):
        matrix = self.green_matrix()
        lane = next(
            lane for lane in matrix["lanes"]
            if lane["core"] == "app" and lane["language"] == "flutter"
        )
        source_lane = next(
            source for source in LANES
            if source["core"] == "app" and source["language"] == "flutter"
        )
        source_owner = lifecycle_owner_for(source_lane, "build")
        lane["lifecycle_owners"]["build"] = (
            "mgc-native" if source_owner == "plain-delegation" else "plain-delegation"
        )
        errors = platform_evidence_errors(
            matrix, "a" * 40, current_tree_clean=True
        )
        self.assertTrue(any("build" in error and "source contract" in error for error in errors))

    def test_native_lifecycle_status_must_match_declared_owner(self):
        matrix = self.green_matrix()
        lane = next(
            lane for lane in matrix["lanes"]
            if lane["core"] == "app" and lane["language"] == "flutter"
        )
        source_lane = next(
            source for source in LANES
            if source["core"] == "app" and source["language"] == "flutter"
        )
        expected_status = lifecycle_status_for_owner(
            lifecycle_owner_for(source_lane, "build")
        )
        lane["dimensions"]["build"] = (
            STATUS_NATIVE if expected_status != STATUS_NATIVE else STATUS_PLAIN
        )
        errors = platform_evidence_errors(
            matrix, "a" * 40, current_tree_clean=True
        )
        self.assertTrue(any("status/owner mismatch for build" in error for error in errors))

    def test_platform_evidence_rejects_owner_claim_not_in_source_registry(self):
        matrix = self.green_matrix()
        lane = next(
            lane for lane in matrix["lanes"]
            if lane["core"] == "app" and lane["language"] == "flutter"
        )
        lane["lifecycle_owners"]["build"] = "mgc-native"
        lane["dimensions"]["build"] = STATUS_NATIVE
        errors = platform_evidence_errors(
            matrix, "a" * 40, current_tree_clean=True
        )
        self.assertTrue(
            any("lifecycle owner differs from source contract for build" in error for error in errors)
        )

    def test_lifecycle_owner_status_mapping_is_fail_closed(self):
        self.assertEqual(lifecycle_status_for_owner("mgc-native"), STATUS_NATIVE)
        self.assertEqual(lifecycle_status_for_owner("plain-delegation"), STATUS_PLAIN)
        self.assertEqual(lifecycle_status_for_owner("unknown"), STATUS_UNVERIFIED)

    def test_external_test_and_build_wrappers_are_not_native_evidence(self):
        for lane in LANES:
            for dimension in ("test", "build", "run"):
                with self.subTest(core=lane["core"], language=lane["language"], dimension=dimension):
                    owner = lifecycle_owner_for(lane, dimension)
                    if owner == "unsupported":
                        self.assertEqual(
                            (lane["core"], lane["language"], lane.get("framework_id", "")),
                            ("lib", "dotnet", "dotnet"),
                        )
                        self.assertEqual(
                            lifecycle_pass_status(lane, dimension), STATUS_UNSUPPORTED
                        )
                        continue
                    self.assertEqual(owner, "plain-delegation")
                    self.assertEqual(lifecycle_pass_status(lane, dimension), STATUS_PLAIN)

    def test_only_explicit_mgc_dev_server_lane_gets_native_dev_owner(self):
        web = next(
            lane for lane in LANES
            if lane["core"] == "web" and lane["language"] == "javascript"
        )
        other = next(lane for lane in LANES if lane is not web)
        self.assertEqual(lifecycle_owner_for(web, "dev"), "mgc-native")
        self.assertEqual(lifecycle_pass_status(web, "dev"), STATUS_NATIVE)
        self.assertEqual(lifecycle_owner_for(other, "dev"), "unverified")
        self.assertEqual(lifecycle_pass_status(other, "dev"), STATUS_UNVERIFIED)

    def test_missing_lifecycle_owner_is_unverified_not_native(self):
        lane = {"lifecycle_owners": {}}
        self.assertEqual(lifecycle_owner_for(lane, "optimizer"), "unverified")
        self.assertEqual(lifecycle_pass_status(lane, "optimizer"), STATUS_UNVERIFIED)

    def test_matrix_from_another_checkout_cannot_increment_green_counter(self):
        self.assertTrue(
            platform_evidence_errors(
                self.green_matrix(), "b" * 40, current_tree_clean=True
            )
        )

    def test_matrix_from_another_schema_cannot_increment_green_counter(self):
        matrix = self.green_matrix()
        matrix["schema_version"] = SCHEMA_VERSION - 1
        errors = platform_evidence_errors(
            matrix, "a" * 40, current_tree_clean=True
        )
        self.assertTrue(any("schema_version" in error for error in errors))

    def test_missing_required_dimension_cannot_increment_green_counter(self):
        matrix = self.green_matrix()
        del matrix["lanes"][0]["dimensions"]["install"]
        self.assertTrue(
            platform_evidence_errors(
                matrix, "a" * 40, current_tree_clean=True
            )
        )

    def test_record_green_rejects_unknown_platform_key(self):
        with patch("lifecycle_matrix.evidence.platform._fail", side_effect=SystemExit):
            with self.assertRaises(SystemExit):
                record_platform_green("macos-arm64")

    def test_record_green_refuses_dirty_checkout_without_writing_counter(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            matrix_path = Path(temp_dir) / "matrix.json"
            matrix_path.write_text(json.dumps(self.green_matrix()), encoding="utf-8")
            with (
                patch.dict(
                    "os.environ",
                    {
                        "MGC_LIFECYCLE_MATRIX_OUT": str(matrix_path),
                        "MGC_PLATFORM_EVIDENCE_SHA": "a" * 40,
                        "CI": "true",
                    },
                    clear=True,
                ),
                patch(
                    "lifecycle_capability_matrix.subprocess.run",
                    return_value=subprocess.CompletedProcess(
                        ["git"], 0, stdout="a" * 40, stderr=""
                    ),
                ),
                patch(
                    "lifecycle_matrix.evidence.platform.lifecycle_working_tree_clean",
                    return_value=False,
                ),
                patch("lifecycle_matrix.evidence.platform._platform_evidence_write") as write,
                patch("lifecycle_matrix.evidence.platform._fail", side_effect=SystemExit),
            ):
                with self.assertRaises(SystemExit):
                    record_platform_green("windows-latest")
                write.assert_not_called()

    def test_record_green_refuses_workflow_sha_mismatch_without_writing(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            matrix_path = Path(temp_dir) / "matrix.json"
            matrix_path.write_text(json.dumps(self.green_matrix()), encoding="utf-8")
            with (
                patch.dict(
                    "os.environ",
                    {
                        "MGC_LIFECYCLE_MATRIX_OUT": str(matrix_path),
                        "MGC_PLATFORM_EVIDENCE_SHA": "b" * 40,
                        "CI": "true",
                    },
                    clear=True,
                ),
                patch(
                    "lifecycle_capability_matrix.subprocess.run",
                    return_value=subprocess.CompletedProcess(
                        ["git"], 0, stdout="a" * 40, stderr=""
                    ),
                ),
                patch(
                    "lifecycle_matrix.evidence.platform.lifecycle_working_tree_clean",
                    return_value=True,
                ),
                patch("lifecycle_matrix.evidence.platform._platform_evidence_write") as write,
                patch("lifecycle_matrix.evidence.platform._fail", side_effect=SystemExit),
            ):
                with self.assertRaises(SystemExit):
                    record_platform_green("windows-latest")
                write.assert_not_called()

    def test_record_green_increments_only_a_clean_same_sha_matrix(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            matrix_path = Path(temp_dir) / "matrix.json"
            matrix_path.write_text(json.dumps(self.green_matrix()), encoding="utf-8")
            with (
                patch.dict(
                    "os.environ",
                    {
                        "MGC_LIFECYCLE_MATRIX_OUT": str(matrix_path),
                        "MGC_PLATFORM_EVIDENCE_SHA": "a" * 40,
                        "CI": "true",
                    },
                    clear=True,
                ),
                patch(
                    "lifecycle_capability_matrix.subprocess.run",
                    return_value=subprocess.CompletedProcess(
                        ["git"], 0, stdout="a" * 40, stderr=""
                    ),
                ),
                patch(
                    "lifecycle_matrix.evidence.platform.lifecycle_working_tree_clean",
                    return_value=True,
                ),
                patch(
                    "lifecycle_matrix.evidence.platform._platform_evidence_load",
                    return_value={"windows-latest": {"runs": 2}},
                ),
                patch("lifecycle_matrix.evidence.platform._platform_evidence_write") as write,
            ):
                self.assertEqual(record_platform_green("windows-latest"), 0)
                written = write.call_args.args[0]
                self.assertEqual(written["windows-latest"]["runs"], 3)
                self.assertEqual(
                    written["windows-latest"]["last_green_sha"], "a" * 40
                )

    def test_ci_green_record_rejects_missing_recorded_sha(self):
        self.assertIsNotNone(
            evidence_commit_error(
                "a" * 40,
                "a" * 40,
                is_ci=True,
                recorded_sha=None,
                require_recorded=True,
            )
        )
