#!/usr/bin/env python3
"""Reject a release graph that can publish without its same-commit lifecycle gate.
Chặn workflow phát hành thiếu dependency hoặc bỏ qua kết quả lifecycle.
"""
from pathlib import Path
import re
import unittest

ROOT = Path(__file__).resolve().parents[2]
GATES = {
    "release-lifecycle-evidence-gate": "lifecycle-matrix.yml",
    "release-source-ci-gate": "ci.yml",
}
PUBLISHERS = ("publish-cli-container", "publish-github-release")


def job_sections(text):
    matches = list(re.finditer(r"^  ([a-zA-Z0-9_-]+):\s*$", text.split("\njobs:\n", 1)[1], re.MULTILINE))
    body = text.split("\njobs:\n", 1)[1]
    sections = {}
    for index, match in enumerate(matches):
        name = match.group(1)
        if name in sections:
            raise ValueError("duplicate job id")
        sections[name] = body[match.end():matches[index + 1].start() if index + 1 < len(matches) else len(body)]
    return sections


def field(section, key):
    match = re.search(r"^    " + re.escape(key) + r":\s*(.+)$", section, re.MULTILINE)
    return match.group(1).strip() if match else ""


def audit(release, workflows):
    errors = []
    sections = job_sections(release)
    for gate, filename in GATES.items():
        workflow = workflows.get(filename, "")
        if not re.search(r"^  workflow_call:\s*$", workflow, re.MULTILINE):
            errors.append(f"{filename} must be reusable")
        if field(sections.get(gate, ""), "uses") != f"./.github/workflows/{filename}":
            errors.append(f"{gate} must call the same-repository workflow")
        if field(sections.get(gate, ""), "continue-on-error") not in ("", "false"):
            errors.append(f"{gate} cannot allow failure")
        for publisher in PUBLISHERS:
            section = sections.get(publisher, "")
            needs = field(section, "needs")
            if not (needs.startswith("[") and needs.endswith("]") and gate in [item.strip() for item in needs[1:-1].split(",")]):
                errors.append(f"{publisher} must depend on {gate}")
            condition = field(section, "if")
            expression = condition.removeprefix("${{").removesuffix("}}").strip()
            clauses = [part.strip() for part in expression.split("&&")]
            if "||" in expression or f"needs.{gate}.result == 'success'" not in clauses:
                errors.append(f"{publisher} must require {gate} success without an OR bypass")
    return errors


class ReleaseLifecycleGateTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.release = (ROOT / ".github/workflows/release.yml").read_text()
        cls.workflows = {name: (ROOT / ".github/workflows" / name).read_text() for name in GATES.values()}

    def test_repository_release_requires_validation_gates(self):
        self.assertEqual(audit(self.release, self.workflows), [])

    def test_removing_any_publisher_dependency_is_rejected(self):
        for publisher in PUBLISHERS:
            for gate in GATES:
                with self.subTest(publisher=publisher, gate=gate):
                    section = job_sections(self.release)[publisher]
                    changed = re.sub(r"^(    needs:.*), " + gate + r"([,\]].*)$", r"\1\2", section, flags=re.MULTILINE)
                    self.assertNotEqual(changed, section, "dependency mutation must change source")
                    self.assertTrue(audit(self.release.replace(section, changed, 1), self.workflows))

    def test_failed_gate_cannot_be_waived_by_condition(self):
        for publisher in PUBLISHERS:
            for gate in GATES:
                with self.subTest(publisher=publisher, gate=gate):
                    section = job_sections(self.release)[publisher]
                    expected = f"needs.{gate}.result == 'success'"
                    self.assertIn(expected, section)
                    for replacement in ["true", expected + " || true"]:
                        changed = section.replace(expected, replacement, 1)
                        self.assertTrue(audit(self.release.replace(section, changed, 1), self.workflows))

    def test_nonreusable_or_wrong_workflow_is_rejected(self):
        for gate, filename in GATES.items():
            with self.subTest(gate=gate):
                altered = dict(self.workflows)
                self.assertIn("  workflow_call:\n", altered[filename])
                altered[filename] = altered[filename].replace("  workflow_call:\n", "", 1)
                self.assertTrue(audit(self.release, altered))
                changed = self.release.replace(f"uses: ./.github/workflows/{filename}", "uses: ./.github/workflows/unrelated.yml", 1)
                self.assertNotEqual(changed, self.release)
                self.assertTrue(audit(changed, self.workflows))


if __name__ == "__main__":
    unittest.main()
