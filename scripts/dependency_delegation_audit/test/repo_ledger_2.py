"""RepoLedgerContract test slice 2; method bodies retained verbatim."""

from .support import *

class RepoLedgerContractPart2(unittest.TestCase):
    def test_locked_build_and_device_control_executor_routes_are_exactly_scoped(self):
        repo_root = REPO_ROOT
        cases = (
            (
                "cli/src/commands/build/build/lib/lib.rs",
                "build_lib",
                "node",
                (
                    "ExecutionScope::BuildRunner",
                    'run_inherited("node"',
                    'node_bin_args(root, "tsc"',
                ),
            ),
            (
                "cli/src/commands/build/build/cloud/cloud.rs",
                "build_cloud",
                "node",
                (
                    "ExecutionScope::BuildRunner",
                    'run_inherited("node"',
                    'node_bin_args(root, "cdk"',
                ),
            ),
            (
                "cli/src/commands/build/web_engine/rust_build/rust_build.rs",
                "build_rust_with_env",
                "cargo",
                ("ExecutionScope::BuildRunner", '"--locked"', '"--offline"'),
            ),
            (
                "cli/src/commands/core/dev/app/ios_simulator/ios_simulator.rs",
                "run_xcrun",
                "xcrun",
                ("ExecutionScope::DeviceControl", 'run("xcrun"'),
            ),
            (
                "cli/src/commands/core/dev/app/android_device/android_device.rs",
                "run_android_device_command",
                "adb",
                ("ExecutionScope::DeviceControl", 'run("adb"'),
            ),
        )
        for rel_path, function, tool, required_tokens in cases:
            with self.subTest(function=function, tool=tool):
                abs_path = os.path.join(repo_root, rel_path)
                with open(abs_path, "r", encoding="utf-8") as handle:
                    source = handle.read()
                start = source.index(f"fn {function}(")
                end = source.index("\n}", start) + 2
                function_body = source[start:end]
                self.assertTrue(
                    all(token in function_body for token in required_tokens),
                    f"{rel_path}:{function} must enforce its executor boundary",
                )
                findings = gate.scan_file(rel_path, abs_path)
                route = (rel_path, function, tool)
                route_findings = [
                    item for item in findings
                    if (item["function"], item["tool"]) == (function, tool)
                ]
                self.assertIn(route, gate.AUDITED_EXECUTOR_ROUTES)
                self.assertEqual(len(route_findings), 1)
                self.assertEqual(route_findings[0]["status"], "allowed")
                self.assertEqual(route_findings[0]["op_class"], "audited-executor-route")
                self.assertEqual(
                    route_findings[0]["classification_reason"],
                    gate.AUDITED_EXECUTOR_ROUTES[route],
                )

    def test_web_lifecycle_executor_is_scoped_to_policy_approved_install(self):
        repo_root = REPO_ROOT
        rel_path = "adapters/web/src/lifecycle/script_execution.rs"
        abs_path = os.path.join(repo_root, rel_path)
        with open(abs_path, "r", encoding="utf-8") as handle:
            lifecycle = handle.read()
        with open(os.path.join(repo_root, "adapters/web/src/lifecycle/lifecycle.rs")) as owner:
            lifecycle += "\n" + owner.read()
        route = (rel_path, "run_script", "<dynamic:invocation>")
        findings = gate.scan_file(rel_path, abs_path)
        route_findings = [
            item for item in findings
            if (item["function"], item["tool"]) == ("run_script", "<dynamic:invocation>")
        ]
        self.assertIn(route, gate.AUDITED_EXECUTOR_ROUTES)
        self.assertEqual(len(route_findings), 1)
        self.assertEqual(route_findings[0]["status"], "allowed")
        self.assertEqual(route_findings[0]["op_class"], "audited-executor-route")
        self.assertIn("parse_script_invocation(script)", lifecycle)
        self.assertIn("ExecutionScope::Install", lifecycle)
        self.assertIn("#[cfg(test)]\n    pub(crate) fn run_scripts(", lifecycle)

        install_path = os.path.join(repo_root, "adapters/web/src/install/run_install/lifecycle/lifecycle.rs")
        with open(install_path, "r", encoding="utf-8") as handle:
            install = handle.read()
        policy_position = install.index("match decide_lifecycle_scripts(")
        runner_position = install.index("LifecycleRunner::run_scripts_with_snapshot(")
        self.assertLess(install.index("load_trust_policies(&state.layout)"), policy_position)
        self.assertLess(policy_position, runner_position)

        production_callers = []
        for root, _, filenames in os.walk(os.path.join(repo_root, "adapters/web/src")):
            for filename in filenames:
                if not filename.endswith(".rs") or "/test/" in (root + "/"):
                    continue
                path = os.path.join(root, filename)
                with open(path, "r", encoding="utf-8") as handle:
                    if "LifecycleRunner::run_scripts_with_snapshot(" in handle.read():
                        production_callers.append(os.path.relpath(path, repo_root).replace(os.sep, "/"))
        self.assertEqual(production_callers, ["adapters/web/src/install/run_install/lifecycle/lifecycle.rs"])

    def test_publish_lifecycle_executor_is_scoped_to_explicit_opt_in(self):
        repo_root = REPO_ROOT
        rel_path = "cli/src/commands/publish/lifecycle/lifecycle.rs"
        abs_path = os.path.join(repo_root, rel_path)
        with open(abs_path, "r", encoding="utf-8") as handle:
            source = handle.read()
        with open(os.path.join(repo_root, "cli/src/commands/publish/project_publish/project_publish.rs")) as owner:
            source += "\n" + owner.read()

        route = (rel_path, "run_lifecycle", "<dynamic:invocation>")
        route_findings = [
            item for item in gate.scan_file(rel_path, abs_path)
            if (item["function"], item["tool"]) == ("run_lifecycle", "<dynamic:invocation>")
        ]
        self.assertIn(route, gate.AUDITED_EXECUTOR_ROUTES)
        self.assertEqual(len(route_findings), 1)
        self.assertEqual(route_findings[0]["status"], "allowed")
        self.assertEqual(route_findings[0]["op_class"], "audited-executor-route")

        function_start = source.index("fn run_lifecycle(")
        function_end = source.index("\n}\n", function_start) + 2
        runner = source[function_start:function_end]
        self.assertIn("reject_forbidden_pm_script(cmd)", runner)
        self.assertIn("parse_script_invocation(cmd)", runner)
        self.assertIn("ExecutionScope::Install", runner)
        self.assertIn("cwd: Some(project_root.to_path_buf())", runner)

        publish_start = source.index("async fn publish_project(")
        publish_end = source.index("\n}\n", publish_start) + 2
        publish = source[publish_start:publish_end]
        gate_position = publish.index("publish_lifecycle_decision(")
        for hook in ("prepublishOnly", "prepublish", "prepare"):
            hook_position = publish.index(f'run_lifecycle(&pkg_json, "{hook}"')
            self.assertLess(gate_position, hook_position)

    def test_publish_git_checks_are_closed_and_project_scoped(self):
        repo_root = REPO_ROOT
        rel_path = "cli/src/commands/publish/git_checks/git_checks.rs"
        abs_path = os.path.join(repo_root, rel_path)
        with open(abs_path, "r", encoding="utf-8") as handle:
            source = handle.read()
        for dependency in ("project_publish/project_publish.rs", "git_checks/types.rs"):
            with open(os.path.join(repo_root, "cli/src/commands/publish", dependency)) as owner:
                source += "\n" + owner.read()

        route = (rel_path, "run_git_capture", "git")
        findings = [
            item for item in gate.scan_file(rel_path, abs_path)
            if (item["function"], item["tool"]) == ("run_git_capture", "git")
        ]
        self.assertIn(route, gate.AUDITED_EXECUTOR_ROUTES)
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0]["status"], "allowed")
        self.assertEqual(findings[0]["op_class"], "audited-executor-route")

        publish_start = source.index("async fn publish_project(")
        publish_end = source.index("\n}\n", publish_start) + 2
        publish = source[publish_start:publish_end]
        git_checks_start = source.index("fn git_checks(")
        git_checks_end = source.index("\n}\n", git_checks_start) + 2
        git_checks = source[git_checks_start:git_checks_end]
        self.assertIn(
            "git_checks(args.publish_branch.as_deref(), project_root)",
            publish,
        )
        self.assertIn("GitReadQuery::ValidateBranch", git_checks)
        self.assertIn("publish_branch.is_some()", git_checks)
        self.assertIn("GitReadQuery::CommitsBehind(branch.clone())", git_checks)

        query_start = source.index("enum GitReadQuery")
        query_end = source.index("impl GitReadQuery", query_start)
        queries = source[query_start:query_end]
        for variant in (
            "WorkingTreeStatus",
            "CurrentBranch",
            "ValidateBranch",
            "UpstreamRef",
            "CommitsBehind",
        ):
            self.assertIn(variant, queries)
        argv_start = query_end
        argv_end = source.index("\n}\n", argv_start) + 2
        self.assertIn("--end-of-options", source[argv_start:argv_end])

        runner_start = source.index("fn run_git_capture(")
        runner_end = source.index("\n}\n", runner_start) + 2
        runner = source[runner_start:runner_end]
        self.assertIn("git_exec_options(project_root)", runner)
        self.assertIn('run("git", &args, &opts)', runner)
        self.assertNotIn('Command::new("git")', source)

    def test_workflow_blindspot_distinguishes_pinned_refs_from_unreviewed_commands(self):
        workflow = next(
            item for item in gate.UNSCANNED_PROCESS_SURFACES
            if item["path"] == ".github/workflows"
        )
        self.assertIn("immutable action-reference syntax has a dedicated CI gate", workflow["reason"])
        self.assertIn("command provisioning", workflow["reason"])
        self.assertIn("upstream SHA provenance", workflow["reason"])

    def test_shell_startup_environment_blindspot_is_explicit(self):
        startup = next(
            item for item in gate.UNSCANNED_PROCESS_SURFACES
            if item["path"] == "shell interpreter startup environment"
        )
        self.assertIn("before an entrypoint script starts", startup["reason"])
        self.assertIn("invoking runner must sanitize", startup["reason"])
        self.assertIn("only child launches", startup["reason"])

    def test_indirect_javascript_blindspot_remains_explicit(self):
        javascript = next(
            item for item in gate.UNSCANNED_PROCESS_SURFACES
            if "JavaScript" in item["path"]
        )
        self.assertIn("arbitrary alias/data-flow", javascript["reason"])
        self.assertIn("template-literal expressions", javascript["reason"])
        self.assertIn("reflective or module-loader wrappers", javascript["reason"])
        self.assertIn("common built-in process aliases", javascript["reason"])
        self.assertIn("user-defined aliases/data-flow", javascript["reason"])


__all__ = ['RepoLedgerContractPart2']
