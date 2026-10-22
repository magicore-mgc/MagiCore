"""Focused GitHubWorkflowAnnotationEscaping test owner.

Owner test hẹp cho GitHubWorkflowAnnotationEscaping.
"""

from .support import (
    matrix_module,
    unittest,
)

class GitHubWorkflowAnnotationEscaping(unittest.TestCase):
    def test_annotation_data_cannot_inject_another_workflow_command(self):
        self.assertEqual(
            matrix_module.github_actions_annotation_escape(
                "gate%failure\r\n::warning::injected"
            ),
            "gate%25failure%0D%0A::warning::injected",
        )

    def test_gate_annotations_stay_within_limit_and_count_omitted_failures(self):
        failures = [f"gate failure {index}" for index in range(12)]

        annotations = matrix_module.github_actions_error_annotations(failures)

        self.assertEqual(len(annotations), 10)
        self.assertEqual(annotations[:9], [f"::error::{failure}" for failure in failures[:9]])
        self.assertEqual(
            annotations[-1],
            "::error::3 additional gate failures; full details are in this step log",
        )
