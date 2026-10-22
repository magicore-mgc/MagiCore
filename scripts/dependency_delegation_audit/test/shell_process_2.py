"""ShellProcessInventory test slice 2; method bodies retained verbatim."""

from .support import *

class ShellProcessInventoryPart2(unittest.TestCase):
    def test_known_pm_in_argument_substitution_keeps_its_literal_identity(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            'echo "$(npm install demo)"\n'
            'VERSION="`cargo fetch`"\n',
        )
        self.assertEqual(
            [item["executable"] for item in findings],
            ["npm", "cargo"],
        )
        self.assertTrue(all(item["review_status"] == "unreviewed" for item in findings))

    def test_eval_and_source_are_dynamic_execution_boundaries(self):
        findings = gate.scan_shell_text(
            "scripts/example.sh",
            'eval "$TOOL install"\n'
            'source "$PROJECT_HOOK"\n',
        )
        self.assertEqual(
            [item["executable"] for item in findings],
            ["<dynamic-shell-command>", "<dynamic-shell-command>"],
        )
        self.assertTrue(all(item["review_status"] == "unreviewed" for item in findings))

    def test_shell_interpreter_execution_is_dynamic_even_with_static_script_name(self):
        findings = gate.scan_shell_text("scripts/example.sh", "bash -c 'cargo fetch'\n")
        self.assertEqual([item["executable"] for item in findings], ["<dynamic-shell-command>"])

    def test_shell_interpreter_rejects_variable_and_untracked_script_paths(self):
        variable_path = gate.scan_shell_text(
            "scripts/harness.sh",
            'bash "$CI_HOOK"\n',
        )
        untracked_path = gate.scan_shell_text(
            "scripts/harness.sh",
            "sh /tmp/untracked-hook.sh\n",
        )
        cwd_relative_path = gate.scan_shell_text(
            "scripts/harness.sh",
            "sh runner.sh\n",
            {"scripts/runner.sh"},
        )
        for findings in (variable_path, untracked_path, cwd_relative_path):
            self.assertEqual(len(findings), 1)
            self.assertEqual(findings[0]["review_status"], "unreviewed")
            self.assertEqual(findings[0]["executable"], "<dynamic-shell-command>")

    def test_shell_interpreter_accepts_only_a_scanned_static_script(self):
        source = (
            'SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"\n'
            'bash "$SCRIPT_DIR/runner.sh"\n'
        )
        scanned = gate.scan_shell_text(
            "scripts/harness.sh",
            source.replace(
                'bash "$SCRIPT_DIR/runner.sh"',
                '/usr/bin/env -i PATH="$PATH" bash "$SCRIPT_DIR/runner.sh"',
            ),
            {"scripts/runner.sh": "#!/usr/bin/env bash\n"},
        )
        unscanned = gate.scan_shell_text(
            "scripts/harness.sh",
            source,
            {"scripts/other.sh"},
        )
        self.assertEqual(scanned[0]["executable"], "shell-script")
        self.assertEqual(scanned[0]["review_status"], "observed")
        self.assertEqual(unscanned[0]["executable"], "<dynamic-shell-command>")
        self.assertEqual(unscanned[0]["review_status"], "unreviewed")
        environment_root = gate.scan_shell_text(
            "scripts/harness.sh",
            'SCRIPT_DIR="$UNTRUSTED_ROOT"\n'
            'bash "$SCRIPT_DIR/runner.sh"\n',
            {"scripts/runner.sh"},
        )
        self.assertEqual(environment_root[0]["review_status"], "unreviewed")

    def test_shell_path_alias_comment_cannot_forge_a_repository_anchor(self):
        scanned_paths = {"scripts/runner.sh": "#!/usr/bin/env bash\n"}
        for executable in ('source "$SCRIPT_DIR/runner.sh"', '"$SCRIPT_DIR/runner.sh"'):
            for assignment in (
                "SCRIPT_DIR=/tmp # ${BASH_SOURCE[0]}\n",
                'SCRIPT_DIR="/tmp # ${BASH_SOURCE[0]}"\n',
            ):
                with self.subTest(executable=executable, assignment=assignment):
                    findings = gate.scan_shell_text(
                        "scripts/harness.sh",
                        assignment + executable + "\n",
                        scanned_paths,
                    )
                    self.assertEqual(len(findings), 1)
                    self.assertEqual(findings[0]["review_status"], "unreviewed")
                    self.assertEqual(findings[0]["executable"], "<dynamic-shell-command>")

    def test_bash_startup_environment_must_be_cleared_before_scanned_script(self):
        source = (
            'SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"\n'
            '/usr/bin/env -i PATH="$PATH" bash "$SCRIPT_DIR/runner.sh"\n'
        )
        scanned = gate.scan_shell_text(
            "scripts/harness.sh", source, {"scripts/runner.sh": "#!/bin/bash\n"}
        )
        inherited_hook = gate.scan_shell_text(
            "scripts/harness.sh",
            source.replace("/usr/bin/env -i PATH=\"$PATH\" bash", "bash"),
            {"scripts/runner.sh": "#!/bin/bash\n"},
        )
        command_hook = gate.scan_shell_text(
            "scripts/harness.sh",
            source.replace(
                '/usr/bin/env -i PATH="$PATH" bash "$SCRIPT_DIR/runner.sh"',
                '/usr/bin/env -i BASH_ENV=/tmp/hook.sh PATH="$PATH" bash "$SCRIPT_DIR/runner.sh"',
            ),
            {"scripts/runner.sh": "#!/bin/bash\n"},
        )
        exported_function = gate.scan_shell_text(
            "scripts/harness.sh",
            source.replace(
                '/usr/bin/env -i PATH="$PATH" bash',
                "/usr/bin/env -i 'BASH_FUNC_npm%%=() { :; }' PATH=\"$PATH\" bash",
            ),
            {"scripts/runner.sh": "#!/bin/bash\n"},
        )
        unset_only = gate.scan_shell_text(
            "scripts/harness.sh",
            source.replace("/usr/bin/env -i PATH=\"$PATH\" bash", "env -u BASH_ENV bash"),
            {"scripts/runner.sh": "#!/bin/bash\n"},
        )
        shadowable_env = gate.scan_shell_text(
            "scripts/harness.sh",
            source.replace("/usr/bin/env -i PATH=\"$PATH\" bash", "env -i PATH=\"$PATH\" bash"),
            {"scripts/runner.sh": "#!/bin/bash\n"},
        )
        self.assertEqual(scanned[0]["executable"], "shell-script")
        for findings in (inherited_hook, command_hook, exported_function, unset_only, shadowable_env):
            self.assertEqual(findings[0]["executable"], "<dynamic-shell-command>")
            self.assertEqual(findings[0]["review_status"], "unreviewed")

    def test_direct_bash_script_launch_must_clear_startup_environment(self):
        text = (
            'SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"\n'
            '"$SCRIPT_DIR/runner.sh"\n'
        )
        unguarded = gate.scan_shell_text(
            "scripts/harness.sh", text, {"scripts/runner.sh": "#!/usr/bin/env bash\n"}
        )
        guarded = gate.scan_shell_text(
            "scripts/harness.sh",
            text.replace('"$SCRIPT_DIR/runner.sh"', '/usr/bin/env -i PATH="$PATH" "$SCRIPT_DIR/runner.sh"'),
            {"scripts/runner.sh": "#!/usr/bin/env bash\n"},
        )
        self.assertEqual(unguarded[0]["executable"], "<dynamic-shell-command>")
        self.assertEqual(guarded[0]["executable"], "shell-script")

        absolute_env = gate.scan_shell_text(
            "scripts/harness.sh",
            text.replace(
                '"$SCRIPT_DIR/runner.sh"',
                '/usr/bin/env -i PATH="$PATH" "$SCRIPT_DIR/runner.sh"',
            ),
            {"scripts/runner.sh": "#!/usr/bin/env bash\n"},
        )
        self.assertEqual(absolute_env[0]["executable"], "shell-script")

        misplaced_unset = gate.scan_shell_text(
            "scripts/harness.sh",
            text.replace(
                '"$SCRIPT_DIR/runner.sh"',
                '"$SCRIPT_DIR/runner.sh" env -u BASH_ENV',
            ),
            {"scripts/runner.sh": "#!/usr/bin/env bash\n"},
        )
        self.assertEqual(misplaced_unset[0]["executable"], "<dynamic-shell-command>")
        self.assertEqual(misplaced_unset[0]["review_status"], "unreviewed")

    def test_bash_login_profiles_are_rejected_even_when_bash_env_is_cleared(self):
        source = (
            'SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"\n'
            '/usr/bin/env -i PATH="$PATH" bash -l "$SCRIPT_DIR/runner.sh"\n'
        )
        findings = gate.scan_shell_text(
            "scripts/harness.sh", source, {"scripts/runner.sh": "#!/bin/bash\n"}
        )
        self.assertEqual(findings[0]["executable"], "<dynamic-shell-command>")
        self.assertEqual(findings[0]["review_status"], "unreviewed")

    def test_bash_interactive_profiles_are_rejected_for_interpreters_and_shebangs(self):
        inline = gate.scan_shell_text(
            "scripts/harness.sh",
            'SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"\n'
            '/usr/bin/env -i PATH="$PATH" HOME="$HOME" bash -i "$SCRIPT_DIR/runner.sh"\n',
            {"scripts/runner.sh": "#!/bin/bash\n"},
        )
        shebang = gate.scan_shell_text(
            "scripts/harness.sh",
            'SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"\n'
            '/usr/bin/env -i PATH="$PATH" HOME="$HOME" "$SCRIPT_DIR/runner.sh"\n',
            {"scripts/runner.sh": "#!/bin/bash -i\n"},
        )
        for findings in (inline, shebang):
            self.assertEqual(findings[-1]["executable"], "<dynamic-shell-command>")
            self.assertEqual(findings[-1]["review_status"], "unreviewed")

    def test_source_script_alias_is_invalidated_after_runtime_mutation(self):
        root = 'SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"\n'
        target = '/usr/bin/env -i PATH="$PATH" bash "$SCRIPT_DIR/runner.sh"\n'
        for mutation in (
            "read -r SCRIPT_DIR\n",
            "printf -v SCRIPT_DIR '%s' /tmp\n",
            "unset SCRIPT_DIR\n",
        ):
            with self.subTest(mutation=mutation):
                findings = gate.scan_shell_text(
                    "scripts/harness.sh",
                    root + mutation + target,
                    {"scripts/runner.sh": "#!/bin/bash\n"},
                )
                launch = findings[-1]
                self.assertEqual(launch["executable"], "<dynamic-shell-command>")
                self.assertEqual(launch["review_status"], "unreviewed")

        prompt_read = gate.scan_shell_text(
            "scripts/harness.sh",
            root
            + 'read -p "$(echo continue?)" -n 1 -r\n'
            + target,
            {"scripts/runner.sh": "#!/bin/bash\n"},
        )
        self.assertEqual(prompt_read[-1]["executable"], "shell-script")

        for mutation in (': "${SCRIPT_DIR:=/tmp}"\n', ': "${SCRIPT_DIR=/tmp}"\n'):
            with self.subTest(mutation=mutation):
                findings = gate.scan_shell_text(
                    "scripts/harness.sh",
                    root + mutation + target,
                    {"scripts/runner.sh": "#!/bin/bash\n"},
                )
                self.assertEqual(findings[-1]["executable"], "<dynamic-shell-command>")
                self.assertEqual(findings[-1]["review_status"], "unreviewed")

        for mutation in ("((SCRIPT_DIR=1))\n", "((SCRIPT_DIR++))\n", ": \"$((SCRIPT_DIR=1))\"\n"):
            with self.subTest(mutation=mutation):
                findings = gate.scan_shell_text(
                    "scripts/harness.sh",
                    root + mutation + target,
                    {"scripts/runner.sh": "#!/bin/bash\n"},
                )
                self.assertEqual(findings[-1]["executable"], "<dynamic-shell-command>")
                self.assertEqual(findings[-1]["review_status"], "unreviewed")

        read_only_arithmetic = gate.scan_shell_text(
            "scripts/harness.sh",
            root + "((SCRIPT_DIR == 1))\n" + target,
            {"scripts/runner.sh": "#!/bin/bash\n"},
        )
        self.assertEqual(read_only_arithmetic[-1]["executable"], "shell-script")

    def test_literal_script_heads_inside_command_substitutions_are_unreviewed(self):
        findings = gate.scan_shell_text(
            "scripts/harness.sh",
            'VERSION="$(./untracked-hook.sh)"\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["executable"], "<dynamic-shell-command>")
        self.assertEqual(findings[0]["review_status"], "unreviewed")
        self.assertEqual(
            gate.scan_shell_text("scripts/harness.sh", 'VERSION="$(printf x)"\n'),
            [],
        )

    def test_literal_shell_script_command_heads_fail_closed(self):
        for command in ("./runner.sh", "../scripts/runner.sh", "/tmp/hook.bash"):
            with self.subTest(command=command):
                findings = gate.scan_shell_text(
                    "scripts/harness.sh",
                    command + "\n",
                    {"scripts/runner.sh": "#!/usr/bin/env bash\n"},
                )
                self.assertEqual(len(findings), 1)
                self.assertEqual(findings[0]["executable"], "<dynamic-shell-command>")
                self.assertEqual(findings[0]["review_status"], "unreviewed")


__all__ = ['ShellProcessInventoryPart2']
