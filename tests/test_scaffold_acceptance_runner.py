#!/usr/bin/env python3
"""Test runner verdicts using a controlled binary, not MagiCore capability.
Kiểm cách runner chốt verdict; fixture không phải evidence product đã pass.
"""
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "cli/tests/scripts/acceptance_phase2.sh"
FIXTURE = r'''#!/usr/bin/env bash
set -eu
if [[ "$1" == --version ]]; then echo fixture; exit 0; fi
if [[ "$2" == nextjs@laster ]]; then echo 'Did you mean latest?' >&2; exit 2; fi
if [[ "${MGC_FIXTURE_MODE:-}" == failed_create ]]; then echo 'Created project'; exit 3; fi
project="$3"
core="${1#create-}"
case "$core" in
web) manifest=package.json; source=index.html;;
ai) manifest=pyproject.toml; source=src/agent.py;;
app) manifest=pubspec.yaml; source=lib/main.dart;;
lib) manifest=Cargo.toml; source=src/lib.rs;;
*) exit 4;;
esac
mkdir -p "$project/$(dirname "$source")"
printf '%s' "$core" > "$project/.mgc.core"
printf config > "$project/mgc.toml"
if [[ "${MGC_FIXTURE_MODE:-}" != missing_manifest ]]; then printf manifest > "$project/$manifest"; fi
if [[ "${MGC_FIXTURE_MODE:-}" != missing_source ]]; then printf source > "$project/$source"; fi
'''


class ScaffoldAcceptanceRunnerTests(unittest.TestCase):
    def run_fixture(self, mode):
        with tempfile.TemporaryDirectory(prefix="mgc-runner-contract-") as directory:
            binary = Path(directory) / "mgc-fixture"
            binary.write_text(FIXTURE)
            binary.chmod(0o700)
            environment = dict(os.environ, MGC_BIN=str(binary), MGC_FIXTURE_MODE=mode)
            for key in ["BASH_ENV", "ENV", "SHELLOPTS", "BASHOPTS"]:
                environment.pop(key, None)
            return subprocess.run(["bash", str(SCRIPT)], env=environment, capture_output=True, text=True, timeout=20)

    def test_runner_counts_only_executed_checks(self):
        result = self.run_fixture("")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("12 executed scaffold checks", result.stdout)
        self.assertIn("lifecycle was not evaluated", result.stdout)

    def test_missing_outputs_never_pass_from_identity_alone(self):
        for mode in ["missing_manifest", "missing_source"]:
            with self.subTest(mode=mode):
                result = self.run_fixture(mode)
                self.assertNotEqual(result.returncode, 0)
                self.assertNotIn("12 executed scaffold checks", result.stdout)

    def test_success_text_does_not_override_failed_process(self):
        result = self.run_fixture("failed_create")
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn("12 executed scaffold checks", result.stdout)


if __name__ == "__main__":
    unittest.main()
