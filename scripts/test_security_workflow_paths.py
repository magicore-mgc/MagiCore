#!/usr/bin/env python3
"""Keep security owners covered by both automatic workflow triggers.

Giữ owner bảo mật trong trigger PR và push sau khi di chuyển module.
"""

import fnmatch
import re
import unittest
from pathlib import Path


WORKFLOW = Path(__file__).resolve().parents[1] / '.github/workflows/security.yml'


class SecurityWorkflowPaths(unittest.TestCase):
    def test_refactored_security_owners_trigger_pr_and_push(self):
        workflow = WORKFLOW.read_text(encoding='utf-8')
        for event in ('pull_request', 'push'):
            with self.subTest(event=event):
                block = re.search(
                    rf'^  {event}:\n(.*?)(?=^  [a-z_]+:|^\S|\Z)',
                    workflow,
                    re.MULTILINE | re.DOTALL,
                )
                self.assertIsNotNone(block)
                paths = re.findall(r'^      - "([^"]+)"$', block.group(1), re.MULTILINE)
                for owner in (
                    'adapters/web/src/audit/report/report.rs',
                    'adapters/ai/src/install/verify/verify.rs',
                    'adapters/app/src/adapter/dependency_resolver/dependency_resolver.rs',
                    'adapters/lib/src/install/python_native_install/python_native_install.rs',
                    'core/crates/mgc-exec/src/run/spawn/spawn.rs',
                    'core/crates/mgc-types/src/adapter/package_adapter/package_adapter.rs',
                    'core/crates/mgc-registry-server/src/auth/service/credentials/credentials.rs',
                    'cli/src/commands/publish/git_checks/git_checks.rs',
                    'scripts/dependency_delegation_audit/shell/aliases.py',
                    'scripts/lifecycle_matrix/catalog/operation_owners.py',
                ):
                    self.assertTrue(
                        any(fnmatch.fnmatchcase(owner, pattern) for pattern in paths),
                        f'{event} misses security owner {owner}',
                    )


if __name__ == '__main__':
    unittest.main()
