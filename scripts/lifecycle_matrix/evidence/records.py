"""Platform evidence constants and release scope."""

from __future__ import annotations

from ..paths.paths import SCRIPTS_ROOT

from ..catalog.catalog import (
    LANES,
)
import os

FRAMEWORK_QUALIFICATION_EVIDENCE = []

# ---------------------------------------------------------------------------
# P0-3 platform-evidence counter (Tech Lead 2026-09-16): windows-latest
# runs the SAME lifecycle matrix but is evidence-only — and "experimental"
# must never be permanent. The gitignored counter file (same treatment as
# the matrix JSON) records CONSECUTIVE fully-green matrix runs per OS:
#   {"<os>": {"runs": N, "last_green_sha": "...", "last_green_at": "..."}}
# The CI workflow increments it after every green Windows run
# (--record-green), zeroes it whenever the Windows lane breaks (--reset),
# and the windows-evidence-promotion job fails the workflow once the count
# reaches PLATFORM_EVIDENCE_PROMOTION_THRESHOLD — the self-enforcing
# "flip Windows to required now" tripwire.
# (── Counter evidence platform P0-3 (Tech Lead 2026-09-16):
# windows-latest chạy CÙNG lifecycle matrix nhưng chỉ thu evidence — và
# "experimental" không được thành vĩnh viễn. File counter gitignored (xử
# lý như matrix JSON) ghi số lần chạy matrix XANH TRỌN VẸN LIÊN TIẾP theo
# OS: {"<os>": {"runs": N, "last_green_sha": "...", "last_green_at":
# "..."}}. Workflow CI tăng sau mỗi lần Windows xanh (--record-green), về
# 0 khi lane Windows vỡ (--reset), và job windows-evidence-promotion FAIL
# workflow khi đủ PLATFORM_EVIDENCE_PROMOTION_THRESHOLD — bẫy tự-ép "đổi
# Windows thành required ngay".)
PLATFORM_EVIDENCE_PATH = os.path.join(
    SCRIPTS_ROOT,
    os.pardir, "docs", "specs", "lifecyclePlatformEvidence.json",
)

PLATFORM_EVIDENCE_PROMOTION_THRESHOLD = 3

PLATFORM_EVIDENCE_OS_NAMES = {
    "windows-latest": "Windows",
    "macos-latest": "Darwin",
    "ubuntu-latest": "Linux",
}

# V1.2 promotion scope is the complete declared platform, not the old six-lane
# RC subset. Adding a lane therefore expands the gate automatically.
# (Phạm vi promotion V1.2 là toàn nền tảng đã khai báo, không phải sáu lane
# RC cũ. Thêm lane tự động mở rộng gate.)
PLATFORM_EVIDENCE_RELEASE_SCOPE = frozenset(
    (lane["core"], lane["language"], lane.get("framework_id", ""))
    for lane in LANES
)

__all__ = ['FRAMEWORK_QUALIFICATION_EVIDENCE', 'PLATFORM_EVIDENCE_OS_NAMES', 'PLATFORM_EVIDENCE_PATH', 'PLATFORM_EVIDENCE_PROMOTION_THRESHOLD', 'PLATFORM_EVIDENCE_RELEASE_SCOPE']
