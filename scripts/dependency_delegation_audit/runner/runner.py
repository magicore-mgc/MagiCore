"""Assemble scanner results and enforce the audit ledger verdict."""

from __future__ import annotations
from ..inventory.inventory import _iter_files_with_suffixes, _iter_python_files, _iter_rust_files, _iter_shell_files, scan_python_file, scan_shell_file
from ..javascript.javascript import scan_javascript_text
from ..paths.paths import OUTPUT_PATH
from ..powershell.powershell import scan_powershell_text
from ..python.helper_review import review_python_process_helpers
from ..python.process_calls import review_python_process_calls
from ..rules.common import ALLOWED_OPS, MARKER, SCAN_ROOTS, TOOLS, UNSCANNED_PROCESS_SURFACES
from ..rules.python import PYTHON_PROCESS_HELPER_REVIEWS, PYTHON_SCAN_ROOTS
from ..rules.shell import SHELL_DEPENDENCY_COMMAND_REVIEWS, SHELL_EXACT_DYNAMIC_COMMAND_REVIEWS, SHELL_SCAN_ROOTS
from ..rust.rust import scan_file
from ..supplemental.supplemental import scan_supplemental_file
import datetime
import json
import os
import subprocess
import sys

def run_audit(repo_root: str, check_lane_gate_callback) -> int:
    findings = []
    for abs_path in _iter_rust_files():
        rel_path = os.path.relpath(abs_path, repo_root)
        findings.extend(scan_file(rel_path, abs_path))
    python_files = _iter_python_files()
    python_process_calls = review_python_process_calls([
        item
        for abs_path in python_files
        for item in scan_python_file(abs_path, repo_root)
    ], expected_files={os.path.relpath(path, repo_root) for path in python_files})
    python_process_helper_sources = {}
    for rel_path in PYTHON_PROCESS_HELPER_REVIEWS:
        abs_path = os.path.join(repo_root, *rel_path.split("/"))
        try:
            with open(abs_path, "r", encoding="utf-8") as handle:
                python_process_helper_sources[rel_path] = handle.read()
        except (OSError, UnicodeError):
            continue
    python_process_helper_reviews = review_python_process_helpers(
        python_process_helper_sources,
        expected_files={os.path.relpath(path, repo_root) for path in python_files},
    )
    shell_files = _iter_shell_files()
    powershell_files = _iter_files_with_suffixes((".ps1", ".psm1", ".psd1"))
    scanned_shell_sources = {}
    for path in [*shell_files, *powershell_files]:
        rel_path = os.path.relpath(path, repo_root).replace(os.sep, "/")
        try:
            with open(path, "r", encoding="utf-8") as handle:
                scanned_shell_sources[rel_path] = handle.read()
        except (OSError, UnicodeError):
            # The dedicated file scan emits the blocking read error below.
            # (Lượt quét file riêng sẽ ghi lỗi đọc thành finding chặn.)
            continue
    shell_process_calls = [
        item
        for abs_path in shell_files
        for item in scan_shell_file(abs_path, repo_root, scanned_shell_sources)
    ]
    observed_dependency_reviews = {
        (item["file"], item["command_fingerprint"])
        for item in shell_process_calls
        if item.get("dependency_action") is not None
    }
    for rel_path, fingerprints in SHELL_DEPENDENCY_COMMAND_REVIEWS.items():
        for fingerprint in sorted(fingerprints):
            if (rel_path, fingerprint) not in observed_dependency_reviews:
                shell_process_calls.append({
                    "file": rel_path,
                    "line": 0,
                    "api": "shell-command-review-record",
                    "executable": "<stale-review>",
                    "review_status": "unreviewed",
                    "review_reason": "review fingerprint no longer matches a live dependency command",
                    "dependency_action": None,
                    "command_fingerprint": fingerprint,
                    "parse_error": None,
                })
    observed_exact_dynamic_reviews = {
        (item["file"], item["command_fingerprint"])
        for item in shell_process_calls
        if item.get("review_status") == "reviewed"
        and item.get("command_fingerprint")
        in SHELL_EXACT_DYNAMIC_COMMAND_REVIEWS.get(item["file"], {})
    }
    for rel_path, reviews in SHELL_EXACT_DYNAMIC_COMMAND_REVIEWS.items():
        for fingerprint in sorted(reviews):
            if (rel_path, fingerprint) not in observed_exact_dynamic_reviews:
                shell_process_calls.append({
                    "file": rel_path,
                    "line": 0,
                    "api": "shell-command-review-record",
                    "executable": "<stale-review>",
                    "review_status": "unreviewed",
                    "review_reason": "source-bound dynamic command review no longer matches a live process command",
                    "dependency_action": None,
                    "command_fingerprint": fingerprint,
                    "parse_error": None,
                })
    javascript_files = _iter_files_with_suffixes((".js", ".jsx", ".mjs", ".cjs", ".ts", ".tsx"))
    javascript_process_calls = [
        item
        for abs_path in javascript_files
        for item in scan_supplemental_file(abs_path, repo_root, scan_javascript_text)
    ]
    powershell_process_calls = [
        item
        for abs_path in powershell_files
        for item in scan_supplemental_file(abs_path, repo_root, scan_powershell_text)
    ]

    counts = {"allowed": 0, "violation": 0, "review-required": 0}
    for finding in findings:
        counts[finding["status"]] = counts.get(finding["status"], 0) + 1

    lane_checked, lane_missing = check_lane_gate_callback(repo_root)

    try:
        commit = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=repo_root, capture_output=True, text=True,
        ).stdout.strip()
    except OSError:
        commit = ""

    try:
        status = subprocess.run(
            ["git", "status", "--porcelain", "--untracked-files=all"],
            cwd=repo_root, capture_output=True, text=True, check=True,
        )
        dirty_paths = [line for line in status.stdout.splitlines() if line.strip()]
        working_tree_clean = not dirty_paths
    except (OSError, subprocess.CalledProcessError):
        dirty_paths = []
        working_tree_clean = None

    report = {
        "schema": "dependency-delegation-audit/11",
        "generated_at": datetime.datetime.now(datetime.timezone.utc).strftime(
            "%Y-%m-%dT%H:%M:%SZ"
        ),
        "commit": commit,
        "working_tree_clean": working_tree_clean,
        "dirty_paths_count": (
            len(dirty_paths) if working_tree_clean is not None else None
        ),
        "scan_roots": SCAN_ROOTS,
        "python_scan_roots": PYTHON_SCAN_ROOTS,
        "python_files_scanned": len(python_files),
        "python_process_reviewed": sum(
            item["review_status"] == "reviewed" for item in python_process_calls
        ),
        "python_process_unreviewed": sum(
            item["review_status"] != "reviewed" for item in python_process_calls
        ),
        "python_process_helper_reviewed": sum(
            item["review_status"] == "reviewed"
            for item in python_process_helper_reviews
        ),
        "python_process_helper_unreviewed": sum(
            item["review_status"] != "reviewed"
            for item in python_process_helper_reviews
        ),
        "shell_scan_roots": SHELL_SCAN_ROOTS,
        "shell_files_scanned": len(shell_files),
        "shell_process_unreviewed": sum(
            item["review_status"] == "unreviewed" for item in shell_process_calls
        ),
        "javascript_files_scanned": len(javascript_files),
        "powershell_files_scanned": len(powershell_files),
        "scan_languages": [
            "Rust process boundaries",
            "Python stdlib process APIs",
            "Shell command/process surfaces (static inventory)",
            "JavaScript/TypeScript process APIs (lexical inventory)",
            "PowerShell process APIs (lexical inventory)",
        ],
        "coverage_complete": not UNSCANNED_PROCESS_SURFACES,
        "unscanned_process_surfaces": UNSCANNED_PROCESS_SURFACES,
        "python_process_calls": python_process_calls,
        "python_process_helper_reviews": python_process_helper_reviews,
        "shell_process_calls": shell_process_calls,
        "javascript_process_calls": javascript_process_calls,
        "powershell_process_calls": powershell_process_calls,
        "tools": TOOLS,
        "allowed_ops": list(ALLOWED_OPS),
        "declaration_marker": MARKER,
        "summary": {
            "total": len(findings),
            "allowed": counts["allowed"],
            "violation": counts["violation"],
            "review_required": counts["review-required"],
            "blocking": counts["violation"] + counts["review-required"],
            "declared_delegations": sum(
                1 for finding in findings if finding["declared_delegation"]
            ),
        },
        "lane_gate": {
            "checked": lane_checked,
            "missing": lane_missing,
        },
        "findings": findings,
    }

    out_path = os.path.abspath(os.path.join(repo_root, OUTPUT_PATH))
    os.makedirs(os.path.dirname(out_path), exist_ok=True)
    with open(out_path, "w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)
        handle.write("\n")

    print(
        f"dependency-delegation audit: {len(findings)} finding(s) — "
        f"allowed={counts['allowed']} "
        f"violation={counts['violation']} "
        f"python-process-api={len(python_process_calls)} "
        f"reviewed={sum(item['review_status'] == 'reviewed' for item in python_process_calls)} "
        f"unreviewed={sum(item['review_status'] != 'reviewed' for item in python_process_calls)} "
        f"python-process-helper-review={sum(item['review_status'] == 'reviewed' for item in python_process_helper_reviews)} "
        f"helper-unreviewed={sum(item['review_status'] != 'reviewed' for item in python_process_helper_reviews)} "
        f"shell-process-surfaces={len(shell_process_calls)} "
        f"shell-dynamic={sum(item['review_status'] == 'unreviewed' for item in shell_process_calls)} "
        f"js-process-api={len(javascript_process_calls)} "
        f"powershell-process-api={len(powershell_process_calls)}"
    )
    blocked = [f for f in findings if f.get("blocking", f["status"] == "violation")]
    if blocked:
        print(
            "PLATFORM OWNERSHIP BLOCKED — resolve production process spawns and "
            "unreviewed command routes; `DELEGATED:` does not waive them:",
            file=sys.stderr,
        )
        for finding in blocked:
            declaration = " [documented debt]" if finding["declared_delegation"] else ""
            label = finding["status"]
            print(
                f"  [{label}] {finding['file']}:{finding['line']} "
                f"{finding['tool']} ({finding['function']}){declaration}",
                file=sys.stderr,
            )
        print(f"ledger: {os.path.relpath(out_path, repo_root)}", file=sys.stderr)
        return 1
    if lane_missing:
        print(
            "LANE OWNERSHIP GATE INCOMPLETE — fix scan errors or add "
            "`dep_gate::gate()` / a reasoned `GATE-EXEMPT:`:",
            file=sys.stderr,
        )
        for rel_path in lane_missing:
            label = "coverage-error" if rel_path.startswith("<scan-error:") else "no-gate"
            print(f"  [{label}] {rel_path}", file=sys.stderr)
        print(f"ledger: {os.path.relpath(out_path, repo_root)}", file=sys.stderr)
        return 1
    python_process_unreviewed = [
        item for item in python_process_calls
        if item["review_status"] != "reviewed"
    ]
    if python_process_unreviewed:
        print(
            "PYTHON PROCESS REVIEW REQUIRED — unreviewed or stale exact-call "
            "records remain in the ledger:",
            file=sys.stderr,
        )
        for item in python_process_unreviewed:
            print(
                f"  [{item['review_status']}] {item['file']}:{item['line']} "
                f"{item['api']} {item['executable']}",
                file=sys.stderr,
            )
        print(f"ledger: {os.path.relpath(out_path, repo_root)}", file=sys.stderr)
        return 1
    python_process_helper_unreviewed = [
        item for item in python_process_helper_reviews
        if item["review_status"] != "reviewed"
    ]
    if python_process_helper_unreviewed:
        print(
            "PYTHON PROCESS HELPER REVIEW REQUIRED — executable-selection "
            "helpers changed, missing, or unreadable:",
            file=sys.stderr,
        )
        for item in python_process_helper_unreviewed:
            print(
                f"  [{item['review_status']}] {item['file']}:{item['line']} "
                f"{item['function']}: {item['parse_error']}",
                file=sys.stderr,
            )
        print(f"ledger: {os.path.relpath(out_path, repo_root)}", file=sys.stderr)
        return 1
    shell_process_unreviewed = [
        item for item in shell_process_calls
        if item["review_status"] == "unreviewed"
    ]
    if shell_process_unreviewed:
        print(
            "SHELL PROCESS REVIEW REQUIRED — dynamic/unparsed command heads or "
            "dependency lifecycle commands without an exact source review remain:",
            file=sys.stderr,
        )
        for item in shell_process_unreviewed:
            print(
                f"  [{item['review_status']}] {item['file']}:{item['line']} "
                f"{item['executable']}"
                + (
                    f" {item['dependency_action']}"
                    if item.get("dependency_action")
                    else ""
                ),
                file=sys.stderr,
            )
        print(f"ledger: {os.path.relpath(out_path, repo_root)}", file=sys.stderr)
        return 1
    unreviewed_javascript_calls = [
        item for item in javascript_process_calls
        if item.get("review_status") == "unreviewed"
    ]
    unreviewed_powershell_calls = [
        item for item in powershell_process_calls
        if item.get("review_status") == "unreviewed"
    ]
    if unreviewed_javascript_calls or unreviewed_powershell_calls:
        print(
            "JAVASCRIPT/POWERSHELL PROCESS REVIEW REQUIRED — unreviewed lexical "
            "findings block a clean result:",
            file=sys.stderr,
        )
        print(f"ledger: {os.path.relpath(out_path, repo_root)}", file=sys.stderr)
        return 1
    print(
        f"lane-gate coverage: {lane_checked} lane file(s) reference "
        "dep_gate:: or GATE-EXEMPT:"
    )
    if UNSCANNED_PROCESS_SURFACES:
        print(
            "PROCESS AUDIT COVERAGE LIMITED — direct process boundaries are clean, "
            "but this result is not proof of complete repository-wide coverage:",
            file=sys.stderr,
        )
        for surface in UNSCANNED_PROCESS_SURFACES:
            print(f"  [unscanned] {surface['path']}: {surface['reason']}", file=sys.stderr)
        print(f"coverage details: {os.path.relpath(out_path, repo_root)}", file=sys.stderr)
    print(f"ledger: {os.path.relpath(out_path, repo_root)}")
    return 0


__all__ = ["run_audit"]
