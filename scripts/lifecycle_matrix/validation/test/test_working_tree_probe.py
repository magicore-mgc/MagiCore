"""Focused WorkingTreeCleanProbe test owner.

Owner test hẹp cho WorkingTreeCleanProbe.
"""

from .support import (
    lifecycle_source_matches,
    lifecycle_working_tree_clean,
    patch,
    subprocess,
    unittest,
)

class WorkingTreeCleanProbe(unittest.TestCase):
    @patch("lifecycle_capability_matrix.subprocess.run")
    def test_clean_probe_fails_closed_for_dirty_git_and_command_errors(self, run):
        run.return_value.returncode = 0
        run.return_value.stdout = ""
        self.assertTrue(lifecycle_working_tree_clean())

        run.return_value.stdout = " M cli/src/main.rs\n"
        self.assertFalse(lifecycle_working_tree_clean())

        run.return_value.stdout = ""
        run.return_value.returncode = 128
        self.assertFalse(lifecycle_working_tree_clean())

        run.side_effect = OSError("git unavailable")
        self.assertFalse(lifecycle_working_tree_clean())

    @patch("lifecycle_capability_matrix.subprocess.run")
    def test_matrix_source_must_remain_clean_at_the_same_commit(self, run):
        clean = subprocess.CompletedProcess([], 0, stdout="", stderr="")
        same_head = subprocess.CompletedProcess([], 0, stdout="a" * 40, stderr="")
        other_head = subprocess.CompletedProcess([], 0, stdout="b" * 40, stderr="")
        failed_git = subprocess.CompletedProcess([], 128, stdout="", stderr="fatal")

        run.side_effect = [clean, same_head]
        self.assertTrue(lifecycle_source_matches("a" * 40, True))

        run.side_effect = [
            subprocess.CompletedProcess([], 0, stdout=" M cli/src/main.rs", stderr="")
        ]
        self.assertFalse(lifecycle_source_matches("a" * 40, True))

        run.side_effect = [clean, other_head]
        self.assertFalse(lifecycle_source_matches("a" * 40, True))

        run.side_effect = [clean, failed_git]
        self.assertFalse(lifecycle_source_matches("a" * 40, True))

        run.reset_mock()
        self.assertFalse(lifecycle_source_matches("a" * 40, False))
        run.assert_not_called()
