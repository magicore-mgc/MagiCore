"""Persistent platform-green evidence handling."""

from __future__ import annotations

from ..errors import (
    _fail,
)
from ..paths.paths import (
    REPOSITORY_ROOT,
)
from ..process.process import (
    run_text_capture,
)
from ..validation.native_pm import (
    evidence_commit_error,
)
from ..validation.platform import (
    platform_evidence_errors,
)
from .records import (
    PLATFORM_EVIDENCE_OS_NAMES,
    PLATFORM_EVIDENCE_PATH,
    PLATFORM_EVIDENCE_PROMOTION_THRESHOLD,
)
from .source import (
    lifecycle_working_tree_clean,
)
import datetime
import json
import os

def _platform_evidence_load() -> dict:
    """Load the platform-evidence counter; {} when absent or unreadable
    (fresh checkout / first run). A corrupt file reads as {} — never a
    crash, never a fake streak.
    (Đọc counter evidence platform; {} khi chưa có hoặc hỏng (checkout
    mới / lần chạy đầu). File hỏng đọc thành {} — không bao giờ crash,
    không bao giờ giả chuỗi xanh.)"""
    try:
        with open(PLATFORM_EVIDENCE_PATH, "r", encoding="utf-8") as f:
            data = json.load(f)
    except (OSError, ValueError):
        return {}
    return data if isinstance(data, dict) else {}

def _platform_evidence_write(evidence: dict) -> None:
    os.makedirs(os.path.dirname(PLATFORM_EVIDENCE_PATH) or ".", exist_ok=True)
    with open(PLATFORM_EVIDENCE_PATH, "w", encoding="utf-8") as f:
        json.dump(evidence, f, indent=2)
        f.write("\n")

def record_platform_green(os_name: str) -> int:
    """--record-green <os>: increment the CONSECUTIVE green-run counter
    for `os` and stamp the run SHA + UTC time. SHA source:
    MGC_PLATFORM_EVIDENCE_SHA env (CI passes GITHUB_SHA), falling back to
    `git rev-parse HEAD` for local runs.
    (--record-green <os>: tăng counter lần xanh LIÊN TIẾP cho `os` và đóng
    dấu SHA + giờ UTC của lần chạy. Nguồn SHA: env
    MGC_PLATFORM_EVIDENCE_SHA (CI truyền GITHUB_SHA), dự phòng
    `git rev-parse HEAD` cho lần chạy local.)"""
    if os_name not in PLATFORM_EVIDENCE_OS_NAMES:
        _fail(f"refusing to record unknown platform evidence key: {os_name}")
    matrix_path = os.environ.get(
        "MGC_LIFECYCLE_MATRIX_OUT",
        "docs/specs/lifecycleCapabilityMatrix.json",
    )
    try:
        with open(matrix_path, "r", encoding="utf-8") as matrix_file:
            matrix = json.load(matrix_file)
    except (OSError, ValueError) as exc:
        _fail(f"cannot read lifecycle matrix before recording platform green: {exc}")
    checkout = run_text_capture(
        ["git", "rev-parse", "HEAD"],
        capture_output=True,
        text=True,
        cwd=REPOSITORY_ROOT,
        check=False,
    )
    checkout_sha = checkout.stdout.strip() if checkout.returncode == 0 else ""
    current_tree_clean = lifecycle_working_tree_clean()
    expected_platform = PLATFORM_EVIDENCE_OS_NAMES[os_name]
    errors = platform_evidence_errors(
        matrix,
        checkout_sha,
        expected_platform=expected_platform,
        current_tree_clean=current_tree_clean,
    )
    workflow_sha = (os.environ.get("MGC_PLATFORM_EVIDENCE_SHA") or "").strip()
    commit_error = evidence_commit_error(
        checkout_sha,
        workflow_sha or None,
        is_ci=os.environ.get("CI", "").lower() in {"true", "1", "yes"},
        recorded_sha=workflow_sha or None,
        require_recorded=os.environ.get("CI", "").lower() in {"true", "1", "yes"},
    )
    if commit_error:
        errors.append(commit_error)
    if errors:
        _fail(
            f"refusing to record {os_name} as green: lifecycle evidence is not green",
            "\n".join(errors),
        )

    evidence = _platform_evidence_load()
    entry = evidence.get(os_name)
    if not isinstance(entry, dict):
        entry = {}
    entry["runs"] = int(entry.get("runs", 0)) + 1
    sha = workflow_sha or checkout_sha
    entry["last_green_sha"] = sha or "unknown"
    entry["last_green_at"] = datetime.datetime.now(
        datetime.timezone.utc
    ).strftime("%Y-%m-%dT%H:%M:%SZ")
    evidence[os_name] = entry
    _platform_evidence_write(evidence)
    print(
        f"platform evidence: {os_name} consecutive green runs = {entry['runs']} "
        f"(last_green_sha={entry['last_green_sha']}, last_green_at={entry['last_green_at']})"
    )
    if entry["runs"] >= PLATFORM_EVIDENCE_PROMOTION_THRESHOLD:
        print(
            f"platform evidence: {os_name} reached the promotion threshold "
            f"({PLATFORM_EVIDENCE_PROMOTION_THRESHOLD}) — flip it to a required gate now"
        )
    return 0

def reset_platform_counter(os_name: str) -> int:
    """--reset <os>: zero the consecutive-green counter — a failed run
    broke the streak and an honest counter must show it. The last_green_*
    fields stay: they are historical facts, not claims.
    (--reset <os>: về 0 counter lần xanh liên tiếp — lần chạy fail đã phá
    chuỗi và counter trung thực phải thể hiện điều đó. Trường last_green_*
    giữ nguyên: đó là sự thật lịch sử, không phải claim.)"""
    evidence = _platform_evidence_load()
    entry = evidence.get(os_name)
    if not isinstance(entry, dict):
        entry = {}
    entry["runs"] = 0
    evidence[os_name] = entry
    _platform_evidence_write(evidence)
    print(f"platform evidence: {os_name} consecutive-green counter reset to 0 (streak broken)")
    return 0

__all__ = ['_platform_evidence_load', '_platform_evidence_write', 'record_platform_green', 'reset_platform_counter']
