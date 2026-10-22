"""New-tool detection test slice 2; test method bodies retained verbatim."""

from .support import *

class NewToolDetectionPart2(unittest.TestCase):
    def test_dynamic_mgc_run_is_a_blocker(self):
        findings = scan_snippet(
            "adapters/cloud/src/deploy/mod.rs",
            'fn deploy_with_tool(tool: &str) -> Result<()> {\n'
            '    mgc_run(tool, &args, &opts)?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["tool"], "<dynamic:tool>")
        self.assertEqual(findings[0]["status"], "violation")

    def test_dynamic_executable_in_build_is_a_blocker(self):
        findings = scan_snippet(
            "cli/src/commands/build/build.rs",
            'fn build_rust_with_env(program: &str) -> Result<()> {\n'
            '    mgc_exec::prelude::run_inherited(program, &args, &opts)?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["tool"], "<dynamic:program>")
        self.assertEqual(findings[0]["op_class"], "build")
        self.assertEqual(findings[0]["status"], "violation")
        self.assertEqual(findings[0]["evidence_kind"], "dynamic_spawn_call")

    def test_dynamic_command_new_in_unclassified_production_path_is_blocker(self):
        findings = scan_snippet(
            "cli/src/commands/hooks.rs",
            'fn run_hook(program: &str) -> Result<()> {\n'
            '    std::process::Command::new(program).status()?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["tool"], "<dynamic:program>")
        self.assertEqual(findings[0]["status"], "violation")

    def test_mgc_run_dispatcher_is_not_misidentified_as_process_spawn(self):
        findings = scan_snippet(
            "cli/src/commands/core/dev/cicd.rs",
            'async fn run_test_step() -> Result<()> {\n'
            '    crate::commands::run::run("test".to_string(), args, core, None).await?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(findings, [])

    def test_other_cli_run_functions_are_not_process_spawn_patterns(self):
        findings = scan_snippet(
            "cli/src/commands/add.rs",
            'async fn dispatch_add(package: String) -> Result<()> {\n'
            '    crate::commands::core::add::run(package, args).await?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(findings, [])

    def test_dynamic_mgc_exec_run_is_a_blocker(self):
        findings = scan_snippet(
            "cli/src/commands/build/build.rs",
            'fn build_with_selected_tool(program: &str) -> Result<()> {\n'
            '    mgc_exec::prelude::run(program, &args, &opts)?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["tool"], "<dynamic:program>")
        self.assertEqual(findings[0]["op_class"], "build")
        self.assertEqual(findings[0]["status"], "violation")

    def test_mgc_exec_run_inherited_literal_is_detected(self):
        findings = scan_snippet(
            "cli/src/commands/build/build.rs",
            'fn build_runtime() -> Result<()> {\n'
            '    mgc_exec::prelude::run_inherited("node", &args, &opts)?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["tool"], "node")
        self.assertEqual(findings[0]["evidence_kind"], "spawn_or_wrapper_call")
        self.assertEqual(findings[0]["op_class"], "build")
        self.assertEqual(findings[0]["status"], "violation")

    def test_production_test_command_cannot_delegate_to_package_manager(self):
        findings = scan_snippet(
            "cli/src/commands/core/dev/cicd.rs",
            'fn run_test_step() -> Result<()> {\n'
            '    std::process::Command::new("cargo").arg("test").status()?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["op_class"], "test")
        self.assertEqual(findings[0]["status"], "violation")

    def test_windows_path_lookup_process_spawn_is_not_exempt(self):
        findings = scan_snippet(
            "core/crates/mgc-exec/src/run/run.rs",
            'fn resolve_windows_shim(cmd: &str) {\n'
            '    Command::new("where.exe").arg(cmd).output();\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["status"], "violation")
        self.assertTrue(findings[0]["blocking"])

    def test_process_tree_os_tools_are_reviewed_but_still_blocking(self):
        cases = (
            ("terminate_process_tree", "taskkill", "Windows process-tree termination"),
            ("find_forbidden_descendant", "ps", "Unix process-table inspection"),
        )
        for function, executable, reason in cases:
            with self.subTest(function=function):
                findings = scan_snippet(
                    "core/crates/mgc-exec/src/run/run.rs",
                    f'fn {function}() {{\n'
                    f'    Command::new("{executable}").status();\n'
                    '}\n',
                )
                self.assertEqual(len(findings), 1)
                self.assertEqual(findings[0]["op_class"], "platform-process-boundary")
                self.assertEqual(findings[0]["status"], "review-required")
                self.assertTrue(findings[0]["blocking"])
                self.assertIn(reason, findings[0]["classification_reason"])

    def test_os_tool_allowlist_does_not_allow_package_manager_in_same_helper(self):
        findings = scan_snippet(
            "core/crates/mgc-exec/src/run/run.rs",
            'fn terminate_process_tree() {\n'
            '    Command::new("taskkill").status();\n'
            '    Command::new("cargo").arg("fetch").status();\n'
            '}\n',
        )
        self.assertEqual(len(findings), 2)
        by_tool = {finding["tool"]: finding for finding in findings}
        self.assertEqual(by_tool["taskkill"]["status"], "review-required")
        self.assertEqual(by_tool["cargo"]["status"], "violation")
        self.assertTrue(all(finding["blocking"] for finding in findings))

    def test_test_prefix_directory_does_not_waive_production_spawn(self):
        findings = scan_snippet(
            "cli/src/commands/core/testing/build.rs",
            'fn build_app() -> Result<()> {\n'
            '    std::process::Command::new("gradle").status()?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["status"], "violation")

    def test_production_spawn_in_exact_test_named_source_directory_is_not_exempt(self):
        findings = scan_snippet(
            "cli/src/commands/test/runtime.rs",
            'pub fn install_package() -> Result<()> {\n'
            '    std::process::Command::new("cargo").arg("fetch").status()?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["op_class"], "install")
        self.assertEqual(findings[0]["status"], "violation")

    def test_source_subdirectory_named_tests_is_not_a_harness_exemption(self):
        findings = scan_snippet(
            "core/crates/mgc-store/src/tests/runtime.rs",
            'pub fn install_package() -> Result<()> {\n'
            '    std::process::Command::new("cargo").arg("fetch").status()?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["op_class"], "install")
        self.assertEqual(findings[0]["status"], "violation")

    def test_external_integration_harness_remains_exempt(self):
        findings = scan_snippet(
            "core/crates/mgc-store/tests/runtime.rs",
            'fn fixture_installs_dependency() -> Result<()> {\n'
            '    std::process::Command::new("cargo").arg("fetch").status()?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["op_class"], "tests")
        self.assertEqual(findings[0]["status"], "allowed")

    def test_test_attribute_is_a_narrow_exemption_inside_source_tree(self):
        findings = scan_snippet(
            "cli/src/commands/test/runtime_test.rs",
            '#[tokio::test]\n'
            'async fn test_external_fixture_tool() -> Result<()> {\n'
            '    std::process::Command::new("cargo").arg("fetch").status()?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["op_class"], "test-fixture")
        self.assertEqual(findings[0]["status"], "allowed")

    def test_dev_path_does_not_waive_dependency_named_function(self):
        findings = scan_snippet(
            "cli/src/commands/core/dev/app.rs",
            'fn install_dependencies() -> Result<()> {\n'
            '    let status = std::process::Command::new("gradle")\n'
            '        .args(["dependencies"])\n'
            '        .status()?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["op_class"], "install")
        self.assertEqual(
            findings[0]["status"],
            "violation",
            "a dev path must not turn a dependency lifecycle spawn into an allowed runtime call",
        )

    def test_dev_path_does_not_waive_unclassified_spawn(self):
        findings = scan_snippet(
            "cli/src/commands/core/dev/app.rs",
            'fn prepare_project() -> Result<()> {\n'
            '    std::process::Command::new("gradle").status()?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["op_class"], "unclassified")
        self.assertEqual(findings[0]["status"], "violation")

__all__ = ['NewToolDetectionPart2']
