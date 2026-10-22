"""Focused CommandOutputExcerpt test owner.

Owner test hẹp cho CommandOutputExcerpt.
"""

from .support import (
    matrix_module,
    unittest,
)

class CommandOutputExcerpt(unittest.TestCase):
    def test_preserves_failure_context_and_tail_within_artifact_limit(self):
        output = (
            "Flutter diagnostic header\n"
            + "verbose progress line\n" * 180
            + "Exception: Missing generated plugin registration\n"
            + "stack frame with useful source context\n"
            + "verbose completion line\n" * 180
            + "final tail marker\n"
        )

        excerpt = matrix_module.command_output_excerpt(output)

        self.assertLessEqual(len(excerpt), matrix_module.COMMAND_OUTPUT_EXCERPT_LIMIT)
        self.assertIn("Flutter diagnostic header", excerpt)
        self.assertIn("Exception: Missing generated plugin registration", excerpt)
        self.assertIn("final tail marker", excerpt)
        self.assertIn("...[failure context]...", excerpt)
        self.assertIn("...[tail]...", excerpt)
        self.assertLessEqual(len(matrix_module.command_output_excerpt(output, 20)), 20)

    def test_middle_diagnostics_are_expanded_only_for_failing_ci_lanes(self):
        output = (
            "head marker\n"
            + "routine output\n" * 180
            + "Exception: framework root cause\n"
            + "routine output\n" * 180
            + "tail marker\n"
        )

        flutter = matrix_module.lifecycle_failure_excerpt(
            {"core": "app", "language": "flutter"}, output
        )
        pulumi = matrix_module.lifecycle_failure_excerpt(
            {"core": "clo", "language": "pulumi"}, output
        )
        other = matrix_module.lifecycle_failure_excerpt(
            {"core": "web", "language": "nextjs"}, output
        )

        self.assertIn("Exception: framework root cause", flutter)
        self.assertIn("Exception: framework root cause", pulumi)
        self.assertNotIn("Exception: framework root cause", other)
        self.assertEqual(other, output[-matrix_module.COMMAND_OUTPUT_EXCERPT_LIMIT:])
