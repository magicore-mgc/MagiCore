"""JavaScriptProcessInventory tests, kept adjacent to the scanner contract."""

from .support import *

class JavaScriptProcessInventory(unittest.TestCase):
    """JavaScript runtime process APIs are visible to the static gate."""

    def test_detects_child_process_bun_and_deno_apis(self):
        findings = gate.scan_javascript_text(
            "cli/tooling.js",
            "import { spawn as launch } from 'node:child_process';\n"
            "launch('npm', ['install']);\n"
            "Bun.spawn(['cargo', 'fetch']);\n"
            "new Deno.Command('deno', { args: ['install'] });\n",
        )
        self.assertEqual(
            [(item["line"], item["api"]) for item in findings],
            [(2, "child_process.spawn"), (3, "Bun.spawn"), (4, "Deno.Command")],
        )
        self.assertTrue(all(item["review_status"] == "unreviewed" for item in findings))

    def test_detects_commonjs_destructuring_and_inline_require(self):
        findings = gate.scan_javascript_text(
            "scripts/commonjs.cjs",
            "const { execFile: launch } = require('child_process');\n"
            "launch('npm', ['install']);\n"
            "require('child_process').spawn('cargo', ['fetch']);\n",
        )
        self.assertEqual(
            [(item["line"], item["api"]) for item in findings],
            [(2, "child_process.execFile"), (3, "child_process.spawn")],
        )

    def test_detects_esm_namespace_alias(self):
        findings = gate.scan_javascript_text(
            "scripts/namespace.mjs",
            "import * as cp from 'node:child_process';\n"
            "cp.execFileSync('npm', ['install']);\n",
        )
        self.assertEqual(
            [(item["line"], item["api"]) for item in findings],
            [(2, "child_process.execFileSync")],
        )

    def test_detects_computed_optional_and_dynamic_import_aliases(self):
        findings = gate.scan_javascript_text(
            "scripts/indirect-process.js",
            "const cp = require('child_process');\n"
            "const launch = cp['spawn'];\n"
            "launch('npm', ['install']);\n"
            "cp?.['execFileSync']('cargo', ['fetch']);\n"
            "const dynamicCp = await import('node:child_process');\n"
            "dynamicCp.spawnSync('pnpm', ['install']);\n",
        )
        self.assertEqual(
            [(item["line"], item["api"]) for item in findings],
            [
                (3, "child_process.spawn"),
                (4, "child_process.execFileSync"),
                (6, "child_process.spawnSync"),
            ],
        )

    def test_detects_dynamic_import_destructure_and_namespace_reassignment(self):
        findings = gate.scan_javascript_text(
            "scripts/dynamic-process.mjs",
            "const { exec: launch } = await import('node:child_process');\n"
            "launch('npm', ['install']);\n"
            "const cp = await import('node:child_process');\n"
            "const child = cp;\n"
            "child.spawn('cargo', ['fetch']);\n",
        )
        self.assertEqual(
            [(item["line"], item["api"]) for item in findings],
            [(2, "child_process.exec"), (5, "child_process.spawn")],
        )

    def test_computed_process_examples_inside_strings_are_not_findings(self):
        findings = gate.scan_javascript_text(
            "scripts/process-docs.js",
            "// cp['spawn']('npm') is an example\n"
            'const docs = "const run = cp[\'exec\']; run(\\\'npm\\\')";\n',
        )
        self.assertEqual(findings, [])

    def test_ignores_comments_and_string_examples(self):
        findings = gate.scan_javascript_text(
            "cli/example.js",
            "// spawn('npm')\n"
            "const docs = \"Bun.spawn(['npm']) child_process.exec('npm install')\";\n",
        )
        self.assertEqual(findings, [])

__all__ = ['JavaScriptProcessInventory']
