"""Focused MatrixProvenance test owner.

Owner test hẹp cho MatrixProvenance.
"""

from .support import (
    evidence_commit_error,
    unittest,
)

class MatrixProvenance(unittest.TestCase):
    def test_run_sha_mismatch_is_rejected(self):
        self.assertIsNotNone(evidence_commit_error("1" * 40, "2" * 40))

    def test_matching_run_sha_is_accepted(self):
        self.assertIsNone(evidence_commit_error("a" * 40, "a" * 40))

    def test_local_run_without_expected_sha_remains_supported(self):
        self.assertIsNone(evidence_commit_error("a" * 40, None))

    def test_ci_run_without_expected_sha_is_rejected(self):
        self.assertIsNotNone(evidence_commit_error("a" * 40, None, is_ci=True))

    def test_ci_green_record_requires_sha_bound_to_run(self):
        self.assertIsNotNone(
            evidence_commit_error(
                "a" * 40,
                "a" * 40,
                is_ci=True,
                recorded_sha="b" * 40,
                require_recorded=True,
            )
        )
