"""ShellProcessInventory test slice 3; method bodies retained verbatim."""

from .support import *

class ShellProcessInventoryPart3(unittest.TestCase):
    def test_builtin_alias_and_dynamic_trap_actions_are_unreviewed(self):
        findings = gate.scan_shell_text(
            "scripts/aliases.sh",
            'builtin source "$CI_HOOK"\n'
            "alias deps='npm install'\n"
            "trap 'npm install' EXIT\n",
        )
        self.assertEqual(len(findings), 3)
        self.assertTrue(all(item["review_status"] == "unreviewed" for item in findings))
        self.assertTrue(all(item["executable"] == "<dynamic-shell-command>" for item in findings))

        delimiter = gate.scan_shell_text(
            "scripts/aliases.sh", "trap -- 'npm install evil' EXIT\n"
        )
        self.assertEqual(delimiter[0]["executable"], "<dynamic-shell-command>")
        self.assertEqual(delimiter[0]["review_status"], "unreviewed")

    def test_trap_callback_requires_a_multiline_scanned_function_body(self):
        for body in (
            'cleanup() { builtin source "$CI_HOOK"; }; trap cleanup EXIT\n',
            'cleanup() { npm install evil; }; trap cleanup EXIT\n',
        ):
            with self.subTest(body=body):
                findings = gate.scan_shell_text("scripts/harness.sh", body)
                self.assertEqual(len(findings), 1)
                self.assertEqual(findings[0]["executable"], "<dynamic-shell-command>")
                self.assertEqual(findings[0]["review_status"], "unreviewed")

        safe = gate.scan_shell_text(
            "scripts/harness.sh",
            'cleanup() {\n  rm -rf "$TMP_DIR"\n}\ntrap cleanup EXIT\n',
        )
        trap = next(item for item in safe if item["executable"] == "shell-trap-static")
        self.assertEqual(trap["review_status"], "observed")

        dangerous = gate.scan_shell_text(
            "scripts/harness.sh",
            'cleanup() {\n  builtin source "$CI_HOOK"\n}\ntrap cleanup EXIT\n',
        )
        self.assertTrue(any(item["review_status"] == "unreviewed" for item in dangerous))

    def test_shell_script_aliases_require_a_scanned_non_traversing_target(self):
        source = (
            'SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"\n'
            'source "$SCRIPT_DIR/runner.sh"\n'
            '"$SCRIPT_DIR/runner.sh"\n'
        )
        scanned = gate.scan_shell_text(
            "scripts/harness.sh", source, {"scripts/runner.sh"}
        )
        self.assertEqual([item["executable"] for item in scanned], [
            "shell-source", "<dynamic-shell-command>"
        ])
        self.assertEqual(scanned[0]["review_status"], "observed")
        self.assertEqual(scanned[1]["review_status"], "unreviewed")

        direct_only = gate.scan_shell_text(
            "scripts/harness.sh",
            source.splitlines()[0] + '\n"$SCRIPT_DIR/runner.sh"\n',
            {"scripts/runner.sh"},
        )
        self.assertEqual(direct_only[0]["executable"], "shell-script")
        self.assertEqual(direct_only[0]["review_status"], "observed")

        for command in (
            'source "$SCRIPT_DIR/../../../../tmp/evil.sh"\n',
            '"$SCRIPT_DIR/../../../../tmp/evil.sh"\n',
            'source "$SCRIPT_DIR/untracked.sh"\n',
        ):
            with self.subTest(command=command):
                findings = gate.scan_shell_text(
                    "scripts/harness.sh",
                    source.splitlines()[0] + "\n" + command,
                    {"scripts/runner.sh"},
                )
                self.assertEqual(len(findings), 1)
                self.assertEqual(findings[0]["review_status"], "unreviewed")
                self.assertEqual(findings[0]["executable"], "<dynamic-shell-command>")

    def test_env_split_string_and_shell_startup_hooks_are_dynamic(self):
        findings = gate.scan_shell_text(
            "scripts/harness.sh",
            "env -S 'npm install --ignore-scripts'\n"
            "bash -i --rcfile \"$CI_HOOK\" \"$SCRIPT_DIR/runner.sh\"\n",
            {"scripts/runner.sh"},
        )
        self.assertEqual(len(findings), 2)
        self.assertTrue(all(item["review_status"] == "unreviewed" for item in findings))
        self.assertTrue(all(item["executable"] == "<dynamic-shell-command>" for item in findings))

    def test_shell_dependency_review_fingerprint_binds_location_and_guard(self):
        first = gate.scan_shell_text(
            "scripts/reviewed.sh",
            'if [ "$SAFE" = 1 ]; then\n'
            "  npm install example\n"
            "fi\n",
        )[0]["command_fingerprint"]
        moved = gate.scan_shell_text(
            "scripts/reviewed.sh",
            "echo ready\n"
            'if [ "$SAFE" = 1 ]; then\n'
            "  npm install example\n"
            "fi\n",
        )[0]["command_fingerprint"]
        changed_guard = gate.scan_shell_text(
            "scripts/reviewed.sh",
            'if [ "$SAFE" = 0 ]; then\n'
            "  npm install example\n"
            "fi\n",
        )[0]["command_fingerprint"]
        self.assertNotEqual(first, moved, "moving an invocation must invalidate its review")
        self.assertNotEqual(first, changed_guard, "changing the enclosing guard must invalidate its review")

    def test_real_lockfile_fixture_writers_are_exact_and_scriptless(self):
        repo_root = REPO_ROOT
        rel_path = "scripts/maintenance/gen-real-lockfile-fixtures.sh"
        with open(os.path.join(repo_root, rel_path), encoding="utf-8") as handle:
            source = handle.read()
        findings = gate.scan_shell_text(rel_path, source)
        installs = [item for item in findings if item["dependency_action"] == "install"]

        self.assertEqual({item["executable"] for item in installs}, {"bun", "pnpm"})
        self.assertEqual(len(installs), 2)
        self.assertTrue(all(item["review_status"] == "reviewed" for item in installs))
        lines = source.splitlines()
        for item in installs:
            command = lines[item["line"] - 1]
            self.assertIn("--lockfile-only", command)
            self.assertIn("--ignore-scripts", command)
            self.assertIn('HOME="$TMP_ROOT/home"', command)
            self.assertIn("$TMP_ROOT/", command)

    def test_flutter_bootstrap_dynamic_commands_need_the_exact_source_review(self):
        repo_root = REPO_ROOT
        rel_path = "scripts/install/bootstrap_flutter_sdk.sh"
        source_path = os.path.join(repo_root, rel_path)
        with open(source_path, encoding="utf-8") as handle:
            source = handle.read()

        reviewed = gate.scan_shell_text(rel_path, source)
        dynamic_commands = [
            item for item in reviewed if item["executable"] == "<dynamic-shell-command>"
        ]
        self.assertEqual(len(dynamic_commands), 2)
        self.assertTrue(
            all(item["review_status"] == "reviewed" for item in dynamic_commands)
        )

        changed_source = source.replace('"$flutter_bin" --version', '"$flutter_bin" --machine')
        changed = gate.scan_shell_text(rel_path, changed_source)
        changed_dynamic_commands = [
            item for item in changed if item["executable"] == "<dynamic-shell-command>"
        ]
        self.assertEqual(len(changed_dynamic_commands), 2)
        self.assertTrue(
            all(item["review_status"] == "unreviewed" for item in changed_dynamic_commands)
        )

    def test_fixed_mgc_command_alias_is_resolved_but_environment_alias_is_not(self):
        fixed = gate.scan_shell_text(
            "cli/tests/scripts/example.sh",
            'MGC_BIN="${PROJECT_ROOT}/target/debug/mgc"\n'
            '"$MGC_BIN" --version\n',
        )
        self.assertEqual([item["executable"] for item in fixed], ["mgc"])
        self.assertEqual(fixed[0]["review_status"], "observed")

        external = gate.scan_shell_text(
            "cli/tests/scripts/example.sh",
            'MGC_BIN="$UNTRUSTED_TOOL"\n'
            '"$MGC_BIN" --version\n',
        )
        self.assertEqual([item["executable"] for item in external], ["<dynamic-shell-command>"])
        self.assertEqual(external[0]["review_status"], "unreviewed")

    def test_finds_pipeline_and_continued_package_manager_commands(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            "echo prepare | cargo fetch \\\n  --locked\n"
            "npm ci\n",
        )
        self.assertEqual(
            [(item["line"], item["executable"]) for item in findings],
            [(1, "cargo"), (3, "npm")],
        )

    def test_ignores_comments_and_echoed_command_names(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            "# cargo fetch is intentionally not executed\n"
            "echo 'npm install is a displayed example'\n",
        )
        self.assertEqual(findings, [])

    def test_multiline_single_quoted_program_is_not_misread_as_unclosed(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            "awk 'BEGIN { print \"npm install\" }\n"
            "END { print \"cargo fetch\" }'\n",
        )
        self.assertEqual(findings, [])

    def test_malformed_shell_quoting_is_unparsed_not_clean(self):
        findings = gate.scan_shell_text("scripts/broken.sh", "cargo fetch 'unterminated\n")
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["executable"], "<unparsed-shell>")
        self.assertIsNotNone(findings[0]["parse_error"])

    def test_heredoc_data_is_not_scanned_as_outer_shell_commands(self):
        findings = gate.scan_shell_text(
            "scripts/generator.sh",
            "cat <<'EOF' > generated.txt\n"
            "npm install\n"
            "cargo fetch\n"
            "EOF\n",
        )
        self.assertEqual(findings, [])

    def test_expandable_heredoc_command_substitution_is_scanned(self):
        expandable = gate.scan_shell_text(
            "scripts/generator.sh",
            "cat <<EOF > generated.txt\n"
            "$(npm install evil)\n"
            "EOF\n",
        )
        quoted = gate.scan_shell_text(
            "scripts/generator.sh",
            "cat <<'EOF' > generated.txt\n"
            "$(npm install evil)\n"
            "EOF\n",
        )
        self.assertTrue(any(item["executable"] == "npm" for item in expandable))
        self.assertTrue(any(item["review_status"] == "unreviewed" for item in expandable))
        self.assertEqual(quoted, [])

    def test_multiline_heredoc_substitutions_are_scanned_but_quoted_data_is_not(self):
        for substitution in (
            "$(\nnpm install evil\n)",
            "`\nnpm install evil\n`",
        ):
            with self.subTest(substitution=substitution):
                findings = gate.scan_shell_text(
                    "scripts/generator.sh",
                    "cat <<EOF > generated.txt\n"
                    + substitution
                    + "\nEOF\n",
                )
                self.assertTrue(any(item["executable"] == "npm" for item in findings))
                self.assertTrue(any(item["review_status"] == "unreviewed" for item in findings))

        escaped_delimiter = gate.scan_shell_text(
            "scripts/generator.sh",
            "cat <<\\EOF > generated.txt\n"
            "$(npm install evil)\n"
            "EOF\n",
        )
        quoted_backtick = gate.scan_shell_text(
            "scripts/generator.sh",
            "cat <<'EOF' > generated.txt\n"
            "`npm install evil`\n"
            "EOF\n",
        )
        self.assertEqual(escaped_delimiter, [])
        self.assertEqual(quoted_backtick, [])

    def test_here_string_is_not_misread_as_an_unterminated_heredoc(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            'grep -q "pattern" <<<"$output"\n'
            "cargo fetch\n",
        )
        self.assertEqual([item["executable"] for item in findings], ["cargo"])

    def test_dynamic_argv_command_substitution_is_not_silently_dropped(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            'output=$("$@" 2>&1 || true)\n',
        )
        self.assertEqual([item["executable"] for item in findings], ["<dynamic-shell-command>"])

    def test_shell_interpreter_with_heredoc_remains_a_dynamic_boundary(self):
        findings = gate.scan_shell_text(
            "scripts/runner.sh",
            "bash <<'EOF'\n"
            "cargo fetch\n"
            "EOF\n",
        )
        self.assertEqual([item["executable"] for item in findings], ["<dynamic-shell-command>"])
        self.assertEqual(findings[0]["review_status"], "unreviewed")

    def test_extensionless_relative_script_command_is_not_skipped(self):
        findings = gate.scan_shell_text("scripts/runner.sh", "./hook\n")
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["executable"], "<dynamic-shell-command>")
        self.assertEqual(findings[0]["review_status"], "unreviewed")


__all__ = ['ShellProcessInventoryPart3']
