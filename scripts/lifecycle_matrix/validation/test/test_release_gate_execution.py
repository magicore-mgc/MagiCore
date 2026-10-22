"""Exercise extracted release-gate owners with positive and negative evidence.

Kiểm tra gate đã tách module bằng evidence đầy đủ và các mutation phải bị chặn.
"""

import copy
import unittest

from .support import PlatformGreenMatrixFixture
from ..release_gate.context import prepare_gate
from ..release_gate.matrix import validate_lanes
from ..release_gate.scope import validate_scope


class ReleaseGateExecution(PlatformGreenMatrixFixture, unittest.TestCase):
    def validate(self, matrix):
        state = prepare_gate(matrix)
        validate_lanes(state)
        validate_scope(state)
        return state.failures

    def test_complete_source_consistent_evidence_passes(self):
        self.assertEqual(self.validate(self.green_matrix()), [])

    def test_missing_or_laundered_evidence_fails_closed(self):
        base = self.green_matrix()
        for mutation in ('schema', 'missing_lane', 'missing_dimension',
                         'unknown_status', 'owner_mismatch', 'framework_catalog'):
            with self.subTest(mutation=mutation):
                matrix = copy.deepcopy(base)
                lane = next(row for row in matrix['lanes'] if not row['evidence_only'])
                dimension = lane['required_dimensions'][0]
                if mutation == 'schema':
                    matrix['schema_version'] = -1
                elif mutation == 'missing_lane':
                    matrix['lanes'].remove(lane)
                elif mutation == 'missing_dimension':
                    del lane['dimensions'][dimension]
                elif mutation == 'unknown_status':
                    lane['dimensions'][dimension] = 'pretend-pass'
                elif mutation == 'owner_mismatch':
                    lane['lifecycle_owners'][dimension] = 'unverified'
                elif mutation == 'framework_catalog':
                    matrix['framework_catalog'] = []
                self.assertTrue(self.validate(matrix), mutation)
