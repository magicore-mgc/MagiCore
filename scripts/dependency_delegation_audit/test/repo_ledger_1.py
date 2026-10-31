"""RepoLedgerContract test slice 1; method bodies retained verbatim."""

from .support import *


def function_body(source, signature):
    """Return the body text of the first function starting at `signature`."""
    # Helper holds no assertion; callers assert on the returned slice.
    # (Helper không assert; caller assert trên đoạn code trả về.)
    start = source.index(signature)
    return source[start:source.index("\n}\n", start) + 2]


class RepoLedgerContractPart1(unittest.TestCase):
    def test_global_tool_self_install_route_is_fixed_and_scoped(self):
        repo_root = REPO_ROOT
        rel_path = "cli/src/commands/tool/native_install/native_install.rs"
        abs_path = os.path.join(repo_root, rel_path)
        with open(abs_path, "r", encoding="utf-8") as handle:
            source = handle.read()

        route = (rel_path, "install_with_project_credentials", "run_native_install")
        findings = [
            item for item in gate.scan_file(rel_path, abs_path)
            if (item["file"], item["function"], item["tool"]) == route
        ]
        self.assertIn(route, gate.AUDITED_EXECUTOR_ROUTES)
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["status"], "allowed")
        self.assertEqual(findings[0]["op_class"], "audited-executor-route")

        start = source.index("fn run_native_install(")
        end = source.index("\n}\n", start) + 2
        runner = source[start:end]
        for token in (
            "std::env::current_exe()",
            "registry_install_args(allow_scripts)",
            # Native install still runs in the install scope: it is the package
            # manager path, not project code (SEC-02 scoped only project hooks).
            # (Install native vẫn chạy trong install scope: đó là đường package
            # manager, không phải code project — SEC-02 chỉ scope hook project.)
            "ExecutionScope::Install",
            "run_project_binary_inherited(&executable, &args, &options)",
        ):
            self.assertIn(token, runner)
        self.assertIn("let install = run_native_install(project_root, allow_scripts);", source)

        dynamic = scan_snippet(
            "cli/src/commands/core/web/web.rs",
            'fn install_member(root: &Path) -> Result<()> {\n'
            '    run_native_install(root, program, &args)\n'
            '}\n',
        )
        self.assertEqual(dynamic[0]["status"], "violation")

    def test_windows_user_path_process_is_fixed_and_narrow(self):
        repo_root = REPO_ROOT
        rel_path = "core/crates/mgc-platform/src/paths/shell/windows/windows.rs"
        abs_path = os.path.join(repo_root, rel_path)
        with open(abs_path, "r", encoding="utf-8") as handle:
            source = handle.read()

        route = (rel_path, "ensure_windows_user_path", "powershell.exe")
        findings = [
            item for item in gate.scan_file(rel_path, abs_path)
            if (item["function"], item["tool"]) == (route[1], route[2])
        ]
        self.assertIn(route, gate.AUDITED_DIRECT_PROCESS_ROUTES)
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["status"], "allowed")
        self.assertEqual(findings[0]["op_class"], "audited-executor-route")

        start = source.index("fn ensure_windows_user_path(")
        end = len(source)
        function_body = source
        self.assertIn("USER_PATH_SCRIPT,", source[start:end])
        for token in (
            'const USER_PATH_SCRIPT: &str = r#"',
            '"-NoProfile"',
            '"-NonInteractive"',
            '"-Command"',
            '.env("MGC_TOOL_BIN_PATH", bin)',
            "[Environment]::GetEnvironmentVariable('Path', 'User')",
            "[Environment]::SetEnvironmentVariable('Path',",
        ):
            self.assertIn(token, function_body)
        for forbidden in ("Invoke-Expression", "Start-Process", "-ExecutionPolicy Bypass"):
            self.assertNotIn(forbidden, function_body)

        unreviewed = scan_snippet(
            rel_path,
            'fn arbitrary_powershell(command: &str) {\n'
            '    std::process::Command::new("powershell.exe")\n'
            '        .arg("-Command")\n'
            '        .arg(command)\n'
            '        .status();\n'
            '}\n',
        )
        self.assertEqual(unreviewed[0]["status"], "violation")

    def test_local_readiness_helpers_have_exact_bounded_process_reviews(self):
        expected = {
            (
                "docs/specs/productReadinessAudit20261012/build-cache.py",
                "subprocess.run",
                "8751fdaf374dc887f5b90b093179398b5e2e99cc5faf1581ba92450cbb033d3e",
            ),
            (
                "docs/specs/productReadinessAudit20261012/probes.py",
                "subprocess.run",
                "e69c692857603200253248ee1653689d02fa04698850b7e932a6b04f071b8422",
            ),
        }
        actual = {
            (path, api, fingerprint)
            for path, reviews in gate.PYTHON_PROCESS_REVIEWS.items()
            for api, fingerprint in reviews
            if path.startswith("docs/specs/productReadinessAudit20261012/")
        }
        self.assertEqual(actual, expected)

    def test_windows_batch_interpreter_has_an_exact_guarded_process_route(self):
        repo_root = REPO_ROOT
        rel_path = "core/crates/mgc-exec/src/run/spawn/windows_command/windows_command.rs"
        abs_path = os.path.join(repo_root, rel_path)
        with open(abs_path, "r", encoding="utf-8") as handle:
            source = handle.read()
        function_start = source.index("fn windows_batch_command(")
        function_end = source.index("\n}\n", function_start) + 2
        function_body = source[function_start:function_end]
        route = (
            rel_path,
            "windows_batch_command",
            "<dynamic:windows_system_tool_path>",
        )
        findings = [
            item for item in gate.scan_file(rel_path, abs_path)
            if (item["function"], item["tool"]) == (route[1], route[2])
        ]
        self.assertIn(route, gate.AUDITED_DIRECT_PROCESS_ROUTES)
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["status"], "allowed")
        self.assertEqual(findings[0]["op_class"], "audited-executor-route")
        self.assertIn('windows_system_tool_path("cmd.exe")', function_body)
        self.assertLess(
            function_body.index("validate_windows_batch_invocation(&cmd_path, args)"),
            function_body.index('Command::new(windows_system_tool_path("cmd.exe")?)'),
        )

    def test_docker_publish_async_route_keeps_its_security_constraints(self):
        repo_root = REPO_ROOT
        rel_path = "cli/src/commands/publish/trusted_oci_image/trusted_oci_image.rs"
        abs_path = os.path.join(repo_root, rel_path)
        with open(abs_path, "r", encoding="utf-8") as handle:
            source = handle.read()
        function_start = source.index("async fn docker_command(")
        function_end = source.index("\n}\n", function_start) + 2
        function_body = source[function_start:function_end]
        route = (rel_path, "docker_command", "docker")
        findings = [
            item for item in gate.scan_file(rel_path, abs_path)
            if (item["function"], item["tool"]) == (route[1], route[2])
        ]
        self.assertIn(route, gate.AUDITED_DIRECT_PROCESS_ROUTES)
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["status"], "allowed")
        for token in (
            'tokio::process::Command::new("docker")',
            '.args(args)',
            '.env("DOCKER_CONFIG", docker_config)',
            'Stdio::piped()',
            'stdin.write_all(secret.as_bytes()).await?',
            'tokio::time::timeout(timeout, child.wait())',
            '.kill_on_drop(true)',
        ):
            self.assertIn(token, function_body)

    def test_managed_rust_build_has_reviewed_executor_boundary(self):
        # Bind the reviewed route to its source/preflight invariants.
        # Khóa route đã review với source và preflight hiện hành.
        #
        # The managed store is a fallback: only the offline mode may replace
        # crates-io with the directory source, otherwise Cargo can never fetch.
        # (Store managed là fallback: chỉ chế độ offline được thay crates-io bằng
        # directory source, nếu không Cargo không bao giờ tải được.)
        repo_root = REPO_ROOT
        rel = "cli/src/commands/build/web_engine/rust_build/rust_build.rs"
        with open(os.path.join(repo_root, rel), encoding="utf-8") as handle:
            source = handle.read()
        body = function_body(source, "pub(crate) fn build_rust_with_managed_source(")
        for token in ("managed_source_config(fetch, source_path)",
                      "cargo_build_args(fetch, &base, has_lock(root))", 'ExecutionScope::BuildRunner',
                      'mgc_exec::prelude::run("cargo", &args, &opts)'):
            self.assertIn(token, body)
        for token in ('"--locked"', '"--offline"'):
            self.assertIn(token, function_body(source, "pub(crate) fn cargo_build_args("))
        config = function_body(source, "pub(crate) fn managed_source_config(")
        self.assertIn('"source.{MANAGED_SOURCE_NAME}.directory={}"', config)
        self.assertIn("if fetch.is_offline() {", config)
        self.assertIn('source.crates-io.replace-with=', config)
        config_start = config.index("if fetch.is_offline() {")
        self.assertLess(
            config_start,
            config.index("source.crates-io.replace-with="),
            "the crates-io replacement must stay inside the offline branch",
        )
        with open(os.path.join(repo_root, "cli/src/commands/build/build/lib/lib.rs"), encoding="utf-8") as handle:
            caller = handle.read()
        caller = function_body(caller, "async fn build_lib(")
        freshness = caller.index("check_lib_cargo_freshness(root)?")
        verify = caller.index("cargo_compat::validate_build_for_core_with_fetch(")
        launch = caller.index("finish_managed_rust_build(root,")
        self.assertLess(freshness, verify)
        self.assertLess(verify, launch)
        findings = [row for row in gate.scan_file(rel, os.path.join(repo_root, rel))
                    if row["function"] == "build_rust_with_managed_source"]
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["status"], "allowed")
        direct = scan_snippet(rel, 'fn build_rust_with_managed_source() {\n'
                              'std::process::Command::new("cargo").status();\n}\n')
        self.assertEqual(direct[0]["status"], "violation")

    def test_direct_spawn_in_a_delegated_function_does_not_inherit_its_route(self):
        findings = scan_snippet(
            "cli/src/commands/build/web_engine.rs",
            'fn build_rust_with_env() {\n'
            '    std::process::Command::new("cargo").status();\n'
            '}\n',
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["tool"], "cargo")
        self.assertEqual(findings[0]["status"], "violation")
        self.assertEqual(findings[0]["op_class"], "build")

    def test_mgc_dist_routes_cargo_and_rustc_through_scoped_executor(self):
        repo_root = REPO_ROOT
        rel_path = "tools/mgc-dist/src/main.rs"
        abs_path = os.path.join(repo_root, rel_path)
        with open(abs_path, "r", encoding="utf-8") as handle:
            source = handle.read()
        self.assertNotIn("std::process::Command", source)
        self.assertIn('mgc_exec::run::run_inherited("cargo"', source)
        self.assertIn('mgc_exec::run::run("rustc"', source)
        self.assertIn('"--locked"', source)
        self.assertIn('"--offline"', source)
        findings = gate.scan_file(rel_path, abs_path)
        tool_findings = [item for item in findings if item["tool"] in {"cargo", "rustc"}]
        self.assertEqual({item["tool"] for item in tool_findings}, {"cargo", "rustc"})
        self.assertTrue(
            all(
                item["status"] == "allowed"
                and item["op_class"] == "audited-executor-route"
                and item["classification_reason"]
                for item in tool_findings
            ),
            f"mgc-dist routes must stay visible with an explicit executor classification: {tool_findings}",
        )


    def test_command_builder_stays_behind_executor_preflight_and_isolation(self):
        root = os.path.join(REPO_ROOT, "core/crates/mgc-exec/src/run/spawn")
        with open(os.path.join(root, "spawn.rs")) as handle:
            caller = handle.read()
        with open(os.path.join(root, "command_builder/command_builder.rs")) as handle:
            builder = handle.read()
        self.assertIn("pub(in crate::run::implementation::spawn::implementation) fn build_command(", builder)
        self.assertNotIn(".spawn(", builder)
        prepare = caller.index("preflight::prepare(cmd, args, opts, cancel_flag)?")
        complete = caller.index("preflight::complete(")
        construct = caller.index("command_builder::build_command(cmd, args, &cwd, opts, mode)?")
        isolate = caller.index("configure_process_isolation(&mut command)?")
        spawn = caller.index(".spawn()", caller.index("let mut child = command"))
        self.assertLess(prepare, complete)
        self.assertLess(complete, construct)
        self.assertLess(construct, isolate)
        self.assertLess(isolate, spawn)
        # No additional production caller may bypass this checked owner.
        # Caller production bổ sung không được bypass owner đã kiểm này.
        import re
        callers = []
        for directory, _, names in os.walk(root):
            if "test" in os.path.relpath(directory, root).split(os.sep):
                continue
            for name in names:
                if name.endswith(".rs"):
                    path = os.path.join(directory, name)
                    with open(path) as handle:
                        source = handle.read()
                    calls = re.findall(r"\bbuild_command\s*\(", source)
                    if calls:
                        callers.append((os.path.relpath(path, root), len(calls)))
        self.assertEqual(sorted(callers), [("command_builder/command_builder.rs", 1), ("spawn.rs", 1)])

__all__ = ['RepoLedgerContractPart1']
