"""Focused NativePackageManagerVerdict test owner.

Owner test hẹp cho NativePackageManagerVerdict.
"""

from .support import (
    ALL_DIMENSIONS,
    LANES,
    NATIVE_PM_REQUIRED_DIMENSIONS,
    NATIVE_PM_USER_OPERATIONS,
    STATUS_FAILED,
    STATUS_UNSUPPORTED,
    evidence,
    io,
    lifecycle_status_owner_matches,
    native_pm_supported,
    native_pm_unavailable_verdict,
    print_dependency_owner_summary,
    redirect_stdout,
    unittest,
)

class NativePackageManagerVerdict(unittest.TestCase):
    def test_native_verdict_requires_every_user_dependency_operation(self):
        expected = {
            "install", "add", "remove", "update", "list",
            "frozen-install", "offline-reinstall", "gc",
        }
        self.assertEqual(set(NATIVE_PM_USER_OPERATIONS), expected)
        self.assertTrue(expected.issubset(set(NATIVE_PM_REQUIRED_DIMENSIONS)))
        for operation in expected:
            with self.subTest(operation=operation):
                dimensions, owners = evidence({operation: STATUS_UNSUPPORTED})
                self.assertFalse(
                    native_pm_supported("mgc-native", "native-engine", [], dimensions, owners)
                )

    def test_missing_owner_for_a_native_user_operation_blocks_claim(self):
        dimensions, owners = evidence()
        del owners["list"]
        self.assertFalse(
            native_pm_supported("mgc-native", "native-engine", [], dimensions, owners)
        )

    def test_owner_summary_never_presents_declared_native_owner_as_a_verdict(self):
        output = io.StringIO()
        with redirect_stdout(output):
            print_dependency_owner_summary()
        self.assertNotIn("native-pm-supported (ceiling)", output.getvalue())
        self.assertIn("native evidence required; not a verdict", output.getvalue())
        self.assertIn(
            '"native_pm_verdict": "not-evaluated-without-lifecycle-evidence"',
            output.getvalue(),
        )

    def test_integrity_verification_is_a_first_class_matrix_dimension(self):
        self.assertIn("verify", ALL_DIMENSIONS)
        self.assertTrue(set(NATIVE_PM_USER_OPERATIONS).issubset(ALL_DIMENSIONS))
        self.assertEqual(len(ALL_DIMENSIONS), 23)
        for lane in LANES:
            with self.subTest(core=lane["core"], language=lane["language"]):
                self.assertTrue(lane.get("owner_by_operation", {}).get("verify"))

    def test_lifecycle_only_pass_does_not_claim_native_package_manager(self):
        dimensions, owners = evidence(
            {"store": STATUS_UNSUPPORTED, "offline-reinstall": STATUS_UNSUPPORTED}
        )
        self.assertFalse(native_pm_supported("mgc-native", "native-engine", [], dimensions, owners))

    def test_missing_dependency_dimension_does_not_claim_native(self):
        dimensions, owners = evidence()
        del dimensions["fetch"]
        self.assertFalse(native_pm_supported("mgc-native", "native-engine", [], dimensions, owners))

    def test_missing_integrity_verification_evidence_does_not_claim_native(self):
        dimensions, owners = evidence({"verify": STATUS_UNSUPPORTED})
        self.assertFalse(native_pm_supported("mgc-native", "native-engine", [], dimensions, owners))

    def test_delegated_integrity_verification_does_not_claim_native(self):
        dimensions, owners = evidence(owners={"verify": "cargo"})
        self.assertFalse(native_pm_supported("mgc-native", "native-engine", [], dimensions, owners))

    def test_delegated_operation_does_not_claim_native(self):
        dimensions, owners = evidence(owners={"fetch": "cargo"})
        self.assertFalse(native_pm_supported("mgc-native", "native-engine", [], dimensions, owners))

    def test_any_delegated_package_manager_blocks_native_claim(self):
        dimensions, owners = evidence()
        self.assertFalse(native_pm_supported("mgc-native", "native-engine", ["cargo"], dimensions, owners))

    def test_complete_native_dependency_evidence_passes(self):
        dimensions, owners = evidence()
        self.assertTrue(native_pm_supported("mgc-native", "native-engine", [], dimensions, owners))

    def test_missing_toolchain_preserves_explicit_unsupported_native_pm_verdict(self):
        self.assertEqual(native_pm_unavailable_verdict("unsupported"), "unsupported")
        self.assertEqual(native_pm_unavailable_verdict("scaffold-only"), "unsupported")
        self.assertEqual(native_pm_unavailable_verdict("mgc-native"), "not-native-pm")
        self.assertEqual(native_pm_unavailable_verdict("delegated"), "not-native-pm")

    def test_failed_probe_is_not_rewritten_as_an_owner_status_mismatch(self):
        self.assertTrue(lifecycle_status_owner_matches(STATUS_FAILED, "plain-delegation"))
