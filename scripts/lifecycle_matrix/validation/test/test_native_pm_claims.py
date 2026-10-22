"""Focused NativePackageManagerVerdict test owner.

Owner test hẹp cho NativePackageManagerVerdict.
"""

from .support import (
    LANES,
    STATUS_UNVERIFIED,
    evidence,
    lifecycle_lane_owner_contract_errors,
    native_pm_claim_errors,
    native_pm_claim_scope,
    native_pm_lane_errors,
    unittest,
)

class NativePackageManagerVerdict(unittest.TestCase):
    def test_release_gate_scope_tracks_every_declared_native_lane(self):
        expected = {
            (lane["core"], lane["language"], lane.get("framework_id", ""))
            for lane in LANES
            if lane.get("dependency_owner") == "mgc-native"
        }
        self.assertEqual(native_pm_claim_scope(LANES), expected)

    def test_missing_native_evidence_blocks_each_claimed_lane(self):
        lanes = [
            {
                "core": "app",
                "language": "flutter",
                "dependency_owner": "mgc-native",
                "native_pm_verdict": "not-native-pm",
            },
            {
                "core": "app",
                "language": "objc",
                "dependency_owner": "unsupported",
                "native_pm_verdict": "unsupported",
            },
        ]
        failures = native_pm_claim_errors(lanes)
        self.assertEqual(len(failures), 1)
        self.assertIn("app/flutter", failures[0])

    def test_native_pm_claim_with_missing_evidence_shape_fails_closed(self):
        failures = native_pm_lane_errors({
            "core": "web",
            "language": "javascript",
            "dependency_owner": "mgc-native",
            "install_owner": "native-engine",
            "native_pm_verdict": "not-native-pm",
        })
        self.assertTrue(any("malformed native-PM dimensions" in failure for failure in failures))

    def test_matrix_cannot_downgrade_source_native_owner(self):
        source_lane = next(
            lane for lane in LANES
            if lane["core"] == "app" and lane["language"] == "swift"
        )
        matrix_lane = dict(source_lane)
        matrix_lane["dependency_owner"] = "delegated"
        matrix_lane["install_owner"] = "plain-delegation"
        errors = lifecycle_lane_owner_contract_errors(matrix_lane)
        self.assertTrue(any("dependency owner differs from source contract" in error for error in errors))
        self.assertTrue(any("install owner differs from source contract" in error for error in errors))

    def test_matrix_cannot_downgrade_a_source_operation_owner(self):
        source_lane = next(
            lane for lane in LANES
            if lane["core"] == "lib" and lane["language"] == "rust"
        )
        matrix_lane = dict(source_lane)
        matrix_lane["owner_by_operation"] = dict(source_lane["owner_by_operation"])
        matrix_lane["owner_by_operation"]["install"] = "cargo"
        errors = lifecycle_lane_owner_contract_errors(matrix_lane)
        self.assertTrue(any("operation owner differs from source contract" in error for error in errors))

    def test_partial_native_ownership_is_not_mislabeled_as_full_native_pm(self):
        source_lane = next(
            lane for lane in LANES
            if lane["core"] == "lib" and lane["language"] == "rust"
        )
        dimensions, _ = evidence()
        dimensions["list"] = STATUS_UNVERIFIED
        self.assertEqual(
            native_pm_lane_errors({
                "core": "lib",
                "language": "rust",
                "verdict": "orchestration-lifecycle-passed",
                "dependency_owner": "mgc-native",
                "install_owner": "native-engine",
                "native_pm_verdict": "not-native-pm",
                "native_pm_delegated": [],
                "dimensions": dimensions,
                "owner_by_operation": dict(source_lane["owner_by_operation"]),
            }),
            [],
        )

    def test_native_claim_errors_recheck_runtime_evidence(self):
        dimensions, operation_owners = evidence()
        operation_owners["fetch"] = "cargo"
        failures = native_pm_claim_errors([{
            "core": "lib",
            "language": "rust",
            "verdict": "orchestration-lifecycle-passed",
            "dependency_owner": "mgc-native",
            "install_owner": "native-engine",
            "native_pm_verdict": "native-pm-supported",
            "native_pm_delegated": [],
            "dimensions": dimensions,
            "owner_by_operation": operation_owners,
        }])
        self.assertTrue(
            any(
                "without complete package-manager evidence" in failure
                for failure in failures
            )
        )

    def test_native_pm_lane_gate_rechecks_dimensions_and_operation_owners(self):
        dimensions, operation_owners = evidence()
        lane = {
            "core": "lib",
            "language": "rust",
            "verdict": "orchestration-lifecycle-passed",
            "dependency_owner": "mgc-native",
            "install_owner": "native-engine",
            "native_pm_verdict": "native-pm-supported",
            "native_pm_delegated": [],
            "dimensions": dimensions,
            "owner_by_operation": operation_owners,
            "release_scope_out_reason": "Lifecycle qualification is evidence-only.",
        }
        self.assertEqual(native_pm_lane_errors(lane), [])
        operation_owners["fetch"] = "cargo"
        failures = native_pm_lane_errors(lane)
        self.assertTrue(
            any(
                "without complete package-manager evidence" in failure
                for failure in failures
            )
        )

    def test_explicit_non_native_owners_do_not_get_misclassified_as_native_claims(self):
        for dependency_owner, install_owner, verdict in (
            ("delegated", "plain-delegation", "compatibility-passed"),
            ("unsupported", "unsupported", "unsupported"),
            ("scaffold-only", "unsupported", "unsupported"),
        ):
            with self.subTest(dependency_owner=dependency_owner):
                self.assertEqual(
                    native_pm_lane_errors({
                        "core": "app",
                        "language": "swift",
                        "verdict": "orchestration-lifecycle-passed",
                        "dependency_owner": dependency_owner,
                        "install_owner": install_owner,
                        "native_pm_verdict": verdict,
                        "release_scope_out_reason": (
                            "Current release contract excludes this unsupported lane."
                            if dependency_owner in {"unsupported", "scaffold-only"}
                            else None
                        ),
                    }),
                    [],
                )

    def test_unsupported_owner_without_scopeout_reason_fails_closed(self):
        failures = native_pm_lane_errors({
            "core": "app",
            "language": "objc",
            "dependency_owner": "unsupported",
            "install_owner": "unsupported",
            "native_pm_verdict": "unsupported",
        })
        self.assertTrue(any("missing an explicit release scope-out reason" in f for f in failures))

    def test_scopeout_with_unknown_lifecycle_verdict_fails_closed(self):
        failures = native_pm_lane_errors({
            "core": "app",
            "language": "objc",
            "verdict": "made-up-verdict",
            "dependency_owner": "unsupported",
            "install_owner": "unsupported",
            "native_pm_verdict": "unsupported",
            "release_scope_out_reason": "Explicitly outside this release scope.",
        })
        self.assertTrue(any("invalid lifecycle verdict" in failure for failure in failures))

    def test_unknown_dependency_owner_fails_closed(self):
        failures = native_pm_lane_errors({
            "core": "app",
            "language": "swift",
            "dependency_owner": "maybe-native",
            "install_owner": "native-engine",
            "native_pm_verdict": "native-pm-supported",
        })
        self.assertTrue(any("invalid dependency_owner" in failure for failure in failures))
