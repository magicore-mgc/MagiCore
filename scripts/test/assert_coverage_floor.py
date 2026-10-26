#!/usr/bin/env python3
"""Fail when measured line coverage drops below the recorded floor.

A floor, not a target: it cannot be reached by writing tests that do not run, and
it fails the change that removes coverage rather than the one that adds code
nobody has touched yet.

Sàn, không phải mục tiêu: không thể đạt bằng test không chạy, và nó chặn thay
đổi làm mất coverage thay vì thay đổi thêm code chưa ai chạm.

Usage: assert_coverage_floor.py <coverage-report.txt> [floor-percent]
"""

import re
import sys

DEFAULT_FLOOR = 70.0


def main() -> int:
    if len(sys.argv) < 2:
        print("usage: assert_coverage_floor.py <report> [floor]", file=sys.stderr)
        return 2
    path = sys.argv[1]
    floor = float(sys.argv[2]) if len(sys.argv) > 2 else DEFAULT_FLOOR

    try:
        text = open(path, encoding="utf-8", errors="replace").read()
    except OSError as error:
        print(f"cannot read coverage report: {error}", file=sys.stderr)
        return 1

    totals = [line for line in text.splitlines() if line.startswith("TOTAL")]
    if not totals:
        print("coverage report has no TOTAL line; did the run fail?", file=sys.stderr)
        return 1

    match = re.search(r"(\d+\.\d+)%", totals[-1])
    if not match:
        print(f"cannot parse coverage from: {totals[-1]}", file=sys.stderr)
        return 1

    percent = float(match.group(1))
    print(f"line coverage {percent}% (floor {floor}%)")
    if percent < floor:
        print(
            f"coverage fell below the floor: {percent}% < {floor}%. "
            "Add tests for the code this change added",
            file=sys.stderr,
        )
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
