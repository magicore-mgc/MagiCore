#!/usr/bin/env python3
"""Regression tests for workflow shell-input interpolation auditing.
Kiểm thử hồi quy cho audit input workflow nội suy vào shell.
"""

from __future__ import annotations

import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path


AUDITOR_PATH = Path(__file__).resolve().parents[1] / "audit" / "audit_workflow_shell_inputs.py"
SPEC = importlib.util.spec_from_file_location("audit_workflow_shell_inputs", AUDITOR_PATH)
assert SPEC and SPEC.loader
AUDITOR = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = AUDITOR
SPEC.loader.exec_module(AUDITOR)


class WorkflowShellInputTests(unittest.TestCase):
    def test_rejects_dispatch_input_in_block_run(self) -> None:
        workflow = (
            "jobs:\n  release:\n    steps:\n      - name: resolve\n"
            "        run: |\n          VERSION=\"${{ inputs.version }}\"\n"
        )
        findings = AUDITOR.audit_text(workflow, Path("release.yml"))
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0].path, Path("release.yml"))
        self.assertEqual(findings[0].line, 6)
        self.assertIn("inputs.version", findings[0].expression)

    def test_rejects_dispatch_input_in_inline_run(self) -> None:
        workflow = 'steps:\n  - run: echo "${{ github.event.inputs.tag }}"\n'
        findings = AUDITOR.audit_text(workflow, "verify.yml")
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0].line, 2)

    def test_rejects_dispatch_input_in_flow_mapping_run(self) -> None:
        workflow = 'steps:\n  - {name: verify, run: echo "${{ inputs.tag }}"}\n'
        findings = AUDITOR.audit_text(workflow, "verify.yml")
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0].line, 2)

    def test_rejects_bracket_property_and_object_input_references(self) -> None:
        workflow = (
            'steps:\n  - run: echo "${{ inputs[\'version\'] }}"\n'
            '  - run: echo "${{ toJSON(inputs) }}"\n'
        )
        findings = AUDITOR.audit_text(workflow, "release.yml")
        self.assertEqual(len(findings), 2)

    def test_rejects_multiline_expression_in_variant_block_scalar(self) -> None:
        workflow = (
            'steps:\n  - run: |+2\n'
            '      echo "${{\n        inputs.tag\n      }}"\n'
        )
        findings = AUDITOR.audit_text(workflow, "verify.yml")
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0].line, 3)

    def test_allows_env_transfer_and_trusted_context_in_run(self) -> None:
        workflow = (
            "steps:\n  - env:\n      RELEASE_TAG: ${{ inputs.tag }}\n"
            "    run: |\n      echo \"$RELEASE_TAG ${{ github.sha }}\"\n"
        )
        self.assertEqual(AUDITOR.audit_text(workflow, "verify.yml"), [])

    def test_audits_nested_workflow_files(self) -> None:
        with tempfile.TemporaryDirectory(prefix="mgc-workflow-input-audit-") as temporary:
            root = Path(temporary)
            (root / "nested").mkdir()
            (root / "safe.yml").write_text(
                "steps:\n  - run: echo safe\n", encoding="utf-8"
            )
            (root / "nested" / "unsafe.yaml").write_text(
                "steps:\n  - run: echo \"${{ inputs.version }}\"\n",
                encoding="utf-8",
            )
            findings = AUDITOR.audit_workflows(root)
            self.assertEqual(len(findings), 1)
            self.assertEqual(findings[0].path, Path("nested/unsafe.yaml"))


if __name__ == "__main__":
    unittest.main()
