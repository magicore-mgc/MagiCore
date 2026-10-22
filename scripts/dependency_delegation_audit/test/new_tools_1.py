"""New-tool detection test slice 1; test method bodies retained verbatim."""

from .support import *

class NewToolDetectionPart1(unittest.TestCase):
    def test_cfg_test_only_helper_is_a_test_fixture_without_test_attribute(self):
        findings = scan_snippet(
            "cli/src/commands/audit/signatures.rs",
            "#[cfg(test)]\n"
            "fn verify_bundle() {\n"
            '    "npm",\n'
            "}\n",
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["op_class"], "test-fixture")
        self.assertEqual(findings[0]["status"], "allowed")

    def test_web_layout_does_not_spawn_a_shell_for_windows_junctions(self):
        repo_root = REPO_ROOT
        rel_path = "adapters/web/src/layout.rs"
        abs_path = os.path.join(repo_root, rel_path)
        findings = gate.scan_file(rel_path, abs_path)
        shell_spawns = [item for item in findings if item["tool"] == "cmd"]
        self.assertEqual(
            shell_spawns,
            [],
            "project paths must never be interpolated into cmd.exe command text",
        )

    def test_python3_quantize_spawn_is_violation_without_marker(self):
        findings = scan_snippet(
            "cli/src/commands/model/mod.rs",
            'fn quantize(path: &str) -> Result<()> {\n'
            '    let status = std::process::Command::new("python3")\n'
            '        .args(["-m", "x"])\n'
            '        .status()?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["tool"], "python3")
        self.assertEqual(findings[0]["status"], "violation")
        self.assertEqual(findings[0]["op_class"], "quantize")

    def test_pip_audit_spawn_is_violation_without_marker(self):
        findings = scan_snippet(
            "core/crates/mgc-audit/src/scanners/mod.rs",
            'pub async fn audit_python(project_root: &Path) -> MgResult<AuditReport> {\n'
            '    let result = mgc_exec::run::run("pip-audit", &args, &exec_opts)?;\n'
            '    Ok(result)\n'
            '}\n',
        )
        tools = [f["tool"] for f in findings]
        self.assertIn("pip-audit", tools)
        for finding in findings:
            if finding["tool"] == "pip-audit":
                self.assertEqual(finding["status"], "violation")

    def test_cargo_audit_which_probe_is_found(self):
        findings = scan_snippet(
            "core/crates/mgc-audit/src/scanners/mod.rs",
            'pub async fn audit_rust(project_root: &Path) -> MgResult<AuditReport> {\n'
            '    if which::which("cargo-audit").is_err() {\n'
            '        return Ok(AuditReport::tool_missing("cargo-audit"));\n'
            '    }\n'
            '    Ok(AuditReport::clean(0))\n'
            '}\n',
        )
        self.assertTrue(
            any(f["tool"] == "cargo-audit" for f in findings),
            "which-probe of cargo-audit must be measured",
        )

    def test_govulncheck_spawn_is_found(self):
        findings = scan_snippet(
            "core/crates/mgc-audit/src/scanners/govulncheck.rs",
            'pub async fn audit_go(project_root: &Path) -> MgResult<AuditReport> {\n'
            '    let result = mgc_exec::run::run("govulncheck", &args, &exec_opts)?;\n'
            '    Ok(result)\n'
            '}\n',
        )
        self.assertTrue(
            any(f["tool"] == "govulncheck" for f in findings),
            "govulncheck spawn must be measured",
        )

    def test_composer_and_pub_package_manager_spawns_are_found(self):
        findings = scan_snippet(
            "adapters/lib/src/install/mod.rs",
            'pub async fn run_install() -> Result<()> {\n'
            '    mgc_exec::run::run("composer", &["install".to_string()], &opts)?;\n'
            '    mgc_exec::run::run("pub", &["add".to_string()], &opts)?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual({finding["tool"] for finding in findings}, {"composer", "pub"})
        self.assertTrue(all(finding["status"] == "violation" for finding in findings))

    def test_git_fetch_spawn_is_dependency_delegation(self):
        findings = scan_snippet(
            "core/crates/mgc-resolver/src/protocols/swift.rs",
            'async fn resolve_git() -> Result<()> {\n'
            '    let report = mgc_exec::run::run("git", &args, &opts)?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["tool"], "git")
        self.assertEqual(findings[0]["status"], "violation")

    def test_unreadable_rust_source_is_reported_not_silently_skipped(self):
        findings = gate.scan_file(
            "core/crates/mgc-exec/src/missing.rs",
            "/definitely/not/a/real/magicore-source.rs",
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["evidence_kind"], "scan_error")
        self.assertEqual(findings[0]["status"], "violation")

    def test_dynamic_native_install_wrapper_is_forbidden(self):
        findings = scan_snippet(
            "cli/src/commands/core/web/web.rs",
            'fn install_member(root: &Path) -> Result<()> {\n'
            '    run_native_install(root, program, &args)\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["tool"], "run_native_install")
        self.assertEqual(findings[0]["op_class"], "install")
        self.assertEqual(findings[0]["status"], "violation")

    def test_tool_selector_return_values_are_not_process_spawns(self):
        findings = scan_snippet(
            "cli/src/commands/build/build.rs",
            'fn python_cmd() -> &\'static str {\n'
            '    if !tool_unavailable("python") {\n'
            '        "python"\n'
            '    } else if !tool_unavailable("python3") {\n'
            '        "python3"\n'
            '    } else {\n'
            '        "python"\n'
            '    }\n'
            '}\n',
        )
        self.assertEqual(findings, [])

    def test_internal_test_script_sentinel_is_not_an_executable(self):
        findings = scan_snippet(
            "cli/src/commands/test/test.rs",
            'fn detect_test_script() {\n'
            '    Some(("mgc-internal-run-script".to_string(), vec![]));\n'
            '}\n',
        )
        self.assertEqual(findings, [])

    def test_multiline_process_tool_argument_is_still_found(self):
        findings = scan_snippet(
            "adapters/lib/src/install/mod.rs",
            'fn install_deps() -> Result<()> {\n'
            '    mgc_exec::run::run(\n'
            '        "cargo",\n'
            '        &args,\n'
            '        &opts,\n'
            '    )?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["tool"], "cargo")
        self.assertEqual(findings[0]["status"], "violation")
        self.assertEqual(findings[0]["line"], 3)

    def test_production_dev_command_cannot_delegate_to_external_runtime(self):
        findings = scan_snippet(
            "cli/src/commands/core/install/app.rs",
            'fn dev_command() {\n'
            '    let command = InstallCommand { tool: "flutter".to_string() };\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["op_class"], "dev")
        self.assertEqual(findings[0]["status"], "review-required")
        self.assertTrue(findings[0]["blocking"])

    def test_production_build_command_cannot_delegate_to_build_system(self):
        findings = scan_snippet(
            "cli/src/commands/build/build.rs",
            'fn build_app() -> Result<()> {\n'
            '    std::process::Command::new("gradle").status()?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["op_class"], "build")
        self.assertEqual(findings[0]["status"], "violation")
        self.assertEqual(findings[0]["evidence_kind"], "spawn_or_wrapper_call")

    def test_unlisted_executable_literal_in_build_is_still_a_blocker(self):
        findings = scan_snippet(
            "cli/src/commands/build/build.rs",
            'fn build_app() -> Result<()> {\n'
            '    std::process::Command::new("ninja").status()?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["tool"], "ninja")
        self.assertEqual(findings[0]["op_class"], "build")
        self.assertEqual(findings[0]["status"], "violation")
        self.assertEqual(findings[0]["evidence_kind"], "spawn_or_wrapper_call")

    def test_unlisted_tool_descriptor_in_build_is_still_a_blocker(self):
        findings = scan_snippet(
            "cli/src/commands/build/build.rs",
            'fn build_app() {\n'
            '    let command = BuildCommand { tool: "ninja".to_string() };\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["tool"], "ninja")
        self.assertEqual(findings[0]["evidence_kind"], "command_descriptor")
        self.assertEqual(findings[0]["status"], "review-required")
        self.assertTrue(findings[0]["blocking"])

    def test_program_comparison_without_process_call_is_not_a_spawn(self):
        findings = scan_snippet(
            "cli/src/commands/core/web/web.rs",
            'fn native_install_env(project_root: &Path, program: &str) {\n'
            '    if program == "composer" { configure_cache(project_root); }\n'
            '    if program == "go" { configure_cache(project_root); }\n'
            '}\n',
        )
        self.assertEqual(findings, [])

    def test_command_selection_branch_is_not_a_second_spawn(self):
        findings = scan_snippet(
            "cli/src/commands/dev.rs",
            'fn run(cmd: &str) {\n'
            '    if cmd == "godot" { show_tool_error(); }\n'
            '}\n',
        )
        self.assertEqual(findings, [])

    def test_process_helper_declaration_is_not_a_spawn(self):
        findings = scan_snippet(
            "adapters/iot/src/flash/mod.rs",
            'fn exec_tool(tool: &str, args: &[&str]) -> Result<()> {\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(findings, [])

    def test_dynamic_exec_tool_is_a_blocker(self):
        findings = scan_snippet(
            "adapters/cloud/src/deploy/mod.rs",
            'fn deploy_with_tool(program: &str) -> Result<()> {\n'
            '    exec_tool(program, &args, root)?;\n'
            '    Ok(())\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["tool"], "<dynamic:program>")
        self.assertEqual(findings[0]["status"], "violation")

    def test_missing_tool_error_metadata_never_approves_an_execution_route(self):
        owners = (
            ("ai", "AiBuildError", "cargo"),
            ("iot", "IotBuildError", "cargo"),
            ("cloud", "CloudBuildError", "pulumi"),
            ("cloud", "CloudBuildError", "terraform"),
            ("lib", "LibBuildError", "dotnet"),
        )
        for core, owner, tool in owners:
            path = f"cli/src/commands/build/build/{core}/{core}.rs"
            body = (f"fn build_{core}() {{\n"
                    f'return Err({owner}::ToolchainMissing {{ tool: "{tool}".to_string() }}.into());\n'
                    "}\n")
            with self.subTest(core=core, tool=tool):
                self.assertEqual(scan_snippet(path, body), [])
                process = body.replace("return Err", f'Command::new("{tool}").arg("install").status();\nreturn Err')
                findings = scan_snippet(path, process)
                self.assertEqual(len(findings), 1)
                self.assertEqual(findings[0]["status"], "violation")
                self.assertEqual(findings[0]["evidence_kind"], "spawn_or_wrapper_call")
                self.assertTrue(scan_snippet(path, body.replace(owner, "UnreviewedError")))
                self.assertTrue(scan_snippet("cli/src/commands/unreviewed.rs", body))
                self.assertTrue(scan_snippet(path, body.replace(' }.into()', ', other: true }.into()')))

__all__ = ['NewToolDetectionPart1']
