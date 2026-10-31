"""RepoLedgerContract test slice 3; method bodies retained verbatim."""

from .support import *

class RepoLedgerContractPart3(unittest.TestCase):
    def test_repo_ledger_is_green_after_all_production_routes_are_reviewed(self):
        code = gate.main()
        self.assertEqual(
            code,
            0,
            "all production process spawns must be delegated or narrowly reviewed",
        )
        with open(gate.OUTPUT_PATH, "r", encoding="utf-8") as handle:
            ledger = json.load(handle)
        findings = ledger["findings"]
        self.assertEqual(ledger["schema"], "dependency-delegation-audit/11")
        self.assertEqual(ledger["summary"]["review_required"], 0)
        self.assertEqual(ledger["summary"]["violation"], 0)
        self.assertEqual(ledger["summary"]["blocking"], 0)
        self.assertEqual(
            ledger["scan_languages"],
            [
                "Rust process boundaries",
                "Python stdlib process APIs",
                "Shell command/process surfaces (static inventory)",
                "JavaScript/TypeScript process APIs (lexical inventory)",
                "PowerShell process APIs (lexical inventory)",
            ],
        )
        self.assertEqual(ledger["python_scan_roots"], gate.PYTHON_SCAN_ROOTS)
        self.assertGreater(ledger["python_files_scanned"], 0)
        self.assertFalse(ledger["coverage_complete"])
        self.assertTrue(ledger["unscanned_process_surfaces"])
        self.assertTrue(ledger["python_process_calls"])
        self.assertEqual(
            ledger["python_process_unreviewed"],
            0,
            "all Python process routes must match an exact reviewed AST record",
        )
        self.assertEqual(
            ledger["python_process_reviewed"], len(ledger["python_process_calls"])
        )
        self.assertEqual(ledger["python_process_helper_unreviewed"], 0)
        self.assertEqual(ledger["python_process_helper_reviewed"], 1)
        self.assertEqual(
            [item["function"] for item in ledger["python_process_helper_reviews"]],
            ["bootstrap_bash_command"],
        )
        self.assertEqual(ledger["shell_scan_roots"], gate.SHELL_SCAN_ROOTS)
        self.assertGreater(ledger["shell_files_scanned"], 0)
        self.assertTrue(ledger["shell_process_calls"])
        self.assertGreater(ledger["javascript_files_scanned"], 0)
        self.assertGreater(ledger["powershell_files_scanned"], 0)
        self.assertIsInstance(ledger["javascript_process_calls"], list)
        self.assertIsInstance(ledger["powershell_process_calls"], list)
        self.assertTrue(
            all(
                item["review_status"] == "reviewed" and item["review_reason"]
                for item in ledger["python_process_calls"]
            )
        )
        self.assertTrue(
            all(
                item["review_status"] in {"observed", "reviewed"}
                and item["review_reason"]
                for item in ledger["shell_process_calls"]
            )
        )
        # A JavaScript process call is admissible only when it is explicitly
        # reviewed (the npm CLI shim is the one intentional executor boundary);
        # anything unreviewed still blocks.
        # (Cuộc gọi process JS chỉ hợp lệ khi đã review tường minh; cái chưa
        # review vẫn chặn.)
        self.assertTrue(
            all(
                item["review_status"] in {"reviewed", "unreviewed"}
                for item in ledger["javascript_process_calls"]
            )
        )
        self.assertFalse(
            any(
                item["review_status"] == "unreviewed"
                for item in ledger["javascript_process_calls"]
            ),
            "unreviewed javascript process calls remain",
        )
        self.assertTrue(
            all(
                item["review_status"] in {"observed", "reviewed"}
                and item["review_reason"]
                for item in ledger["powershell_process_calls"]
            )
        )
        self.assertIsInstance(ledger["dirty_paths_count"], int)
        self.assertIsInstance(ledger["working_tree_clean"], bool)
        if ledger["working_tree_clean"]:
            self.assertEqual(ledger["dirty_paths_count"], 0)
        else:
            self.assertGreater(ledger["dirty_paths_count"], 0)
        self.assertTrue(
            all(
                item["evidence_kind"]
                in {
                    "command_descriptor",
                    "dynamic_spawn_call",
                    "spawn_or_wrapper_call",
                }
                for item in findings
            )
        )
        self.assertTrue(findings, "current product tree must expose the known debt")
        allowed = [item for item in findings if item["status"] == "allowed"]
        audited_routes = {
            **gate.AUDITED_EXECUTOR_ROUTES,
            **gate.AUDITED_DIRECT_PROCESS_ROUTES,
        }
        self.assertTrue(
            all(
                (
                    any(
                        segment in ("test", "tests", "bench", "benches")
                        for segment in item["file"].split("/")[:-1]
                    )
                    or item["op_class"] == "test-fixture"
                    or (
                        item["file"] == "cli/src/commands/doctor.rs"
                        and (item["function"], item["tool"])
                        in {
                            ("tool_version", "<dynamic:bin>"),
                            ("fs_avail", "df"),
                        }
                    )
                    or (
                        (
                            item["file"],
                            item["function"],
                            item["tool"],
                        ) in audited_routes
                        and item["classification_reason"]
                        == audited_routes[
                            (item["file"], item["function"], item["tool"])
                        ]
                    )
                )
                for item in allowed
            ),
            "only harnesses, test fixtures, exact doctor probes, and exact audited routes may be allowed",
        )
        self.assertTrue(
            any(
                item["file"] == "cli/src/commands/build/build/app/app.rs"
                and item["tool"] == "gradle"
                for item in findings
            ),
            "the Java/Android build delegation must not be hidden by the ledger",
        )
        self.assertFalse(
            any(
                item["file"] == "core/crates/mgc-exec/src/run/run.rs"
                and item["tool"] == "where.exe"
                for item in findings
            ),
            "Windows PATH resolution must stay in-process instead of spawning where.exe",
        )
        self.assertFalse(
            any(
                item["file"] == "core/crates/mgc-exec/src/run/run.rs"
                and item["tool"] == "taskkill"
                for item in findings
            ),
            "production Windows process-tree termination must use its in-process Job Object",
        )
        self.assertTrue(
            any(
                item["file"] == "tools/mgc-dist/src/main.rs"
                and item["tool"] == "cargo"
                for item in findings
            ),
            "the release packaging build invocation must be inventoried",
        )
        self.assertTrue(
            any(
                item["file"] == "tools/mgc-dist/src/main.rs"
                and item["tool"] == "rustc"
                and item["function"] == "detect_host_target"
                for item in findings
            ),
            "the read-only host-target query must be inventoried",
        )


__all__ = ['RepoLedgerContractPart3']
