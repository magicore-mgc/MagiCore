"""ShellProcessInventory test slice 1; method bodies retained verbatim."""

from .support import *

class ShellProcessInventoryPart1(unittest.TestCase):
    def test_source_bound_dynamic_review_is_invalidated_by_any_file_edit(self):
        path = "scripts/ci/docker/app/build_flutter_container.sh"
        with open(os.path.join(REPO_ROOT, path), encoding="utf-8") as handle:
            source = handle.read()

        findings = gate.scan_shell_text(path, source)
        reviewed = [
            item for item in findings
            if item["line"] == 18 and item["review_status"] == "reviewed"
        ]
        self.assertEqual(len(reviewed), 1)

        changed = gate.scan_shell_text(path, source + "\n# changed\n")
        stale = [item for item in changed if item["line"] == 18]
        self.assertEqual(len(stale), 1)
        self.assertEqual(stale[0]["review_status"], "unreviewed")

    def test_finds_pm_commands_after_env_sudo_and_absolute_path(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            "sudo env PIP_DISABLE_PIP_VERSION_CHECK=1 /usr/local/bin/pip install demo\n",
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["line"], 1)
        self.assertEqual(findings[0]["executable"], "pip")
        self.assertEqual(findings[0]["review_status"], "unreviewed")
        self.assertEqual(findings[0]["dependency_action"], "install")

    def test_finds_pm_command_after_env_unset_option(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            "env -u HOME PATH=/tmp /usr/bin/npm install\n",
        )
        self.assertEqual([item["executable"] for item in findings], ["npm"])
        self.assertEqual(findings[0]["review_status"], "unreviewed")

    def test_command_v_with_dynamic_argument_is_only_a_builtin_probe(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            'command -v "$PM_NAME" >/dev/null 2>&1\n',
        )
        self.assertEqual(findings, [])

    def test_standalone_redirection_path_is_not_an_executable(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            "if ! command -v pnpm &>/dev/null; then\n"
            "  :\n"
            "fi\n",
        )
        self.assertEqual(findings, [])

    def test_finds_pm_command_after_timeout_and_nice_wrappers(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            "timeout 20 nice -n 5 cargo fetch\n",
        )
        self.assertEqual([item["executable"] for item in findings], ["cargo"])
        self.assertEqual(findings[0]["review_status"], "unreviewed")

    def test_command_substitution_is_not_hidden_in_an_assignment(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            'VERSION="$(npm install demo)"\n',
        )
        self.assertEqual([item["executable"] for item in findings], ["npm"])
        self.assertEqual(findings[0]["review_status"], "unreviewed")

    def test_shell_variables_in_arguments_do_not_mark_command_dynamic(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            'echo "$HOME"\n'
            'printf "%s\\n" "$PROJECT_ROOT"\n',
        )
        self.assertEqual(findings, [])

    def test_case_selector_is_not_misread_as_a_command(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            'case "$PM_NAME" in\n'
            "  npm) npm install ;;\n"
            "  mgc) mgc install ;;\n"
            "  *) exit 1 ;;\n"
            "esac\n",
        )
        self.assertEqual([item["executable"] for item in findings], ["npm"])

    def test_shell_condition_or_and_array_arguments_are_not_process_calls(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            'if [[ "${CI:-}" == "true" || "${GITHUB_ACTIONS:-}" == "true" ]]; then\n'
            '  args+=(--target "$TARGET")\n'
            "fi\n",
        )
        self.assertEqual(findings, [])

    def test_arithmetic_or_is_not_split_into_command_heads(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            "if ((INLINE_TEST_COUNT > 0 || MISPLACED_TEST_COUNT > 0)); then\n"
            "  cargo fetch\n"
            "fi\n",
        )
        self.assertEqual([item["executable"] for item in findings], ["cargo"])

    def test_arithmetic_assignment_with_division_is_not_a_command(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            "MINUTES=$(((ELAPSED % 3600) / 60))\n"
            "DURATION=$(( (END - START) / 1000000 ))\n",
        )
        self.assertEqual(findings, [])

    def test_case_alternative_pattern_keeps_branch_executable_visible(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            "case $PM in\n"
            "  npm|pnpm) npm install ;;\n"
            "  *) exit 1 ;;\n"
            "esac\n",
        )
        self.assertEqual([item["executable"] for item in findings], ["npm"])

    def test_fixed_mgc_and_pnpm_binary_aliases_are_resolved(self):
        findings = gate.scan_shell_text(
            "cli/tests/scripts/example.sh",
            'MGC_BIN="mgc"\n'
            'PNPM_BIN="$(command -v pnpm)"\n'
            '"$MGC_BIN" --version\n'
            '"$PNPM_BIN" --version\n',
        )
        self.assertEqual(
            [item["executable"] for item in findings], ["mgc", "pnpm"]
        )
        self.assertTrue(all(item["review_status"] == "observed" for item in findings))

    def test_validated_external_mgc_binary_is_recognized(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            '[[ -f "$MGC_BIN" && -x "$MGC_BIN" ]] || exit 1\n'
            '"$MGC_BIN" sign-release\n',
        )
        self.assertEqual([item["executable"] for item in findings], ["mgc"])
        self.assertEqual(findings[0]["review_status"], "observed")

    def test_dependency_mutation_requires_exact_path_and_command_fingerprint(self):
        repo_root = REPO_ROOT
        rel_path = "benchmark/scripts/run_benchmark.sh"
        abs_path = os.path.join(repo_root, rel_path)
        with open(abs_path, "r", encoding="utf-8") as handle:
            source = handle.read()
        reviewed = [
            item
            for item in gate.scan_shell_text(rel_path, source)
            if item.get("dependency_action") == "install"
        ]
        self.assertTrue(reviewed)
        self.assertTrue(all(item["review_status"] == "reviewed" for item in reviewed))

        changed = gate.scan_shell_text(
            "benchmark/scripts/run_benchmark.sh",
            "npm install --ignore-scripts --force\n",
        )
        self.assertEqual(changed[0]["review_status"], "unreviewed")

        duplicated = gate.scan_shell_text(
            rel_path,
            source + "\nnpm install --ignore-scripts\n",
        )
        appended = [item for item in duplicated if item["line"] > len(source.splitlines())]
        self.assertEqual(len(appended), 1)
        self.assertEqual(appended[0]["review_status"], "unreviewed")

    def test_static_cargo_fetch_in_production_shell_is_blocking(self):
        findings = gate.scan_shell_text("scripts/bench/build.sh", "cargo fetch --locked\n")
        self.assertEqual(findings[0]["executable"], "cargo")
        self.assertEqual(findings[0]["dependency_action"], "fetch")
        self.assertEqual(findings[0]["review_status"], "unreviewed")

    def test_package_manager_exec_and_run_commands_require_exact_review(self):
        commands = (
            "npx cowsay hi",
            "bunx cowsay hi",
            "npm exec --package=cowsay cowsay hi",
            "pnpm dlx cowsay hi",
            "yarn dlx cowsay hi",
            "bun x cowsay hi",
            "uv run task.py",
            "cargo run --bin app",
            "go run ./cmd/app",
        )
        for command in commands:
            with self.subTest(command=command):
                findings = gate.scan_shell_text("scripts/tool.sh", command + "\n")
                self.assertEqual(len(findings), 1)
                self.assertIsNotNone(findings[0]["dependency_action"])
                self.assertEqual(findings[0]["review_status"], "unreviewed")

    def test_alias_writes_invalidate_scanned_script_paths(self):
        root = (
            'SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"\n'
        )
        launch = 'source "$SCRIPT_DIR/known.sh"\n'
        for mutation in (
            "readonly SCRIPT_DIR=/tmp\n",
            "SCRIPT_DIR[0]=/tmp\n",
            "((SCRIPT_DIR=1))\n",
            "rewrite_path() {\n  SCRIPT_DIR=/tmp\n}\n",
        ):
            with self.subTest(mutation=mutation):
                findings = gate.scan_shell_text(
                    "scripts/harness.sh",
                    root + mutation + launch,
                    {"scripts/known.sh"},
                )
                self.assertEqual(findings[-1]["executable"], "<dynamic-shell-command>")
                self.assertEqual(findings[-1]["review_status"], "unreviewed")

    def test_process_substitutions_and_dynamic_substitution_heads_are_unreviewed(self):
        process_substitution = gate.scan_shell_text(
            "scripts/harness.sh", "cat <(npm install evil)\n"
        )
        dynamic_head = gate.scan_shell_text(
            "scripts/harness.sh", 'result="$($CMD npm install evil)"\n'
        )
        self.assertTrue(any(item["executable"] == "npm" for item in process_substitution))
        self.assertTrue(any(item["review_status"] == "unreviewed" for item in process_substitution))
        self.assertTrue(any(item["executable"] == "<dynamic-shell-command>" for item in dynamic_head))
        self.assertTrue(any(item["review_status"] == "unreviewed" for item in dynamic_head))

    def test_shell_like_interpreters_with_inline_commands_are_unreviewed(self):
        for command in (
            "ksh -c 'npm install evil'\n",
            "fish -c 'npm install evil'\n",
            "csh -c 'npm install evil'\n",
            "ash -c 'npm install evil'\n",
            "busybox sh -c 'npm install evil'\n",
            "busybox ash -c 'npm install evil'\n",
        ):
            with self.subTest(command=command):
                findings = gate.scan_shell_text("scripts/harness.sh", command)
                self.assertEqual(len(findings), 1)
                self.assertEqual(findings[0]["executable"], "<dynamic-shell-command>")
                self.assertEqual(findings[0]["review_status"], "unreviewed")

    def test_sudo_chroot_options_do_not_hide_the_command_head(self):
        for command in (
            "sudo -R /tmp npm install evil\n",
            "sudo --chroot /tmp npm install evil\n",
        ):
            with self.subTest(command=command):
                findings = gate.scan_shell_text("scripts/tool.sh", command)
                self.assertEqual(len(findings), 1)
                self.assertEqual(findings[0]["executable"], "npm")
                self.assertEqual(findings[0]["dependency_action"], "install")
                self.assertEqual(findings[0]["review_status"], "unreviewed")

    def test_sudo_prompt_options_do_not_hide_the_command_head(self):
        for command in (
            "sudo -p 'password' npm install evil\n",
            "sudo --prompt 'password' npm install evil\n",
        ):
            with self.subTest(command=command):
                findings = gate.scan_shell_text("scripts/tool.sh", command)
                self.assertEqual(len(findings), 1)
                self.assertEqual(findings[0]["executable"], "npm")
                self.assertEqual(findings[0]["dependency_action"], "install")
                self.assertEqual(findings[0]["review_status"], "unreviewed")

    def test_sudo_timeout_options_do_not_hide_the_command_head(self):
        for command in (
            "sudo -T 5 npm install evil\n",
            "sudo --command-timeout 5 npm install evil\n",
        ):
            with self.subTest(command=command):
                findings = gate.scan_shell_text("scripts/tool.sh", command)
                self.assertEqual(len(findings), 1)
                self.assertEqual(findings[0]["executable"], "npm")
                self.assertEqual(findings[0]["dependency_action"], "install")
                self.assertEqual(findings[0]["review_status"], "unreviewed")

    def test_smoke_test_binary_calls_have_exact_shell_boundary_review(self):
        lines = (
            'if ! version_output=$("$MGC_PATH" --version 2>&1); then\n',
            'if ! help_output=$("$MGC_PATH" --help 2>&1); then\n',
            'if version_cmd=$("$MGC_PATH" version 2>&1); then\n',
        )
        findings = [
            item
            for line in lines
            for item in gate.scan_shell_text("scripts/test/smoke-test.sh", line)
        ]
        self.assertEqual(len(findings), 3)
        self.assertTrue(
            all(
                item["executable"] == "<reviewed-dynamic-command>"
                and item["review_status"] == "reviewed"
                and "caller-selected binary behavior is outside this inventory"
                in item["review_reason"]
                for item in findings
            )
        )
        changed = gate.scan_shell_text(
            "scripts/test/smoke-test.sh",
            'if ! version_output=$("$MGC_PATH" install evil 2>&1); then\n',
        )
        self.assertTrue(any(item["review_status"] == "unreviewed" for item in changed))

    def test_benchmark_install_uses_repository_manifest_not_environment_override(self):
        repo_root = REPO_ROOT
        path = os.path.join(repo_root, "benchmark/scripts/run_benchmark_phased.sh")
        with open(path, "r", encoding="utf-8") as handle:
            source = handle.read()
        self.assertIn('PACKAGE_JSON="$BENCHMARK_ROOT/env/package-unified.json"', source)
        self.assertNotIn('${PACKAGE_JSON:-', source)


__all__ = ['ShellProcessInventoryPart1']
