"""Focused PlatformGreenCounter test owner.

Owner test hẹp cho PlatformGreenCounter.
"""

from .support import (
    LANES,
    PLATFORM_EVIDENCE_RELEASE_SCOPE,
    PlatformGreenMatrixFixture,
    STATUS_FAILED,
    STATUS_NATIVE,
    STATUS_UNSUPPORTED,
    lifecycle_owner_for,
    lifecycle_platform_name,
    platform_evidence_errors,
    unittest,
)

class PlatformGreenCounter(PlatformGreenMatrixFixture, unittest.TestCase):
    def test_green_matrix_fixture_matches_source_owner_contracts(self):
        errors = platform_evidence_errors(
            self.green_matrix(), "a" * 40, current_tree_clean=True
        )
        self.assertEqual(errors, [])

    def test_lifecycle_output_has_a_runtime_platform_name(self):
        self.assertTrue(lifecycle_platform_name())

    def test_v12_platform_scope_covers_every_declared_core_language_lane(self):
        self.assertEqual(
            PLATFORM_EVIDENCE_RELEASE_SCOPE,
            frozenset(
                (lane["core"], lane["language"], lane.get("framework_id", ""))
                for lane in LANES
            ),
        )

    def test_only_complete_release_scope_can_record_platform_green(self):
        # A complete matrix that preserves source ownership may pass the gate.
        self.assertEqual(
            platform_evidence_errors(
                self.green_matrix(), "a" * 40, current_tree_clean=True
            ),
            [],
        )

    def test_current_checkout_must_still_be_clean_when_recording_green(self):
        errors = platform_evidence_errors(
            self.green_matrix(), "a" * 40, current_tree_clean=False
        )
        self.assertTrue(any("current checkout" in error for error in errors))

    def test_platform_green_rejects_unproven_native_claim_outside_old_scope(self):
        matrix = self.green_matrix()
        matrix["lanes"].append(
            {
                "core": "lib",
                "language": "java",
                "verdict": "orchestration-lifecycle-passed",
                "dependency_owner": "mgc-native",
                "install_owner": "native-engine",
                "native_pm_verdict": "native-pm-supported",
                "native_pm_delegated": [],
                "dimensions": {},
                "owner_by_operation": {},
            }
        )
        errors = platform_evidence_errors(
            matrix, "a" * 40, current_tree_clean=True
        )
        self.assertTrue(any("lib/java declares native-pm-supported without complete package-manager evidence" in error for error in errors))

    def test_platform_green_rejects_lane_owner_that_differs_from_source_contract(self):
        matrix = self.green_matrix()
        lane = matrix["lanes"][0]
        lane["dependency_owner"] = "delegated"
        lane["install_owner"] = "plain-delegation"
        lane["native_pm_verdict"] = "compatibility-passed"
        errors = platform_evidence_errors(
            matrix, "a" * 40, current_tree_clean=True
        )
        self.assertTrue(any("dependency owner differs from source contract" in error for error in errors))

    def test_failed_required_probe_blocks_without_owner_status_mismatch(self):
        matrix = self.green_matrix()
        lane = next(
            row for row in matrix["lanes"]
            if row["core"] == "web" and row["language"] == "node"
        )
        lane["dimensions"]["test"] = STATUS_FAILED
        lane["dimensions"]["build"] = STATUS_FAILED
        lane["verdict"] = "partial"
        errors = platform_evidence_errors(
            matrix, "a" * 40, current_tree_clean=True
        )
        self.assertTrue(any("non-passing required dimensions" in error and "test=failed" in error for error in errors))
        self.assertFalse(any("status/owner mismatch for test" in error for error in errors))
        self.assertFalse(any("status/owner mismatch for build" in error for error in errors))

    def test_platform_green_requires_exact_reason_for_every_scoped_out_lane(self):
        matrix = self.green_matrix()
        lane = next(
            lane for lane in matrix["lanes"]
            if lane["core"] == "app" and lane["language"] == "objc"
        )
        lane["release_scope_out_reason"] = ""
        errors = platform_evidence_errors(
            matrix, "a" * 40, current_tree_clean=True
        )
        self.assertTrue(any("scope-out reason differs from source contract" in error for error in errors))

    def test_scoped_out_lane_keeps_owner_status_consistency_when_toolchain_is_available(self):
        matrix = self.green_matrix()
        lane = next(
            lane for lane in matrix["lanes"]
            if lane["core"] == "clo" and lane["language"] == "terraform"
        )
        source_lane = next(
            source for source in LANES
            if source["core"] == "clo" and source["language"] == "terraform"
        )
        lane["lifecycle_owners"] = {
            dimension: lifecycle_owner_for(source_lane, dimension)
            for dimension in source_lane["required_dims"]
        }
        lane["dimensions"]["install"] = STATUS_NATIVE
        errors = platform_evidence_errors(
            matrix, "a" * 40, current_tree_clean=True
        )
        self.assertTrue(any("status/owner mismatch for install" in error for error in errors))

    def test_platform_green_rejects_lane_without_its_required_toolchain(self):
        matrix = self.green_matrix()
        matrix["lanes"][0]["toolchain_available"] = False
        errors = platform_evidence_errors(
            matrix, "a" * 40, current_tree_clean=True
        )
        self.assertTrue(any("toolchain is unavailable" in error for error in errors))

    def test_lane_cannot_shrink_source_required_dimensions(self):
        matrix = self.green_matrix()
        lane = next(
            lane for lane in matrix["lanes"]
            if (lane["core"], lane["language"]) == ("web", "javascript")
        )
        lane["required_dimensions"] = ["create"]
        lane["dimensions"]["dev"] = STATUS_UNSUPPORTED
        errors = platform_evidence_errors(
            matrix, "a" * 40, current_tree_clean=True
        )
        self.assertTrue(any("required dimensions differ from source contract" in error for error in errors))
        self.assertTrue(any("dev=unsupported" in error for error in errors))

    def test_explicitly_non_applicable_dimension_may_be_unsupported(self):
        matrix = self.green_matrix()
        lane = next(
            lane for lane in matrix["lanes"]
            if (lane["core"], lane["language"]) == ("lib", "rust")
        )
        lane["dimensions"]["dev"] = STATUS_UNSUPPORTED
        errors = platform_evidence_errors(
            matrix, "a" * 40, current_tree_clean=True
        )
        self.assertFalse(any("dimension dev" in error for error in errors))

    def test_unknown_dimension_cannot_be_added_outside_the_v12_contract(self):
        matrix = self.green_matrix()
        matrix["lanes"][0]["dimensions"]["shadow-security"] = STATUS_NATIVE
        errors = platform_evidence_errors(
            matrix, "a" * 40, current_tree_clean=True
        )
        self.assertTrue(any("unknown dimensions" in error for error in errors))
