#!/usr/bin/env python3
"""Exercise scripts/release/release.sh argument forwarding without building or publishing."""

import json
import os
import subprocess
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
RELEASE_SCRIPT = ROOT / "scripts" / "release/release.sh"


class ReleaseScriptTests(unittest.TestCase):
    def run_release(self, arguments):
        with tempfile.TemporaryDirectory(prefix="mgc-release-script-test-") as tmp:
            temp_dir = Path(tmp)
            fake_bin = temp_dir / "bin"
            fake_bin.mkdir()
            capture_path = temp_dir / "cargo-args.json"
            fake_cargo = fake_bin / "cargo"
            fake_cargo.write_text(
                "#!/usr/bin/env python3\n"
                "import json, os, sys\n"
                "with open(os.environ['MGC_CAPTURE_ARGS'], 'w', encoding='utf-8') as out:\n"
                "    json.dump(sys.argv[1:], out)\n",
                encoding="utf-8",
            )
            fake_cargo.chmod(0o755)
            env = os.environ.copy()
            env["PATH"] = f"{fake_bin}{os.pathsep}{env['PATH']}"
            env["MGC_CAPTURE_ARGS"] = str(capture_path)
            result = subprocess.run(
                ["bash", str(RELEASE_SCRIPT), *arguments],
                cwd=ROOT,
                env=env,
                capture_output=True,
                text=True,
                check=False,
            )
            captured_args = (
                json.loads(capture_path.read_text(encoding="utf-8"))
                if capture_path.exists()
                else None
            )
            return result, captured_args

    def test_default_build_does_not_expand_an_empty_target_array(self):
        result, captured_args = self.run_release([])
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(
            captured_args,
            ["run", "-p", "mgc-dist", "--", "build-all", "--profile", "release"],
        )

    def test_profile_and_target_are_forwarded(self):
        result, captured_args = self.run_release(
            ["--profile", "release", "--target", "aarch64-apple-darwin"]
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(
            captured_args,
            [
                "run",
                "-p",
                "mgc-dist",
                "--",
                "build-all",
                "--profile",
                "release",
                "--target",
                "aarch64-apple-darwin",
            ],
        )

    def test_missing_or_duplicate_target_fails_before_cargo(self):
        for arguments in (["--target"], ["--target", "", "--profile", "release"]):
            with self.subTest(arguments=arguments):
                result, captured_args = self.run_release(arguments)
                self.assertEqual(result.returncode, 2)
                self.assertIn("--target requires a non-empty value", result.stderr)
                self.assertIsNone(captured_args)

        result, captured_args = self.run_release(
            ["--target", "aarch64-apple-darwin", "--target", "x86_64-unknown-linux-gnu"]
        )
        self.assertEqual(result.returncode, 2)
        self.assertIn("--target may only be provided once", result.stderr)
        self.assertIsNone(captured_args)


if __name__ == "__main__":
    unittest.main()
