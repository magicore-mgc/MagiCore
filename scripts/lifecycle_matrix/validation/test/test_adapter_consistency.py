"""Focused AdapterConsistencyGate test owner.

Owner test hẹp cho AdapterConsistencyGate.
"""

from .support import (
    io,
    redirect_stdout,
    unittest,
    validate_adapter_consistency,
)

class AdapterConsistencyGate(unittest.TestCase):
    def test_every_lane_matches_its_real_adapter_or_embedded_native_route(self):
        output = io.StringIO()
        with redirect_stdout(output):
            result = validate_adapter_consistency()

        self.assertEqual(result, 0, output.getvalue())
        self.assertIn("22 lanes", output.getvalue())

    def test_cloud_native_exceptions_are_framework_scoped(self):
        from lifecycle_capability_matrix import CLI_NATIVE_DEPENDENCY_ROUTES

        self.assertIn(("clo", "cdk"), CLI_NATIVE_DEPENDENCY_ROUTES)
        self.assertIn(("clo", "pulumi"), CLI_NATIVE_DEPENDENCY_ROUTES)
        self.assertNotIn(("clo", "terraform"), CLI_NATIVE_DEPENDENCY_ROUTES)
